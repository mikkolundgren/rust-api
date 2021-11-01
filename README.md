Simple rest api with rust rocket, diesel and postgres db

## setup postgres

``docker run --name some-postgres -e POSTGRES_PASSWORD=postgres -p 8888:5432 -d postgres``

## setup diesel

See: https://diesel.rs/guides/getting-started

You might have to install pg natively for needed libraries. Set PQ_LIB_DIR to path to postgres libraries. for ex ``export PQ_LIB_DIR=/Library/PostgreSQL/14/lib/``

Set pg connection string to .env

```
diesel setup
diesel migration generate albums
...
diesel migration run
diesel migration redo
```
see migrations/up.sql and down.sql for creating tables etc.

## running the web server

Run with ``cargo run``

You should see something like:
```
...
🛰  Mounting /api/v1/:
    => GET /api/v1/albums application/json (get_all)
    => POST /api/v1/albums application/json (new_album)
🚀 Rocket has launched from http://localhost:8000
```
