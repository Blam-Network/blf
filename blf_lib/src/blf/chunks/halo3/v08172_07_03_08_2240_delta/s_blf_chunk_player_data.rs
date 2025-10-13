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
#[Header("fupd", 1.0)]
#[brw(big)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_08172_07_03_08_2240_delta"))]
pub struct s_blf_chunk_player_data {
    pub hopper_access: u32, // not 100% sure if this is the bnet flags or hopper access tbh
    pub highest_skill: u32,
}

impl BlfChunkHooks for s_blf_chunk_player_data {}

impl Default for s_blf_chunk_player_data {
    fn default() -> Self {
        s_blf_chunk_player_data {
            hopper_access: 0,
            highest_skill: 1,
        }
    }
}
