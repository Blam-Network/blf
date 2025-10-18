use binrw::binrw;
use serde::{Deserialize, Serialize};
use blf_lib::blam::halo3::v10015_07_05_14_2217_delta::game::game_engine_variant::c_game_variant;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("mpvr", 1.1)]
#[brw(big)]
pub struct s_blf_chunk_game_variant
{
    pub game_variant: c_game_variant,
}

impl BlfChunkHooks for s_blf_chunk_game_variant {}
