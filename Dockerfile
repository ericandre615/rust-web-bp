# select build image
FROM rust:1.46.0 as builder

ENV PKG_CONFIG_ALLOW_CROSS=1

RUN apt-get update && apt-get -y install ca-certificates cmake musl-tools libssl-dev && rm -rf /var/lib/apt/lists/*

# Static lib linking
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /home/app

RUN USER=root cargo new --bin server
RUN USER=root cargo new --bin data
RUN USER=root cargo new --lib client

COPY ./Cargo.* ./
COPY ./client/Cargo.* ./client/
COPY ./data/Cargo.* ./data/
COPY ./server/Cargo.* ./server/

# this build step will cache your dependencies
RUN cargo build --target x86_64-unknown-linux-musl --release

# copy your source tree
COPY ./data/src ./data/src
COPY ./client/src ./client/src
COPY ./server/src ./server/src

# our final base
FROM rust:1.46.0-alpine

RUN apk --no-cache add ca-certificates
# copy the build artifact from the build stage
COPY --from=builder /home/app/target/x86_64-unknown-linux-musl/release ./home/app/target/x86_64-unknown-linux-musl/release
