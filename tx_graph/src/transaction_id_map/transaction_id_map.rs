pub type TransactionHash<'a> = &'a [u8];
pub type TransactionId = u64;

pub trait TransactionIdMap {
  fn set_id(
    &self,
    transaction_hash: TransactionHash,
    transaction_id: TransactionId,
  );
  fn get_id(&self, transaction_hash: TransactionHash) -> TransactionId;
}
