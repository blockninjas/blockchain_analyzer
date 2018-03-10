#!/bin/bash

diesel setup --database-url=$1 --migration-dir=db_persistence/migrations && db_importer /blk_files --database-url=$1
