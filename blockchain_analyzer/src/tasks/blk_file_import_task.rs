use blk_file_reader;
use config::Config;
use db::*;
use diesel::prelude::*;
use failure::Error;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use rayon::prelude::*;
use std::collections::HashSet;
use std::result::Result;
use task_manager::{Index, Task};

pub struct BlkFileImportTask {}

impl BlkFileImportTask {
    pub fn new() -> BlkFileImportTask {
        BlkFileImportTask {}
    }
}

impl Task for BlkFileImportTask {
    fn run(
        &self,
        config: &Config,
        db_connection_pool: &Pool<ConnectionManager<PgConnection>>,
    ) -> Result<(), Error> {
        info!("Import blk files");

        let db_connection = db_connection_pool.get()?;

        db_connection.transaction::<_, Error, _>(|| {
            continue_import_of_latest_blk_file(config, &db_connection)
        })?;

        let blk_files = {
            let db_connection = db_connection_pool.get()?;
            get_blk_files_to_import(&db_connection, &config.blk_file_path)?
        };

        // TODO Make number of threads configurable.
        // TODO Handle failing threads.
        blk_files.par_iter().for_each(|blk_file| {
            let import_result =
                import_blk_file_in_separate_connection(db_connection_pool, blk_file);

            match import_result {
                Ok(_) => {
                    info!("Finished import of {}", blk_file);
                }
                Err(ref err) => {
                    error!("Could not import {} (reason {})", blk_file, err);
                    // TODO Return error.
                }
            };
        });

        Ok(())
    }

    fn get_indexes(&self) -> Vec<Index> {
        vec![
            Index {
                table: String::from("blocks"),
                column: String::from("hash"),
                unique: false,
            },
            Index {
                table: String::from("blocks"),
                column: String::from("previous_block_hash"),
                unique: false,
            },
            Index {
                table: String::from("transactions"),
                column: String::from("block_id"),
                unique: false,
            },
            Index {
                table: String::from("transactions"),
                column: String::from("hash"),
                unique: false,
            },
            Index {
                table: String::from("inputs"),
                column: String::from("transaction_id"),
                unique: false,
            },
            Index {
                table: String::from("inputs"),
                column: String::from("previous_tx_hash"),
                unique: false,
            },
            Index {
                table: String::from("outputs"),
                column: String::from("transaction_id"),
                unique: false,
            },
            Index {
                table: String::from("output_addresses"),
                column: String::from("base58check"),
                unique: false,
            },
        ]
    }
}

fn continue_import_of_latest_blk_file(
    config: &Config,
    db_connection: &PgConnection,
) -> Result<(), Error> {
    if let Some(latest_imported_blk_file) = BlkFile::read_latest_blk_file(db_connection)? {
        let blk_file_path = ::std::path::Path::new(&config.blk_file_path);
        let latest_imported_blk_file_path = blk_file_path.join(&latest_imported_blk_file.name);

        info!("Continue import of {:?}", latest_imported_blk_file_path);

        let mut blocks =
            blk_file_reader::read_blocks(latest_imported_blk_file_path.to_str().unwrap())?;
        let mut blocks = blocks.skip(latest_imported_blk_file.number_of_blocks as usize);
        import_blocks(db_connection, blocks, &latest_imported_blk_file)?;
    }

    Ok(())
}

fn get_blk_files_to_import(
    db_connection: &PgConnection,
    blk_file_path: &str,
) -> Result<Vec<String>, Error> {
    // Get the blk files that have already been imported by previous runs.
    let imported_blk_file_names: HashSet<_> = BlkFile::read_all_names(db_connection)?
        .into_iter()
        .collect();

    // TODO Return error instead of panicking.
    let blk_files = blk_file_reader::read_blk_files(blk_file_path)?;

    let blk_files_to_import = blk_files
        .into_iter()
        .filter(|blk_file| !imported_blk_file_names.contains(&get_blk_file_name(blk_file)))
        .collect();

    Ok(blk_files_to_import)
}

