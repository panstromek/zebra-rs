use wasm_bindgen::prelude::*;
use engine::src::game::{engine_global_setup};
use engine::src::getcoeff::CoeffSource;
extern crate engine;
extern crate libc;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
struct CoeffSourceWasm;
impl CoeffSource for CoeffSourceWasm {
    fn next_word(&mut self) -> i16 {
        // unimplemented!()
        0
    }
}
#[wasm_bindgen]
pub fn greet(name: &str) {
    let coeffs = CoeffSourceWasm {};
    unsafe { engine_global_setup(0, 7, None, coeffs); }

    alert(&format!("Hello, {}!", name));
}