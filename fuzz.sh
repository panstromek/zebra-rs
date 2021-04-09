##!/bin/bash
RUSTFLAGS="-C link-arg=-Wl,--wrap=time" cargo build --release --bin zebra \
 && BLESS=true cargo run --bin tests