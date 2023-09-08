# Backend Practice Project

The simple implementation of stack overflow API using Rust.

## DB set up

PostgreSQL is used as the database for this project. To set up the database, run the following command:

```bash
$ docker run --name stack-overflow-db -e POSTGRES_PASSWORD=postgrespw -p 55008:5432 -d postgres
```

A `.env`  contains DATABASE_URL which should point to your local Postgres instance. Double-check that the url string is correct (specifically the port number).

First, install sqlx-cli. This is SQLx's associated command-line utility for managing databases, migrations, and more:

```bash
$ cargo install sqlx-cli
```

Then, run the following command to create the database:

```bash
$ sqlx migrate run
```

To revert the database, run the following command:

```bash
$ sqlx migrate revert
```

## Run the server

```bash
$ cargo run
```

to show the log

`cmd`
```
set RUST_LOG=info
cargo run
```
`bash`
```
RUST_LOG=info cargo run
```