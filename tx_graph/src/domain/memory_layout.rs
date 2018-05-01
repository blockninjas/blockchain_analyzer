use std::mem::size_of;

pub fn size_of_transaction_header() -> usize {
  size_of::<u32>() * 2
}

pub fn size_of_input_output() -> usize {
  size_of::<u64>() * 2
}
