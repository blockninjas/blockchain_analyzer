use super::{Index, Task};
use config::Config;
use db_persistence::schema::blocks::dsl::*;
use diesel::{self, prelude::*};

pub struct BlockHeightCalculationTask {}

impl BlockHeightCalculationTask {
  pub fn new() -> BlockHeightCalculationTask {
    BlockHeightCalculationTask {}
  }
}

impl Task for BlockHeightCalculationTask {
  fn run(&self, _config: &Config, db_connection: &PgConnection) {
    info!("Run BlockHeightCalculationTask");

    db_connection.transaction::<_, diesel::result::Error, _>(|| {
      // TODO Do not always start from genesis block.

      let mut current_block_hash =
        get_next_block_hash(db_connection, &vec![0u8; 32]);
      let mut current_block_height = 0;

      while let Some(current_block_hash_value) = current_block_hash {
        set_block_height(
          db_connection,
          &current_block_hash_value,
          current_block_height,
        );

        current_block_hash =
          get_next_block_hash(db_connection, &current_block_hash_value);

        current_block_height += 1;
      }

      info!("Current block height is {}", current_block_height);

      Ok(())
    })
    // TODO Return error instead of panicking.
    .unwrap();

    info!("Finished BlockHeightCalculationTask");
  }

  fn get_indexes(&self) -> Vec<Index> {
    vec![Index {
      table: String::from("blocks"),
      column: String::from("height"),
      unique: false,
    }]
  }
}

fn set_block_height(
  db_connection: &PgConnection,
  current_hash: &[u8],
  current_height: i32,
) {
  // TODO Return error instead of panicking.
  diesel::update(blocks.filter(hash.eq(current_hash)))
    .set(height.eq(current_height))
    .execute(db_connection)
    .unwrap();
}

fn get_next_block_hash(
  db_connection: &PgConnection,
  current_hash: &[u8],
) -> Option<Vec<u8>> {
  // TODO Return error instead of panicking.
  blocks
    .select(hash)
    .filter(previous_block_hash.eq(current_hash))
    .first(db_connection)
    .optional()
    .unwrap()
}

#[cfg(test)]
mod test {

  extern crate data_encoding;

  use self::data_encoding::HEXLOWER;
  use super::*;
  use db_persistence::domain::NewBlock;
  use db_persistence::repository::BlockRepository;

  #[test]
  pub fn can_calculate_block_height() {
    // Given
    let config = Config::load_test();
    let db_connection = PgConnection::establish(&config.db_url).unwrap();

    let new_block0 = NewBlock {
      version: 1,
      hash: HEXLOWER
        .decode(
          b"000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f",
        )
        .unwrap(),
      previous_block_hash: HEXLOWER
        .decode(
          b"0000000000000000000000000000000000000000000000000000000000000000",
        )
        .unwrap(),
      merkle_root: vec![],
      creation_time: 0,
      nonce: 0,
    };

    let new_block1 = NewBlock {
      version: 1,
      hash: HEXLOWER
        .decode(
          b"00000000839a8e6886ab5951d76f411475428afc90947ee320161bbf18eb6048",
        )
        .unwrap(),
      previous_block_hash: HEXLOWER
        .decode(
          b"000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f",
        )
        .unwrap(),
      merkle_root: vec![],
      creation_time: 0,
      nonce: 0,
    };

    let new_block2 = NewBlock {
      version: 1,
      hash: HEXLOWER
        .decode(
          b"000000006a625f06636b8bb6ac7b960a8d03705d1ace08b1a19da3fdcc99ddbd",
        )
        .unwrap(),
      previous_block_hash: HEXLOWER
        .decode(
          b"00000000839a8e6886ab5951d76f411475428afc90947ee320161bbf18eb6048",
        )
        .unwrap(),
      merkle_root: vec![],
      creation_time: 0,
      nonce: 0,
    };

    db_connection.test_transaction::<_, diesel::result::Error, _>(|| {
      // When
      let block_repository = BlockRepository::new(&db_connection);
      let _ = block_repository.save(&new_block0);
      let _ = block_repository.save(&new_block1);
      let _ = block_repository.save(&new_block2);
      let block_height_calculation_task = BlockHeightCalculationTask::new();
      block_height_calculation_task.run(&config, &db_connection);

      // Then
      let saved_blocks = block_repository.read_all();
      assert_eq!(saved_blocks.len(), 3);
      assert_eq!(saved_blocks[0].height, Some(0));
      assert_eq!(saved_blocks[1].height, Some(1));
      assert_eq!(saved_blocks[2].height, Some(2));
      Ok(())
    });
  }
}
