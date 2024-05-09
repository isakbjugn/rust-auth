# Build Stage
FROM rust:latest AS builder

RUN USER=root cargo new --bin rust-auth
WORKDIR ./rust-auth
COPY ./Cargo.toml ./Cargo.toml

# Build empty app with downloaded dependencies to produce a stable image layer for next build
RUN cargo build --release

# Remove unused source and dependencies that must be rebuilt
RUN rm src/*.rs
RUN rm ./target/release/deps/rust_auth*

# Copy the source files and migrations
ADD ./src ./src
ADD ./migrations ./migrations

# Install sqlx CLI and run database migrations
ARG DATABASE_URL
RUN cargo install sqlx-cli
RUN sqlx migrate run

# Build web app with own code
RUN cargo build --release


FROM debian:bookworm-slim AS runtime
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /rust-auth/target/release/rust-auth ${APP}/rust-auth
COPY templates ${APP}/templates

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./rust-auth"]