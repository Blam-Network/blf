use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use crate::blf::chunks::BlfChunkHooks;
use crate::BlfChunk;
use crate::types::bool::Bool;
use crate::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::s_campaign_metagame_runtime_globals_secondary;

#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize,BinRead,BinWrite)]
#[Header("cmrs", 1.1)]
#[Size(0x6C)]
#[brw(big)]
pub struct s_blf_chunk_campaign_metagame_results_secondary {
    #[brw(pad_after = 3)]
    pub valid: Bool,
    pub results: s_campaign_metagame_runtime_globals_secondary,
}

impl BlfChunkHooks for s_blf_chunk_campaign_metagame_results_secondary {}
