use binrw::{binrw, BinRead, BinWrite};
use crate::types::net::Ipv4Addr;
use serde::{Deserialize, Serialize};
use crate::types::array::StaticArray;
use crate::blf::chunks::BlfChunkHooks;
use crate::{BlfChunk, TestSize};
use crate::types::bool::Bool;
use crate::types::u64::Unsigned64;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("mpma", 2.1)]
#[Size(0x1744)]
#[brw(big)]
pub struct s_blf_chunk_multiplayer_machines {
    pub in_squad_session: Bool,
    pub in_group_session: Bool,
    #[brw(pad_before = 2)]
    pub machines: StaticArray<s_blf_chunk_multiplayer_machines_machine, 16>,
}

impl BlfChunkHooks for s_blf_chunk_multiplayer_machines {}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(0x174)]
pub struct s_blf_chunk_multiplayer_machines_machine {
    pub machine_data: s_game_results_machine_data, // 21
    pub session_info: s_machine_session_info,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
pub struct s_machine_identifier {
    pub identifier: StaticArray<u8, 6>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
pub struct s_game_results_machine_data {
    pub identifier: s_machine_identifier,
    pub exists: Bool,
    pub connected_to_host: Bool,
    pub host: Bool,
    pub initial_host: Bool,
    pub voluntary_quit: Bool,
    #[brw(pad_before = 1)]
    pub bandwidth_events: StaticArray<i16, 5>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
pub struct s_transport_secure_address {
    pub data: StaticArray<u8, 320>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(repr = i32)]
#[repr(i32)]
pub enum e_online_nat_type {
    #[default]
    Unknown = 0,
    Open = 1,
    Moderate = 2,
    Strict = 3,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
pub struct s_machine_session_info {
    pub exists: Bool,
    pub has_hard_drive: Bool,
    pub party_nonce: Unsigned64, // 10
    pub secure_address: s_transport_secure_address, // - 34
    pub network_version_number: i32,
    pub peer_estimated_downstream_bandwidth_bps: i32,
    pub peer_estimated_upstream_bandwidth_bps: i32,
    pub peer_nat_type: e_online_nat_type,
    pub peer_to_peer_connectivity_mask: u16,
    pub peer_to_peer_probed_mask: u16, // 24
}