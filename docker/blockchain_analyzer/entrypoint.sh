#!/bin/bash

echo DATABASE_URL=$DATABASE_URL
echo BLK_FILE_PATH=$BLK_FILE_PATH

diesel setup --database-url=${DATABASE_URL} --migration-dir=/analysis_suite/db_persistence/migrations && blockchain_analyzer
