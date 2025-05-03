use wasm_bindgen::prelude::wasm_bindgen;
use crate::halo3::HALO3;

mod halo3;

#[wasm_bindgen]
struct BLF {}

#[wasm_bindgen]
impl BLF {
    #[wasm_bindgen(method, getter)]
    pub fn halo3() -> HALO3 {
        HALO3 {}
    }
}