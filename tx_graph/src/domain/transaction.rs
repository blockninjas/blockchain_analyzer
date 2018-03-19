use super::{Input, Output};

pub struct Transaction {
  _offset: usize,
}

impl Transaction {
  pub fn new(offset: usize) -> Transaction {
    Transaction { _offset: offset }
  }

  pub fn get_number_of_inputs(&self) -> usize {
    // TODO Implement
    0
  }

  pub fn get_number_of_outputs(&self) -> usize {
    // TODO Implement
    0
  }

  pub fn get_input(&self, _input_index: usize) -> Input {
    // TODO Implement
    Input::new(0)
  }

  pub fn get_output(&self, _output_index: usize) -> Output {
    // TODO Implement
    Output::new(0)
  }
}
