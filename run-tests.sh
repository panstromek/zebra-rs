##!/bin/bash
cargo build --bin zebra --release && cargo test --release --package tests -- --test-threads 6 --nocapture
