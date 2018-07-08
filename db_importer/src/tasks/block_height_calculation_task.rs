use config::Config;
use db_persistence::schema::blocks::dsl::*;
use diesel::{self, prelude::*};
use {Index, Task};

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
      let mut current_block_hashes =
        get_successor_block_hashes(db_connection, &vec![0u8; 32]);
      let mut current_block_height = 0;

      while !current_block_hashes.is_empty() {
        let mut all_successor_block_hashes = vec![];

        for current_block_hash in current_block_hashes {
          set_block_height(
            db_connection,
            &current_block_hash,
            current_block_height,
          );

          let mut successor_hashes = get_successor_block_hashes(db_connection, &current_block_hash);
          all_successor_block_hashes.append(&mut successor_hashes);
        }

        current_block_hashes = all_successor_block_hashes;
        current_block_height += 1;
      }

      info!("New block height is {}", current_block_height);

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

fn get_successor_block_hashes(
  db_connection: &PgConnection,
  block_hash: &[u8],
) -> Vec<Vec<u8>> {
  // TODO Return error instead of panicking.
  blocks
    .select(hash)
    .filter(previous_block_hash.eq(block_hash))
    .load(db_connection)
    .unwrap()
}

#[cfg(test)]
mod test {

  extern crate data_encoding;

  use self::data_encoding::HEXLOWER;
  use super::*;
  use db_persistence::domain::NewBlock;
  use db_persistence::repository::BlockRepository;

  fn block0() -> NewBlock {
    NewBlock {
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
    }
  }

  fn block1a() -> NewBlock {
    NewBlock {
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
    }
  }

  fn block1b() -> NewBlock {
    NewBlock {
      version: 1,
      hash: HEXLOWER
        .decode(
          b"000000000000000000000000000000000000000000000000000000000000001b",
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
    }
  }

  fn block2a() -> NewBlock {
    NewBlock {
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
    }
  }

  fn block2b() -> NewBlock {
    NewBlock {
      version: 1,
      hash: HEXLOWER
        .decode(
          b"000000000000000000000000000000000000000000000000000000000000002b",
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
    }
  }

  fn block2c() -> NewBlock {
    NewBlock {
      version: 1,
      hash: HEXLOWER
        .decode(
          b"000000000000000000000000000000000000000000000000000000000000002c",
        )
        .unwrap(),
      previous_block_hash: HEXLOWER
        .decode(
          b"000000000000000000000000000000000000000000000000000000000000001b",
        )
        .unwrap(),
      merkle_root: vec![],
      creation_time: 0,
      nonce: 0,
    }
  }

  #[test]
  fn can_calculate_block_height() {
    // Given
    let config = Config::load_test();
    let db_connection = PgConnection::establish(&config.db_url).unwrap();

    let new_block0 = block0();
    let new_block1 = block1a();
    let new_block2 = block2a();

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

  #[test]
  fn can_handle_forks() {
    // Given
    let config = Config::load_test();
    let db_connection = PgConnection::establish(&config.db_url).unwrap();

    let new_block0 = block0();
    let new_block1a = block1a();
    let new_block1b = block1b();
    let new_block2a = block2a();
    let new_block2b = block2b();
    let new_block2c = block2c();

    db_connection.test_transaction::<_, diesel::result::Error, _>(|| {
      // When
      let block_repository = BlockRepository::new(&db_connection);
      let _ = block_repository.save(&new_block0);
      let _ = block_repository.save(&new_block1a);
      let _ = block_repository.save(&new_block1b);
      let _ = block_repository.save(&new_block2a);
      let _ = block_repository.save(&new_block2b);
      let _ = block_repository.save(&new_block2c);
      let block_height_calculation_task = BlockHeightCalculationTask::new();
      block_height_calculation_task.run(&config, &db_connection);

      // Then
      let saved_blocks = block_repository.read_all();
      assert_eq!(saved_blocks.len(), 6);
      assert_eq!(saved_blocks[0].height, Some(0));
      assert_eq!(saved_blocks[1].height, Some(1));
      assert_eq!(saved_blocks[2].height, Some(1));
      assert_eq!(saved_blocks[3].height, Some(2));
      assert_eq!(saved_blocks[4].height, Some(2));
      assert_eq!(saved_blocks[5].height, Some(2));
      Ok(())
    });
  }
}
