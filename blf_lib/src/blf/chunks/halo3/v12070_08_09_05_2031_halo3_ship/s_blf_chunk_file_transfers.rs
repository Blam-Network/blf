use std::u32;
use binrw::{binrw, BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
use blf_lib::types::array::StaticArray;
use blf_lib::types::c_string::StaticWcharString;
use blf_lib::types::u64::Unsigned64;
#[cfg(feature = "napi")]
use napi_derive::napi;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
#[Header("filq", 1.1)]
#[brw(big)]
#[Size(0x280)]
pub struct s_blf_chunk_file_transfers
{
    pub transfers: StaticArray<s_files_user_auto_download_queue_item, 8>
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
#[brw(big)]
pub struct s_files_user_auto_download_queue_item {
    pub player_xuid: Unsigned64,
    pub slot: u32,
    #[brw(pad_after = 2)]
    pub title_index: u16,
    pub server_id: Unsigned64,
    pub file_name: StaticWcharString<16>,
    pub file_type: i32,
    pub campaign_id: u32,
    pub map_id: u32,
    pub game_engine_type: u32,
    pub size_bytes: Unsigned64,
}

impl BlfChunkHooks for s_blf_chunk_file_transfers { }
