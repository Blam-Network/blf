use binrw::binrw;
use blf_lib::blf::chunks::BlfChunkHooks;
use blf_lib::BlfChunk;
use serde::{Deserialize, Serialize};
use blf_lib::types::array::StaticArray;
#[cfg(feature = "napi")]
use napi_derive::napi;

#[binrw]
#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize,Default)]
#[Header("arhs", 3.1)]
#[brw(big)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_blf_chunk_arena_hopper_stats {
    // TODO: Map
    pub data: StaticArray<u8, 0x16>,
}

impl BlfChunkHooks for s_blf_chunk_arena_hopper_stats {}

