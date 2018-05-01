use read::ReadTransactionHeader;
use super::Transaction;
use super::transaction::TransactionMemoryLayout;
use std::io::Cursor;

pub struct Transactions<'a> {
  offset: u64,
  bytes: &'a [u8],
}

impl<'a> Transactions<'a> {
  pub fn new(bytes: &'a [u8]) -> Transactions<'a> {
    Transactions { offset: 0, bytes }
  }
}

impl<'a> Iterator for Transactions<'a> {
  type Item = Transaction<'a>;

  fn next(&mut self) -> Option<Transaction<'a>> {
    if self.offset < self.bytes.len() as u64 {
      let mut cursor = Cursor::new(self.bytes);
      let transaction_header = cursor.read_transaction_header().unwrap();
      let transaction =
        Transaction::new(self.offset, self.bytes, transaction_header);
      self.offset += transaction.get_size();
      Some(transaction)
    } else {
      None
    }
  }
}