pub fn get_blk_file_name(blk_file_path: &str) -> String {
    String::from(
        ::std::path::Path::new(blk_file_path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap(),
    )
}

fn import_blk_file_in_separate_connection(
    db_connection_pool: &Pool<ConnectionManager<PgConnection>>,
    blk_file: &str,
) -> Result<(), Error> {
    info!("Import {}", blk_file);
    let blocks = blk_file_reader::read_blocks(blk_file)?;
    let db_connection = db_connection_pool.get()?;
    db_connection.transaction(|| import_blk_file(&db_connection, blk_file, blocks))?;
    Ok(())
}

/// Imports a blk file into the database at `database_url`.
fn import_blk_file<B>(
    db_connection: &PgConnection,
    blk_file_path: &str,
    blocks: B,
) -> Result<(), Error>
where
    B: IntoIterator<Item = ::std::io::Result<blk_file_reader::Block>>,
{
    // TODO Save blk file index instead of its name?
    let blk_file_name = get_blk_file_name(blk_file_path);
    let new_blk_file = NewBlkFile {
        number_of_blocks: 0,
        name: blk_file_name,
    };
    let blk_file = new_blk_file.save(db_connection)?;

    import_blocks(&db_connection, blocks, &blk_file)?;

    Ok(())
}

pub fn import_blocks<B>(
    db_connection: &PgConnection,
    blocks: B,
    blk_file: &BlkFile,
) -> Result<(), Error>
where
    B: IntoIterator<Item = ::std::io::Result<blk_file_reader::Block>>,
{
    let mut number_of_blocks = 0;

    for block in blocks.into_iter() {
        let block = block.unwrap();
        let _ = import_block(db_connection, &block, blk_file.id)?;
        number_of_blocks += 1;
    }

    let new_number_of_blocks_in_blk_file = blk_file.number_of_blocks + number_of_blocks;
    BlkFile::update_number_of_blocks(db_connection, blk_file.id, new_number_of_blocks_in_blk_file)?;

    info!("Imported {} blocks for {}", number_of_blocks, blk_file.name);

    Ok(())
}

fn import_block(
    db_connection: &PgConnection,
    block: &blk_file_reader::Block,
    blk_file_id: i64,
) -> Result<(), Error> {
    let new_block = NewBlock::new(block, blk_file_id);
    let saved_block = new_block.save(db_connection)?;
    import_transactions(db_connection, &block.transactions, saved_block.id)
}

fn import_transactions(
    db_connection: &PgConnection,
    transactions: &[blk_file_reader::Transaction],
    block_id: i64,
) -> Result<(), Error> {
    for transaction in transactions.iter() {
        import_transaction(db_connection, transaction, block_id)?;
    }
    Ok(())
}

fn import_transaction(
    db_connection: &PgConnection,
    transaction: &blk_file_reader::Transaction,
    block_id: i64,
) -> Result<(), Error> {
    let new_transaction = NewTransaction::new(transaction, block_id);
    let saved_transaction = new_transaction.save(db_connection)?;
    import_inputs(db_connection, transaction, saved_transaction.id)?;
    import_outputs(db_connection, &transaction.outputs, saved_transaction.id)?;
    Ok(())
}

fn import_inputs(
    db_connection: &PgConnection,
    transaction: &blk_file_reader::Transaction,
    transaction_id: i64,
) -> Result<(), Error> {
    for (input_index, input) in transaction.inputs.iter().enumerate() {
        import_input(
            db_connection,
            input,
            input_index,
            transaction,
            transaction_id,
        )?;
    }

    Ok(())
}

fn import_input(
    db_connection: &PgConnection,
    input: &blk_file_reader::Input,
    input_index: usize,
    transaction: &blk_file_reader::Transaction,
    transaction_id: i64,
) -> Result<(), Error> {
    let new_input = NewInput::new(input, transaction_id);
    let saved_input = new_input.save(db_connection)?;

    let is_segwit_tx = transaction.script_witnesses.len() > 0;
    if is_segwit_tx {
        for script_witness_item in transaction.script_witnesses[input_index].items.iter() {
            let new_script_witness_item = NewScriptWitnessItem {
                content: script_witness_item.to_vec(),
                input_id: saved_input.id,
            };

            new_script_witness_item.save(db_connection)?;
        }
    }

    Ok(())
}

fn import_outputs(
    db_connection: &PgConnection,
    outputs: &[blk_file_reader::Output],
    transaction_id: i64,
) -> Result<(), Error> {
    for output in outputs.iter() {
        import_output(db_connection, output, transaction_id)?;
    }

    Ok(())
}

fn import_output(
    db_connection: &PgConnection,
    output: &blk_file_reader::Output,
    transaction_id: i64,
) -> Result<(), Error> {
    let new_output = NewOutput::new(output, transaction_id);
    let saved_output = new_output.save(db_connection)?;

    if let Some(ref address) = output.address {
        import_address(db_connection, address, saved_output.id);
    };

    Ok(())
}

fn import_address(
    db_connection: &PgConnection,
    address: &blk_file_reader::Address,
    output_id: i64,
) {
    let new_output_address = NewOutputAddress::new(address, output_id);
    let _ = new_output_address.save(db_connection);
}

#[cfg(test)]
mod test {

    use super::*;
    use diesel::result::Error;
    use std;

    const TEST_DATABASE_URL: &'static str =
        "postgres://postgres:test@127.0.0.1:5432/bitcoin_blockchain";

    #[test]
    pub fn genesis_block_is_imported_correctly() {
        // Given
        let db_connection = PgConnection::establish(TEST_DATABASE_URL).unwrap();
        let blocks = blk_file_reader::read_blocks("../sample-blk-files/blk00000.dat").unwrap();
        let blocks = blocks.take(1);

        db_connection.test_transaction::<_, Error, _>(|| {
            // When
            let _ = import_blk_file(&db_connection, "blk00000.dat", blocks.into_iter()).unwrap();

            // Then
            let imported_blocks = Block::read_all(&db_connection).unwrap();
            assert_eq!(imported_blocks.len(), 1);

            let genesis_block = &imported_blocks[0];
            assert_eq!(genesis_block.version, 1);
            assert_eq!(
                genesis_block.hash,
                vec![
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0xd6, 0x68, 0x9c, 0x08, 0x5a, 0xe1, 0x65,
                    0x83, 0x1e, 0x93, 0x4f, 0xf7, 0x63, 0xae, 0x46, 0xa2, 0xa6, 0xc1, 0x72, 0xb3,
                    0xf1, 0xb6, 0x0a, 0x8c, 0xe2, 0x6f,
                ]
            );
            assert!(genesis_block.previous_block_hash.iter().all(|&b| b == 0));
            assert_eq!(
                genesis_block.merkle_root,
                vec![
                    0x4a, 0x5e, 0x1e, 0x4b, 0xaa, 0xb8, 0x9f, 0x3a, 0x32, 0x51, 0x8a, 0x88, 0xc3,
                    0x1b, 0xc8, 0x7f, 0x61, 0x8f, 0x76, 0x67, 0x3e, 0x2c, 0xc7, 0x7a, 0xb2, 0x12,
                    0x7b, 0x7a, 0xfd, 0xed, 0xa3, 0x3b,
                ]
            );
            assert_eq!(genesis_block.creation_time, 1231006505);
            assert_eq!(genesis_block.nonce, 2083236893);

            Ok(())
        });
    }

    #[test]
    pub fn imports_all_provided_blocks() {
        // Given
        let db_connection = PgConnection::establish(TEST_DATABASE_URL).unwrap();
        let blocks = blk_file_reader::read_blocks("../sample-blk-files/blk00000.dat").unwrap();
        let blocks = blocks.take(5);

        db_connection.test_transaction::<_, Error, _>(|| {
            // When
            let _ = import_blk_file(&db_connection, "blk00000.dat", blocks).unwrap();

            // Then
            assert_eq!(Block::count(&db_connection).unwrap(), 5);
            let blk_files = BlkFile::read_all(&db_connection).unwrap();
            assert_eq!(blk_files.len(), 1);
            let blk_file = &blk_files[0];
            assert_eq!(blk_file.name, "blk00000.dat");
            assert_eq!(blk_file.number_of_blocks, 5);

            Ok(())
        });
    }

    #[test]
    pub fn can_import_empty_blk_file() {
        // Given
        let db_connection = PgConnection::establish(TEST_DATABASE_URL).unwrap();
        let blocks = Vec::<std::io::Result<blk_file_reader::Block>>::new();

        db_connection.test_transaction::<_, Error, _>(|| {
            // When
            let _ = import_blk_file(&db_connection, "blk12345.dat", blocks).unwrap();

            // Then
            assert_eq!(Block::count(&db_connection).unwrap(), 0);
            let blk_files = BlkFile::read_all(&db_connection).unwrap();
            assert_eq!(blk_files.len(), 1);
            let blk_file = &blk_files[0];
            assert_eq!(blk_file.name, "blk12345.dat");
            assert_eq!(blk_file.number_of_blocks, 0);

            Ok(())
        });
    }
}
