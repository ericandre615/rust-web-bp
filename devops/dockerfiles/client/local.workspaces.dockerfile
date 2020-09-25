FROM rust:1.46.0 as builder

WORKDIR /home/app

RUN rustup target add wasm32-unknown-unknown

RUN cargo install wasm-pack

RUN mkdir ./static


RUN mkdir ./client
RUN USER=root cargo new --bin server
RUN USER=root cargo new --bin data
#RUN USER=root cargo new --lib client

COPY ./.env ./.env
COPY ./Cargo.* ./
COPY ./client/Cargo.* ./client/
COPY ./data/Cargo.* ./data/
COPY ./server/Cargo.* ./server/

COPY ./client/src ./client/src
# cache dependency compiles
# RUN wasm-pack build client --target web --out-name wasm --out-dir ../static/build

COPY ./static ./static

RUN rm -rf ./static/build

#WORKDIR /home/app
RUN wasm-pack build client --target web --out-name wasm --out-dir ../static/build

FROM nginx:stable-alpine

COPY --from=builder /home/app/static ./bin/www
COPY devops/nginx/client.nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
