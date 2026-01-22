use std::u32;
use binrw::{binrw, BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
use crate::types::u64::Unsigned64;
#[cfg(feature = "napi")]
use napi_derive::napi;

// const k_recent_players_max_count: usize = -1;
#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("furp", 2.1)]
#[brw(little)]
#[cfg_attr(feature = "napi", napi(object, namespace = "ares_untracked"))]
pub struct s_blf_chunk_user_recent_players
{
    #[bw(try_calc(u32::try_from(players.len())))]
    player_count: u32,
    #[br(count = player_count)]
    pub players: Vec<s_files_user_recent_players_entry> //  Max 384?
}

#[cfg_attr(feature = "napi", napi(object, namespace = "ares_untracked"))]
#[derive(Default,PartialEq,Debug,Clone,Serialize,Deserialize,BinRead,BinWrite)]
pub struct s_files_user_recent_players_entry {
    pub hopper_identifier: u16,
    pub xuid: Unsigned64,
}

impl BlfChunkHooks for s_blf_chunk_user_recent_players {}

impl s_blf_chunk_user_recent_players {
    pub fn create() -> Self {
        Self {
            players: Vec::new()
        }
    }
}