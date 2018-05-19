use super::{BlockHash, BlockHeight, PreviousBlockHash};
use blk_file_reader::Block;
use std::collections::{HashMap, VecDeque};

pub struct OrderedBlock {
  pub block: Block,
  pub height: BlockHeight,
}

pub struct OrderedBlocks<B>
where
  B: Iterator<Item = Block>,
{
  unordered_blocks: B,
  consumed_blocks: HashMap<BlockHash, BlockHeight>,
  unresolved_blocks: HashMap<PreviousBlockHash, Vec<Block>>,
  consumable_blocks: VecDeque<Block>,
}

impl<B> OrderedBlocks<B>
where
  B: Iterator<Item = Block>,
{
  pub fn new(
    mut consumed_blocks: HashMap<BlockHash, BlockHeight>,
    unresolved_blocks: HashMap<PreviousBlockHash, Vec<Block>>,
    unordered_blocks: B,
  ) -> OrderedBlocks<B> {
    // TODO Make configurable.
    if consumed_blocks.is_empty() {
      consumed_blocks.insert([0u8; 32], -1);
    }

    OrderedBlocks {
      unordered_blocks,
      consumed_blocks,
      unresolved_blocks,
      consumable_blocks: VecDeque::new(), /* TODO Should also be part of
                                           * persistable state? */
    }
  }

  fn is_block_consumable(&mut self, block: &Block) -> bool {
    self
      .consumed_blocks
      .contains_key(&block.previous_block_hash.0)
  }

  fn add_unresolved_block(&mut self, block: Block) {
    let unresolved_blocks_entry = self
      .unresolved_blocks
      .entry(block.previous_block_hash.0.clone())
      .or_insert_with(Vec::new);
    (*unresolved_blocks_entry).push(block);
  }

  fn get_previous_block_height(&self, block: &Block) -> BlockHeight {
    *self
      .consumed_blocks
      .get(&block.previous_block_hash.0)
      .unwrap()
  }

  fn consume_block(&mut self, block: Block) -> OrderedBlock {
    let previous_block_height = self.get_previous_block_height(&block);

    let block_height = previous_block_height + 1;
    self
      .consumed_blocks
      .insert(block.hash.0.clone(), block_height);

    let successors = self.unresolved_blocks.remove(&block.hash.0);
    if let Some(successors) = successors {
      let mut successors: VecDeque<Block> = successors.into();
      self.consumable_blocks.append(&mut successors);
    }

    OrderedBlock {
      block,
      height: block_height,
    }
  }
}

impl<B> Iterator for OrderedBlocks<B>
where
  B: Iterator<Item = Block>,
{
  type Item = OrderedBlock;

  fn next(&mut self) -> Option<Self::Item> {
    while self.consumable_blocks.is_empty() {
      if let Some(unordered_block) = self.unordered_blocks.next() {
        if self.is_block_consumable(&unordered_block) {
          self
            .consumable_blocks
            .push_back(unordered_block);
        } else {
          self.add_unresolved_block(unordered_block);
        }
      } else {
        info!(
          "unresolved blocks: {}",
          self.unresolved_blocks.len()
        );
        return None;
      }
    }

    let next_block = self.consumable_blocks.pop_front().unwrap();
    let ordered_block = self.consume_block(next_block);
    Some(ordered_block)
  }
}

#[cfg(test)]
mod test {

  extern crate data_encoding;

  use self::data_encoding::HEXLOWER;
  use super::*;
  use blk_file_reader::Hash;

  fn hash_from_hex(hex: &[u8]) -> Hash {
    let mut buffer = [0u8; 32];
    let hash = HEXLOWER.decode(hex).unwrap();
    buffer.clone_from_slice(hash.as_ref());
    Hash(buffer)
  }

  // TODO Make available for reuse in other tests.
  fn block0_hash() -> Hash {
    hash_from_hex(
      b"000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f",
    )
  }

  // TODO Make available for reuse in other tests.
  fn block1_hash() -> Hash {
    hash_from_hex(
      b"00000000839a8e6886ab5951d76f411475428afc90947ee320161bbf18eb6048",
    )
  }

