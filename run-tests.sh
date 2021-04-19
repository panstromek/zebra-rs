##!/bin/bash
CARGO_TARGET_DIR=test-target RUSTFLAGS="-Zinstrument-coverage -C link-arg=-Wl,--wrap=time" cargo build --release && cargo test --release --package tests -- --test-threads 8 --nocapture
