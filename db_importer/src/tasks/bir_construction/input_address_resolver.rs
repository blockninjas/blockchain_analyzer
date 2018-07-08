use super::address_map::AddressMap;
use super::{OrderedBlock, Utxo, UtxoCache, UtxoId};
use bir;
use blk_file_reader;
use db_persistence::schema;
use diesel::{
  BoolExpressionMethods, ExpressionMethods, JoinOnDsl, OptionalExtension,
  PgConnection, QueryDsl, RunQueryDsl,
};

pub struct InputAddressResolver<'a, A>
where
  A: AddressMap,
{
  address_map: A,
  db_connection: PgConnection,
  utxo_cache: &'a mut UtxoCache,
}

impl<'a, A> InputAddressResolver<'a, A>
where
  A: AddressMap,
{
  pub fn new(
    address_map: A,
    db_connection: PgConnection,
    utxo_cache: &'a mut UtxoCache,
  ) -> InputAddressResolver<'a, A> {
    InputAddressResolver {
      address_map,
      db_connection,
      utxo_cache,
    }
  }

  pub fn resolve_input_addresses(
    &mut self,
    ordered_block: OrderedBlock,
  ) -> bir::Block {
    let block = ordered_block.block;
    let height = ordered_block.height as u32;

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
      height,
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
        let utxo = if input.previous_tx_hash.0 == [0u8; 32] {
          // TODO Use enum to distinguish resolved and unresolved utxos.
          Utxo {
            address: bir::UnresolvedAddress,
            value: 0,
          }
        } else {
          let utxo = self.get_utxo(&UtxoId {
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

    let resolved_transaction = bir::Transaction { inputs, outputs };

    resolved_transaction
  }

  fn get_utxo(&mut self, utxo_id: &UtxoId) -> Option<Utxo> {
    if let utxo @ Some(_) = self.utxo_cache.remove(&utxo_id) {
      utxo
    } else if let Some(resolved_output) = load_resolved_output(
      &self.db_connection,
      utxo_id.tx_hash.to_vec(), // TODO Avoid expensive copy
      utxo_id.output_index as i32,
    ) {
      Some(Utxo {
        address: bir::ResolvedAddress {
          address_id: resolved_output.address_id as u64,
        },
        value: resolved_output.value as u64,
      })
    } else {
      None
    }
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

#[derive(Queryable, PartialEq, Eq, Debug)]
pub struct ResolvedOutput {
  pub address_id: i64,
  pub value: i64,
}

fn load_resolved_output(
  db_connection: &PgConnection,
  tx_hash: Vec<u8>,
  output_index: i32,
) -> Option<ResolvedOutput> {
  schema::transactions::dsl::transactions
    .inner_join(
      schema::outputs::dsl::outputs.inner_join(
        schema::output_addresses::dsl::output_addresses.inner_join(
          schema::addresses::dsl::addresses.on(
            schema::addresses::dsl::base58check
              .eq(schema::output_addresses::dsl::base58check),
          ),
        ),
      ),
    )
    .filter(
      schema::transactions::dsl::hash
        // TODO Avoid copy.
        .eq(tx_hash)
        .and(
          schema::outputs::dsl::output_index.eq(output_index),
        ),
    )
    .select((schema::addresses::dsl::id, schema::outputs::dsl::value))
    .first(db_connection)
    .optional()
    .unwrap()
}

#[cfg(test)]
mod load_resolved_output_test {

  use super::*;
  use config;
  use db_persistence::domain::{
    Address, Block, NewAddress, NewBlock, NewOutput, NewOutputAddress,
    NewTransaction, Output, Transaction,
  };
  use diesel::{self, Connection, PgConnection};

  #[test]
  fn loads_correct_values() {
    // Given
    let config = config::Config::load_test();
    let db_connection = PgConnection::establish(&config.db_url).unwrap();
    let base58check = String::from("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa");

    db_connection.test_transaction::<_, diesel::result::Error, _>(|| {
      let new_block = NewBlock::default();
      let block: Block = diesel::insert_into(schema::blocks::dsl::blocks)
        .values(new_block)
        .get_result(&db_connection)
        .unwrap();

      let mut new_transaction = NewTransaction::default();
      new_transaction.block_id = block.id;
      new_transaction.hash = vec![0xFFu8; 32];
      let transaction: Transaction = diesel::insert_into(
        schema::transactions::dsl::transactions,
      ).values(&new_transaction)
        .get_result(&db_connection)
        .unwrap();

      let new_output = NewOutput {
        transaction_id: transaction.id,
        output_index: 1,
        value: 50,
        script: vec![],
      };
      let output: Output = diesel::insert_into(schema::outputs::dsl::outputs)
        .values(&new_output)
        .get_result(&db_connection)
        .unwrap();

      let new_output_address = NewOutputAddress {
        output_id: output.id,
        hash: vec![],
        base58check: base58check.clone(),
      };
      diesel::insert_into(schema::output_addresses::dsl::output_addresses)
        .values(&new_output_address)
        .execute(&db_connection)
        .unwrap();

      let new_address = NewAddress {
        base58check: base58check.clone(),
      };
      let address: Address = diesel::insert_into(
        schema::addresses::dsl::addresses,
      ).values(&new_address)
        .get_result(&db_connection)
        .unwrap();

      // When
      let resolved_output = load_resolved_output(
        &db_connection,
        new_transaction.hash,
        new_output.output_index,
      );

      // Then
      assert_eq!(
        resolved_output,
        Some(ResolvedOutput {
          address_id: address.id,
          value: 50
        })
      );
      Ok(())
    });
  }

  #[test]
  fn returns_none_if_requested_output_does_not_exist() {
    // Given
    let config = config::Config::load_test();
    let db_connection = PgConnection::establish(&config.db_url).unwrap();
    let tx_hash = vec![0xFFu8; 32];
    let output_index = 1;

    // When
    let resolved_output =
      load_resolved_output(&db_connection, tx_hash, output_index);

    // Then
    assert!(resolved_output.is_none());
  }
}
