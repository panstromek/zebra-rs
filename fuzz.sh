##!/bin/bash
CARGO_TARGET_DIR=test-target RUSTFLAGS="-Zinstrument-coverage -C link-arg=-Wl,--wrap=time" cargo build --release --bin zebra \
 && RUST_BACKTRACE=1 BLESS=true cargo run --release --bin tests