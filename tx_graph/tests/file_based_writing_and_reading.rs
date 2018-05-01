extern crate tempdir;
extern crate tx_graph;

use tempdir::TempDir;
use tx_graph::domain::{InputOutput, NewTransaction, Transaction};
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

  let transaction = NewTransaction {
    inputs: vec![].into_boxed_slice(),
    outputs: vec![].into_boxed_slice(),
  };

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

  let input = InputOutput {
    address_id: 42,
    value: 13,
  };

  let output0 = InputOutput {
    address_id: 1000,
    value: 123,
  };

  let output1 = InputOutput {
    address_id: 10001,
    value: 123_456_789_987_654_321,
  };

  let new_transaction = NewTransaction {
    inputs: vec![input].into_boxed_slice(),
    outputs: vec![output0, output1].into_boxed_slice(),
  };

  // When
  file.write_transaction(&new_transaction).unwrap();
  file.flush().unwrap();
  let mmap = map_file_into_readable_memory(&path).unwrap();
  let transactions: Vec<Transaction> = mmap.transactions().collect();

  // Then
  assert_eq!(transactions.len(), 1);
  let transaction = &transactions[0];
  let inputs: Vec<InputOutput> = transaction.get_inputs().collect();
  let outputs: Vec<InputOutput> = transaction.get_outputs().collect();
  assert_eq!(transaction.get_number_of_inputs(), 1);
  assert_eq!(transaction.get_number_of_outputs(), 2);
  assert_eq!(new_transaction.inputs.to_vec(), inputs);
  assert_eq!(new_transaction.outputs.to_vec(), outputs);
}
