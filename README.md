# WASM PoC

Constructs an array of numbers in JS, then does some random computation in Rust, and returns back a `Uint8Array`.

### Running

Dependencies:

- Cargo
- wasm-pack
- Node

Build both the wasm library and web app by running

```sh
(cd app; npm install)
make
```

Then use a web server like VSCode Live Server or `python -m http.server` and navigate to index.html.

You can also run the following two commands in separate terminals to watch for changes:

```sh
make watch_lib
make watch_app
```
