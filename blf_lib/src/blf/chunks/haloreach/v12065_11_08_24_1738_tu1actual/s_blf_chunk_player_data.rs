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
    #[brw(pad_after = 1)]
    pub hopper_access: u8,
    pub bungie_user_role: u16, // includes blue flames at bit 3
    pub hopper_directory: StaticString<32>,
    pub unlock_achievements: StaticArray<u8, 32>, // 59 achievements, capacity of 256 bits
    pub extras_portal_debug: u8,
}

impl BlfChunkHooks for s_blf_chunk_player_data {}

impl Default for s_blf_chunk_player_data {
    fn default() -> Self {
        s_blf_chunk_player_data {
            hopper_access: 0,
            bungie_user_role: 0xffff,
            hopper_directory: StaticString::from_string("default_hoppers")
                .expect("Default hopper_directory must be valid."),
            unlock_achievements: StaticArray::default(),
            extras_portal_debug: 0,
        }
    }
}
