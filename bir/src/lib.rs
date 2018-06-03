//! Blockchain Intermediate Representation (BIR)

#[macro_use]
extern crate serde_derive;
extern crate bincode;

mod address;
mod block;
mod input;
mod output;
mod transaction;

pub type AddressId = u64;
pub type Hash = [u8; 32];
pub use address::Address;
pub use address::Address::ResolvedAddress;
pub use address::Address::UnresolvedAddress;
pub use block::Block;
pub use input::Input;
pub use output::Output;
pub use transaction::Transaction;
