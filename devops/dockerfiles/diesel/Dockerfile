FROM rust:1.46.0 AS builder

ENV DIESEL_CLI_VERSION 1.3.1

RUN apt-get update && \
    apt-get install -y \
    libpq-dev && \
    rm -rf /var/lib/apt/lists/*

RUN cargo install diesel_cli --version ${DIESEL_CLI_VERSION} --no-default-features --features postgres

FROM debian:stretch-slim

COPY --from=builder /usr/local/cargo/bin/diesel /bin/diesel

WORKDIR /usr/src

CMD [ "diesel" ]
