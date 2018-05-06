extern crate tempdir;
extern crate tx_graph;

mod test_data;

use tempdir::TempDir;
use tx_graph::domain::Transaction;
use tx_graph::read::TransactionIterable;
use tx_graph::write::WriteTransaction;
use tx_graph::memory_mapping::map_file_into_readable_memory;
use std::fs::File;
use std::io::{BufWriter, Read, Write};

#[test]
fn can_write_empty_transaction() {
  // Given
  let dir = TempDir::new("memory_map_test").unwrap();
  let path = dir.path().join("new_file");
  let file = File::create(&path).unwrap();
  let mut file = BufWriter::new(file);
  let transaction = test_data::create_empty_transaction();

  // When
  file.write_transaction(&transaction).unwrap();
  file.flush().unwrap();

  // Then
  let mut file = File::open(&path).unwrap();
  let mut bytes = vec![];
  file.read_to_end(&mut bytes).unwrap();
  assert_eq!(bytes.len(), 8);
}

#[test]
fn can_write_transaction_and_read_it_from_memory_mapped_file() {
  // Given
  let dir = TempDir::new("memory_map_test").unwrap();
  let path = dir.path().join("new_file");
  let file = File::create(&path).unwrap();
  let mut file = BufWriter::new(file);
  let new_transaction = test_data::create_non_empty_transaction();

  // When
  file.write_transaction(&new_transaction).unwrap();
  file.flush().unwrap();
  let mmap = map_file_into_readable_memory(&path).unwrap();
  let transactions: Vec<Transaction> = mmap.transactions().collect();

  // Then
  assert_eq!(transactions.len(), 1);
  test_data::assert_transaction_eq_new_transaction(
    &transactions[0],
    &new_transaction,
  );
}
