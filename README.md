# BlockNinjas Bitcoin Analysis Suite

This repository contains a cargo workspace consisting of a number of crates that
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

Some tests require external services such as a PostgreSQL database or a Redis
instance. A suitable test environment can easily be set up with the help of
docker.

For this purpose make sure to

* install [docker](https://www.docker.com/community-edition#/download),
* install [docker-compose](https://docs.docker.com/compose/install/),
* and copy the `.env.sample` to `.env`.

Based on the container definitions in `docker-compose.tests.yml` you can now
start up your test environment:

```
docker-compose -f docker-compose.tests.yml up
```

The tests that are provided by the different crates can then be run via the
following command from the workspace root:

```
cargo test
```

Again, use `-p` to run the tests of a specific crate:

```
cargo test -p blk_file_reader
```

If you are finished, the docker-based test environment can be stopped via:

```
docker-compose -f docker-compose.tests.yml down
```
