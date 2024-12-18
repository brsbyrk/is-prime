use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn is_prime(n: u64) -> bool {
    primal::is_prime(n)
}

#[wasm_bindgen]
pub fn nth_prime(n: u64) -> u64 {
    let prime = primal::StreamingSieve::nth_prime(n as usize);
    prime as u64
}
