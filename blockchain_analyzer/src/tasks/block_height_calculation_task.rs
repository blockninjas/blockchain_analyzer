use config::Config;
use db::schema::blocks::dsl::*;
use diesel::{self, prelude::*};
use failure::Error;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use std::result::Result;
use task_manager::{Index, Task};

pub struct BlockHeightCalculationTask {}

impl BlockHeightCalculationTask {
    pub fn new() -> BlockHeightCalculationTask {
        BlockHeightCalculationTask {}
    }
}

impl Task for BlockHeightCalculationTask {
    fn run(
        &self,
        _config: &Config,
        db_connection_pool: &Pool<ConnectionManager<PgConnection>>,
    ) -> Result<(), Error> {
        info!("Run BlockHeightCalculationTask");

        let db_connection = db_connection_pool.get()?;

        db_connection.transaction(|| calculate_height_for_all_blocks(&db_connection))?;

        info!("Finished BlockHeightCalculationTask");

        Ok(())
    }

    fn get_indexes(&self) -> Vec<Index> {
        vec![Index {
            table: String::from("blocks"),
            column: String::from("height"),
            unique: false,
        }]
    }
}

fn calculate_height_for_all_blocks(
    db_connection: &PgConnection,
) -> Result<(), diesel::result::Error> {
    // TODO Do not always start from genesis block.
    let mut current_block_hashes = get_successor_block_hashes(&db_connection, &vec![0u8; 32]);
    let mut current_block_height = 0;

    while !current_block_hashes.is_empty() {
        let mut all_successor_block_hashes = vec![];

        for current_block_hash in current_block_hashes {
            set_block_height(&db_connection, &current_block_hash, current_block_height)?;

            let mut successor_hashes =
                get_successor_block_hashes(&db_connection, &current_block_hash);
            all_successor_block_hashes.append(&mut successor_hashes);
        }

        current_block_hashes = all_successor_block_hashes;
        current_block_height += 1;
    }

    info!("New block height is {}", current_block_height - 1);

    Ok(())
}

fn set_block_height(
    db_connection: &PgConnection,
    current_hash: &[u8],
    current_height: i32,
) -> Result<(), diesel::result::Error> {
    diesel::update(blocks.filter(hash.eq(current_hash)))
        .set(height.eq(current_height))
        .execute(db_connection)?;
    Ok(())
}

