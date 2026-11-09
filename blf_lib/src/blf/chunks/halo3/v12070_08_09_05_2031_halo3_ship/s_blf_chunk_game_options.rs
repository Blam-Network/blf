use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use crate::blf::chunks::BlfChunkHooks;
use crate::{BlfChunk, TestSize};
use crate::blam::halo3::v12070_08_09_05_2031_halo3_ship::game::game_engine_variant::c_game_variant;
use crate::blam::halo3::v12070_08_09_05_2031_halo3_ship::game::game_options::game_options;
use crate::blam::halo3::v12070_08_09_05_2031_halo3_ship::saved_games::scenario_map_variant::c_map_variant;
use crate::types::array::StaticArray;
use crate::types::bool::Bool;
use crate::types::string::{StaticString, StaticWcharString};
use crate::blam::halo3::v12070_08_09_05_2031_halo3_ship::game::players::{s_player_configuration_from_client, s_player_configuration_from_host};
use crate::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::s_machine_identifier;

#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize,BinRead,BinWrite)]
#[Header("gmop", 1.1)]
#[Size(0xF814)]
#[brw(big)]
pub struct s_blf_chunk_game_options {
    #[brw(pad_before = 4)]
    pub options: game_options,
}

impl BlfChunkHooks for s_blf_chunk_game_options {}
