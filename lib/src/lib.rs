use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub fn do_thing(column: Box<[f64]>) -> Vec<u8> {
    console::log_1(&"Hello from Rust!".into());

    let mut buffer = Vec::<u8>::with_capacity(column.len() * 8);
    for (i, x) in column.into_iter().enumerate() {
        let y = (x as u64) ^ i as u64;
        buffer.extend_from_slice(&y.to_le_bytes());
    }

    buffer
}
