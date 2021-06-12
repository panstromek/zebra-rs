#!/bin/bash
cd crate
cargo watch -i .gitignore -i "pkg/*" -s "wasm-pack build --target=web --release"

