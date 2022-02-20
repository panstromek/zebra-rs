# Webzebra

Web version of zebra, runnning in wasm. This project is built on Vue, Vite and Tailwind. 
For wasm, it uses `wasm-pack` and `wasm-bindgen` tools.

# Prerequiesites
You need to have `node`, `yarn`, `cargo` and `wasm-pack` installed.

# Running in development

```shell
# Install dependencies
yarn install

# Build wasm library.
cd ./crate
# You can omit --release flag here to build in debug mode
wasm-pack build --target web --release
cd ..
# Run the dev server
yarn vite
```

To build the production bundle, run:
```shell
yarn vite build
```