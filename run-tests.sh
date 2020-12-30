##!/bin/bash
 cargo build --release && cargo test --release --package tests -- --test-threads 1 --nocapture
