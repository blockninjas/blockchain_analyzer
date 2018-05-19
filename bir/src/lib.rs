//! Blockchain Intermediate Representation (BIR)

#[macro_use]
extern crate serde_derive;
extern crate bincode;

mod block;
mod input;
mod output;
mod transaction;

pub type AddressId = u64;
pub type Hash = [u8; 32];
pub use block::Block;
pub use input::Input;
pub use output::Output;
pub use transaction::Transaction;
