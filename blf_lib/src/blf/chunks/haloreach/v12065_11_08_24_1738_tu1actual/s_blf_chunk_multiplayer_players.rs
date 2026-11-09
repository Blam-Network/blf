use binrw::{binrw, BinRead, BinWrite};
use crate::types::array::StaticArray;
#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use crate::blf::chunks::BlfChunkHooks;
use crate::{BlfChunk, TestSize};
use crate::types::bool::Bool;
use crate::types::u64::Unsigned64;
use serde_hex::{SerHex, StrictCap};
use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::players::{
    s_player_configuration_from_client, s_player_configuration_from_host,
};

#[binrw]
#[derive(BlfChunk, Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[Header("mppl", 8.1)]
#[Size(0x1404)]
#[brw(big)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_blf_chunk_multiplayer_players {
    #[brw(pad_before = 4)]
    pub players: StaticArray<s_blf_chunk_multiplayer_players_player, 16>,
}

impl BlfChunkHooks for s_blf_chunk_multiplayer_players {}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(0x140)]
#[brw(big)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_blf_chunk_multiplayer_players_player {
    pub player_exists: Bool,
    pub machine_identifier: StaticArray<u8, 6>,
    #[serde(with = "SerHex::<StrictCap>")]
    pub player_identifier: Unsigned64,
    #[brw(pad_before = 1)]
    pub player_configuration_from_client: s_player_configuration_from_client,
    pub player_configuration_from_host: s_player_configuration_from_host,
    #[brw(pad_after = 3)]
    pub standing: i8,
    pub player_ratings: StaticArray<u32, 16>, // maybe this is game_engine_get_player_rating which is 60 long with some padding after
    pub result: i8,
    #[brw(pad_before = 1)]
    pub score: i16,
}
