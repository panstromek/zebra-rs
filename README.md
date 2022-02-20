
# Zebra.rs

Rust port of [Zebra](http://www.radagast.se/othello/zebra.html). A strong Othello program.

This repository also contains a source code for [Webzebra](https://webzebra.netlify.app/), wasm version
based on the Rust port, which runs in the browser.

## Building

Following instructions are for CLI version of zebra and its tools. For building Webzebra,
see [README.md](https://github.com/panstromek/zebra-rs/blob/master/webzebra/README.md) in webzebra directory.

### Requirements
To build the project, you need nightly Rust. You can install it using `rustup`.
See [here](https://www.rust-lang.org/tools/install) for more information on how to get `rustup`.

```shell
# Install nightly toolchain
rustup toolchain install nightly
# Override the toolchain for the project directory
rustup override set nightly
```

### Linux
On Linux (or WSL), you should now be able to build the whole project.
```shell
cargo build
```

### Windows
Not all crates in this repository can be built on Windows. This is because some original crates depend on some linux specific APIs from libc.
The goal is to eventually get rid of these dependencies and make all crates multiplatform.
You can only build `engine`,`flate2-coeff-source`,`webzebra/crate`, `script`, `tests` and their dependencies.
To do that, just go to the specific directory and run `cargo build`.

## Running Tests

Main test suite is composed of a ~180 snapshot tests running against CLI apps. Run them with:
```shell
bash ./run-tests.sh
```

These tests will try to report coverage, which fails if you don't have LLVM/Cargo tools for that. You can install them with:
```shell
rustup component add llvm-tools-preview
cargo install cargo-binutils
```

## Running the game (CLI)

To run the game in terminal, run:

```shell
cargo run --bin zebra
```

See available options in `-help`. Options are identical to the original `zebra` program.
Source code for this binary is located in `/legacy-zebra` directory.

```shell
cargo run --bin zebra -- -help
```

## Related projects

 - [Zebra](http://www.radagast.se/othello)
   - Original program written in C by Gunnar Andersson between 1997-2005 
   - Original version is just a CLI, so it's not as easy to use as UI versions below
   - Source code can be downloaded from the website.
     - This Rust port is based on [slightly cleaned version of the original source code](https://github.com/hoshir/zebra) by Ryuichi Hoshi
       - This version with some bug fixes found during testing of this Rust port can be found in [my fork](https://github.com/panstromek/zebra-1)

 - [WZebra](http://www.radagast.se/othello/download.html)
   - Windows version with UI. Probably the most popular version.

 - [Reversatile](https://github.com/oers/reversatile)
   - Android version - uses slightly adjusted original zebra code (CLI)
   - Continuation of [Droidzebra](https://github.com/alkom/droidzebra), which was abandoned

### Other Othello engines

 - [Saio](https://www.romanobenedetto.it/)
   - Probably the most advanced actively developed engine at the moment
 - [NText](https://github.com/weltyc/ntest) - updated version on GitHub
 - [Edax](https://github.com/abulmo/edax-reversi)

## License

The project is licensed under GNU GENERAL PUBLIC LICENSE, version 2 (same as the original source, which this code is based on).

See LICENSE file for more details.
