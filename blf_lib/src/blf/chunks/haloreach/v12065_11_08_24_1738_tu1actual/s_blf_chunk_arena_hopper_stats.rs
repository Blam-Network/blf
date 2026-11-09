use binrw::binrw;
use blf_lib::blf::chunks::BlfChunkHooks;
use blf_lib::BlfChunk;
use serde::{Deserialize, Serialize};
use blf_lib::types::array::StaticArray;

#[binrw]
#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize,Default)]
#[Header("arhs", 3.1)]
#[brw(big)]
pub struct s_blf_chunk_arena_hopper_stats {
    // TODO: Map
    pub season: u32,
    pub unknown04: u32,
    pub unknown08: u32,
    pub unknown0C: u32,
    pub unknown10: u32,
    pub unknown14: u16,
}

impl BlfChunkHooks for s_blf_chunk_arena_hopper_stats {}

