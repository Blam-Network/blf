use binrw::binrw;
use serde::{Deserialize, Serialize};
use crate::blf::chunks::BlfChunkHooks;
use crate::BlfChunk;
use crate::types::bool::Bool;
use crate::types::string::StaticWcharString;
use crate::types::time::time32_t;
use crate::types::u64::Unsigned64;
use serde_hex::{SerHex, StrictCap};
pub use crate::blam::halo3::v12070_08_09_05_2031_halo3_ship::game::game_options::s_scenario_path;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("mpgd", 2.1)]
#[Size(0x174)]
#[brw(big)]
pub struct s_blf_chunk_multiplayer_game_description {
    pub team_game: Bool,
    #[serde(with = "SerHex::<StrictCap>")]
    pub game_id: Unsigned64,
    pub map_id: u32,
    pub scenario_path: s_scenario_path,
    pub map_variant_name: StaticWcharString<0x20>,
    #[serde(with = "SerHex::<StrictCap>")]
    pub game_variant_unique_id: Unsigned64,
    #[serde(with = "SerHex::<StrictCap>")]
    pub map_variant_unique_id: Unsigned64,
    pub started: Bool,
    pub start_time: time32_t,
    pub finished: Bool,
    pub finish_time: time32_t,
    pub migrated_to_group: Bool,
    pub migrated_solo: Bool,
    #[brw(pad_after = 6)]
    pub simulation_aborted: Bool
}

impl BlfChunkHooks for s_blf_chunk_multiplayer_game_description { }
