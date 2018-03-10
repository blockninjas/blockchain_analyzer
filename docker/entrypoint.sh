#!/bin/bash
diesel setup --database-url=$1 --migration-dir=db_persistence/migrations && db_importer --database-url=$1
