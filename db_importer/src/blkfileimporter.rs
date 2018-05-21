use diesel;
use diesel::prelude::*;
use std::io;

use blk_file_reader;
use db_persistence::domain::*;
use db_persistence::repository::*;

pub type Result<T> = ::std::result::Result<T, diesel::result::Error>;

pub struct BlkFileImporter<'a> {
  block_repository: BlockRepository<'a>,
  transaction_repository: TransactionRepository<'a>,
  input_repository: InputRepository<'a>,
  output_repository: OutputRepository<'a>,
  output_address_repository: OutputAddressRepository<'a>,
  blk_file_repository: BlkFileRepository<'a>,
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
    }
  }

  pub fn import<B>(&self, blk_file_path: &str, blocks: B) -> Result<()>
  where
    B: IntoIterator<Item = io::Result<blk_file_reader::Block>>,
  {
    let mut number_of_blocks = 0;
    for block in blocks {
      // TODO Return error instead of panicking.
      let block = block.unwrap();
      let _ = self.import_block(&block).unwrap();
      number_of_blocks += 1;
    }

    // TODO Save blk file index instead of its name?
    let blk_file_name = super::get_blk_file_name(blk_file_path);
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
    let saved_transaction = self
      .transaction_repository
      .save(&new_transaction);
    self.import_inputs(&transaction.inputs, saved_transaction.id)?;
    self.import_outputs(&transaction.outputs, saved_transaction.id)?;
    Ok(())
  }

  fn import_inputs(
    &self,
    inputs: &[blk_file_reader::Input],
    transaction_id: i64,
  ) -> Result<()> {
    for input in inputs.iter() {
      self.import_input(input, transaction_id);
    }
    Ok(())
  }

  fn import_input(&self, input: &blk_file_reader::Input, transaction_id: i64) {
    let new_input = NewInput::new(input, transaction_id);
    let _ = self.input_repository.save(&new_input);
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
    let _ = self
      .output_address_repository
      .save(&new_output_address);
  }
}
