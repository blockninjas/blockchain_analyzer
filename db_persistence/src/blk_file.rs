use diesel::{self, pg::PgConnection, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use schema;
use std::result::Result;

#[derive(Queryable)]
pub struct BlkFile {
    pub id: i64,
    pub name: String,
    pub number_of_blocks: i32,
}

impl BlkFile {
    pub fn read_latest_blk_file(
        db_connection: &PgConnection,
    ) -> Result<Option<BlkFile>, diesel::result::Error> {
        schema::blk_files::table
            .order(schema::blk_files::dsl::name.desc())
            .first(db_connection)
            .optional()
    }

    pub fn count(db_connection: &PgConnection) -> Result<i64, diesel::result::Error> {
        schema::blk_files::table.count().get_result(db_connection)
    }

    pub fn read_all(db_connection: &PgConnection) -> Result<Vec<BlkFile>, diesel::result::Error> {
        schema::blk_files::table.load::<BlkFile>(db_connection)
    }

    pub fn read_all_names(
        db_connection: &PgConnection,
    ) -> Result<Vec<String>, diesel::result::Error> {
        schema::blk_files::table
            .select(schema::blk_files::dsl::name)
            .load::<String>(db_connection)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use diesel::{self, prelude::*};
    use NewBlkFile;

    // TODO Make database URL configurable.
    const TEST_DATABASE_URL: &'static str =
        "postgres://postgres:test@127.0.0.1:5432/bitcoin_blockchain";

    #[test]
    pub fn can_save_blk_files() {
        // Given
        let new_blk_file = NewBlkFile {
            name: String::from("blk00000.dat"),
            number_of_blocks: 42,
        };
        let db_connection = PgConnection::establish(TEST_DATABASE_URL).unwrap();

        db_connection.test_transaction::<_, diesel::result::Error, _>(|| {
            // When
            let saved_blk_file = new_blk_file.save(&db_connection)?;

            // Then
            assert_eq!(saved_blk_file.name, new_blk_file.name);
            assert_eq!(
                saved_blk_file.number_of_blocks,
                new_blk_file.number_of_blocks
            );
            Ok(())
        });
    }

    #[test]
    pub fn can_read_all_saved_blk_files() {
        // Given
        let new_blk_file1 = NewBlkFile {
            name: String::from("blk00000.dat"),
            number_of_blocks: 42,
        };
        let new_blk_file2 = NewBlkFile {
            name: String::from("blk00001.dat"),
            number_of_blocks: 43,
        };
        let db_connection = PgConnection::establish(TEST_DATABASE_URL).unwrap();

        db_connection.test_transaction::<_, diesel::result::Error, _>(|| {
            // When
            let _ = new_blk_file1.save(&db_connection)?;
            let _ = new_blk_file2.save(&db_connection)?;
            let blk_files = BlkFile::read_all(&db_connection)?;

            // Then
            assert_eq!(blk_files.len(), 2);
            Ok(())
        });
    }

    #[test]
    pub fn can_read_all_saved_blk_file_names() {
        // Given
        let new_blk_file1 = NewBlkFile {
            name: String::from("blk00000.dat"),
            number_of_blocks: 42,
        };
        let new_blk_file2 = NewBlkFile {
            name: String::from("blk00001.dat"),
            number_of_blocks: 43,
        };
        let db_connection = PgConnection::establish(TEST_DATABASE_URL).unwrap();

        db_connection.test_transaction::<_, diesel::result::Error, _>(|| {
            // When
            let _ = new_blk_file1.save(&db_connection)?;
            let _ = new_blk_file2.save(&db_connection)?;
            let blk_file_names = BlkFile::read_all_names(&db_connection)?;

            // Then
            assert_eq!(blk_file_names.len(), 2);
            assert_eq!(blk_file_names[0], "blk00000.dat");
            assert_eq!(blk_file_names[1], "blk00001.dat");
            Ok(())
        });
    }
}
