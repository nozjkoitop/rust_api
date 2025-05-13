FROM rust:1.86 AS builder
WORKDIR /usr/src/app

RUN apt-get update \
 && apt-get install -y \
      libpq-dev libssl-dev pkg-config build-essential \
 && cargo install diesel_cli --no-default-features --features postgres \
 && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main(){}" > src/main.rs
RUN cargo fetch

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update \
 && apt-get install -y \
      libpq5 postgresql-client  \
      ca-certificates \
 && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/local/bin

COPY --from=builder /usr/src/app/target/release/salon_api .
COPY --from=builder /usr/local/cargo/bin/diesel .
COPY --from=builder /usr/src/app/migrations ./migrations

ENTRYPOINT ["sh","-c", "\
  until pg_isready -h postgres -p 5432 -U root; do sleep 1; done; \
  echo 'running diesel migrations…'; \
  diesel migration run && \
  echo 'migrations ok, starting server'; \
  exec ./salon_api --bind 0.0.0.0:8080 \
"]

EXPOSE 8080
ENV RUST_LOG=info
