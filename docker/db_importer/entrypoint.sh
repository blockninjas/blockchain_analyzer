#!/bin/sh

# From where to read the blk files to import into the database.
BLK_FILE_DIR=/home/blockninjas/blk_files

diesel setup --database-url=$1 --migration-dir=/analysis_suite/db_persistence/migrations && db_importer --database-url=$1 $BLK_FILE_DIR
