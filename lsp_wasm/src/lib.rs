mod screenshot;
mod film;

use wasm_bindgen::prelude::*;
use js_sys::Uint8Array;
use crate::film::FilmVariants;
use crate::screenshot::ScreenshotData;

#[wasm_bindgen]
pub fn get_screenshot_data(bytes: Uint8Array) -> Option<ScreenshotData> {
    screenshot::get_screenshot_data(bytes.to_vec())
}

#[wasm_bindgen]
pub fn get_film_variants(bytes: Uint8Array) -> Option<FilmVariants> {
    film::get_film_variants(bytes.to_vec())
}

