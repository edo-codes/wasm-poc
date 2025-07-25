use js_sys::Array;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub fn do_thing(column: Array) -> Box<[u8]> {
    console::log_1(&"Hello from Rust!".into());

    let mut buffer = Vec::<u8>::with_capacity(column.length() as usize * 8);
    for i in 0..column.length() {
        let value = column.get(i).as_f64().unwrap() as u64;
        let value = value ^ (i as u64);
        buffer.extend_from_slice(&value.to_le_bytes());
    }

    buffer.into_boxed_slice()
}
