use binrw::binrw;
use serde::{Deserialize, Serialize};
use blf_lib::bitfield;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
use blf_lib::blam::halo3::v12070_08_09_05_2031_halo3_ship::cseries::language::k_language_count;
use blf_lib::types::array::StaticArray;
use blf_lib::types::c_string::StaticWcharString;

#[binrw]
#[derive(BlfChunk, Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[Header("cmpn", 1.0)]
#[brw(big)]
pub struct s_blf_chunk_campaign {
    pub campaign_id: e_campaign_id,
    pub flags: e_campaign_flags,
    pub name: StaticArray<StaticWcharString<64>, k_language_count>,
    pub description: StaticArray<StaticWcharString<128>, k_language_count>,
    pub scenarios: StaticArray<e_map_id, 64>,
    pub pad: u32,
}

impl BlfChunkHooks for s_blf_chunk_campaign {}

bitfield! {
    #[derive(Serialize, Deserialize)]
    pub struct e_campaign_flags: u32 {
        campaign_from_dlc,
        campaign_unlockable,
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Serialize, Deserialize, binrw::BinRead, binrw::BinWrite)]
#[brw(transparent)]
pub struct e_campaign_id(pub i32);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Serialize, Deserialize, binrw::BinRead, binrw::BinWrite)]
#[brw(transparent)]
pub struct e_map_id(pub i32);
