use binrw::{BinRead, BinWrite};
#[cfg(feature = "napi")]
use napi_derive::napi;
use blf_lib::TestSize;
use blf_lib::types::c_string::StaticWcharString;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(20)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_queried_player_hopper_statistics {
    #[brw(pad_after = 1)]
    pub valid: i8,
    pub hopper_id: i16,
    pub hopper_mu: i32,
    pub hopper_sigma: i32,
    pub games_played: i32,
    pub games_won: i32,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(40)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_player_appearance {
    pub voice: i8,
    pub primary_color: i8,
    pub secondary_color: i8,
    pub tertiary_color: i8,
    #[brw(pad_after = 3)]
    pub player_model_choice: i8,
    pub foreground_emblem: i8,
    pub background_emblem: u8,
    pub emblem_flags: u8,
    pub emblem_primary_color: i8,
    pub emblem_secondary_color: i8,
    #[brw(pad_after = 2)]
    pub emblem_background_color: i8,
    pub model_permutations: [i8; 8],
    pub non_model_customization: [i8; 4],
    #[brw(pad_after = 2)]
    pub service_tag: StaticWcharString<5>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(30)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_queried_player_hopper_lsp_statistics {
    #[brw(pad_after = 1)]
    pub flags: i8,
    pub hopper_identifier: u16,
    pub hopper_day: i16,
    pub qualified_games_played_today: u32,
    pub qualifying_days_this_season: u32,
    pub required_qualifying_daily_games_for_rating: u32,
    pub required_qualifying_days_for_tier: u32,
    pub tier: i16,
    pub tier_pct: i16,
    pub day_rating: u32,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(20)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_player_challenge_state {
    #[brw(pad_after = 3)]
    pub flags: u8,
    pub daily_completed_count: u32,
    pub daily_count: u32,
    pub weekly_completed_count: u32,
    pub weekly_count: u32,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(184)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_player_configuration_from_client {
    pub desired_name: StaticWcharString<16>,
    pub xuid: i64,
    pub appearance: s_player_appearance,
    pub flags: u16,
    pub user_selected_multiplayer_team: i8,
    pub hopper_access_flags: i8,
    pub campaign_highest_difficulty: i8,
    pub supply_depot_pct: i8,
    pub commendation_unlock_pct: i8,
    pub grade: i8,
    #[brw(pad_after = 1)]
    pub sub_grade: i8,
    pub bnet_flags: u16,
    pub cheat_flags: i8,
    #[brw(pad_after = 1)]
    pub ban_flags: u16,
    pub repeated_play_coefficient: i32,
    #[brw(pad_after = 1)]
    pub global_stats_valid: i8,
    pub matchmade_games_played: u32,
    pub hopper_stats: s_queried_player_hopper_statistics,
    #[brw(pad_after = 4)]
    pub lsp_stats: s_queried_player_hopper_lsp_statistics,
    #[brw(pad_after = 4)]
    pub challenge_state: s_player_challenge_state,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(48)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_player_configuration_from_host {
    pub player_name: StaticWcharString<16>,
    pub team: i8,
    #[brw(pad_after = 2)]
    pub assigned_team: i8,
    pub hopper_stats_valid: u8,
    #[brw(pad_after = 3)]
    pub hopper_skill: u32,
    pub hopper_weight: u32,
}
