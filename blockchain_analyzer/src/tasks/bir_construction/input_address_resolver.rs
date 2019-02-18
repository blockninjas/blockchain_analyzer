use super::{OrderedBlock, Utxo, UtxoCache, UtxoId};
use bir;
use blk_file_reader;
use db::schema;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, OptionalExtension, PgConnection, QueryDsl,
    RunQueryDsl,
};

pub struct InputAddressResolver<'conn, 'utxo> {
    db_connection: &'conn PgConnection,
    utxo_cache: &'utxo mut UtxoCache,
}

impl<'conn, 'utxo> InputAddressResolver<'conn, 'utxo> {
    pub fn new(
        db_connection: &'conn PgConnection,
        utxo_cache: &'utxo mut UtxoCache,
    ) -> InputAddressResolver<'conn, 'utxo> {
        InputAddressResolver {
            db_connection,
            utxo_cache,
        }
    }

    pub fn resolve_input_addresses(&mut self, ordered_block: OrderedBlock) -> bir::Block {
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
                        address: bir::Address::UnresolvedAddress,
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
                            address: bir::Address::UnresolvedAddress,
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
                address: bir::Address::Base58Check(resolved_output.base58check),
                value: resolved_output.value as u64,
            })
        } else {
            None
        }
    }

    fn get_output_address(&mut self, output: &blk_file_reader::Output) -> bir::Address {
        if let Some(ref address) = output.address {
            bir::Address::Base58Check(address.base58check.clone())
        } else {
            bir::Address::UnresolvedAddress
        }
    }
}

#[derive(Queryable, PartialEq, Eq, Debug)]
pub struct ResolvedOutput {
    pub base58check: String,
    pub value: i64,
}

fn load_resolved_output(
    db_connection: &PgConnection,
    tx_hash: Vec<u8>,
    output_index: i32,
) -> Option<ResolvedOutput> {
    schema::transactions::dsl::transactions
        .inner_join(
            schema::outputs::dsl::outputs
                .inner_join(schema::output_addresses::dsl::output_addresses),
        )
        .filter(
            schema::transactions::dsl::hash
                .eq(tx_hash)
                .and(schema::outputs::dsl::output_index.eq(output_index)),
        )
        .select((
            schema::output_addresses::dsl::base58check,
            schema::outputs::dsl::value,
        ))
        .first(db_connection)
        .optional()
        .unwrap()
}

#[cfg(test)]
mod load_resolved_output_test {

    use super::*;
    use config;
    use db::*;
    use diesel::{self, Connection, PgConnection};

    #[test]
    fn loads_correct_values() {
        // Given
        let config = config::Config::load_test().unwrap();
        let db_connection = PgConnection::establish(&config.db_url).unwrap();
        let base58check = String::from("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa");

        db_connection.test_transaction::<_, diesel::result::Error, _>(|| {
            let new_blk_file = NewBlkFile {
                name: String::new(),
                number_of_blocks: 0,
            };
            let blk_file = new_blk_file.save(&db_connection).unwrap();

            let mut new_block = NewBlock::default();
            new_block.blk_file_id = blk_file.id;
            let block: Block = diesel::insert_into(schema::blocks::dsl::blocks)
                .values(new_block)
                .get_result(&db_connection)
                .unwrap();

            let mut new_transaction = NewTransaction::default();
            new_transaction.block_id = block.id;
            new_transaction.hash = vec![0xFFu8; 32];
            let transaction: Transaction =
                diesel::insert_into(schema::transactions::dsl::transactions)
                    .values(&new_transaction)
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
                    base58check: base58check,
                    value: 50
                })
            );
            Ok(())
        });
    }

    #[test]
    fn returns_none_if_requested_output_does_not_exist() {
        // Given
        let config = config::Config::load_test().unwrap();
        let db_connection = PgConnection::establish(&config.db_url).unwrap();
        let tx_hash = vec![0xFFu8; 32];
        let output_index = 1;

        // When
        let resolved_output = load_resolved_output(&db_connection, tx_hash, output_index);

        // Then
        assert!(resolved_output.is_none());
    }
}
