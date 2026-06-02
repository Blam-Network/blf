import { c } from "@craftycodie/cstruct";

/** Reach TU1 `s_queried_player_hopper_statistics` (20 bytes). */
@c.struct()
export class s_queried_player_hopper_statistics {
  @c.field("i8", { pad_after: 1 })
  valid = 0;

  @c.field("i16")
  hopper_id = 0;

  @c.field("i32")
  hopper_mu = 0;

  @c.field("i32")
  hopper_sigma = 0;

  @c.field("i32")
  games_played = 0;

  @c.field("i32")
  games_won = 0;
}

/** Reach TU1 `s_player_appearance` (40 bytes). */
@c.struct()
export class s_player_appearance {
  @c.field("i8")
  voice = 0;

  @c.field("i8")
  primary_color = 0;

  @c.field("i8")
  secondary_color = 0;

  @c.field("i8")
  tertiary_color = 0;

  @c.field("i8", { pad_after: 3 })
  player_model_choice = 0;

  @c.field("i8")
  foreground_emblem = 0;

  @c.field("u8")
  background_emblem = 0;

  @c.field("u8")
  emblem_flags = 0;

  @c.field("i8")
  emblem_primary_color = 0;

  @c.field("i8")
  emblem_secondary_color = 0;

  @c.field("i8", { pad_after: 2 })
  emblem_background_color = 0;

  @c.field("u8", { count: 8 })
  model_permutations = Array.from({ length: 8 }, () => 0);

  @c.field("u8", { count: 4 })
  non_model_customization = Array.from({ length: 4 }, () => 0);

  @c.field(c.WString(5), { pad_after: 2 })
  service_tag = "";
}

/** Reach TU1 `s_queried_player_hopper_lsp_statistics` (30 bytes). */
@c.struct()
export class s_queried_player_hopper_lsp_statistics {
  @c.field("i8", { pad_after: 1 })
  flags = 0;

  @c.field("u16")
  hopper_identifier = 0;

  @c.field("i16")
  hopper_day = 0;

  @c.field("u32")
  qualified_games_played_today = 0;

  @c.field("u32")
  qualifying_days_this_season = 0;

  @c.field("u32")
  required_qualifying_daily_games_for_rating = 0;

  @c.field("u32")
  required_qualifying_days_for_tier = 0;

  @c.field("i16")
  tier = 0;

  @c.field("i16")
  tier_pct = 0;

  @c.field("u32")
  day_rating = 0;
}

/** Reach TU1 `s_player_challenge_state` (20 bytes). */
@c.struct()
export class s_player_challenge_state {
  @c.field("u8", { pad_after: 3 })
  flags = 0;

  @c.field("u32")
  daily_completed_count = 0;

  @c.field("u32")
  daily_count = 0;

  @c.field("u32")
  weekly_completed_count = 0;

  @c.field("u32")
  weekly_count = 0;
}

/** Reach TU1 `s_player_configuration_from_client` (184 bytes). */
@c.struct()
export class s_player_configuration_from_client {
  @c.field(c.WString(16))
  desired_name = "";

  @c.field("i64")
  xuid = 0n;

  @c.field(s_player_appearance)
  appearance = new s_player_appearance();

  @c.field("u16")
  flags = 0;

  @c.field("i8")
  user_selected_multiplayer_team = 0;

  @c.field("i8")
  hopper_access_flags = 0;

  @c.field("i8")
  campaign_highest_difficulty = 0;

  @c.field("i8")
  supply_depot_pct = 0;

  @c.field("i8")
  commendation_unlock_pct = 0;

  @c.field("i8")
  grade = 0;

  @c.field("i8", { pad_after: 1 })
  sub_grade = 0;

  @c.field("u16")
  bnet_flags = 0;

  @c.field("i8", { pad_after: 1 })
  cheat_flags = 0;

  @c.field("u16")
  ban_flags = 0;

  @c.field("i32")
  repeated_play_coefficient = 0;

  @c.field("i8", { pad_after: 1 })
  global_stats_valid = 0;

  @c.field("u32")
  matchmade_games_played = 0;

  @c.field(s_queried_player_hopper_statistics)
  hopper_stats = new s_queried_player_hopper_statistics();

  @c.field(s_queried_player_hopper_lsp_statistics, { pad_after: 4 })
  lsp_stats = new s_queried_player_hopper_lsp_statistics();

  @c.field(s_player_challenge_state, { pad_after: 4 })
  challenge_state = new s_player_challenge_state();
}

/** Reach TU1 `s_player_configuration_from_host` (48 bytes). */
@c.struct()
export class s_player_configuration_from_host {
  @c.field(c.WString(16))
  player_name = "";

  @c.field("i8", { pad_after: 2 })
  team = 0;

  @c.field("i8")
  assigned_team = 0;

  @c.field("u8", { pad_after: 3 })
  hopper_stats_valid = 0;

  @c.field("u32")
  hopper_skill = 0;

  @c.field("u32")
  hopper_weight = 0;
}
