# Rust Backend Actix Template Clean Architecture

```
src
├── config
│   ├── db
│   │   └── mod.rs
│   └── mod.rs
├── dto
│   ├── health_check.rs
│   ├── mod.rs
│   └── users.rs
├── handler
│   ├── health_check.rs
│   ├── mod.rs
│   ├── post_handler.rs
│   └── user_handler.rs
├── main.rs
├── models.rs
├── repository
│   ├── mod.rs
│   ├── post_repo.rs
│   └── user_repo.rs
├── schema.rs
└── service
│   ├── mod.rs
│   ├── post_service.rs
│   └── user_service.rs
├── migrations
├──Cargo.lock
├──Cargo.toml
├──Dockerfile
├──README.md
├──diesel.toml
```

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
