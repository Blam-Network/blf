use binrw::binrw;
#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
use crate::types::array::StaticArray;
use crate::types::c_string::StaticString;

#[binrw]
#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("fupd", 7.1)]
#[brw(big)]
#[Size(0x45)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_blf_chunk_player_data {
    pub hopper_access: u8,
    pub bungie_user_role: u16,
    pub unknown1: u8,
    pub hopper_directory: StaticString<32>,
    pub unknown2: StaticArray<u8, 0x20>,
    pub unknown3: u8,
}

impl BlfChunkHooks for s_blf_chunk_player_data {}

impl Default for s_blf_chunk_player_data {
    fn default() -> Self {
        s_blf_chunk_player_data {
            hopper_access: 0,
            bungie_user_role: 0xffff,
            unknown1: 0,
            hopper_directory: StaticString::from_string("default_hoppers")
                .expect("Default hopper_directory must be valid."),
            unknown2: StaticArray::default(),
            unknown3: 1,
        }
    }
}
