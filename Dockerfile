FROM rust:latest AS chef
RUN cargo install cargo-chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
ARG DATABASE_URL
RUN cargo install sqlx-cli
COPY ./migrations ./migrations
RUN ls -a
RUN sqlx migrate run
RUN cargo build --release --bin rust-auth

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR app
COPY --from=builder /app/target/release/rust-auth /usr/local/bin
ARG PORT
EXPOSE $PORT
RUN apt-get update && apt-get install -y libssl-dev
ENTRYPOINT ["/usr/local/bin/rust-auth"]