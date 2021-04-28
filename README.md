# wasm-game-lib

## This project is discontinued.

Please note that this was an experimental game library made while I was learning Rust.  
Feel free to request the ownership of the crate on crates.io at [mubelotix@gmail.com](mailto:mubelotix@gmail.com).

Welcome!

The goal of this crate is to help you to make great 2d games running in web browsers.
This crate is inspired by SFML.

## How to use

Getting started with Wasm may be painful.
That's why I created a simple template you can use to quickly start wasm development.

First, you need to install `wasm-pack` and `cargo-generate`.

```sh
cargo install wasm-pack
cargo install cargo-generate
```

Then, use cargo-generate to generate a new crate (modify the name at the end of the command):

```sh
cargo generate --git https://github.com/Mubelotix/wasm-game-lib-template --name amazing-game
```

Your game is now generated and is ready to be compiled with the command:

```sh
wasm-pack build --target=web
```

To test you game, you need a web server.
Open a terminal and go to your project directory.
Then, launch the webserver.

```sh
cd pkg/
python3 -m http.server 8001
```

Your game is now testable on [http://localhost:8001](http://localhost:8001)!

In case this does not work, make sure you followed every instruction and [contact me](mailto:mubelotix@gmail.com).

License: MIT
