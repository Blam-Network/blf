use binrw::{binrw, BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
use crate::types::c_string::StaticString;
#[cfg(feature = "napi")]
use napi_derive::napi;
use num_derive::FromPrimitive;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, FromPrimitive)]
#[brw(big, repr = u32)]
#[cfg_attr(feature = "napi", napi(namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
#[repr(u32)]
pub enum e_bungienet_user_flags {
    #[default]
    none =                      0b00000000000000000000000000000000,
    registered =                0b00000000000000000000000000000001,
    pro_member =                0b00000000000000000000000000000010,
    // unlocks bungie chestpiece w/ flames
    staff =                     0b00000000000000000000000000000100,
    // unlocks recon
    community =                 0b00000000000000000000000000001000,
    // these two appear unused, code to unlock armour exists
    // but no armour use these strings for unlock.
    community2 =                0b00000000000000000000000000010000,
    community3 =                0b00000000000000000000000000100000,
    // this is set by the game at runtime based on if you're playing mythic
    // it's then used to lock campaign
    // is_blue_disk =              0b10000000000000000000000000000000
}

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
            bungie_user_role: 0,
            highest_skill: 0,
            hopper_directory: StaticString::from_string("default_hoppers")
                .expect("Default hopper_directory must be valid."),
        }
    }
}
