// This module is based on ManagedDonkey's real_math module.
// It has been significantly altered in moving from C++ to Rust,
// though most of it's interface is in-tact.
// https://github.com/twist84/ManagedDonkey/blob/main/game/source/math/real_math.hpp

#![allow(dead_code)]

use std::convert::Into;
use std::error::Error;
use binrw::{BinRead, BinWrite};
#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;
use blf_lib::blam::common::math::integer_math::int32_point3d;
use blf_lib::blam::common::math::unit_vector_quanitzation::get_unit_vector_encoding_constants;
use blf_lib_derivable::result::BLFLibResult;
use blf_lib_derive::TestSize;
use crate::types::numbers::Float32;

const k_3d_count: usize = 3;

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Default, PartialEq, Debug, Clone, Copy, Serialize, Deserialize, BinRead, BinWrite)]
#[wasm_bindgen]
pub struct real_vector3d {
    pub i: Float32,
    pub j: Float32,
    pub k: Float32,
}

impl real_vector3d {
    pub const fn new(i: f32, j: f32, k: f32) -> Self {
        Self { i: Float32(i), j: Float32(j), k: Float32(k) }
    }
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Default, PartialEq, Debug, Clone, Copy, Serialize, Deserialize, BinRead, BinWrite)]
#[wasm_bindgen(getter_with_clone)]
pub struct real_vector2d {
    pub i: Float32,
    pub j: Float32,
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite, Copy)]
#[wasm_bindgen(getter_with_clone)]
pub struct real_bounds {
    pub lower: Float32,
    pub upper: Float32,
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite, Copy)]
#[wasm_bindgen(getter_with_clone)]
pub struct real_rectangle3d {
    pub x: real_bounds,
    pub y: real_bounds,
    pub z: real_bounds,
}

pub fn point_in_rectangle3d(point: &real_point3d, rect: &real_rectangle3d) -> bool {
    (rect.x.lower <= point.x && point.x <= rect.x.upper) &&
        (rect.y.lower <= point.y && point.y <= rect.y.upper) &&
        (rect.z.lower <= point.z && point.z <= rect.z.upper)
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite, Copy)]
#[wasm_bindgen(getter_with_clone)]
pub struct real_rectangle2d {
    pub x: real_bounds,
    pub y: real_bounds,
}

#[derive(Default, PartialEq, Debug, Clone, Copy, Serialize, Deserialize, BinRead, BinWrite, TestSize)]
#[Size(0xC)]
#[wasm_bindgen(getter_with_clone)]
#[cfg_attr(feature = "napi", napi(object))]
pub struct real_point3d {
    pub x: Float32,
    pub y: Float32,
    pub z: Float32,
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Default, PartialEq, Debug, Clone, Copy, Serialize, Deserialize, BinRead, BinWrite, TestSize)]
#[Size(0x8)]
#[wasm_bindgen(getter_with_clone)]
pub struct real_point2d {
    pub x: Float32,
    pub y: Float32,
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Default, PartialEq, Debug, Clone, Copy, Serialize, Deserialize, BinRead, BinWrite)]
#[wasm_bindgen(getter_with_clone)]
pub struct real_matrix3x3 {
    pub forward: real_vector3d,
    pub left: real_vector3d,
    pub up: real_vector3d,
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Default, PartialEq, Debug, Clone, Copy, Serialize, Deserialize, BinRead, BinWrite)]
#[wasm_bindgen(getter_with_clone)]
pub struct real_matrix4x3 {
    pub scale: Float32,
    pub matrix: real_matrix3x3,
    pub center: real_point3d,
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Default, PartialEq, Debug, Clone, Copy, Serialize, Deserialize, BinRead, BinWrite)]
#[wasm_bindgen(getter_with_clone)]
pub struct real_plane3d {
    pub n: real_vector3d,
    pub d: Float32,
}

