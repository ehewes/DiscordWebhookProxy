FROM rustlang/rust:nightly AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config libssl-dev

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY README.md ./
COPY Rocket.toml ./

RUN cargo +nightly build --release -Z unstable-options

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/discord-webhook-proxy .
COPY Rocket.toml ./

EXPOSE 8000

CMD ["./discord-webhook-proxy"]