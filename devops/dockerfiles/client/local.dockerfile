FROM rust:1.46.0 as builder

RUN rustup target add wasm32-unknown-unknown

RUN cargo install wasm-pack

WORKDIR /home/app

RUN USER=root cargo new --lib client

COPY ./.env ./.env
COPY ./client/Cargo.* ./client/

WORKDIR /home/app/client
# cache dependency compiles
RUN wasm-pack build --target web --out-name wasm --out-dir ../static/build
RUN rm ./src/*.rs

COPY ./static/ ../static/
COPY ./client/src/ ./src/

RUN rm -rf ../static/build

# remove our compiled "dep" so we get our source recompiled
RUN rm ./target/wasm32-unknown-unknown/release/deps/client*

#WORKDIR /home/app
RUN wasm-pack build --target web --out-name wasm --out-dir ../static/build

FROM nginx:stable-alpine

COPY --from=builder /home/app/static ./bin/www
COPY devops/nginx/client.nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
