use binrw::{BinRead, BinWrite};
#[cfg(feature = "napi")]
use napi_derive::napi;
use blf_lib::TestSize;
use blf_lib::types::array::StaticArray;
use blf_lib::types::u64::Unsigned64;
use serde::{Deserialize, Serialize};

use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::players::s_player_appearance;
pub use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::networking::online::online_guide_xenon::s_network_session_privacy_mode;

/// 9 bytes — `s_static_array<...,16>` stride in Reach TU1.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(9)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_network_lsp_heartbeat_session_player_data {
    pub unknown0: StaticArray<u8, 8>,
    pub team: u8,
}

/// 201 bytes — host session block inside the heartbeat payload.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(201)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_network_lsp_heartbeat_session_data {
    pub gui_game_mode: u8,
    pub session_game_mode: u8,
    pub hopper_id: i16,
    pub session_piracy_mode: s_network_session_privacy_mode,
    pub unknownfa: u8,
    pub local_player_count: u8,
    pub player_count: u8,
    pub incoming_join_failed: u8,
    pub unknownfe: u8,
    pub session_players: StaticArray<s_network_lsp_heartbeat_session_player_data, 16>,
    pub unknown18f: StaticArray<u8, 44>,
}

/// 56 bytes — local profile slots in the heartbeat payload.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(56)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_network_lsp_heartbeat_player_data {
    pub player_xuid: Unsigned64,
    pub flags: i16,
    pub bungienet_user_flags: i16,
    pub player_grade: u8,
    pub player_sub_grade: u8,
    pub unknowne: i16,
    pub player_appearance: s_player_appearance,
}

/// 443-byte LSP presence heartbeat core (`phbt` 5.1 body; prefix of `phbt` 6.0).
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(0x1BB)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_network_lsp_heartbeat_data {
    pub has_players: u8,
    pub local_player_count: u8,
    pub players: StaticArray<s_network_lsp_heartbeat_player_data, 4>,
    pub machine_id: Unsigned64,
    pub unknowneeA: StaticArray<u8, 8>,
    pub session_data: s_network_lsp_heartbeat_session_data,
}

/// 0x93-byte LSP presence heartbeat response body (`phbr` chunk payload).
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(0x93)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_network_lsp_heartbeat_response_data {
    pub machine_file_requires_download: i8,
    pub flags: i8,
    pub xuid_count: i32,
    pub xuids: StaticArray<Unsigned64, 16>,
    pub session_id: Unsigned64,
    pub ack_number: i32,
    pub join_result: i8,
}
