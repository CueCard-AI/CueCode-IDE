FROM rust:1-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
COPY tooling ./tooling
RUN cargo build --release -p harness_stub --bin harness-stub

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/harness-stub /usr/local/bin/harness-stub
ENV HARNESS_PORT=8787
EXPOSE 8787
CMD ["harness-stub"]
