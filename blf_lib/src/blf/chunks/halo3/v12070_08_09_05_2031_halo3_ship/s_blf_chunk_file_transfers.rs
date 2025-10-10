use std::u32;
use binrw::{binrw, BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
use blf_lib::types::array::StaticArray;
use blf_lib::types::c_string::StaticWcharString;
use crate::types::u64::Unsigned64;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("filq", 1.1)]
#[brw(big)]
#[Size(0x280)]
pub struct s_blf_chunk_file_transfers
{
    pub transfers: StaticArray<s_files_user_auto_download_queue_item, 8>
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
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
    pub size_bytes: u64,
}

impl BlfChunkHooks for s_blf_chunk_file_transfers { }
