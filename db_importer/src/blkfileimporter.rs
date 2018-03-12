use diesel;
use diesel::prelude::*;
use std::error::Error;

use blk_file_reader;
use blk_file_reader::BlockRead;
use blk_file_reader::BlockReader;
use db_persistence::repository::*;
use db_persistence::domain::*;

pub type Result<T> = ::std::result::Result<T, diesel::result::Error>;

pub struct BlkFileImporter<'a> {
  block_repository: BlockRepository<'a>,
  transaction_repository: TransactionRepository<'a>,
  input_repository: InputRepository<'a>,
  output_repository: OutputRepository<'a>,
  address_repository: AddressRepository<'a>,
}

impl<'a> BlkFileImporter<'a> {
  pub fn new(db_connection: &PgConnection) -> BlkFileImporter {
    BlkFileImporter {
      block_repository: BlockRepository::new(db_connection),
      transaction_repository: TransactionRepository::new(db_connection),
      input_repository: InputRepository::new(db_connection),
      output_repository: OutputRepository::new(db_connection),
      address_repository: AddressRepository::new(db_connection),
    }
  }

  pub fn import(&self, blk_file_path: &str) -> Result<()> {
    let mut block_reader = BlockReader::new(blk_file_path);
    loop {
      match block_reader.read() {
        Ok(ref block) => {
          let _ = self.import_block(block).unwrap();
        }
        Err(ref error) => {
          if error.kind() != ::std::io::ErrorKind::UnexpectedEof {
            error!("Could not read file (reason: {})", error.description());
          }
          break;
        }
      }
    }

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
    block_id: i32,
  ) -> Result<()> {
    for transaction in transactions.iter() {
      self.import_transaction(transaction, block_id)?;
    }
    Ok(())
  }

  fn import_transaction(
    &self,
    transaction: &blk_file_reader::Transaction,
    block_id: i32,
  ) -> Result<()> {
    let new_transaction = NewTransaction::new(transaction, block_id);
    let saved_transaction = self.transaction_repository.save(&new_transaction);
    self.import_inputs(&transaction.inputs, saved_transaction.id)?;
    self.import_outputs(&transaction.outputs, saved_transaction.id)?;
    Ok(())
  }

  fn import_inputs(
    &self,
    inputs: &[blk_file_reader::Input],
    transaction_id: i32,
  ) -> Result<()> {
    for input in inputs.iter() {
      self.import_input(input, transaction_id);
    }
    Ok(())
  }

  fn import_input(&self, input: &blk_file_reader::Input, transaction_id: i32) {
    let new_input = NewInput::new(input, transaction_id);
    let _ = self.input_repository.save(&new_input);
  }

  fn import_outputs(
    &self,
    outputs: &[blk_file_reader::Output],
    transaction_id: i32,
  ) -> Result<()> {
    for output in outputs.iter() {
      self.import_output(output, transaction_id)?;
    }
    Ok(())
  }

  fn import_output(
    &self,
    output: &blk_file_reader::Output,
    transaction_id: i32,
  ) -> Result<()> {
    let new_output = NewOutput::new(output, transaction_id);
    let saved_output = self.output_repository.save(&new_output);
    self.import_addresses(&output.addresses, saved_output.id)
  }

  fn import_addresses(
    &self,
    addresses: &[blk_file_reader::Address],
    output_id: i32,
  ) -> Result<()> {
    for address in addresses.iter() {
      self.import_address(address, output_id);
    }
    Ok(())
  }

  fn import_address(&self, address: &blk_file_reader::Address, output_id: i32) {
    let new_address = NewAddress::new(address, output_id);
    let _ = self.address_repository.save(&new_address);
  }
}
