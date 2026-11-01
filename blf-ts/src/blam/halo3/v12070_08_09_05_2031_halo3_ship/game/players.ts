import { c } from "@craftycodie/cstruct";
import { time32_t } from "../cseries/cseries_types";

export const s_queried_player_global_statistics = c.struct({
  valid: c.u8().padAfter(3),
  experience_base: c.u32(),
  experience_penalty: c.u32(),
  highest_skill: c.u32(),
});
export type s_queried_player_global_statistics = c.infer<
  typeof s_queried_player_global_statistics
>;

export const s_queried_player_hopper_statistics = c.struct({
  stats_valid: c.u8().padAfter(1),
  identifier: c.u16(),
  mu: c.f32(),
  sigma: c.f32(),
  hopper_skill: c.u32(),
  games_played: c.u32(),
  games_completed: c.u32(),
  games_won: c.u32(),
});
export type s_queried_player_hopper_statistics = c.infer<
  typeof s_queried_player_hopper_statistics
>;

export const s_queried_player_displayed_statistics = c.struct({
  stats_valid: c.u8().padAfter(3),
  matchmade_ranked_games_played: c.u32(),
  matchmade_ranked_games_completed: c.u32(),
  matchmade_ranked_games_won: c.u32(),
  matchmade_unranked_games_played: c.u32(),
  matchmade_unranked_games_completed: c.u32(),
  hopper_experience_base: c.u32(),
  custom_games_completed: c.u32(),
  hopper_experience_penalty: c.u32(),
  first_played: time32_t(),
  last_played: time32_t(),
});
export type s_queried_player_displayed_statistics = c.infer<
  typeof s_queried_player_displayed_statistics
>;

/** 0x58 bytes. */
export const s_queried_player_statistics = c.struct({
  queried_player_global_statistics: s_queried_player_global_statistics,
  queried_player_displayed_statistics: s_queried_player_displayed_statistics,
  queried_player_hopper_statistics: s_queried_player_hopper_statistics,
});
export type s_queried_player_statistics = c.infer<
  typeof s_queried_player_statistics
>;

/** 0x1E bytes. */
export const s_player_appearance = c.struct({
  appearance_flags: c.u8(),
  primary_color: c.u8(),
  secondary_color: c.u8(),
  tertiary_color: c.u8(),
  player_model_choice: c.u8(),
  foreground_emblem: c.u8().padBefore(1),
  background_emblem: c.u8(),
  emblem_flags: c.u8(),
  emblem_primary_color: c.u8(),
  emblem_secondary_color: c.u8(),
  emblem_background_color: c.u8().padAfter(2),
  spartan_model_area_0: c.u8(),
  spartan_model_area_1: c.u8(),
  spartan_model_area_2: c.u8(),
  spartan_model_area_3: c.u8(),
  elite_model_area_0: c.u8(),
  elite_model_area_1: c.u8(),
  elite_model_area_2: c.u8(),
  elite_model_area_3: c.u8(),
  service_tag: c.wstring(4),
});
export type s_player_appearance = c.infer<typeof s_player_appearance>;

/** 0xC8 bytes. */
export const s_player_configuration_from_client = c.struct({
  player_name: c.wstring(16),
  appearance: s_player_appearance,
  player_xuid: c.u64().padBefore(2),
  is_silver_or_gold_live: c.bool(),
  is_online_enabled: c.bool(),
  is_controller_attached: c.bool(),
  user_selected_team_index: c.i8(),
  desires_veto: c.bool(),
  desires_rematch: c.bool(),
  hopper_access_flags: c.u8(),
  is_free_live_gold_account: c.bool(),
  is_user_created_content_allowed: c.bool(),
  is_friend_created_content_allowed: c.bool(),
  is_griefer: c.i8(),
  campaign_difficulty_completed: c.i8(),
  bungienet_user_flags: c.u32(),
  gamer_region: c.i32(),
  gamer_zone: c.i32(),
  cheat_flags: c.u32(),
  ban_flags: c.u32(),
  repeated_play_coefficient: c.i32(),
  experience_growth_banned: c.bool().padAfter(3),
  queried_player_statistics: s_queried_player_statistics,
});
export type s_player_configuration_from_client = c.infer<
  typeof s_player_configuration_from_client
>;

/** 0x48 bytes. */
export const s_player_configuration_from_host = c.struct({
  player_name: c.wstring(16),
  player_team: c.i32(),
  player_assigned_team: c.i32(),
  stats_global_valid: c.bool().padAfter(3),
  stats_global_experience: c.i32(),
  stats_global_rank: c.i32(),
  stats_global_grade: c.i32(),
  stats_hopper_valid: c.bool().padAfter(3),
  stats_hopper_skill: c.i32(),
  stats_hopper_skill_display: c.i32(),
  stats_hopper_skill_update_weight: c.i32(),
});
export type s_player_configuration_from_host = c.infer<
  typeof s_player_configuration_from_host
>;

/** 0x110 bytes. */
export const s_player_configuration = c.struct({
  client: s_player_configuration_from_client,
  host: s_player_configuration_from_host,
});
export type s_player_configuration = c.infer<typeof s_player_configuration>;
