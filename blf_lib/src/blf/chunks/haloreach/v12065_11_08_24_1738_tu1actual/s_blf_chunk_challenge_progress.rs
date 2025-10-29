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
#[Header("chpr", 2.1)]
#[brw(big)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_blf_chunk_challenge_progress {
    pub active_challenge_set_1: u32,
    pub active_challenge_sset_2: u32,
    pub chalenge_set_1_progress: StaticArray<i32, 10>,
    pub chalenge_set_2_progress: StaticArray<i32, 10>,
}

impl BlfChunkHooks for s_blf_chunk_challenge_progress {}

