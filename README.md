# Usage

## Local deployment

### Requirements

1. Rust
2. Mongo instance

### How to run

1. Setup mongodb - docker example:
```bash
docker run -d --name some-mongo -p 27017:27017\
        -e MONGO_INITDB_ROOT_USERNAME=mongoadmin \
        -e MONGO_INITDB_ROOT_PASSWORD=secret \
        mongo
```
2. Create `api_to_automate` database in mongo you just started

3. Export env variables:
```bash
export MONGODB_URI=mongodb://mongoadmin:secret@127.0.0.1
export MONGODB_DATABASE=api_to_automate
```
4. Start application with `cargo run`

## Unit testing

There are simple unit tests in this api written. Start them with `cargo test`
