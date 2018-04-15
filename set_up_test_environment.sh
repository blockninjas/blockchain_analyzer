#!/bin/bash

docker-compose -f docker-compose.tests.yml up -d \
  && sleep 2 \
  && diesel setup --database-url=postgres://postgres:test@127.0.0.1:5432/bitcoin_blockchain --migration-dir=db_persistence/migrations
