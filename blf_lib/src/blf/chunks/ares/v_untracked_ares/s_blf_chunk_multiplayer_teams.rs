use binrw::{binrw, BinRead, BinWrite};
use crate::types::array::StaticArray;
#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use crate::blf::chunks::BlfChunkHooks;
use crate::{BlfChunk, TestSize};
use crate::types::bool::Bool;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("mptm", 1.1)]
#[Size(0x44)]
#[brw(little)]
#[cfg_attr(feature = "napi", napi(object, namespace = "ares_untracked"))]
pub struct s_blf_chunk_multiplayer_teams {
    #[brw(pad_before = 4)]
    pub teams: StaticArray<s_blf_chunk_multiplayer_teams_team, 16>,
}

impl BlfChunkHooks for s_blf_chunk_multiplayer_teams {}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(4)]
#[cfg_attr(feature = "napi", napi(object, namespace = "ares_untracked"))]
pub struct s_blf_chunk_multiplayer_teams_team {
    pub exists: Bool,
    pub standing: i8,
    #[brw(big)]
    pub score: i16,
}