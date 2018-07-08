use blk_file_reader;
use db_persistence::{domain::*, repository::*};
use diesel::{self, prelude::*};
use std::io;

pub type Result<T> = ::std::result::Result<T, diesel::result::Error>;

pub struct BlkFileImporter<'a> {
  block_repository: BlockRepository<'a>,
  transaction_repository: TransactionRepository<'a>,
  input_repository: InputRepository<'a>,
  output_repository: OutputRepository<'a>,
  output_address_repository: OutputAddressRepository<'a>,
  blk_file_repository: BlkFileRepository<'a>,
  script_witness_item_repository: ScriptWitnessItemRepository<'a>,
}

impl<'a> BlkFileImporter<'a> {
  pub fn new(db_connection: &PgConnection) -> BlkFileImporter {
    BlkFileImporter {
      block_repository: BlockRepository::new(db_connection),
      transaction_repository: TransactionRepository::new(db_connection),
      input_repository: InputRepository::new(db_connection),
      output_repository: OutputRepository::new(db_connection),
      output_address_repository: OutputAddressRepository::new(db_connection),
      blk_file_repository: BlkFileRepository::new(db_connection),
      script_witness_item_repository: ScriptWitnessItemRepository::new(
        db_connection,
      ),
    }
  }

  pub fn import<B>(&self, blk_file_path: &str, blocks: B) -> Result<()>
  where
    B: IntoIterator<Item = io::Result<blk_file_reader::Block>>,
  {
    let mut number_of_blocks = 0;
    for block in blocks.into_iter() {
      // TODO Return error instead of panicking.
      let block = block.unwrap();
      let _ = self.import_block(&block).unwrap();
      number_of_blocks += 1;
    }

    // TODO Save blk file index instead of its name?
    let blk_file_name = get_blk_file_name(blk_file_path);
    let new_blk_file = NewBlkFile {
      number_of_blocks,
      name: blk_file_name,
    };
    let _ = self.blk_file_repository.save(&new_blk_file);

    Ok(())
  }

  fn import_block(&self, block: &blk_file_reader::Block) -> Result<()> {
    let new_block = NewBlock::new(block);
    let saved_block = self.block_repository.save(&new_block);
    self.import_transactions(&block.transactions, saved_block.id)
  }

  fn import_transactions(
    &self,
    transactions: &[blk_file_reader::Transaction],
    block_id: i64,
  ) -> Result<()> {
    for transaction in transactions.iter() {
      self.import_transaction(transaction, block_id)?;
    }
    Ok(())
  }

  fn import_transaction(
    &self,
    transaction: &blk_file_reader::Transaction,
    block_id: i64,
  ) -> Result<()> {
    let new_transaction = NewTransaction::new(transaction, block_id);
    let saved_transaction = self.transaction_repository.save(&new_transaction);
    self.import_inputs(transaction, saved_transaction.id)?;
    self.import_outputs(&transaction.outputs, saved_transaction.id)?;
    Ok(())
  }

  fn import_inputs(
    &self,
    transaction: &blk_file_reader::Transaction,
    transaction_id: i64,
  ) -> Result<()> {
    for (input_index, input) in transaction.inputs.iter().enumerate() {
      self.import_input(input, input_index, transaction, transaction_id);
    }
    Ok(())
  }

  fn import_input(
    &self,
    input: &blk_file_reader::Input,
    input_index: usize,
    transaction: &blk_file_reader::Transaction,
    transaction_id: i64,
  ) {
    let new_input = NewInput::new(input, transaction_id);
    let saved_input = self.input_repository.save(&new_input);

    let is_segwit_tx = transaction.script_witnesses.len() > 0;
    if is_segwit_tx {
      for script_witness_item in
        transaction.script_witnesses[input_index].items.iter()
      {
        let new_script_witness_item = NewScriptWitnessItem {
          content: script_witness_item.to_vec(),
          input_id: saved_input.id,
        };
        self
          .script_witness_item_repository
          .save(&new_script_witness_item);
      }
    }
  }

