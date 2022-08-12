use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// Renders a greeting.
pub fn greet(name: String) -> String {
    return format!("hello, {}", name);
}
