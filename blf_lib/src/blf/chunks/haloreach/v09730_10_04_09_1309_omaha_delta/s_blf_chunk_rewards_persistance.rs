use binrw::binrw;
use blf_lib::blf::chunks::BlfChunkHooks;
use blf_lib::BlfChunk;
use serde::{Deserialize, Serialize};
use blf_lib::types::array::StaticArray;

#[binrw]
#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize,Default)]
#[Header("rpdl", 1.1)]
#[brw(big)]
pub struct s_blf_chunk_rewards_persistance {
    // TODO: Map
    pub unknown1: u32, // controller?
    pub unknown2: StaticArray<u8, 0x180>, // Really not sure on the size of this chunk.
}

impl BlfChunkHooks for s_blf_chunk_rewards_persistance {}

