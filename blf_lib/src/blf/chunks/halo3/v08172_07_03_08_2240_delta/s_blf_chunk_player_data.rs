use binrw::{binrw, BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
use crate::types::string::StaticString;
use num_derive::FromPrimitive;

#[binrw]
#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("fupd", 1.0)]
#[brw(big)]
pub struct s_blf_chunk_player_data {
    pub hopper_access: u32, // not 100% sure if this is the bnet flags or hopper access tbh
    pub highest_skill: u32,
}

impl BlfChunkHooks for s_blf_chunk_player_data {}

impl Default for s_blf_chunk_player_data {
    fn default() -> Self {
        s_blf_chunk_player_data {
            hopper_access: 0,
            highest_skill: 1,
        }
    }
}
