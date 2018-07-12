//! Blockchain Intermediate Representation (BIR)

mod address;
mod block;
mod input;
mod output;
mod transaction;

pub type AddressId = u64;
pub use self::address::Address;
pub use self::block::Block;
pub use self::input::Input;
pub use self::output::Output;
pub use self::transaction::Transaction;
