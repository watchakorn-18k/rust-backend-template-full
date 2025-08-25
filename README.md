# Rust Backend Actix

## Run

```bash
cargo build
cargo watch -x run
```

## Connect Diesel

```bash
cargo install diesel_cli --no-default-features --features postgres
cargo install diesel_cli_ext
diesel setup
diesel migration generate create_table --diff-schema
diesel migration run
```

- Generate schema.rs ถ้ายังไม่มี schema.rs

```
diesel print-schema > src/schema.rs
```

- Generate models.rs

```
diesel_ext --model -t -d "Queryable, Debug, Clone, Serialize, Deserialize" > src/models.rs
```

## Docker

```bash
docker build -t backend-rust .
docker run --rm -p 1323:1323 -e PORT=1323 backend-rust
```
