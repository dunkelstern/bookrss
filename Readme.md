# Ebook to RSS-Feed converter

## Status

### Complete

- Project structure
- DB-Schema
- JSON Readonly API

### Todo

- JSON Update/Write API
- RSS generation
- CLI import of m4b files
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
5. Build and run
```bash
cargo run
```

## API

### Audiobook

- `GET /audiobooks?<query>`
  Fetch a list of audio books. Available filters:
    - `author_id`
    - `series_id`
    - `speaker_id`
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

### Speaker

- `GET /speakers?<query>`
  Fetch a list of speakers. Available filters:
    - `language`
- `GET /speakers`
  Fetch a list of all speakers
- `GET /speaker/<id>`
  Fetch speaker by id

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
