# Build Stage
FROM rust:1.74.0 as builder

RUN USER=root cargo new --bin rust-auth
WORKDIR ./rust-auth
COPY ./Cargo.toml ./Cargo.toml
# Build empty app with downloaded dependencies to produce a stable image layer for next build
RUN cargo build --release

# Build web app with own code
RUN rm src/*.rs
ADD . ./
RUN rm ./target/release/deps/rust_auth*

ARG DATABASE_URL
RUN cargo install sqlx-cli
COPY ./migrations ./migrations
RUN ls -a
RUN sqlx migrate run
RUN cargo build --release


FROM debian:bookworm-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get upgrade -y \
    && apt-get install -y libssl-dev \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 4000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /rust-auth/target/release/rust-auth ${APP}/rust-auth

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./rust-auth"]