# เลือกกำหนด rust version เดียวจบทั้งไฟล์
ARG RUST_VERSION=1.83  # หรือใช้ 'bookworm' เพื่อเอา stable ล่าสุดเสมอ

# ---------- Stage 0: Planner ----------
FROM rust:1.83-bookworm AS chef
WORKDIR /app
RUN cargo install cargo-chef --locked
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ---------- Stage 1: Cacher ----------
FROM rust:1.83-bookworm AS cacher
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev libpq-dev \
 && rm -rf /var/lib/apt/lists/*
RUN cargo install cargo-chef --locked
COPY --from=chef /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# ---------- Stage 2: Builder ----------
FROM rust:1.83-bookworm AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev libpq-dev \
 && rm -rf /var/lib/apt/lists/*
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
COPY . .

RUN cargo build --release --bin wk18k-api

# ---------- Stage 3: Runtime ----------
FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates tzdata libssl3 libpq5 \
 && rm -rf /var/lib/apt/lists/* && update-ca-certificates
ENV TZ=Asia/Bangkok RUST_LOG=info PORT=1323
RUN useradd -m -u 10001 appuser
COPY --from=builder /app/target/release/wk18k-api /app/wk18k-api
# Copy the .env file, credentials, and swagger docs
COPY --from=builder /app/.env.dev /app/.env
EXPOSE 1323
USER appuser
CMD ["/app/wk18k-api"]
