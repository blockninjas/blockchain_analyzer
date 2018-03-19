use std::io;
use domain::{NewTransaction, Transaction};
use super::TransactionRepository;

pub struct MmapTransactionRepository {}

impl TransactionRepository for MmapTransactionRepository {
  fn save(_new_transaction: &NewTransaction) -> io::Result<()> {
    // TODO Implement
    Ok(())
  }

  fn read(_transaction_id: usize) -> io::Result<Transaction> {
    // TODO Implement
    Ok(Transaction::new(0))
  }
}
