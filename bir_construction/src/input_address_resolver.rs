use super::{AddressId, UtxoCache, UtxoId};
use address_map::AddressMap;
use bir;
use blk_file_reader;
use std::collections::HashMap;

const UNKNOWN_ADDRESS: AddressId = 0;

pub struct InputAddressResolver<A: AddressMap> {
  address_map: A,
  utxo_cache: HashMap<UtxoId, AddressId>,
}

impl<A: AddressMap> InputAddressResolver<A> {
  pub fn new(address_map: A, utxo_cache: UtxoCache) -> InputAddressResolver<A> {
    InputAddressResolver {
      address_map,
      utxo_cache,
    }
  }

  pub fn resolve_input_addresses(
    &mut self,
    block: blk_file_reader::Block,
  ) -> bir::Block {
    for transaction in block.transactions.iter() {
      self.record_utxos(transaction);
    }

    let transactions: Vec<bir::Transaction> = block
      .transactions
      .into_vec()
      .into_iter()
      .map(|transaction| self.resolve_transaction(transaction))
      .collect();

    bir::Block {
      hash: block.hash.0,
      bits: block.bits,
      version: block.version,
      creation_time: block.creation_time,
      height: 0,
      merkle_root: block.merkle_root.0,
      nonce: block.nonce,
      previous_block_hash: block.previous_block_hash.0,
      transactions,
    }
  }

  fn record_utxos(&mut self, transaction: &blk_file_reader::Transaction) {
    let utxos: Vec<u64> = transaction
      .outputs
      .iter()
      .map(|output| self.get_output_address_id(output))
      .collect();

    if utxos.len() == 0 {
      return;
    }

    for (output_index, address_id) in utxos.into_iter().enumerate() {
      let utxo_id = UtxoId {
        tx_hash: transaction.tx_hash.0.clone(),
        output_index: output_index as u32,
      };
      self.utxo_cache.insert(utxo_id, address_id);
    }
  }

  fn resolve_transaction(
    &mut self,
    transaction: blk_file_reader::Transaction,
  ) -> bir::Transaction {
    // Resolve inputs.
    let inputs: Vec<bir::Input> = transaction
      .inputs
      .into_vec()
      .into_iter()
      .map(|input| {
        // TODO Handle forks.
        let address_id = if input.previous_tx_hash.0 == [0u8; 32] {
          UNKNOWN_ADDRESS
        } else {
          let utxo = self.utxo_cache.remove(&UtxoId {
            tx_hash: input.previous_tx_hash.0.clone(),
            output_index: input.previous_tx_output_index,
          });
          if let Some(address_id) = utxo {
            address_id
          } else {
            UNKNOWN_ADDRESS
          }
        };

        bir::Input {
          address_id,
          previous_tx_hash: input.previous_tx_hash.0,
          previous_tx_output_index: input.previous_tx_output_index,
          sequence_number: input.sequence_number,
          script: input.script.into_vec(),
        }
      })
      .collect();

    // Map outputs.
    let outputs: Vec<bir::Output> = transaction
      .outputs
      .into_vec()
      .into_iter()
      .map(|output| {
        let address_id = self.get_output_address_id(&output);
        bir::Output {
          value: output.value,
          address_id,
          script: output.script.into_vec(),
          index: output.index,
        }
      })
      .collect();

    let resolved_transaction = bir::Transaction {
      inputs,
      outputs,
      creation_time: transaction.creation_time,
      lock_time: transaction.lock_time,
      tx_hash: transaction.tx_hash.0,
      version: transaction.version,
    };

    resolved_transaction
  }

  fn get_output_address_id(&mut self, output: &blk_file_reader::Output) -> u64 {
    let address_id = if let Some(ref address) = output.address {
      self.address_map.get_id(&address.base58check)
    } else {
      UNKNOWN_ADDRESS
    };

    address_id
  }
}
