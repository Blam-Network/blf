use binrw::{binrw, BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib::types::array::StaticArray;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
#[cfg(feature = "napi")]
use napi_derive::napi;

#[binrw]
#[cfg_attr(feature = "napi", napi(object, namespace = "ares_untracked"))]
#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize,Default)]
#[Header("mmhs", 3.1)]
#[brw(little)]
#[Size(0x104)]
pub struct s_blf_chunk_matchmaking_hopper_statistics {
    pub player_count: u32,
    pub data: StaticArray<hopper_population, 32>,
}

#[cfg_attr(feature = "napi", napi(object, namespace = "ares_untracked"))]
#[derive(PartialEq,Debug,Clone,Serialize,Deserialize,Default,BinRead,BinWrite)]
pub struct hopper_population {
    #[brw(pad_before = 2)]
    pub hopper_identifier: i16,
    pub player_count: u32,
}

impl BlfChunkHooks for s_blf_chunk_matchmaking_hopper_statistics {}