use std::io;
use domain::Block;

/// The `BlockRead` trait allows for reading `Block`s from a source.
///
/// It follows a naming-convention of the standard library which provides
/// `std::io::Read` trait, whose implementors are referred to as `readers`.
pub trait BlockRead {
  /// Advances the `BlockReader` by `number_of_blocks_to_skip` `Blocks`.
  fn skip(&mut self, number_of_blocks_to_skip: usize) -> io::Result<()>;

  /// Returns the next `Block` and advances the `BlockReader`.
  fn read(&mut self) -> io::Result<Block>;
}
