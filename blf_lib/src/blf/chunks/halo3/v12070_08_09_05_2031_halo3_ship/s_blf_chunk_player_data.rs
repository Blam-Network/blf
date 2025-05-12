use binrw::binrw;
use serde::{Deserialize, Serialize};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
use crate::types::c_string::StaticString;
use napi_derive::napi;

#[binrw]
#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("fupd", 3.1)]
#[brw(big)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct s_blf_chunk_player_data {
    pub hopper_access: u32,
    pub bungie_user_role: u32,
    pub highest_skill: u32,
    pub hopper_directory: StaticString<32>,
}

impl BlfChunkHooks for s_blf_chunk_player_data {}

impl Default for s_blf_chunk_player_data {
    fn default() -> Self {
        s_blf_chunk_player_data {
            hopper_access: 0,
            bungie_user_role: 1,
            highest_skill: 0,
            hopper_directory: StaticString::from_string("default_hoppers").unwrap(),
        }
    }
}
