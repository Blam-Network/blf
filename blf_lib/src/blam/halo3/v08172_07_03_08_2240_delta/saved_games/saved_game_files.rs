use binrw::{BinRead, BinWrite};
#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use crate::types::c_string::StaticString;
use crate::types::c_string::StaticWcharString;
use serde_hex::{SerHex,StrictCap};
use wasm_bindgen::prelude::wasm_bindgen;
use blf_lib_derivable::result::BLFLibResult;
use blf_lib::types::time::time64_t;
use blf_lib_derive::TestSize;
use crate::types::bool::Bool;
use crate::types::u64::Unsigned64;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_08172_07_03_08_2240_delta"))]
pub struct s_content_item_metadata {
    pub unique_id: Unsigned64,
    pub name: StaticWcharString<128>,
    pub description: StaticString<128>,
    pub author: StaticString<16>,
    pub file_type: i32,
    #[brw(pad_after = 3)]
    pub author_is_xuid_online: Bool,
    #[serde(with = "SerHex::<StrictCap>")]
    pub author_id: Unsigned64,
    pub size_in_bytes: Unsigned64,
    pub date: time64_t,
    pub length_seconds: u32,
    pub campaign_id: i32,
    pub map_id: i32,
    pub game_engine_type: u32,
    pub campaign_difficulty: i32,
    pub hopper_id: i16,
    #[brw(pad_before = 2)]
    pub game_id: Unsigned64,
}
