use binrw::binrw;
use serde::{Deserialize, Serialize};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use crate::blam::halo3::v08172_07_03_08_2240_delta::saved_games::scenario_map_variant::c_map_variant;
use blf_lib_derive::BlfChunk;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("mapv", 1.2)]
#[brw(big)]
pub struct s_blf_chunk_map_variant
{
    #[brw(pad_after = 4)]
    pub map_variant: c_map_variant,
}

impl BlfChunkHooks for s_blf_chunk_map_variant {}
