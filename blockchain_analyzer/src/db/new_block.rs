use super::Block;
use blk_file_reader;
use diesel::{self, prelude::*};
use schema::blocks;
use std::result::Result;

#[derive(Insertable, Default)]
#[table_name = "blocks"]
pub struct NewBlock {
    pub hash: Vec<u8>,
    pub version: i32,
    pub previous_block_hash: Vec<u8>,
    pub merkle_root: Vec<u8>,
    pub creation_time: i32,
    pub bits: i32,
    pub nonce: i32,
    pub blk_file_id: i64,
}

impl NewBlock {
    pub fn new(block: &blk_file_reader::Block, blk_file_id: i64) -> NewBlock {
        NewBlock {
            hash: block.hash.0.to_vec(),
            version: block.version as i32,
            previous_block_hash: block.previous_block_hash.0.to_vec(),
            merkle_root: block.merkle_root.0.to_vec(),
            creation_time: block.creation_time as i32,
            bits: block.bits as i32,
            nonce: block.nonce as i32,
            blk_file_id,
        }
    }

    pub fn save(&self, db_connection: &PgConnection) -> Result<Block, diesel::result::Error> {
        diesel::insert_into(blocks::table)
            .values(self)
            .get_result(db_connection)
    }
}

#[cfg(test)]
mod test {

    extern crate data_encoding;
    extern crate diesel;

    use self::data_encoding::HEXLOWER;
    use super::*;
    use db::NewBlkFile;
    use diesel::result::Error;

    // TODO Make database URL configurable.
    const TEST_DATABASE_URL: &'static str =
        "postgres://postgres:test@127.0.0.1:5432/bitcoin_blockchain";

    #[test]
    pub fn can_save_block() {
        let db_connection = PgConnection::establish(TEST_DATABASE_URL).unwrap();

        db_connection.test_transaction::<_, Error, _>(|| {
            // Given
            let new_blk_file = NewBlkFile {
                name: String::new(),
                number_of_blocks: 0,
            };
            let blk_file = new_blk_file.save(&db_connection).unwrap();

            let new_block = NewBlock {
                version: 1,
                hash: HEXLOWER
                    .decode(b"000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f")
                    .unwrap(),
                previous_block_hash: HEXLOWER
                    .decode(b"0000000000000000000000000000000000000000000000000000000000000000")
                    .unwrap(),
                merkle_root: HEXLOWER
                    .decode(b"4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b")
                    .unwrap(),
                creation_time: 1231006505,
                bits: 486604799,
                nonce: 2083236893,
                blk_file_id: blk_file.id,
            };

            // When
            let saved_block = new_block.save(&db_connection).unwrap();

            // Then
            assert_eq!(saved_block.version, new_block.version);
            assert_eq!(saved_block.hash, new_block.hash);
            assert_eq!(
                saved_block.previous_block_hash,
                new_block.previous_block_hash
            );
            assert_eq!(saved_block.merkle_root, new_block.merkle_root);
            assert_eq!(saved_block.creation_time, new_block.creation_time);
            assert_eq!(saved_block.nonce, new_block.nonce);
            assert_eq!(saved_block.height, None);
            Ok(())
        });
    }

    #[test]
    pub fn conversions_are_safe() {
        let db_connection = PgConnection::establish(TEST_DATABASE_URL).unwrap();

        db_connection.test_transaction::<_, Error, _>(|| {
            // Given
            let new_blk_file = NewBlkFile {
                name: String::new(),
                number_of_blocks: 0,
            };
            let blk_file = new_blk_file.save(&db_connection).unwrap();

            let new_block = NewBlock {
                version: (u32::max_value() - 1) as i32,
                hash: vec![],
                previous_block_hash: vec![],
                merkle_root: vec![],
                creation_time: 1231006505,
                bits: 486604799,
                nonce: 2083236893,
                blk_file_id: blk_file.id,
            };

            // When
            let saved_block = new_block.save(&db_connection).unwrap();

            // Then
            assert_eq!(saved_block.version as u32, u32::max_value() - 1);
            Ok(())
        });
    }
}
