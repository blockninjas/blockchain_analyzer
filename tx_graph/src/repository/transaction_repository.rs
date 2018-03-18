use std::io;
use domain::Transaction;

pub trait TransactionRepository {
  fn save(transaction: &Transaction) -> io::Result<()>;
}
