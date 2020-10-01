# Isomorphic Rust Web Boilerplate
## Using Yew for client and Actix-web for Server

This is just a basic setup I've been working with using Rust for both
client and server on the web using wasm.

### Usage
Basic setup is using a `package.json` in the root. This is nice for running commands, however maybe there is a better way
that would not rely on having `nodejs`/`npm`? Which would require something other than `webpack` as well, which is current just for
a basic dev server. To run in the current state you will first need to `mv` or `rename` `.env.example` to `.env`. First, you need to `build` with `cargo`.
The root `Cargo.toml` has `workspaces` setup. So it will build both the client and server into a single `target` in root dir. Then you need to run `npm run build:client` separately to transform the build into `wasm` for use in the browser. Then just start the client dev server `npm run start:client` and the `actix-web` server with `npm run start:server` or just `cargo run` server.

- `build`: builds both client (wasm-pack) and server (cargo)
- `build:server`: builds the server with cargo
- `build:client`: builds the client with wasm-pack
- `start`: starts botht he client and server
- `start:client`: starts the client using `webpack-dev-server`
- `start:server`: starts the server using `cargo run`

### Additional
There will additional branches for different types of configuration.

- [`feature/shared`](https://github.com/ericandre615/rust-web-bp/tree/feature/shared): adds a `shared` workspace directory that both `client` and `server` pull in that can have data types shared by
both or any other utilities that may be useful to shared between the entire project.
- [`feature/docker`](https://github.com/ericandre615/rust-web-bp/tree/feature/docker): adds `docker-compose` with containers for postgres database, `client`, `server`, `diesel_cli`. Not fully implemented


