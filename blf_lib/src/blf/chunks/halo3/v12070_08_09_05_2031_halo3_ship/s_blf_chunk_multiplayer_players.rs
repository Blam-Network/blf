use binrw::{binrw, BinRead, BinWrite};
use crate::types::array::StaticArray;
use serde::{Deserialize, Serialize};
use crate::blf::chunks::BlfChunkHooks;
use crate::{BlfChunk, TestSize};
use crate::types::bool::Bool;
use crate::types::u64::Unsigned64;
use serde_hex::{SerHex, StrictCap};
use crate::blam::halo3::v12070_08_09_05_2031_halo3_ship::game::players::{s_player_configuration_from_client, s_player_configuration_from_host};

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("mppl", 2.1)]
#[Size(0x11E4)]
#[brw(big)]
pub struct s_blf_chunk_multiplayer_players {
    #[brw(pad_before = 4)]
    pub players: StaticArray<s_blf_chunk_multiplayer_players_player, 16>,
}

impl BlfChunkHooks for s_blf_chunk_multiplayer_players {}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(0x11E)]
pub struct s_blf_chunk_multiplayer_players_player {
    pub player_exists: Bool,
    pub machine_index: i8,
    #[serde(with = "SerHex::<StrictCap>")]
    pub player_identifier: Unsigned64,
    pub player_configuration_from_client: s_player_configuration_from_client,
    #[brw(align_before = 2)]
    pub player_configuration_from_host: s_player_configuration_from_host,
    #[brw(align_before = 2)]
    pub standing: i8,
    pub result: i8,
    pub score: i16,
}