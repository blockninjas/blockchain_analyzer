use blk_file_reader::Block;
use std::collections::{HashMap, HashSet, VecDeque};

type BlockHash = [u8; 32];
type PreviousBlockHash = [u8; 32];

pub struct OrderedBlocks<B>
where
  B: Iterator<Item = Block>,
{
  unordered_blocks: B,
  consumed_blocks: HashSet<BlockHash>,
  unresolved_blocks: HashMap<PreviousBlockHash, Vec<Block>>,
  consumable_blocks: VecDeque<Block>,
}

impl<B> OrderedBlocks<B>
where
  B: Iterator<Item = Block>,
{
  pub fn new(unordered_blocks: B) -> OrderedBlocks<B> {
    // TODO Either pre-allocate or remove entries of certain age.
    let mut consumed_blocks = HashSet::new();
    consumed_blocks.insert([0u8; 32]);

    OrderedBlocks {
      unordered_blocks,
      consumed_blocks,
      unresolved_blocks: HashMap::new(),
      consumable_blocks: VecDeque::new(),
    }
  }

  fn is_block_consumable(&mut self, block: &Block) -> bool {
    self.consumed_blocks.contains(&block.previous_block_hash.0)
  }

  fn add_unresolved_block(&mut self, block: Block) {
    let unresolved_blocks_entry = self
      .unresolved_blocks
      .entry(block.previous_block_hash.0.clone())
      .or_insert_with(Vec::new);
    (*unresolved_blocks_entry).push(block);
  }

  fn mark_block_as_consumed(&mut self, block: &Block) {
    self.consumed_blocks.insert(block.hash.0.clone());

    let successors = self.unresolved_blocks.remove(&block.hash.0);
    if let Some(successors) = successors {
      let mut successors: VecDeque<Block> = successors.into();
      self.consumable_blocks.append(&mut successors);
    }
  }
}

impl<B> Iterator for OrderedBlocks<B>
where
  B: Iterator<Item = Block>,
{
  type Item = Block;

  fn next(&mut self) -> Option<Self::Item> {
    while self.consumable_blocks.is_empty() {
      if let Some(unordered_block) = self.unordered_blocks.next() {
        if self.is_block_consumable(&unordered_block) {
          self.consumable_blocks.push_back(unordered_block);
        } else {
          self.add_unresolved_block(unordered_block);
        }
      } else {
        return None;
      }
    }

    let next_block = self.consumable_blocks.pop_front().unwrap();
    self.mark_block_as_consumed(&next_block);
    Some(next_block)
  }
}

pub trait IntoOrderedBlocks<B: Iterator<Item = Block>> {
  fn order_blocks_by_height(self) -> OrderedBlocks<B>;
}

/// This equips every implementor of `IntoIterator<Item=Block>` with a
/// `order_blocks_by_height()` method.
impl<I> IntoOrderedBlocks<I::IntoIter> for I
where
  I: IntoIterator<Item = Block>,
{
  fn order_blocks_by_height(self) -> OrderedBlocks<I::IntoIter> {
    OrderedBlocks::new(self.into_iter())
  }
}

#[cfg(test)]
mod test {

  extern crate data_encoding;

  use super::*;
  use blk_file_reader::Hash;
  use self::data_encoding::HEXLOWER;

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
    let next_block = blocks.order_blocks_by_height().next();

    // Then
    assert!(next_block.is_none());
  }

  #[test]
  fn iteration_over_genesis_block_yields_genesis_block() {
    // Given
    let genesis_block = block0();
    let blocks: Vec<Block> = vec![genesis_block.clone()];

    // When
    let next_block = blocks.order_blocks_by_height().next();

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
    let ordered_blocks: Vec<Block> =
      blocks.clone().order_blocks_by_height().collect();

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
    let ordered_blocks: Vec<Block> = blocks.order_blocks_by_height().collect();

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
    let ordered_blocks: Vec<Block> = blocks.order_blocks_by_height().collect();

    // Then
    let expected_blocks = vec![block0, block1.clone(), block1];
    assert_eq!(ordered_blocks, expected_blocks);
  }
}
