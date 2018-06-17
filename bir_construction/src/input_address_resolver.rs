use super::{Utxo, UtxoCache, UtxoId};
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
    let utxos: Vec<Utxo> = transaction
      .outputs
      .iter()
      .map(|output| Utxo {
        address: self.get_output_address(output),
        value: output.value,
      })
      .collect();

    if utxos.len() == 0 {
      return;
    }

    for (output_index, utxo) in utxos.into_iter().enumerate() {
      let utxo_id = UtxoId {
        tx_hash: transaction.tx_hash.0.clone(),
        output_index: output_index as u32,
      };
      self.utxo_cache.insert(utxo_id, utxo);
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
        let utxo = if input.previous_tx_hash.0 == [0u8; 32] {
          // TODO Use enum to distinguish resolved and unresolved utxos.
          Utxo {
            address: bir::UnresolvedAddress,
            value: 0,
          }
        } else {
          let utxo = self.utxo_cache.remove(&UtxoId {
            tx_hash: input.previous_tx_hash.0.clone(),
            output_index: input.previous_tx_output_index,
          });

          if let Some(utxo) = utxo {
            utxo
          } else {
            // TODO Use enum to distinguish resolved and unresolved utxos.
            Utxo {
              address: bir::UnresolvedAddress,
              value: 0,
            }
          }
        };

        bir::Input {
          address: utxo.address,
          value: utxo.value,
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
      })
      .collect();

    let resolved_transaction = bir::Transaction {
      inputs,
      outputs,
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
