# Ebook to RSS-Feed converter

## Status

### Complete

- Project structure
- DB-Schema
- JSON Readonly API
- RSS generation
- CLI import of m4b files

### Todo

- JSON Update/Write API
- Web frontend

## Installation

You'll need rust nightly.

1. Install rustup:
```bash
curl https://sh.rustup.rs -sSf > rustup.sh
sh rustup.sh
```
2. Switch to rust nightly:
```bash
rustup install nightly
rustup default nightly
```
3. Install `diesel_cli` (You need sqlite development headers installed)
```bash
cargo install diesel_cli --no-default-features --features "sqlite"
```
4. Migrate the DB
```bash
diesel migration --database-url ./data/db.sqlite run
```
5. Build
```bash
cargo build
```

## Configuration

Place a configuration file in the format of your liking in one of these places:

- `~/.config/bookrss.{toml,json,yaml}`
- `~/.bookrss.{toml,json,yaml}`
- `/etc/bookrss.{toml,json,yaml}`

Example config in toml format:

```toml
[server]
workers = 4
log = "normal"
template_dir = "./templates/"
address = "127.0.0.1"
port = 8000
secret_key = "<secret_key>"

[database]
url = "./data/db.sqlite"

[path]
data_path = "./data/"
external_url = "http://127.0.0.1:8000"

[audible]
activation_bytes = "<audible_activation_bytes_hex>"

[server.limits]
forms = 32768
```

- To generate a secret key use the following command: `openssl rand -base64 32`
- To fetch your audible activation bytes see https://github.com/pkillnine/audible-activator-robobrowser

You may skip the `[audible]` section if you do not plan to import `aax` or `aa` files.

## CLI Tool

### Importing audio books

Run via: `cargo run --bin bookrss -- import --series "<series_name>" --part <book_number> <audio_file>`

If the series only has one part (so it is not really a series) you may skip the `--part`. If your audio book is split into multiple audio files, use the `-x` flag and import them in the right order.

## HTTP API

Run via: `cargo run --bin bookrssd`

### Audiobook

- `GET /audiobooks?<query>`
  Fetch a list of audio books. Available filters:
    - `author_id`
    - `series_id`
    - `narrator_id`
    - `translation`
- `GET /audiobooks`
  Fetch a list of all audio books
- `GET /audiobook/<id>`
  Fetch audio book by id

### Author

- `GET /authors?<query>`
  Fetch a list of authors. Available filters:
    - `language`
- `GET /authors`
  Fetch a list of all authors
- `GET /author/<id>`
  Fetch author by id

### Narrator

- `GET /narrators?<query>`
  Fetch a list of narrators. Available filters:
    - `language`
- `GET /narrators`
  Fetch a list of all narrators
- `GET /narrator/<id>`
  Fetch narrator by id

### Series

- `GET /series?<query>`
  Fetch a list of all book series. Available filters:
    - `author_id`
    - `translation`
- `GET /series`
  Fetch a list of all book series
- `GET /series/<id>`
  Fetch a book series by id

### Parts

- `GET /parts/<audiobook_id>`
  Fetch a list of parts that belong to the audio book with the id
- `GET /part/<id>`
  Fetch the actual binary file that contains the part

### Cover art

- `GET /cover/<part_id>.jpg`
  Fetch a podcast cover art image

### RSS Feeds

- `GET /series_rss/<slug>`
  Fetch a podcast feed for a series
- `GET /audiobook_rss/<slug>`
  Fetch a podcast feed for a single book
