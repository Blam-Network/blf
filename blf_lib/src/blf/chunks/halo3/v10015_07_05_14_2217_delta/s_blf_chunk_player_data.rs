use binrw::{binrw, BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
use crate::types::c_string::StaticString;
#[cfg(feature = "napi")]
use napi_derive::napi;
use num_derive::FromPrimitive;

#[binrw]
#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("fupd", 2.0)]
#[brw(big)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_10015_07_05_14_2217_delta"))]
pub struct s_blf_chunk_player_data {
    pub hopper_access: u32,
    pub highest_skill: u32,
    pub hopper_directory: StaticString<32>,
}

impl BlfChunkHooks for s_blf_chunk_player_data {}

impl Default for s_blf_chunk_player_data {
    fn default() -> Self {
        s_blf_chunk_player_data {
            hopper_access: 0,
            highest_skill: 0,
            hopper_directory: StaticString::from_string("default_hoppers")
                .expect("Default hopper_directory must be valid."),
        }
    }
}
