use std::io;
use domain::TransactionHash;

pub trait TransactionHashRepository {
  fn save(transaction_hash: &TransactionHash) -> io::Result<()>;
}
