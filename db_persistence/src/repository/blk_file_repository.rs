use diesel::{self, pg::PgConnection, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use domain::BlkFile;
use domain::NewBlkFile;
use schema;
use std::result::Result;

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

pub fn read_all_names(db_connection: &PgConnection) -> Result<Vec<String>, diesel::result::Error> {
    schema::blk_files::table
        .select(schema::blk_files::dsl::name)
        .load::<String>(db_connection)
}

pub fn save(
    db_connection: &PgConnection,
    new_blk_file: &NewBlkFile,
) -> Result<BlkFile, diesel::result::Error> {
    // TODO Return error instead of panicking.
    diesel::insert_into(schema::blk_files::table)
        .values(new_blk_file)
        .get_result(db_connection)
}

#[cfg(test)]
mod test {

    use super::*;
    use diesel::{self, prelude::*};

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
            let saved_blk_file = save(&db_connection, &new_blk_file)?;

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
            let _ = save(&db_connection, &new_blk_file1)?;
            let _ = save(&db_connection, &new_blk_file2)?;
            let blk_files = read_all(&db_connection)?;

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
            let _ = save(&db_connection, &new_blk_file1)?;
            let _ = save(&db_connection, &new_blk_file2)?;
            let blk_file_names = read_all_names(&db_connection)?;

            // Then
            assert_eq!(blk_file_names.len(), 2);
            assert_eq!(blk_file_names[0], "blk00000.dat");
            assert_eq!(blk_file_names[1], "blk00001.dat");
            Ok(())
        });
    }
}
