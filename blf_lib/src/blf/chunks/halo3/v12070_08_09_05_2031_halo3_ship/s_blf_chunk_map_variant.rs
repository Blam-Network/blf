use binrw::binrw;
use serde::{Deserialize, Serialize};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use crate::blam::halo3::release::saved_games::scenario_map_variant::c_map_variant;
use blf_lib_derive::BlfChunk;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("mapv", 12.1)]
#[brw(big)]
pub struct s_blf_chunk_map_variant
{
    #[brw(pad_before = 4, pad_after = 4)]
    pub map_variant: c_map_variant,
}

impl BlfChunkHooks for s_blf_chunk_map_variant {}

impl s_blf_chunk_map_variant {
    pub fn create(map_variant: c_map_variant) -> Self {
        Self {
            map_variant,
        }
    }
}