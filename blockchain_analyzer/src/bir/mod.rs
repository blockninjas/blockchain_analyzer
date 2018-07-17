//! Blockchain Intermediate Representation (BIR)

mod address;
mod bir_file_iterator;
mod block;
mod input;
mod output;
mod transaction;
mod util;

pub type AddressId = u64;
pub use self::address::Address;
pub use self::bir_file_iterator::BirFileIterator;
pub use self::block::Block;
pub use self::input::Input;
pub use self::output::Output;
pub use self::transaction::Transaction;
pub use self::util::read_bir_files;
