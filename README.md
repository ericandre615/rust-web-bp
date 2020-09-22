# Isomorphic Rust Web Boilerplate
## Using Yew for client and Actix-web for Server

This is just a basic setup I've been working with using Rust for both
client and server on the web using wasm.

### Usage
Basic setup is using a `package.json` in the root. This is nice for running commands, however maybe there is a better way
that would not rely on having `nodejs`/`npm`? Which would require something other than `webpack` as well, which is current just for
a basic dev server.

- `build`: builds both client (wasm-pack) and server (cargo)
- `build:server`: builds the server with cargo
- `build:client`: builds the client with wasm-pack
- `start`: starts botht he client and server
- `start:client`: starts the client using `webpack-dev-server`
- `start:server`: starts the server using `cargo run`


