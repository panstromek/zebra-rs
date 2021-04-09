##!/bin/bash
RUSTFLAGS="-C link-arg=-Wl,--wrap=time" cargo build --release && cargo test --package tests -- --test-threads 8 --nocapture