  fn import_outputs(
    &self,
    outputs: &[blk_file_reader::Output],
    transaction_id: i64,
  ) -> Result<()> {
    for output in outputs.iter() {
      self.import_output(output, transaction_id)?;
    }
    Ok(())
  }

  fn import_output(
    &self,
    output: &blk_file_reader::Output,
    transaction_id: i64,
  ) -> Result<()> {
    let new_output = NewOutput::new(output, transaction_id);
    let saved_output = self.output_repository.save(&new_output);

    if let Some(ref address) = output.address {
      self.import_address(address, saved_output.id);
    };

    Ok(())
  }

  fn import_address(&self, address: &blk_file_reader::Address, output_id: i64) {
    let new_output_address = NewOutputAddress::new(address, output_id);
    let _ = self.output_address_repository.save(&new_output_address);
  }
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
    let blk_file_importer = BlkFileImporter::new(&db_connection);
    let blocks = blk_file_reader::read_blocks(
      "../blk_file_reader/sample_blk_files/blk00000.dat",
    ).unwrap();
    let blocks = blocks.take(1);

    db_connection.test_transaction::<_, Error, _>(|| {
      // When
      let _ = blk_file_importer
        .import("blk00000.dat", blocks.into_iter())
        .unwrap();

      // Then
      let block_repository = BlockRepository::new(&db_connection);
      let imported_blocks = block_repository.read_all();
      assert_eq!(imported_blocks.len(), 1);

      let genesis_block = &imported_blocks[0];
      assert_eq!(genesis_block.version, 1);
      assert_eq!(
        genesis_block.hash,
        vec![
          0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0xd6, 0x68, 0x9c, 0x08, 0x5a,
          0xe1, 0x65, 0x83, 0x1e, 0x93, 0x4f, 0xf7, 0x63, 0xae, 0x46, 0xa2,
          0xa6, 0xc1, 0x72, 0xb3, 0xf1, 0xb6, 0x0a, 0x8c, 0xe2, 0x6f,
        ]
      );
      assert!(genesis_block.previous_block_hash.iter().all(|&b| b == 0));
      assert_eq!(
        genesis_block.merkle_root,
        vec![
          0x4a, 0x5e, 0x1e, 0x4b, 0xaa, 0xb8, 0x9f, 0x3a, 0x32, 0x51, 0x8a,
          0x88, 0xc3, 0x1b, 0xc8, 0x7f, 0x61, 0x8f, 0x76, 0x67, 0x3e, 0x2c,
          0xc7, 0x7a, 0xb2, 0x12, 0x7b, 0x7a, 0xfd, 0xed, 0xa3, 0x3b,
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
    let blk_file_importer = BlkFileImporter::new(&db_connection);
    let blocks = blk_file_reader::read_blocks(
      "../blk_file_reader/sample_blk_files/blk00000.dat",
    ).unwrap();
    let blocks = blocks.take(5);

    db_connection.test_transaction::<_, Error, _>(|| {
      // When
      let _ = blk_file_importer
        .import("blk00000.dat", blocks.into_iter())
        .unwrap();

      // Then
      let block_repository = BlockRepository::new(&db_connection);
      assert_eq!(block_repository.count(), 5);
      let blk_file_repository = BlkFileRepository::new(&db_connection);
      let blk_files = blk_file_repository.read_all();
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
    let blk_file_importer = BlkFileImporter::new(&db_connection);
    let blocks = Vec::<std::io::Result<blk_file_reader::Block>>::new();

    db_connection.test_transaction::<_, Error, _>(|| {
      // When
      let _ = blk_file_importer
        .import("blk12345.dat", blocks.into_iter())
        .unwrap();

      // Then
      let block_repository = BlockRepository::new(&db_connection);
      assert_eq!(block_repository.count(), 0);
      let blk_file_repository = BlkFileRepository::new(&db_connection);
      let blk_files = blk_file_repository.read_all();
      assert_eq!(blk_files.len(), 1);
      let blk_file = &blk_files[0];
      assert_eq!(blk_file.name, "blk12345.dat");
      assert_eq!(blk_file.number_of_blocks, 0);

      Ok(())
    });
  }
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
