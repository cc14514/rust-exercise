extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

pub mod sortdemo;

#[wasm_bindgen]
pub fn main() {
    println!("Hello, world!");
}

#[wasm_bindgen]
#[warn(unconditional_recursion)]
pub fn looptest() -> usize {
    return 110;
}