pub fn rotate_vector_about_axis(
    forward: &mut real_vector3d,
    up: &real_vector3d,
    u: impl Into<f32>,
    v: impl Into<f32>,
) {
    let u = u.into();
    let v = v.into();

    let v1 = (1.0 - v) * (((forward.i * up.i) + (forward.j * up.j)) + (forward.k * up.k));
    let v2 = (forward.k * up.i) - (forward.i * up.k);
    let v3 = (forward.i * up.j) - (forward.j * up.i);
    forward.i = ((v * forward.i) + (v1 * up.i)) - (u * ((forward.j * up.k) - (forward.k * up.j)));
    forward.j = ((v * forward.j) + (v1 * up.j)) - (u * v2);
    forward.k = ((v * forward.k) + (v1 * up.k)) - (u * v3);
}

pub fn assert_valid_real_normal3d(vector: &real_vector3d) -> bool {
    // Calculate the squared length of the vector and subtract 1.0
    let squared_length = vector.i * vector.i + vector.j * vector.j + vector.k * vector.k - 1.0;

    // Check if the result is not NaN or infinite
    if squared_length.is_nan() || squared_length.is_infinite() {
        return false;
    }

    // Check if the absolute value of the result is less than 0.001
    squared_length.abs() < 0.001
}

pub fn arctangent(a1: f32, a2: f32) -> f32 {
    a1.atan2(a2)
}

pub fn dot_product3d(a1: &real_vector3d, a2: &real_vector3d) -> f32 {
    ((a1.i * a2.i) + (a1.j * a2.j) + (a1.k * a2.k)).0
}

pub const k_test_real_epsilon: f32 = 0.001;
pub const k_real_epsilon: f32 = 0.0001;
pub const k_pi: f32 = 3.1415927;

pub const global_up3d: real_vector3d = real_vector3d::new(0f32, 0f32, 1f32);

pub const global_forward3d: real_vector3d = real_vector3d::new(1f32, 0f32, 0f32);

pub const global_left3d: real_vector3d = real_vector3d::new(0f32, 1f32, 0f32);

pub fn square_root(value: f32) -> f32 {
    value.sqrt()
}

pub fn magnitude_squared3d(a1: &real_vector3d) -> f32 {
    ((a1.i * a1.i) + (a1.j * a1.j) + (a1.k * a1.k)).into()
}

fn magnitude3d(vector: &real_vector3d) -> f32 {
    square_root(magnitude_squared3d(vector))
}

fn scale_vector3d(vector: &mut real_vector3d, scale: f32) {
    vector.i *= scale;
    vector.j *= scale;
    vector.k *= scale;
}

pub fn normalize3d(vector: &mut real_vector3d) -> f32 {
    let mut result = magnitude3d(vector);

    if result.abs() >= k_real_epsilon {
        let scale = 1.0 / result;
        scale_vector3d(vector, scale);
    } else {
        result = 0.0;
    }

    result
}

pub fn cross_product3d(a: &real_vector3d, b: &real_vector3d, out: &mut real_vector3d) {
    out.i = (a.j * b.k) - (a.k * b.j);
    out.j = (a.k * b.i) - (a.i * b.k);
    out.k = (a.i * b.j) - (a.j * b.i);
}

pub fn valid_real(value: f32) -> bool {
    !value.is_infinite() && !value.is_nan()
}

pub fn valid_realcmp(a1: f32, a2: f32) -> bool {
    valid_real(a1 - a2) && (a1 - a2).abs() < k_test_real_epsilon
}

pub fn valid_real_vector3d_axes2(a: &real_vector3d, b: &real_vector3d) -> bool {
    assert_valid_real_normal3d(a)
        && assert_valid_real_normal3d(b)
        && valid_realcmp(dot_product3d(a, b), 0.0)
}

pub fn valid_real_vector3d_axes3(forward: &real_vector3d, left: &real_vector3d, up: &real_vector3d) -> bool {
    assert_valid_real_normal3d(forward)
    && assert_valid_real_normal3d(left)
    && assert_valid_real_normal3d(up)
    && valid_realcmp(dot_product3d(forward, left), 0.0)
    && valid_realcmp(dot_product3d(left, up), 0.0)
    && valid_realcmp(dot_product3d(up, forward), 0.0)
}