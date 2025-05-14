use binrw::{BinRead, BinWrite};
#[cfg(feature = "napi")]
use napi_derive::napi;
use blf_lib::TestSize;
use blf_lib::types::bool::Bool;
use blf_lib::types::c_string::StaticWcharString;
use serde::{Deserialize, Serialize};
use serde_hex::{SerHex,StrictCap};
use crate::types::numbers::Float32;
use crate::types::time::time32_t;
use crate::types::u64::Unsigned64;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct s_queried_player_global_statistics {
    #[brw(pad_after = 3)]
    pub valid: u8,
    pub experience_base: u32,
    pub experience_penalty: u32,
    pub highest_skill: u32,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct s_queried_player_hopper_statistics {
    #[brw(pad_after = 1)]
    pub stats_valid: u8,
    pub identifier: u16,
    pub mu: Float32,
    pub sigma: Float32,
    pub hopper_skill: u32,
    pub games_played: u32,
    pub games_completed: u32,
    pub games_won: u32,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct s_queried_player_displayed_statistics {
    #[brw(pad_after = 3)]
    pub stats_valid: u8,
    pub matchmade_ranked_games_played: u32,
    pub matchmade_ranked_games_completed: u32,
    pub matchmade_ranked_games_won: u32,
    pub matchmade_unranked_games_played: u32,
    pub matchmade_unranked_games_completed: u32,
    pub hopper_experience_base: u32,
    pub custom_games_completed: u32, // definitely in this order?
    pub hopper_experience_penalty: u32,
    pub first_played: time32_t,
    pub last_played: time32_t,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(88)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct s_queried_player_statistics {
    pub queried_player_global_statistics: s_queried_player_global_statistics,
    pub queried_player_displayed_statistics: s_queried_player_displayed_statistics,
    pub queried_player_hopper_statistics: s_queried_player_hopper_statistics,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(0x1E)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct s_player_appearance {
    pub appearance_flags: u8,
    pub primary_color: u8,
    pub secondary_color: u8,
    pub tertiary_color: u8,
    pub player_model_choice: u8,
    #[brw(pad_before = 1)]
    pub foreground_emblem: u8,
    pub background_emblem: u8,
    pub emblem_flags: u8,
    pub emblem_primary_color: u8,
    pub emblem_secondary_color: u8,
    #[brw(pad_after = 2)]
    pub emblem_background_color: u8,
    pub spartan_model_area_0: u8,
    pub spartan_model_area_1: u8,
    pub spartan_model_area_2: u8,
    pub spartan_model_area_3: u8,
    pub elite_model_area_0: u8,
    pub elite_model_area_1: u8,
    pub elite_model_area_2: u8,
    pub elite_model_area_3: u8,
    pub service_tag: StaticWcharString<4>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(0xC8)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct s_player_configuration_from_client {
    pub player_name: StaticWcharString<16>,
    pub appearance: s_player_appearance, // correct
    #[brw(pad_before = 2)]
    #[serde(with = "SerHex::<StrictCap>")]
    pub player_xuid: Unsigned64,
    pub is_silver_or_gold_live: Bool,
    pub is_online_enabled: Bool,
    pub is_controller_attached: Bool,
    pub user_selected_team_index: i8,
    pub desires_veto: Bool,
    pub desires_rematch: Bool,
    pub hopper_access_flags: u8,
    pub is_free_live_gold_account: Bool,
    pub is_user_created_content_allowed: Bool,
    pub is_friend_created_content_allowed: Bool,
    pub is_griefer: i8, // not a bool?
    pub campaign_difficulty_completed: i8,
    #[serde(with = "SerHex::<StrictCap>")]
    pub bungienet_user_flags: u32,
    pub gamer_region: i32,
    pub gamer_zone: i32,
    pub cheat_flags: u32,
    pub ban_flags: u32,
    pub repeated_play_coefficient: i32,
    #[brw(pad_after = 3)]
    pub experience_growth_banned: Bool,
    pub queried_player_statistics: s_queried_player_statistics, // good
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(0x48)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct s_player_configuration_from_host {
    pub player_name: StaticWcharString<16>,
    pub player_team: i32,
    pub player_assigned_team: i32,
    #[brw(pad_after = 3)]
    pub stats_global_valid: Bool,
    pub stats_global_experience: i32,
    pub stats_global_rank: i32,
    pub stats_global_grade: i32,
    #[brw(pad_after = 3)]
    pub stats_hopper_valid: Bool,
    pub stats_hopper_skill: i32,
    pub stats_hopper_skill_display: i32,
    pub stats_hopper_skill_update_weight: i32,
}