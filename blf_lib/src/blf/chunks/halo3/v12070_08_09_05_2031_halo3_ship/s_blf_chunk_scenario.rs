use binrw::binrw;
use serde::{Deserialize, Serialize};
use crate::types::c_string::StaticWcharString;
use crate::types::array::StaticArray;
use crate::blam::halo3::v12070_08_09_05_2031_halo3_ship::cseries::language::k_language_count;
use blf_lib::blam::halo3::v12070_08_09_05_2031_halo3_ship::game::game_engine_default::k_game_engine_type_count;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
use crate::types::c_string::StaticString;
use crate::types::bool::Bool;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("levl", 3.1)]
#[brw(big)]
pub struct s_blf_chunk_scenario
{
    pub map_id: u32,
    pub map_flags: u32,
    pub names: StaticArray<StaticWcharString<32>, k_language_count>,
    pub descriptions: StaticArray<StaticWcharString<128>, k_language_count>,
    pub image_file_base: StaticString<256>,
    pub scenario_path: StaticString<256>,
    pub presence_context_id: u32,
    pub sort_order: u32,
    pub multiplayer_minimum_desired_players: u8,
    pub multiplayer_maximum_desired_players: u8,
    pub engine_maximum_teams: [u8; k_game_engine_type_count],
    pub allows_saved_films: Bool,
    // __pad112A: [u8; 6],
    #[brw(pad_before = 6)]
    pub insertion_points: StaticArray<s_blf_chunk_scenario_insertion, 4>,
}

impl BlfChunkHooks for s_blf_chunk_scenario {}

#[binrw]
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_blf_chunk_scenario_insertion {
    pub visible: Bool,
    pub flags: u8,
    pub zone_set: u16,
    // __pad4: [u8;4],
    #[brw(pad_before = 4)]
    pub names: StaticArray<StaticWcharString<32>, k_language_count>,
    pub descriptions: StaticArray<StaticWcharString<128>, k_language_count>,
}
