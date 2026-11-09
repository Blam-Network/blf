use binrw::binrw;
use crate::types::array::StaticArray;
#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use crate::blf::chunks::BlfChunkHooks;
use crate::BlfChunk;
use crate::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::s_game_results_statistics;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("mps3", 2.1)]
#[Size(0x664)]
#[brw(big)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct s_blf_chunk_multiplayer_team_statistics {
    #[brw(pad_before = 4)]
    pub teams: StaticArray<s_game_results_statistics, 16>,
}

impl BlfChunkHooks for s_blf_chunk_multiplayer_team_statistics {}
