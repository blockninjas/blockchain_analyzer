# Set up PostgreSQL via Docker for local testing

Install the postgres docker image:

```
$ docker pull postgres
```

Start a postgres instance with user "postgrres" and password "test":

```
$ docker run --rm --name blockninjas_postgres -p 5432:5432 -e POSTGRES_PASSWORD=test -d postgres
```

Run `diesel`'s migration scripts to setup the database:

```
$ diesel setup --database-url=postgres://postgres:test@127.0.0.1:5432/bitcoin_blockchain --migration-dir=db_importer/migrations
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
