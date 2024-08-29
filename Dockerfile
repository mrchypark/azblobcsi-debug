FROM rust:1.80.1-slim AS chef

WORKDIR /usr/src/project

RUN set -eux; \
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

FROM debian:bullseye-slim

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/project/target/release/project .

CMD ["./project"]