use domain::Transactions;

pub trait TransactionIterable {
  fn transactions(&self) -> Transactions;
}

impl<A: AsRef<[u8]>> TransactionIterable for A {
  fn transactions(&self) -> Transactions {
    Transactions::new(self.as_ref())
  }
}
