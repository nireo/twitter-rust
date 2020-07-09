# Twitter in Rust

> A back-end twitter clone built in Rust.

## Features

- CRUD Operations with tweets
- Login/Register
- Sessions
- Data persistance with database

## Running code

In addition to just running the code you need to create `.env` file where things like hosts, ports and a database url are stored. You also need install PostgreSQL for the database and Redis for caching sessions.

```env
# .env 127.0.0.1=localhost
DATABASE_URL=postgres://postgres:password@localhost/twitter-rust
REDIS_HOST=127.0.0.1
REDIS_PORT=6379
PORT=8080
HOST=127.0.0.1
RUST_LOG=twitter_rust=info,actic=info
```

```sh
git clone https://github.com/nireo/twitter-rust
cd twitter-rust
cargo run
```
