FROM rust:latest AS builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/simple-api-rust /usr/local/bin/simple-api-rust

ENV DB_PATH=/app/data/app.db

#by default
EXPOSE 5070

CMD ["simple-api-rust"]
