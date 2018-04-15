use redis::{self, Commands, ConnectionLike, ToRedisArgs};

pub struct RedisIdMap<C>
where
  C: ConnectionLike + Commands,
{
  connection: C,
  id_counter_key_name: String,
}

impl<C> RedisIdMap<C>
where
  C: ConnectionLike + Commands,
{
  pub fn new(connection: C, id_counter_key_name: String) -> RedisIdMap<C> {
    RedisIdMap {
      connection,
      id_counter_key_name,
    }
  }

  pub fn get_id<K>(&mut self, entry: K) -> u64
  where
    K: Copy + ToRedisArgs,
  {
    // First check, without a transaction, if the value already exists in
    // the map.
    let unique_id: Option<u64> = self.connection.get(entry).unwrap();

    if let Some(unique_id) = unique_id {
      // If the value already exists in the map, return its id.
      unique_id
    } else {
      // If the value does not yet exist in the map, it has to be inserted,
      // which is done inside a transaction.
      redis::transaction(&self.connection, &[entry], |_| {
        // Before actually inserting it, make sure again, that it has not yet
        // been inserted in the meantime.
        let unique_id: Option<u64> = self.connection.get(entry)?;

        if let Some(_) = unique_id {
          // If the value already exists in the map, return its id.
          Ok(unique_id)
        } else {
          // Otherwise, get the next free id, assign it to the value and
          // return it.
          let next_free_unique_id: u64 =
            self.connection.incr(&self.id_counter_key_name, 1)?;
          let _: () = self.connection.set(entry, next_free_unique_id)?;
          Ok(Some(next_free_unique_id))
        }
      }).unwrap()
    }
  }
}
