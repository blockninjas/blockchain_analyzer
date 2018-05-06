extern crate tx_graph;

mod test_data;

use tx_graph::domain::Transaction;
use tx_graph::read::TransactionIterable;
use tx_graph::write::WriteTransaction;

#[test]
fn can_write_and_read_empty_transaction() {
  // Given
  let mut bytes = Vec::<u8>::new();
  let new_transaction = test_data::create_empty_transaction();

  // When
  bytes.write_transaction(&new_transaction).unwrap();
  let transactions: Vec<Transaction> = bytes.transactions().collect();

  // Then
  assert_eq!(transactions.len(), 1);
  test_data::assert_transaction_eq_new_transaction(
    &transactions[0],
    &new_transaction,
  );
}

#[test]
fn can_write_and_read_non_empty_transaction() {
  // Given
  let mut bytes = Vec::<u8>::new();
  let new_transaction = test_data::create_non_empty_transaction();

  // When
  bytes.write_transaction(&new_transaction).unwrap();
  let transactions: Vec<Transaction> = bytes.transactions().collect();

  // Then
  assert_eq!(transactions.len(), 1);
  test_data::assert_transaction_eq_new_transaction(
    &transactions[0],
    &new_transaction,
  );
}
