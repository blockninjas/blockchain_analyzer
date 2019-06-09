# Blockchain Analyzer

The `blockchain_analyzer`

* deserializes the raw bitcoin blockchain,
* imports it into a Postgres database,
* and analyzes transactions to find clusters of addresses.

![System Architecture](./system-architecture.png)

## Building

* Everything: `cargo build`
* `blk_file_reader` only: `cargo build -p blk_file_reader`
* `blockchain_analyzer` only: `cargo build -p blockchain_analyzer`

## Running

### `blk_file_reader`

Run `cargo run -p blk_file_reader -- -h` to get a help text for `blk_file_reader`.

### `blockchain_analyzer`

Running the `blockchain_analyzer` requires a little bit of configuration which
is done by setting a handful of environment variables. If the variables are not
set in the environment, the `blockchain_analyzer` tries to read them from a
`.env` file at the path from where it was invoked. These variables are
described in [.env.sample](./.env.sample).

There fore, you can simply

* make a copy of `.env.sample`: `cp .env.sample .env`
* and adapt the variables in `.env` to your needs.

The `blockchain_analyzer` can then be invoked via `cargo run -p blockchain_analyzer`.

## Testing

Running the tests requires additional tools:

* Docker: for example [Docker Community Edition](https://docs.docker.com/install/).
* `diesel_cli`: `cargo install diesel_cli --no-default-features --features postgres`:

Before your run the tests, you have to:

* Spin up a Postgres DB via Docker: `docker run --rm --name blockninjas_postgres -p 5432:5432 -e POSTGRES_PASSWORD=test postgres`
* Set up the DB via `diesel_cli`: `diesel database reset --database-url=postgres://postgres:test@127.0.0.1:5432/bitcoin_blockchain --migration-dir=blockchain_analyzer/migration`

Then run the tests:

* Everything: `cargo test`
* `blk_file_reader` only: `cargo test -p blk_file_reader`
* `blockchain_analyzer` only: `cargo test -p blockchain_analyzer`
