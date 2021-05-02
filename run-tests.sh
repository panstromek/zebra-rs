##!/bin/bash
CARGO_TARGET_DIR=test-target RUSTFLAGS="-Zinstrument-coverage -C link-arg=-Wl,--wrap=time" cargo build --release \
&& cargo test --release --package tests -- --test-threads 8 --nocapture

# Report coverage. Even if some tests fail.
echo "-----"
echo "Coverage:"
cargo-profdata -- merge -sparse ./tests/snapshot-tests/*/*/default.profraw -o __all-tests-coverage.profdata\
&& cargo cov -- report test-target/release/zebra -instr-profile __all-tests-coverage.profdata -ignore-filename-regex /home/matyas/.cargo/ | tail -n 1
