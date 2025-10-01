use std::u32;
use binrw::{binrw, BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;
use blf_lib::blam::common::math::integer_math::{int16_point2d, int16_rectangle2d};
use blf_lib::blam::common::math::real_math::{real_point3d, real_vector3d, real_plane3d, real_point2d, real_matrix4x3, real_vector2d, real_rectangle2d};
use blf_lib::types::bool::Bool;
use blf_lib::types::array::StaticArray;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::{BlfChunk, TestSize};
use crate::types::numbers::Float32;
#[cfg(feature = "napi")]
use napi_derive::napi;

#[binrw]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("scnc", 4.1)]
#[brw(big)]
// This is a stubbed chunk, i have no idea what it contains.
pub struct s_blf_chunk_screenshot_camera
{
    pub data: StaticArray<u8, 4964>,
}

impl BlfChunkHooks for s_blf_chunk_screenshot_camera {}
