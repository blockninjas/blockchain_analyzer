use super::{UtxoCache, UtxoId};
use address_map::AddressMap;
use bir;
use blk_file_reader;

pub struct InputAddressResolver<A: AddressMap> {
  address_map: A,
  utxo_cache: UtxoCache,
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
    let utxos: Vec<bir::Address> = transaction
      .outputs
      .iter()
      .map(|output| self.get_output_address(output))
      .collect();

    if utxos.len() == 0 {
      return;
    }

    for (output_index, address) in utxos.into_iter().enumerate() {
      let utxo_id = UtxoId {
        tx_hash: transaction.tx_hash.0.clone(),
        output_index: output_index as u32,
      };
      self.utxo_cache.insert(utxo_id, address);
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
        let address: bir::Address = if input.previous_tx_hash.0 == [0u8; 32] {
          bir::UnresolvedAddress
        } else {
          let utxo = self.utxo_cache.remove(&UtxoId {
            tx_hash: input.previous_tx_hash.0.clone(),
            output_index: input.previous_tx_output_index,
          });

          if let Some(address) = utxo {
            address
          } else {
            bir::UnresolvedAddress
          }
        };

        bir::Input {
          address,
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
      .map(|output| bir::Output {
        value: output.value,
        address: self.get_output_address(&output),
        script: output.script.into_vec(),
        index: output.index,
      })
      .collect();

    let resolved_transaction = bir::Transaction {
      inputs,
      outputs,
      lock_time: transaction.lock_time,
      tx_hash: transaction.tx_hash.0,
      version: transaction.version,
    };

    resolved_transaction
  }

  fn get_output_address(
    &mut self,
    output: &blk_file_reader::Output,
  ) -> bir::Address {
    if let Some(ref address) = output.address {
      bir::ResolvedAddress {
        address_id: self.address_map.get_id(&address.base58check),
      }
    } else {
      bir::UnresolvedAddress
    }
  }
}
