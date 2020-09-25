# select build image
FROM rust:1.46.0 as builder

ENV PKG_CONFIG_ALLOW_CROSS=1
ARG RUST_TARGET=x86_64-unknown-linux-musl
ARG RUST_BUILD_MODE=release

RUN apt-get update && apt-get -y install ca-certificates cmake musl-tools libssl-dev && rm -rf /var/lib/apt/lists/*

# Static lib linking
RUN rustup target add ${RUST_TARGET}

WORKDIR /home/app

RUN USER=root cargo new --bin server

COPY ./.env ./.env

# copy over your manifests
COPY ./server/Cargo.* ./server/

WORKDIR /home/app/server
# this build step will cache your dependencies
RUN cargo build --target ${RUST_TARGET} --${RUST_BUILD_MODE}
RUN rm ./src/*.rs

COPY ./server/src/ ./src/

# This step is weird to me, also this is never created so it's an error
RUN rm ./target/${RUST_TARGET}/${RUST_BUILD_MODE}/deps/server*

# build for release
RUN cargo build --target ${RUST_TARGET} --${RUST_BUILD_MODE}

# our final base
FROM rust:1.46.0-alpine
# each FROM has a different ARG scope
ARG RUST_TARGET=x86_64-unknown-linux-musl
ARG RUST_BUILD_MODE=release

RUN apk --no-cache add ca-certificates
# copy the build artifact from the build stage
COPY --from=builder /home/app/server/target/${RUST_TARGET}/${RUST_BUILD_MODE}/server ./usr/bin/

EXPOSE 8888
# set the startup command to run your binary
CMD ["/usr/bin/server"]
