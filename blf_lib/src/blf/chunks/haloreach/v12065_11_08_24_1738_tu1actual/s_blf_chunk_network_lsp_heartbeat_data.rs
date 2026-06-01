use binrw::binrw;
use blf_lib::types::array::StaticArray;
use blf_lib::types::u64::Unsigned64;
use serde::{Deserialize, Serialize};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;

pub use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::networking::logic::network_lsp_presence::{
    s_network_lsp_heartbeat_player_data, s_network_lsp_heartbeat_session_data,
};

/// Reach TU1 LSP presence heartbeat upload (`phbt` 5.1, 0x1BB-byte body).
#[binrw]
#[derive(BlfChunk, Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[Header("phbt", 5.1)]
#[Size(0x1BB)]
#[brw(big)]
pub struct s_blf_chunk_network_lsp_heartbeat_data {
    pub has_players: u8,
    pub local_player_count: u8,
    pub players: StaticArray<s_network_lsp_heartbeat_player_data, 4>,
    pub machine_id: Unsigned64,
    pub unknowneeA: StaticArray<u8, 8>,
    pub session_data: s_network_lsp_heartbeat_session_data,
}

impl BlfChunkHooks for s_blf_chunk_network_lsp_heartbeat_data {}
