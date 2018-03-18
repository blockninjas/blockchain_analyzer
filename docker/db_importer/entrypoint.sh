#!/bin/sh

# From where the `db_importer` reads blk files.
BLK_FILE_ROOT=~/.bitcoin/blocks

diesel setup --database-url=$1 --migration-dir=/analysis_suite/db_persistence/migrations && db_importer --database-url=$1 $BLK_FILE_ROOT
