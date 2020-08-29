fn main() {
    pkg_config::Config::new().probe("zlib").unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}

