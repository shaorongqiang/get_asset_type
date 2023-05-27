FROM docker.io/rust:slim AS builder

RUN apt-get update -y && apt-get install -y musl-tools libssl-dev pkg-config make perl
ENV OPENSSL_LIB_DIR="/usr/lib/x86_64-linux-gnu"
ENV OPENSSL_INCLUDE_DIR="/usr/include/openssl"
 
RUN rustup target add x86_64-unknown-linux-musl

COPY . ./project
WORKDIR /project
RUN cargo build --release --target x86_64-unknown-linux-musl


FROM docker.io/busybox:latest
COPY --from=builder  /project/target/x86_64-unknown-linux-musl/release/get_asset_type  /
