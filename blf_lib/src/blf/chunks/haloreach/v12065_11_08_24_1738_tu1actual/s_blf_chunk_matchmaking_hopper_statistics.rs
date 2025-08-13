use binrw::{binrw, BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib::types::array::StaticArray;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
#[cfg(feature = "napi")]
use napi_derive::napi;

#[binrw]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize,Default)]
#[Header("mmhs", 4.1)]
#[brw(big)]
pub struct s_blf_chunk_matchmaking_hopper_statistics {
    pub unknown_population_1: u32,
    pub unknown_population_2: u32,
    pub unknown_population_3: u32,
    #[bw(try_calc(u16::try_from(hoppers.len())))]
    #[br(temp)]
    playlist_count: u16,
    #[br(count = playlist_count)]
    pub hoppers: Vec<hopper_population>,
}

#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
#[derive(PartialEq,Debug,Clone,Serialize,Deserialize,Default,BinRead,BinWrite)]
pub struct hopper_population {
    pub unknown1: u8,
    pub unknown2: u8,
    pub hopper_identifier: i16,
    pub player_count: u32,
}

impl BlfChunkHooks for s_blf_chunk_matchmaking_hopper_statistics {}