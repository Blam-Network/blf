use binrw::{binrw, BinRead, BinWrite};
use crate::types::array::StaticArray;
use serde::{Deserialize, Serialize};
use crate::blf::chunks::BlfChunkHooks;
use crate::BlfChunk;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("mps2", 2.1)]
#[Size(0x404)]
#[brw(little)]
pub struct s_blf_chunk_multiplayer_player_vs_player_statistics {
    #[brw(pad_before = 4)]
    pub players: StaticArray<StaticArray<s_blf_chunk_multiplayer_player_vs_player_statistic, 16>, 16>,
}

impl BlfChunkHooks for s_blf_chunk_multiplayer_player_vs_player_statistics   {}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
pub struct s_blf_chunk_multiplayer_player_vs_player_statistic {
    pub kills: i16,
    pub deaths: i16
}