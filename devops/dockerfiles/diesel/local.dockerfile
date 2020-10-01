FROM rust:1.46.0 AS builder

ENV DIESEL_CLI_VERSION 1.3.1

RUN apt-get update && \
    apt-get install -y \
    libpq-dev && \
    rm -rf /var/lib/apt/lists/*

RUN cargo install diesel_cli --version ${DIESEL_CLI_VERSION} --no-default-features --features postgres

# need to copy over deps too, such as libpq.so.5 into the stretch-slim image
FROM debian:stretch-slim

# lots of library deps for diesel
COPY --from=builder /usr/local/cargo/bin/diesel /bin/diesel
COPY --from=builder /usr/lib/x86_64-linux-gnu/ /usr/lib/x86_64-linux-gnu/
COPY --from=builder /lib/x86_64-linux-gnu/ /lib/x86_64-linux-gnu/

WORKDIR /usr/src

CMD [ "diesel" ]
