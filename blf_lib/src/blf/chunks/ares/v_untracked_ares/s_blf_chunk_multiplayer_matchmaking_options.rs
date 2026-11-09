use binrw::binrw;
use serde::{Deserialize, Serialize};
use crate::blf::chunks::BlfChunkHooks;
use crate::BlfChunk;
use crate::types::bool::Bool;
use crate::types::string::StaticWcharString;
use crate::types::numbers::Float32;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("mpmo", 2.1)]
#[Size(0x5C)]
#[brw(little)]
pub struct s_blf_chunk_multiplayer_matchmaking_options {
    pub hopper_identifier: i16,
    pub hopper_name: StaticWcharString<0x20>,
    pub hopper_ranked: Bool,
    pub hopper_team_based: Bool,
    pub xlast_index: u8,
    pub draw_probability: u32,
    pub beta: Float32,
    pub tau: Float32,
    pub experience_base_increment: u32,
    #[brw(pad_after = 3)]
    pub experience_penalty_decrement: u32
}

impl BlfChunkHooks for s_blf_chunk_multiplayer_matchmaking_options {}