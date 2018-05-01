use redis::{Commands, ConnectionLike};
use data_encoding::HEXLOWER;
use super::{TransactionHash, TransactionId, TransactionIdMap};

const KEY_PREFIX: &'static str = "txhash";

pub struct RedisTransactionIdMap<C>
where
  C: ConnectionLike + Commands,
{
  connection: C,
}

impl<C> RedisTransactionIdMap<C>
where
  C: ConnectionLike + Commands,
{
  pub fn new(connection: C) -> RedisTransactionIdMap<C> {
    RedisTransactionIdMap { connection }
  }
}

impl<C> TransactionIdMap for RedisTransactionIdMap<C>
where
  C: ConnectionLike + Commands,
{
  fn set_id(
    &self,
    transaction_hash: TransactionHash,
    transaction_id: TransactionId,
  ) {
    let redis_key = to_redis_key(transaction_hash);
    // TODO Return error instead of panicking.
    let _: () = self.connection.set(redis_key, transaction_id).unwrap();
  }

  fn get_id(&self, transaction_hash: TransactionHash) -> TransactionId {
    let redis_key = to_redis_key(transaction_hash);
    let transaction_id: TransactionId = self.connection.get(redis_key).unwrap();
    transaction_id
  }
}

fn to_redis_key(transaction_hash: TransactionHash) -> String {
  // TODO Return error instead of panicking.
  let transaction_hash_hex = HEXLOWER.encode(&transaction_hash);
  let redis_key = String::from(KEY_PREFIX) + &transaction_hash_hex;
  redis_key
}

#[cfg(test)]
mod test {
  extern crate redis_test;

  use super::*;
  use self::redis_test::redis_test;

  #[test]
  fn can_set_and_get_id() {
    redis_test(|connection| {
      // Given
      let transaction_id_map = RedisTransactionIdMap::new(connection);
      let transaction_hash = HEXLOWER
        .decode(
          b"4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b",
        )
        .unwrap();
      let transaction_id = 42;

      // When
      transaction_id_map.set_id(&transaction_hash, transaction_id);

      // Then
      let inserted_transaction_id =
        transaction_id_map.get_id(&transaction_hash);
      assert_eq!(inserted_transaction_id, transaction_id);
    });
  }
}
