# BlockNinjas Bitcoin Analysis Suite

## blk_file_reader

Crate which contains a library for parsing `.blk` files. Also provides a
binary which exposes the library's functionality as CLI tool.

### Build

Both the library and the binary can be built via:

```
$ cargo build -p blk_file_reader
```

### Run tests

```
$ cargo test -p blk_file_reader
```

### Run the `blk_file_reader` binary

```
$ cargo run -p blk_file_reader
```

Command line arguments can be passed by appending them to the `cargo run`
command after `--`. For example, to print the `blk_file_reader`'s help-text pass
`-h`:

```
$ cargo run -p blk_file_reader -- -h
```

## database_importer

Crate which contains a library for importing `.blk` files into a postgres
database. Also provides a binary which exposes the library's functionality as
CLI tool.

### Build

```
$ cargo build -p database_importer
```

### Run tests

```
$ cargo build -p database_importer
```

### Exemplary setup for local testing

Install the postgres docker image:

```
$ docker pull postgres
```

Start a postgres instance with user "postgrres" and password "test":

```
$ docker run --rm --name bitcoin_blockchain -p 5432:5432 -e POSTGRES_PASSWORD=test -d postgres
```

Install diesel:

```
cargo install diesel_cli
```

Create a `.env` file in the `database_importer` directory with the following
content:

```
DATABASE_URL=postgres://postgres:test@127.0.0.1:5432/bitcoin_blockchain
```

Run `diesel`'s migration scripts to setup the database:

```
$ diesel setup
```

To inspect the database, first connect to the docker container via:

```
$ docker exec -it blockninjas_postgres bash
```

Once inside the container, the database contents can be inspected via the `psql`
command-line client:

```
$ su postgres
$ psql -d bitcoin_blockchain
```

The database can be shut down by stopping the docker container:

```
$ docker stop blockninjas_postgres
```
