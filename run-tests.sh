##!/bin/bash
rm ./tests/snapshot-tests/*/*/default.profraw;
while getopts t:c: flag
do
    case "${flag}" in
        t) whichtests=${OPTARG};;
        c) fullcoverage=${OPTARG};;
        *)
    esac
done

#CARGO_TARGET_DIR=test-target RUSTFLAGS="-Cinstrument-coverage -C link-arg=-Wl,--wrap=time" cargo build --release \
 #&& cargo test --release --package tests "$whichtests" -- --nocapture -Z unstable-options --report-time && fullcoverage=10000
CARGO_TARGET_DIR=test-target RUSTFLAGS="-Cinstrument-coverage -C link-arg=-Wl,--wrap=time" cargo build --release \
&& LLVM_PROFILE_FILE="default.profraw" cargo test --release --package tests "$whichtests" -- --nocapture -Z unstable-options --report-time && fullcoverage=10000

if test "$fullcoverage"; then
  num_lines=100000
else
  num_lines=1
fi
# Report coverage. Even if some tests fail.
echo "-----"
echo "Coverage:"
cargo-profdata -- merge -sparse ./tests/snapshot-tests/*/*/default.profraw -o __all-tests-coverage.profdata\
&& cargo cov -- report test-target/release/zebra -instr-profile __all-tests-coverage.profdata -ignore-filename-regex "\.cargo/" | tail -n "$num_lines"
