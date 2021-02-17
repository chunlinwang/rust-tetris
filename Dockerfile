FROM rust:1.50 AS base

ENV USER=root

RUN apt-get update && apt-get install -y libghc-postgresql-libpq-dev \
pkg-config libssl-dev argon2 clang llvm-dev libclang-dev \
libxcb-shape0-dev libxcb-xfixes0-dev

WORKDIR /code
RUN cargo init
COPY . /code/

RUN cargo fetch
