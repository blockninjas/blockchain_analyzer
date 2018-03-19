use std::io;
use domain::{NewTransaction, Transaction};
use super::TransactionRepository;
use std::fs::OpenOptions;
use memmap::{MmapMut, MmapOptions};

pub struct MmapTransactionRepository {
  _mmap: MmapMut,
}

impl MmapTransactionRepository {
  pub fn new(mmap: MmapMut) -> MmapTransactionRepository {
    MmapTransactionRepository { _mmap: mmap }
  }

  pub fn from_file(path: &str) -> io::Result<MmapTransactionRepository> {
    let file = OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(path)?;

    let mmap = unsafe { MmapOptions::new().map_mut(&file)? };

    let repository = MmapTransactionRepository::new(mmap);

    Ok(repository)
  }
}

impl TransactionRepository for MmapTransactionRepository {
  fn save(_new_transaction: &NewTransaction) -> io::Result<()> {
    // TODO Implement
    Ok(())
  }

  fn read(_transaction_id: usize) -> io::Result<Transaction> {
    // TODO Implement
    Ok(Transaction::new(0))
  }
}
