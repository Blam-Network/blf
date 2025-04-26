mod screenshot;

use wasm_bindgen::prelude::*;
use js_sys::Uint8Array;

use crate::screenshot::ScreenshotData;

#[wasm_bindgen]
pub fn get_screenshot_data(bytes: Uint8Array) -> Option<ScreenshotData> {
    screenshot::get_screenshot_data(bytes.to_vec())
}

