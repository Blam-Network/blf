import { c } from "@craftycodie/cstruct";

/** Reach TU1 `s_queried_player_hopper_statistics` (20 bytes). */
export const s_queried_player_hopper_statistics = c.struct({
  valid: c.i8().padAfter(1),
  hopper_id: c.i16(),
  hopper_mu: c.i32(),
  hopper_sigma: c.i32(),
  games_played: c.i32(),
  games_won: c.i32(),
});
export type s_queried_player_hopper_statistics = c.infer<
  typeof s_queried_player_hopper_statistics
>;

export const e_player_appearance_flags = ["female_voice"] as const;

export const e_emblem_info_flags = [
  "alternate_foreground",
  "unknown_bit1",
  "unknown_bit2",
] as const;

/** Reach TU1 `s_player_appearance` (40 bytes). */
export const s_player_appearance = c.struct({
  flags: c.Bitfield(e_player_appearance_flags, c.u8()),
  primary_color: c.i8(),
  secondary_color: c.i8(),
  tertiary_color: c.i8(),
  player_model_choice: c.i8().padAfter(3),
  foreground_emblem: c.i8(),
  background_emblem: c.u8(),
  emblem_flags: c.Bitfield(e_emblem_info_flags, c.u8()),
  emblem_primary_color: c.i8(),
  emblem_secondary_color: c.i8(),
  emblem_background_color: c.i8().padAfter(2),
  model_permutations: c.u8().array(8),
  non_model_customization: c.u8().array(4),
  service_tag: c.wstring(5).padAfter(2),
});
export type s_player_appearance = c.infer<typeof s_player_appearance>;

export const e_queried_player_hopper_lsp_statistics_flags = [
  "hopper_valid",
  "day_stats_valid",
  "day_rating_valid",
  "tier_valid",
] as const;

/** Reach TU1 `s_queried_player_hopper_lsp_statistics` (30 bytes). */
export const s_queried_player_hopper_lsp_statistics = c.struct({
  flags: c
    .Bitfield(e_queried_player_hopper_lsp_statistics_flags, c.u8())
    .padAfter(1),
  hopper_identifier: c.u16(),
  hopper_day: c.i16().padAfter(2),
  qualified_games_played_today: c.u32(),
  qualifying_days_this_season: c.u32(),
  required_qualifying_daily_games_for_rating: c.u32(),
  required_qualifying_days_for_tier: c.u32(),
  tier: c.i16(),
  tier_pct: c.i16(),
  day_rating: c.u32(),
});
export type s_queried_player_hopper_lsp_statistics = c.infer<
  typeof s_queried_player_hopper_lsp_statistics
>;

export const e_player_challenge_state_flags = ["valid"] as const;

/** Reach TU1 `s_player_challenge_state` (20 bytes). */
export const s_player_challenge_state = c.struct({
  flags: c.Bitfield(e_player_challenge_state_flags, c.u8()).padAfter(3),
  daily_completed_count: c.u32(),
  daily_count: c.u32(),
  weekly_completed_count: c.u32(),
  weekly_count: c.u32(),
});
export type s_player_challenge_state = c.infer<typeof s_player_challenge_state>;

export const e_player_configuration_from_client_flags = [
  "live_account",
  "unknown_bit1",
  "unknown_bit2",
  "unknown_bit3",
  "live_enabled",
  "unknown_bit5",
  "griefer",
  "recent_grade_advancement",
  "recent_sub_grade_advancement",
] as const;

export enum e_bungienet_user_flags {
  registered_bungienet_user = 0,
  bungie_pro_member = 1,
  bungie_staff = 2,
  community0 = 3,
  community1 = 4,
  community2 = 5,
}

export const k_bungienet_user_background_bitmap_frame_shift = 12;

export function get_bungienet_user_background_bitmap_frame(
  bnet_flags: number
): number {
  const frame = bnet_flags >> k_bungienet_user_background_bitmap_frame_shift;
  const is_staff =
    (bnet_flags & (1 << e_bungienet_user_flags.bungie_staff)) !== 0;
  if (frame === 3 && !is_staff) {
    return -1;
  }
  return frame;
}

export const e_network_banhammer_cheat_flags = [
  "modified_content",
  "webstats_upload_failures_bit",
  "machine_file_upload_failures_bit",
  "reward_sync_upload_failures_bit",
  "arena_upload_failures_bit",
  "title_checksum_failed_bit",
] as const;

export const e_network_banhammer_ban_flags = [
  "banned_generic",
  "banned_from_matchmaking",
  "banned_from_hosting_in_matchmaking",
  "banned_from_hosting",
  "reduced_hosting_chance",
  "bad_client",
  "quick_disconnect",
  "voice_banned",
  "banned_from_live",
  "disallow_cookie_earn",
  "voice_automuted",
  "matchmaking_quit_probation",
  "speculative_migration_input",
  "banned_from_matchmaking_quit_probation_bit",
  "placeholder_14",
  "placeholder_15",
] as const;

/** Reach TU1 `s_player_configuration_from_client` (184 bytes). */
export const s_player_configuration_from_client = c.struct({
  desired_name: c.wstring(16),
  xuid: c.i64(),
  appearance: s_player_appearance,
  flags: c.Bitfield(e_player_configuration_from_client_flags, c.u16()),
  user_selected_multiplayer_team: c.i8(),
  hopper_access_flags: c.i8(),
  campaign_highest_difficulty: c.i8(),
  supply_depot_pct: c.i8(),
  commendation_unlock_pct: c.i8(),
  grade: c.i8(),
  sub_grade: c.i8().padAfter(1),
  bnet_flags: c.u16(),
  cheat_flags: c.Bitfield(e_network_banhammer_cheat_flags, c.u8()).padAfter(1),
  ban_flags: c.Bitfield(e_network_banhammer_ban_flags, c.u16()),
  repeated_play_coefficient: c.i32(),
  global_stats_valid: c.i8().padAfter(3),
  matchmade_games_played: c.u32(),
  hopper_stats: s_queried_player_hopper_statistics,
  lsp_stats: s_queried_player_hopper_lsp_statistics,
  challenge_state: s_player_challenge_state.padAfter(4),
});
export type s_player_configuration_from_client = c.infer<
  typeof s_player_configuration_from_client
>;

/** Reach TU1 `s_player_configuration_from_host` (48 bytes). */
export const s_player_configuration_from_host = c.struct({
  player_name: c.wstring(16),
  team: c.i8(),
  assigned_team: c.i8().padAfter(2),
  hopper_stats_valid: c.u8().padAfter(3),
  hopper_skill: c.u32(),
  hopper_weight: c.u32(),
});
export type s_player_configuration_from_host = c.infer<
  typeof s_player_configuration_from_host
>;
