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
use blf_lib::types::time::time64_t;
use crate::types::c_string::StaticString;

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
#[brw(big)]
pub struct s_files_user_auto_download_queue_item {
    pub server_id: Unsigned64, // i hope it is anyway!
    pub file_type: u8,
    pub activity: u8,
    #[brw(pad_after = 1)]
    pub game_engine_type: u8,
    #[brw(pad_after = 3)]
    pub megalo_category_index: u8,
    #[brw(pad_after = 4)]
    pub map_id: u32,
    pub modified_time: time64_t,
    pub modified_by: StaticString<16>,
    pub file_name: StaticWcharString<128>,
    pub download_share_id: Unsigned64,
}

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
#[Header("filq", 3.0)]
#[brw(big)]
pub struct s_blf_chunk_file_transfers
{
    #[bw(try_calc(u32::try_from(transfers.len())))]
    transfers_count: u32,
    #[br(count = transfers_count)]
    pub transfers: Vec<s_files_user_auto_download_queue_item>
}

impl BlfChunkHooks for s_blf_chunk_file_transfers { }
