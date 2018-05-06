//! # Test Data
//!
//! Provides helper methods to create data that can be used for testing.

extern crate tx_graph;

use tx_graph::domain::{InputOutput, NewTransaction, Transaction};

pub fn create_empty_transaction() -> NewTransaction {
  let new_transaction = NewTransaction {
    inputs: vec![].into_boxed_slice(),
    outputs: vec![].into_boxed_slice(),
  };

  new_transaction
}

pub fn create_non_empty_transaction() -> NewTransaction {
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

  new_transaction
}

pub fn assert_transaction_eq_new_transaction(
  transaction: &Transaction,
  new_transaction: &NewTransaction,
) {
  let inputs: Vec<InputOutput> = transaction.get_inputs().collect();
  let outputs: Vec<InputOutput> = transaction.get_outputs().collect();
  assert_eq!(
    transaction.get_number_of_inputs(),
    new_transaction.inputs.len() as u32
  );
  assert_eq!(
    transaction.get_number_of_outputs(),
    new_transaction.outputs.len() as u32
  );
  assert_eq!(new_transaction.inputs.to_vec(), inputs);
  assert_eq!(new_transaction.outputs.to_vec(), outputs);
}
