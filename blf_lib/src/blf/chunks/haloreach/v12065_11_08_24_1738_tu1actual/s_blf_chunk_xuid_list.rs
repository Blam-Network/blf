use binrw::{binrw, BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib::types::c_string::StaticWcharString;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("idls", 1.1)]
#[brw(big)]
pub struct s_blf_chunk_xuid_list
{
    #[bw(try_calc(u32::try_from(xuids.len())))]
    xuid_count: u32,
    #[br(count = xuid_count)]
    pub xuids: Vec<u64>,
}

impl BlfChunkHooks for s_blf_chunk_xuid_list {}
