use binrw::{binrw, BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib::types::c_string::StaticWcharString;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;

pub const k_manifest_maximum_maps_count: usize = 16;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("fmca", 1.1)]
#[brw(big)]
pub struct s_blf_chunk_megalo_categories
{
    #[bw(try_calc(u32::try_from(categories.len())))]
    category_count: u32,
    #[br(count = category_count)]
    pub categories: Vec<s_blf_chunk_megalo_category>,
}

impl BlfChunkHooks for s_blf_chunk_megalo_categories {}
#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
pub struct s_blf_chunk_megalo_category {
    pub category_id: u32,
    pub category_name: StaticWcharString<32>,
}
