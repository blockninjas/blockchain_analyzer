use std::io;
use domain::NewTransaction;
use domain::Transaction;

pub trait TransactionRepository {
  fn save(new_transaction: &NewTransaction) -> io::Result<()>;

  fn read(transaction_id: usize) -> io::Result<Transaction>;
}
