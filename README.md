# rust-redis-playground
Interacting with redis in async rust

## Use

- Create a redis container:
```
docker run --name my-redis-instance --rm -p 6379:6379 -it redis:6 -- --loglevel verbose
```

- Run cargo watch:
```
cargo watch -qcx 'run -q'
```
Note: you need to install cargo watch if not installed already.

- Ready to experiment in `src/main.rs`

