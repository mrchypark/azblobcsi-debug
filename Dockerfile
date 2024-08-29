# syntax=docker/dockerfile:1
FROM rust:1.55.0-alpine AS chef

WORKDIR /usr/src/project

RUN set -eux; \
    apk add --no-cache musl-dev; \
    cargo install cargo-chef; \
    rm -rf $CARGO_HOME/registry

FROM chef as planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /usr/src/project/recipe.json .
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release

FROM alpine:3.14

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/project/target/release/project .

CMD ["./project"]