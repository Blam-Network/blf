use std::u32;
use binrw::{binrw, BinRead, BinWrite};
#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use blf_lib::types::bool::Bool;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::{BlfChunk, TestSize};
use blf_lib::types::c_string::{StaticString, StaticWcharString};
use blf_lib::types::time::time32_t;
use crate::types::array::StaticArray;
use crate::types::numbers::Float32;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("srid", 7.1)]
#[brw(big)]
#[Size(0xD3C)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_blf_chunk_service_record
{
    pub player_name: StaticWcharString<16>, // Wide, 16 characters max
    pub player_info_available: Bool,
    pub unknown1: StaticArray<u8, 3>,
    pub armour_primary_color: u8, // 0x25
    pub armour_secondary_color: u8, // 0x26
    pub armour_tertiary_color: u8, // 0x27
    pub player_model_choice: u8, // 0x28
    // probably padding
    pub unknown2: StaticArray<u8, 3>,
    pub emblem_primary: u8, // 0x2C
    pub emblem_background: u8,
    pub emblem_secondary: Bool,
    pub emblem_primary_color: u8,
    pub emblem_secondary_color: u8,
    pub emblem_background_color: u8, // 0x31
    pub unknown3: StaticArray<u8, 14>,
    pub service_tag: StaticWcharString<5>, // 0x40
    pub unknown4: StaticArray<u8, 2>,
    pub career_overview_stats_available: Bool,
    pub credits_available: Bool,
    pub credits: i32,
    pub campaign_record_available: Bool,
    pub campaign_completed_at: time32_t,
    pub campaign_completion_difficulty: u32,
    pub campaign_enemies_killed: u32,
    pub campaign_vehicles_destroyed: u32,
    pub campaign_seconds_played: u32,
    // Why not 4? No idea
    pub campaign_difficulty_stats: StaticArray<s_blf_chunk_service_record_campaign_difficulty_stats, 3>,
    // TODO: simplify this with binrw.
    pub campaign_commendations_count: u32, // 0xAF Correct to here
    pub campaign_commendations: StaticArray<s_blf_chunk_service_record_commendation, 16>,
    pub firefight_record_available: Bool,
    pub firefight_covenant_kills: u32,
    pub firefight_vehicles_destroyed: u32,
    pub firefight_highest_set_completed: u32,
    pub firefight_most_kills_in_game: u32,
    pub firefight_waves_completed: u32,
    pub firefight_generators_destroyed: u32,
    pub firefight_enemy_players_killed: u32,
    pub firefight_difficulty_stats: StaticArray<s_blf_chunk_service_record_firefight_difficulty_stats, 3>,
    pub firefight_commendations_count: u32,
    pub firefight_commendations: StaticArray<s_blf_chunk_service_record_commendation, 16>,
    pub matchmaking_record_available: Bool, // 0x228 Correct to here
    pub matchmaking_games_won: u32,
    pub matchmaking_kills: u32,
    pub matchmaking_deaths: u32,
    pub matchmaking_assists: u32,
    pub matchmaking_category_stats: StaticArray<s_blf_chunk_service_record_matchmaking_category_stats, 5>,
    pub arena_season_stats_count: u32,
    pub arena_season_stats: StaticArray<s_blf_chunk_service_record_arena_season_stats, 3>,
    pub matchmaking_commendations_count: u32,
    pub matchmaking_commendations: StaticArray<s_blf_chunk_service_record_commendation, 16>,
    pub custom_games_record_available: Bool, // 0xCFD
    pub custom_games_multiplayer_played: u32,
    pub custom_games_multiplayer_kills: u32,
    pub custom_games_firefight_played: u32,
    pub custom_games_firefight_killed: u32,
    pub legacy_record_available: Bool,
    pub odst_first_played_time: time32_t,
    pub halo3_first_played_time: time32_t,
    pub halo2_first_played_time: time32_t,
    pub halo2_highest_difficulty: u32,
    pub halo2_unknown_1: u32,
    pub halo2_unknown_2: u32,
    pub halo3_highest_difficulty: u32,
    pub halo3_games_played: u32,
    pub halo3_multiplayer_kills: u32,
    pub odst_highest_difficulty: u32,
    pub odst_grunts_killed_in_firefight: u32,
    // I'm not 100% sure this is here, and if it is, it could be padding
    pub unknown5: u16,
}

#[derive(Default,PartialEq,Debug,Clone,Serialize,Deserialize,BinRead,BinWrite,TestSize)]
#[brw(big)]
#[Size(0x18)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_blf_chunk_service_record_campaign_difficulty_stats {
    pub covenant_kills: u32,
    pub vehicles_destroyed: u32,
    pub missions_completed_without_dying_or_restarting: u32,
    pub highest_skull_multiplier: u32,
    pub missions_complete: u32,
    pub unknown1: u32,
}

#[derive(Default,PartialEq,Debug,Clone,Serialize,Deserialize,BinRead,BinWrite,TestSize)]
#[brw(big)]
#[Size(0x1C)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_blf_chunk_service_record_firefight_difficulty_stats {
    pub covenant_kills: u32,
    pub vehicles_destroyed: u32,
    pub highest_set_completed: u32,
    pub most_consecutive_kills_without_dying: u32,
    pub biggest_kill: u32,
    pub times_beat_par: u32,
    pub highest_official_score: u32,
}


#[derive(Default,PartialEq,Debug,Clone,Serialize,Deserialize,BinRead,BinWrite,TestSize)]
#[brw(big)]
#[Size(0x14)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_blf_chunk_service_record_matchmaking_category_stats {
    pub games_won: u32,
    pub kills: u32,
    pub deaths: u32,
    pub assists: u32,
    pub percentage_of_matchmaking_games_played_in_category: u32,
}

#[derive(Default,PartialEq,Debug,Clone,Serialize,Deserialize,BinRead,BinWrite,TestSize)]
#[brw(big)]
#[Size(0x348)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_blf_chunk_service_record_arena_season_stats {
    pub season_number: u32,
    pub hopper_stats_count: u32,
    pub hopper_stats: StaticArray<s_blf_chunk_service_record_arena_hopper_stats, 8>,
}

#[derive(Default,PartialEq,Debug,Clone,Serialize,Deserialize,BinRead,BinWrite,TestSize)]
#[brw(big)]
#[Size(0x68)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_blf_chunk_service_record_arena_hopper_stats {
    pub hopper_name: StaticString<32>,
    pub hames_played_today: u32,
    pub current_best_set: u32,
    pub yesterdays_best_Set: u32,
    pub days_rated: u32,
    pub division_standing: u32,
    pub division: u32,
    pub unknown1: u32,
    pub unknown2: u32,
    pub unknown3: u32,
    pub unknown4: u32,
    pub unknown5: u32,
    pub games_played: u32,
    pub games_won: u32,
    pub kills: u32,
    pub assists: u32,
    pub deaths: u32,
    pub last_7_days_kill_and_assist_death_ratio: Float32,
    pub last_7_days_kill_death_ratio: Float32
}

#[derive(Default,PartialEq,Debug,Clone,Serialize,Deserialize,BinRead,BinWrite,TestSize)]
#[brw(big)]
#[Size(0x8)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_blf_chunk_service_record_commendation {
    pub commendation: u32,
    pub progress: u32,
}

impl BlfChunkHooks for s_blf_chunk_service_record {}
