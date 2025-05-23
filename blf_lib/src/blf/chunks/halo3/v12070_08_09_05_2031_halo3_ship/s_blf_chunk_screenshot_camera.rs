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
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("scnc", 2.1)]
#[brw(big)]
#[Size(0x164)]
pub struct s_blf_chunk_screenshot_camera
{
    pub jpeg_data_length: u32, // length of jpeg_data in the following scnd.
    pub camera: s_saved_camera,
    pub game_tick: u32,
    pub film_tick: u32,
}

#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
#[derive(Default,PartialEq,Debug,Clone,BinRead, BinWrite,Serialize,Deserialize,TestSize)]
#[Size(0x158)]
pub struct s_saved_camera
{
    pub camera: render_camera,
    pub frustum_bounds: real_rectangle2d,
    pub projection: render_projection
}

#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(0xC0)]
pub struct render_projection {
    pub world_to_view: real_matrix4x3,
    pub view_to_world: real_matrix4x3,
    pub projection_bounds: real_rectangle2d,
    pub projection_matrix: StaticArray<StaticArray<Float32, 4>, 4>,
    pub world_to_screen_size: real_vector2d,
}


#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(0x88)]
#[wasm_bindgen(getter_with_clone)]
pub struct render_camera {
    pub position: real_point3d,
    pub forward: real_vector3d,
    pub up: real_vector3d,
    #[brw(pad_after = 3)]
    pub mirrored: Bool,
    pub vertical_field_of_view: Float32,
    pub field_of_view_scale: Float32,
    pub window_pixel_bounds: int16_rectangle2d,
    pub window_title_safe_pixel_bounds: int16_rectangle2d,
    pub window_final_location: int16_point2d,
    pub render_pixel_bounds: int16_rectangle2d,
    pub render_title_safe_pixel_bounds: int16_rectangle2d,
    pub display_pixel_bounds: int16_rectangle2d,
    pub z_near: Float32,
    pub z_far: Float32,
    pub mirror_plane: real_plane3d,
    #[brw(pad_after = 3)]
    pub enlarge_view: Bool,
    pub enlarge_center: real_point2d,
    pub enlarge_size_x: Float32,
    pub enlarge_size_y: Float32,
}


impl BlfChunkHooks for s_blf_chunk_screenshot_camera {}