  // TODO Make available for reuse in other tests.
  fn block2_hash() -> Hash {
    hash_from_hex(
      b"000000006a625f06636b8bb6ac7b960a8d03705d1ace08b1a19da3fdcc99ddbd",
    )
  }

  // TODO Make available for reuse in other tests.
  fn block0() -> Block {
    Block {
      version: 0,
      hash: block0_hash(),
      previous_block_hash: Hash([0u8; 32]),
      merkle_root: Hash([0u8; 32]),
      transactions: Box::new([]),
      bits: 0,
      creation_time: 0,
      nonce: 0,
    }
  }

  // TODO Make available for reuse in other tests.
  fn block1() -> Block {
    Block {
      version: 0,
      hash: block1_hash(),
      previous_block_hash: block0_hash(),
      merkle_root: Hash([0u8; 32]),
      transactions: Box::new([]),
      bits: 0,
      creation_time: 0,
      nonce: 0,
    }
  }

  // TODO Make available for reuse in other tests.
  fn block2() -> Block {
    Block {
      version: 0,
      hash: block2_hash(),
      previous_block_hash: block1_hash(),
      merkle_root: Hash([0u8; 32]),
      transactions: Box::new([]),
      bits: 0,
      creation_time: 0,
      nonce: 0,
    }
  }

  #[test]
  fn iteration_over_empty_blocks_stops_immediately() {
    // Given
    let blocks: Vec<Block> = vec![];

    // When
    let next_block = OrderedBlocks::new(
      HashMap::new(),
      HashMap::new(),
      blocks.into_iter(),
    ).map(|ordered_block| ordered_block.block)
      .next();

    // Then
    assert!(next_block.is_none());
  }

  #[test]
  fn iteration_over_genesis_block_yields_genesis_block() {
    // Given
    let genesis_block = block0();
    let blocks: Vec<Block> = vec![genesis_block.clone()];

    // When
    let next_block = OrderedBlocks::new(
      HashMap::new(),
      HashMap::new(),
      blocks.into_iter(),
    ).map(|ordered_block| ordered_block.block)
      .next();

    // Then
    assert_eq!(next_block, Some(genesis_block));
  }

  #[test]
  fn iteration_over_already_ordered_blocks_maintains_order() {
    // Given
    let block0 = block0();
    let block1 = block1();
    let block2 = block2();
    let blocks = vec![block0.clone(), block1.clone(), block2.clone()];

    // When
    let ordered_blocks: Vec<Block> = OrderedBlocks::new(
      HashMap::new(),
      HashMap::new(),
      blocks.clone().into_iter(),
    ).map(|ordered_block| ordered_block.block)
      .collect();

    // Then
    assert_eq!(ordered_blocks, blocks);
  }

  #[test]
  fn iteration_over_unordered_blocks_establishes_order() {
    // Given
    let block0 = block0();
    let block1 = block1();
    let block2 = block2();
    let blocks = vec![block2.clone(), block1.clone(), block0.clone()];

    // When
    let ordered_blocks: Vec<Block> = OrderedBlocks::new(
      HashMap::new(),
      HashMap::new(),
      blocks.clone().into_iter(),
    ).map(|ordered_block| ordered_block.block)
      .collect();

    // Then
    let expected_blocks = vec![block0, block1, block2];
    assert_eq!(ordered_blocks, expected_blocks);
  }

  #[test]
  fn iteration_over_forks_establishes_order() {
    // Given
    let block0 = block0();
    let block1 = block1();
    let blocks = vec![block1.clone(), block1.clone(), block0.clone()];

    // When
    let ordered_blocks: Vec<Block> = OrderedBlocks::new(
      HashMap::new(),
      HashMap::new(),
      blocks.clone().into_iter(),
    ).map(|ordered_block| ordered_block.block)
      .collect();

    // Then
    let expected_blocks = vec![block0, block1.clone(), block1];
    assert_eq!(ordered_blocks, expected_blocks);
  }
}
