use binrw::{binrw, BinRead, BinWrite};
use blf_lib::blf::chunks::BlfChunkHooks;
use blf_lib::BlfChunk;
use serde::{Deserialize, Serialize};
use blf_lib::types::array::StaticArray;
#[cfg(feature = "napi")]
use napi_derive::napi;
use crate::types::time::time64_t;

#[binrw]
#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize,Default)]
#[Header("dcha", 3.1)]
#[brw(big)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_blf_chunk_challenge_state {
    pub active_challenge_set_1: u32,
    pub active_challenge_sset_2: u32,
    pub chalenge_set_1_timestamp: time64_t,
    pub chalenge_set_2_timestamp: time64_t,
    pub chalenge_set_1_count: u8,
    pub chalenge_set_2_count: u8,
    pub chalenge_set_1: StaticArray<s_challenge_state, 10>,
    pub chalenge_set_2: StaticArray<s_challenge_state, 10>,
}

#[derive(PartialEq,Debug,Clone,Serialize,Deserialize,Default,BinRead,BinWrite)]
#[brw(big)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_challenge_state {
    pub category: u8,
    pub index: u8,
    pub reward_credits: u16,
    // TODO: Map, progress is probs in here.
    pub unknown4: StaticArray<u8, 24>,
}

impl BlfChunkHooks for s_blf_chunk_challenge_state {}

