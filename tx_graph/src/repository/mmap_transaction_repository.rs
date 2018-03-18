use std::io;
use domain::Transaction;
use super::TransactionRepository;

pub struct MmapTransactionRepository {}

impl TransactionRepository for MmapTransactionRepository {
  fn save(_transaction: &Transaction) -> io::Result<()> {
    Ok(())
  }
}
