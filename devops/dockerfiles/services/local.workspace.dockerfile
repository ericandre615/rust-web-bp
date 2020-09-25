# select build image
FROM rust:1.46.0 as builder

ENV PKG_CONFIG_ALLOW_CROSS=1
#ENV CARGO_HOME=/home/app/cargo

RUN apt-get update && apt-get -y install ca-certificates cmake musl-tools libssl-dev && rm -rf /var/lib/apt/lists/*

# Static lib linking
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /home/app

# RUN mkdir ./server

RUN USER=root cargo new --bin server
RUN USER=root cargo new --bin data
RUN USER=root cargo new --lib client

# copy over your manifests
COPY ./.env ./.env
COPY ./Cargo.* ./
COPY ./client/Cargo.* ./client/
COPY ./data/Cargo.* ./data/
COPY ./server/Cargo.* ./server/

# this build step will cache your dependencies
RUN cargo build --target x86_64-unknown-linux-musl --release
RUN rm ./server/src/*.rs

#RUN cp -R /home/app/cargo/* /home/app/cargo

COPY ./server/src ./server/src

# WORKDIR /home/app/server

# This step is weird to me, also this is never created so it's an error
# RUN rm ./target/x86_64-unknown-linux-musl/release/deps/server*

# NOPE! this makes you have to recompile again when we just did
# Need to call this to overrwite the hello_world example
# RUN cargo clean

# build for release
RUN cargo build --target x86_64-unknown-linux-musl --release

# our final base
FROM rust:1.46.0-alpine

RUN apk --no-cache add ca-certificates
# copy the build artifact from the build stage
COPY --from=builder /home/app/target/x86_64-unknown-linux-musl/release/server ./usr/bin/

EXPOSE 8888
# set the startup command to run your binary
CMD ["/usr/bin/server"]
