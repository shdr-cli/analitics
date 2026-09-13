FROM rust:latest AS builder

WORKDIR /app

COPY rust/Cargo.toml rust/Cargo.lock ./
COPY rust/src ./src
COPY .env ./src

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rust /app/rust

ENTRYPOINT ["/app/rust"]