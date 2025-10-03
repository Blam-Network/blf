use binrw::{binrw, BinRead, BinWrite};
#[cfg(feature = "napi")]
use napi_derive::napi;
use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};
use blf_lib::types::bool::Bool;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
use crate::types::array::StaticArray;
use crate::types::c_string::StaticString;

#[binrw]
#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("fupd", 7.0)]
#[brw(big)]
#[Size(0x45)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_blf_chunk_player_data {
    #[brw(pad_after = 1)]
    pub hopper_access: u8,
    pub bungie_user_role: u16,
    pub hopper_directory: StaticString<32>,
    pub unlock_achievements: StaticArray<u8, 32>, // 59 achievements, capacity of 256 bits
    pub extras_portal_debug: Bool,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, FromPrimitive)]
#[brw(big, repr = u16)]
#[cfg_attr(feature = "napi", napi(namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
#[repr(u16)]
pub enum e_bungienet_user_flags {
    #[default]
    none =                      0b0000000000000000,
    community =                 0b0000000000000001,
    community1 =                0b0000000000000010,
    community2 =                0b0000000000000100,
    bungie =                    0b0000000000001000, // unlocks blue flames
    unknown4 =                  0b0000000000010000,
    unknown5 =                  0b0000000000100000,
    activities_unlocked =       0b0000000001000000,
    unknown7 =                  0b0000000010000000,
    unknown8 =                  0b0000000100000000,
    unknown9 =                  0b0000001000000000,
    unknown10 =                 0b0000010000000000,
    unknown11 =                 0b0000100000000000,
    nameplate_seventh_column =  0b0001000000000000,
    nameplate_dmr =             0b0010000000000000,
    nameplate_bungie =          0b0011000000000000, // requires bungie flag to take effect.
    nameplate_marathon =        0b0100000000000000,
    nameplate_halo1 =           0b0101000000000000,
    nameplate_halo2 =           0b0110000000000000,
    nameplate_halo3 =           0b0111000000000000,
    nameplate_odst =            0b1000000000000000,
    nameplate_assault_rifle =   0b1001000000000000,
    nameplate_mk4_helmet =      0b1010000000000000,
    nameplate_halo =            0b1011000000000000,
    nameplate_allstar =         0b1100000000000000
}

impl BlfChunkHooks for s_blf_chunk_player_data {}

impl Default for s_blf_chunk_player_data {
    fn default() -> Self {
        s_blf_chunk_player_data {
            hopper_access: 0,
            bungie_user_role: 0,
            hopper_directory: StaticString::from_string("default_hoppers")
                .expect("Default hopper_directory must be valid."),
            unlock_achievements: StaticArray::default(),
            extras_portal_debug: Bool::from(false),
        }
    }
}