fn get_successor_block_hashes(db_connection: &PgConnection, block_hash: &[u8]) -> Vec<Vec<u8>> {
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
    use db::{Block, NewBlkFile, NewBlock};

    fn block0(parent_blk_file_id: i64) -> NewBlock {
        NewBlock {
            version: 1,
            hash: HEXLOWER
                .decode(b"000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f")
                .unwrap(),
            previous_block_hash: HEXLOWER
                .decode(b"0000000000000000000000000000000000000000000000000000000000000000")
                .unwrap(),
            merkle_root: vec![],
            creation_time: 0,
            bits: 0,
            nonce: 0,
            blk_file_id: parent_blk_file_id,
        }
    }

    fn block1a(parent_blk_file_id: i64) -> NewBlock {
        NewBlock {
            version: 1,
            hash: HEXLOWER
                .decode(b"00000000839a8e6886ab5951d76f411475428afc90947ee320161bbf18eb6048")
                .unwrap(),
            previous_block_hash: HEXLOWER
                .decode(b"000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f")
                .unwrap(),
            merkle_root: vec![],
            creation_time: 0,
            bits: 0,
            nonce: 0,
            blk_file_id: parent_blk_file_id,
        }
    }

    fn block1b(parent_blk_file_id: i64) -> NewBlock {
        NewBlock {
            version: 1,
            hash: HEXLOWER
                .decode(b"000000000000000000000000000000000000000000000000000000000000001b")
                .unwrap(),
            previous_block_hash: HEXLOWER
                .decode(b"000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f")
                .unwrap(),
            merkle_root: vec![],
            creation_time: 0,
            bits: 0,
            nonce: 0,
            blk_file_id: parent_blk_file_id,
        }
    }

    fn block2a(parent_blk_file_id: i64) -> NewBlock {
        NewBlock {
            version: 1,
            hash: HEXLOWER
                .decode(b"000000006a625f06636b8bb6ac7b960a8d03705d1ace08b1a19da3fdcc99ddbd")
                .unwrap(),
            previous_block_hash: HEXLOWER
                .decode(b"00000000839a8e6886ab5951d76f411475428afc90947ee320161bbf18eb6048")
                .unwrap(),
            merkle_root: vec![],
            creation_time: 0,
            bits: 0,
            nonce: 0,
            blk_file_id: parent_blk_file_id,
        }
    }

    fn block2b(parent_blk_file_id: i64) -> NewBlock {
        NewBlock {
            version: 1,
            hash: HEXLOWER
                .decode(b"000000000000000000000000000000000000000000000000000000000000002b")
                .unwrap(),
            previous_block_hash: HEXLOWER
                .decode(b"00000000839a8e6886ab5951d76f411475428afc90947ee320161bbf18eb6048")
                .unwrap(),
            merkle_root: vec![],
            creation_time: 0,
            bits: 0,
            nonce: 0,
            blk_file_id: parent_blk_file_id,
        }
    }

    fn block2c(parent_blk_file_id: i64) -> NewBlock {
        NewBlock {
            version: 1,
            hash: HEXLOWER
                .decode(b"000000000000000000000000000000000000000000000000000000000000002c")
                .unwrap(),
            previous_block_hash: HEXLOWER
                .decode(b"000000000000000000000000000000000000000000000000000000000000001b")
                .unwrap(),
            merkle_root: vec![],
            creation_time: 0,
            bits: 0,
            nonce: 0,
            blk_file_id: parent_blk_file_id,
        }
    }

    #[test]
    fn can_calculate_block_height() {
        let config = Config::load_test().unwrap();
        let db_connection = PgConnection::establish(&config.db_url).unwrap();

        db_connection.test_transaction::<_, diesel::result::Error, _>(|| {
            // Given
            let new_blk_file = NewBlkFile {
                name: String::new(),
                number_of_blocks: 0,
            };
            let blk_file = new_blk_file.save(&db_connection).unwrap();

            let new_block0 = block0(blk_file.id);
            let new_block1 = block1a(blk_file.id);
            let new_block2 = block2a(blk_file.id);

            // When
            let _ = new_block0.save(&db_connection).unwrap();
            let _ = new_block1.save(&db_connection).unwrap();
            let _ = new_block2.save(&db_connection).unwrap();
            calculate_height_for_all_blocks(&db_connection).unwrap();

            // Then
            let saved_blocks = Block::read_all(&db_connection).unwrap();
            assert_eq!(saved_blocks.len(), 3);
            assert_eq!(saved_blocks[0].height, Some(0));
            assert_eq!(saved_blocks[1].height, Some(1));
            assert_eq!(saved_blocks[2].height, Some(2));
            Ok(())
        });
    }

    #[test]
    fn can_handle_forks() {
        let config = Config::load_test().unwrap();
        let db_connection = PgConnection::establish(&config.db_url).unwrap();

        db_connection.test_transaction::<_, diesel::result::Error, _>(|| {
            // Given
            let new_blk_file = NewBlkFile {
                name: String::new(),
                number_of_blocks: 0,
            };
            let blk_file = new_blk_file.save(&db_connection).unwrap();

            let new_block0 = block0(blk_file.id);
            let new_block1a = block1a(blk_file.id);
            let new_block1b = block1b(blk_file.id);
            let new_block2a = block2a(blk_file.id);
            let new_block2b = block2b(blk_file.id);
            let new_block2c = block2c(blk_file.id);

            // When
            let _ = new_block0.save(&db_connection).unwrap();
            let _ = new_block1a.save(&db_connection).unwrap();
            let _ = new_block1b.save(&db_connection).unwrap();
            let _ = new_block2a.save(&db_connection).unwrap();
            let _ = new_block2b.save(&db_connection).unwrap();
            let _ = new_block2c.save(&db_connection).unwrap();
            calculate_height_for_all_blocks(&db_connection).unwrap();

            // Then
            let saved_blocks = Block::read_all(&db_connection).unwrap();
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
