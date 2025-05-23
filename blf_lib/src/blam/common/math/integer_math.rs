// This module is based on ManagedDonkey's integer_math module.
// It has been significantly altered in moving from C++ to Rust,
// though most of it's interface is in-tact.
// https://github.com/twist84/ManagedDonkey/blob/main/game/source/math/integer_math.hpp

#![allow(dead_code)]

use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;
#[cfg(feature = "napi")]
use napi_derive::napi;

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite, Copy)]
#[wasm_bindgen(getter_with_clone)]
pub struct int32_point3d  {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite, Copy)]
#[wasm_bindgen(getter_with_clone)]
pub struct int16_point2d  {
    pub x: i16,
    pub y: i16,
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite, Copy)]
#[wasm_bindgen(getter_with_clone)]
pub struct int32_bounds {
    pub lower: i32,
    pub upper: i32,
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite, Copy)]
#[wasm_bindgen(getter_with_clone)]
pub struct int32_rectangle2d {
    pub x: int32_bounds,
    pub y: int32_bounds,
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite, Copy)]
#[wasm_bindgen(getter_with_clone)]
pub struct int16_bounds {
    pub lower: i16,
    pub upper: i16,
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite, Copy)]
#[wasm_bindgen(getter_with_clone)]
pub struct int16_rectangle2d {
    pub x: int16_bounds,
    pub y: int16_bounds,
}