# BlockNinjas Bitcoin Analysis Suite

## `blk_file_reader`

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

## `database_importer`

Crate which contains a library for importing `.blk` files into a Postgres
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

Install the mysql docker image:

```
$ docker pull mysql
```

Start a mysql instance with root password set to "root":

```
$ docker run --rm --name blockninjas_mysql -p 3306:3306 -e MYSQL_ROOT_PASSWORD=root -d mysql
```

Install mysql-version of diesel:

```
cargo install --force diesel_cli --no-default-features --features mysql
```

Create a `.env` file in the `database_importer` directory with the following
content:

```
DATABASE_URL=mysql://root:root@127.0.0.1:3306/bitcoin_blockchain
```

Run `diesel`'s migration scripts to setup the database:

```
$ diesel setup
```

To inspect the database, first connect to the docker container via:

```
$ docker exec -it blockninjas_mysql bash
```

Once inside the container, the database contents can be inspected via the `mysql`
command-line client:

```
$ mysql --user=root --password=root bitcoin_blockchain
```

The database can be shut down by stopping the docker container:

```
$ docker stop blockninjas_mysql
```
