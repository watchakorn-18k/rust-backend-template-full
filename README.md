# Rust Backend Actix Template Clean Architecture

## Structure

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

## Spec API

```bash
curl http://localhost:1323/health-check
curl http://localhost:1323/users
curl http://localhost:1323/posts
```

## Mock Data

- Generate 10,000 users

```bash
INSERT INTO users (id, name, age)
SELECT g AS id,
       'User_' || g AS name,
       (RANDOM() * 60 + 18)::INT AS age   -- อายุสุ่มระหว่าง 18–78
FROM generate_series(1, 10000) g;
```

- Generate 10,000 posts

```bash
INSERT INTO post (id, name)
SELECT g AS id,
       INITCAP(
         (ARRAY['Neo','Trinity','Morpheus','Zion','Matrix','Cipher','Rogue','Shadow',
                'Blade','Storm','Viper','Ghost','Hunter','Falcon','Raptor','Venom',
                'Quantum','Nova','Lunar','Solar','Oblivion','Cyber','Zero','Alpha',
                'Omega','Echo','Drift','Pulse','Phantom','Inferno','Zenith'])[ (random()*27+1)::int ]
         || '_' ||
         (ARRAY['X','Y','Z','One','Prime','Core','Edge','Flux','Byte','Code','Hack',
                'Shift','Dark','Light','Ultra','Max','Vision','Net','Storm','Gear'])[ (random()*19+1)::int ]
         || '_' || substr(md5(random()::text), 1, 4)
       ) AS name
FROM generate_series(1,10000) g;
```

## Docker

```bash
docker build -t backend-rust .
docker run --rm -p 1323:1323 -e PORT=1323 backend-rust
```
