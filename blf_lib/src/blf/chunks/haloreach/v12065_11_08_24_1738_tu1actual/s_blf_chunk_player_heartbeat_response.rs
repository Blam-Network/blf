use binrw::binrw;
#[cfg(feature = "napi")]
use napi_derive::napi;
use blf_lib::blf::chunks::BlfChunkHooks;
use blf_lib::types::array::StaticArray;
use blf_lib::types::u64::Unsigned64;
use blf_lib_derive::BlfChunk;
use serde::{Deserialize, Serialize};

pub use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::networking::logic::network_lsp_presence::s_network_lsp_heartbeat_response_data;

/// Reach TU1 LSP presence heartbeat response (`phbr` 2.1, 0x93-byte body).
#[binrw]
#[derive(BlfChunk, PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
#[Header("phbr", 2.1)]
#[Size(0x93)]
#[brw(big)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_blf_chunk_player_heartbeat_response {
    pub machine_file_requires_download: i8,
    pub flags: i8,
    pub xuid_count: i32,
    pub xuids: StaticArray<Unsigned64, 16>,
    pub session_id: Unsigned64,
    pub ack_number: i32,
    pub join_result: i8,
}

impl BlfChunkHooks for s_blf_chunk_player_heartbeat_response {}
