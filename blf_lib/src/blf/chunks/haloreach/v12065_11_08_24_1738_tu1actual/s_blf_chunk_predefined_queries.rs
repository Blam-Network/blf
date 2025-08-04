use binrw::{binrw, BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib::types::c_string::StaticWcharString;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("fpre", 1.1)]
#[brw(big)]
pub struct s_blf_chunk_predefined_queries
{
    #[bw(try_calc(u32::try_from(queries.len())))]
    query_count: u32,
    #[br(count = query_count)]
    pub queries: Vec<s_blf_chunk_predefined_query>,
}

impl BlfChunkHooks for s_blf_chunk_predefined_queries {}
#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
pub struct s_blf_chunk_predefined_query {
    pub unknown1: u32,
    pub unknown2: u32,
    pub unknown3: u32,
    pub name: StaticWcharString<32>,
    pub description: StaticWcharString<256>,
}
