# BlockNinjas Bitcoin Analysis Suite

This repository contains a rust workspace consisting of a number of crates that
allow for analysing the bitcoin blockchain.

## Requirements

Since we rely on the ORM-framework `diesel` to import native blockchain data into
a PostgreSQL database it's necessary that an appropriate PostgresSQL driver
library is installed on your system. For more information on `diesel` and
required database drivers refer to its
[official documentation](https://diesel.rs/guides/getting-started/).

If the PostgreSQL driver is in place, install the `diesel` CLI tool via the
following command:

```
cargo install diesel_cli --no-default-features --features postgres
```

## Build

In order to build the `analysis_suite` run the following command from the
workspace root:

```
cargo build
```

This will also download and build all the required dependencies which might
take a while, if done for the first time.

Instead of building the whole workspace it's also possible to build only a
particular crate. This can be achieved via the `--path` option (or `-p` for short).
For example, in order to build the `blk_file_reader`, run the following command
from the workspace root:

```
cargo build -p blk_file_reader
```

## Run Tests

The tests that are provided by the different crates can be run via the
following command from the workspace root:

```
cargo test
```

Again, to run the tests of a specific crate use `--path`/`p`.

```
cargo test -p blk_file_reader
```

Some of the tests require a PostgreSQL database instance which currently has to
be reachable via the following URL (which should obviously be configurable):

```
postgres://postgres:test@127.0.0.1:5432/bitcoin_blockchain
```

Also see this [short summary](./docs/DOCKER_POSTGRES.md) on how to use docker to set up a local database
for testing.
