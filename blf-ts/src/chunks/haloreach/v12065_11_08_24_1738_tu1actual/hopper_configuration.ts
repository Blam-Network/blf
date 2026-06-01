import { c } from "@craftycodie/cstruct";

/** Reach custom hopper category entry in `mhcf`. */
@c.struct()
export class s_game_hopper_custom_category {
  @c.field("u16")
  category_identifier = 0;

  @c.field(c.String(32))
  category_name = "";

  @c.field("u16")
  unknown1 = 0;

  @c.field(c.String(32))
  unknown2 = "";
}

/** Latency desirability row inside {@link s_hopper_query_configuration}. */
@c.struct()
export class s_hopper_query_latency_desirability_configuration {
  @c.field("f32")
  unknown00 = 0;

  @c.field("u32")
  unknown04 = 0;

  @c.field("u32")
  unknown08 = 0;
}

/** Matchmaking query slot on a hopper configuration. */
@c.struct()
export class s_hopper_query_configuration {
  @c.field("u32")
  unknown00 = 0;

  @c.field(c.Bool(), { pad_after: 3 })
  unknown04 = false;

  @c.field("u32")
  unknown08 = 0;

  @c.field(c.Bool(), { pad_after: 3 })
  unknown0c = false;

  @c.field("f32")
  unknown10 = 0;

  @c.field(c.Bool(), { pad_after: 3 })
  unknown14 = false;

  @c.field("f32")
  unknown18 = 0;

  @c.field("f32")
  unknown1c = 0;

  @c.field("f32")
  unknown20 = 0;

  @c.field(c.Bool())
  unknown24 = false;

  @c.field(c.Bool(), { pad_after: 2 })
  unknown25 = false;

  @c.field("u32")
  unknown28 = 0;

  @c.field("u32")
  unknown2c = 0;

  @c.field("u32")
  unknown30 = 0;

  @c.field(c.Bool(), { pad_after: 3 })
  unknown34 = false;

  @c.field(s_hopper_query_latency_desirability_configuration, { count: 2 })
  latency_desirability_configurations = Array.from(
    { length: 2 },
    () => new s_hopper_query_latency_desirability_configuration()
  );

  @c.field("f32", { count: 17 })
  unknown50 = Array.from({ length: 17 }, () => 0);
}

/** Per-team limits inside {@link c_hopper_configuration}. */
@c.struct()
export class s_hopper_configuration_per_team_data {
  @c.field("u32")
  minimum_team_size = 0;

  @c.field("u32")
  maximum_team_size = 0;

  @c.field("u32")
  team_model_override = 0;

  @c.field("u32")
  team_allegiance = 0;
}

/** Jackpot reward row on a hopper configuration. */
@c.struct()
export class s_hopper_jackpot_configuration {
  @c.field("u32")
  chance = 0;

  @c.field("u32")
  minimum_cookies = 0;

  @c.field("u32")
  maximum_cookies = 0;
}

/** Map voting settings on a hopper configuration. */
@c.struct()
export class s_hopper_voting_configuration {
  @c.field("u32")
  voting_categories = 0;

  @c.field("u32")
  number_of_votes_per_peer = 0;

  @c.field("u32")
  voting_rounds = 0;

  @c.field("u32")
  veto_rounds = 0;

  @c.field("u32")
  maximum_load_failures = 0;

  @c.field("u32")
  voting_time_seconds = 0;

  @c.field("u8", { pad_after: 3 })
  flags = 0;
}

/** Single hopper entry in the Reach matchmaking configuration table (`mhcf`). */
@c.struct()
export class c_hopper_configuration {
  @c.field(c.String(32))
  hopper_name = "";

  @c.field("u8", { count: 20 })
  game_set_hash = Array.from({ length: 20 }, () => 0);

  @c.field("u16")
  identifier = 0;

  @c.field("u16")
  category_identifier = 0;

  @c.field("u8")
  category_index = 0;

  @c.field("u8", { pad_after: 2 })
  player_investment_category = 0;

  @c.field("u32")
  image_index = 0;

  @c.field("u32")
  xlast_index = 0;

  @c.field("u8", { pad_after: 3 })
  equivalency_id = 0;

  @c.field("u64")
  start_time = 0n;

  @c.field("u64")
  end_time = 0n;

  @c.field("u32")
  minimum_games_won = 0;

  @c.field("u32")
  maximum_games_won = 0;

  @c.field("u32")
  minimum_games_played = 0;

  @c.field("u32")
  maximum_games_played = 0;

  @c.field("u32")
  minimum_grade = 0;

  @c.field("u32")
  maximum_grade = 0;

  @c.field("u32")
  min_party_size = 0;

  @c.field("u32")
  max_party_size = 0;

  @c.field("u32")
  min_local_players = 0;

  @c.field("u32")
  max_local_players = 0;

  @c.field("i32")
  hopper_access_bit = 0;

  @c.field("u32")
  account_type_access = 0;

  @c.field(c.Bool())
  require_all_party_members_meet_games_played_requirements = false;

  @c.field("u8")
  unknown89 = 0;

  @c.field(c.Bool())
  require_all_party_members_meet_grade_requirements = false;

  @c.field(c.Bool())
  require_all_party_members_meet_access_requirements = false;

  @c.field(c.Bool())
  require_all_party_members_meet_live_account_access_requirements = false;

  @c.field(c.Bool())
  hide_hopper_from_games_played_restricted_players = false;

  @c.field("u8")
  unknown8e = 0;

  @c.field(c.Bool())
  hide_hopper_from_grade_restricted_players = false;

  @c.field(c.Bool())
  hide_hopper_from_access_restricted_players = false;

  @c.field(c.Bool())
  hide_hopper_from_live_account_access_restricted_players = false;

  @c.field(c.Bool())
  hide_hopper_due_to_time_restriction = false;

  @c.field(c.Bool())
  requires_hard_drive = false;

  @c.field(c.Bool(), { pad_after: 3 })
  requires_local_party = false;

  @c.field(s_hopper_voting_configuration)
  voting_configuration = new s_hopper_voting_configuration();

  @c.field("u8")
  is_ranked = 0;

  @c.field("u8")
  is_arbitrated = 0;

  @c.field("u8")
  are_guests_allowed = 0;

  @c.field("u8")
  are_opponents_visible = 0;

  @c.field("u8")
  uses_arena_lsp_stats = 0;

  @c.field("u8", { pad_after: 2 })
  unknownb9 = 0;

  @c.field("u32")
  unknownbc = 0;

  @c.field("u32")
  unknownc0 = 0;

  @c.field("u8")
  unknownc4 = 0;

  @c.field("u8", { pad_after: 2 })
  uses_high_score_leaderboard = 0;

  @c.field("u32")
  posse_formation = 0;

  @c.field("u32")
  post_match_countdown_time_seconds = 0;

  @c.field(c.Bool(), { pad_after: 3 })
  require_hosts_on_multiple_teams = false;

  @c.field("u32")
  repeated_opponents_to_consider_for_penalty = 0;

  @c.field("u32")
  repeated_opponents_skill_throttle_start = 0;

  @c.field("u32")
  repeated_opponents_skill_throttle_stop = 0;

  @c.field("u8")
  matchmaking_composition_build_flags = 0;

  @c.field(c.Bool(), { pad_after: 2 })
  is_team_matching_enabled = false;

  @c.field("u32")
  gather_start_threshold_seconds = 0;

  @c.field("u32")
  get_gather_start_game_early_seconds = 0;

  @c.field("u32")
  get_gather_give_up_seconds = 0;

  @c.field("u8", { count: 16 })
  chance_of_gathering = Array.from({ length: 16 }, () => 0);

  @c.field("u32")
  unknown100 = 0;

  @c.field("u32")
  unknown104 = 0;

  @c.field("u32")
  unknown108 = 0;

  @c.field(c.Bool())
  uses_ffa_scoring_for_leaderboard_writes = false;

  @c.field(c.Bool(), { pad_after: 2 })
  should_modify_skill_update_weight_with_game_quality = false;

  @c.field("f32")
  trueskill_sigma_multiplier = 0;

  @c.field("f32")
  unknown114 = 0;

  @c.field("f32")
  trueskill_tau_dynamics_factor = 0;

  @c.field("u32")
  trueskill_draw_probability = 0;

  @c.field("u32")
  pre_match_voice_configuration = 0;

  @c.field("u32")
  in_match_voice_configuration = 0;

  @c.field("u32")
  post_match_voice_configuration = 0;

  @c.field("u32")
  restrict_open_channel = 0;

  @c.field("u32")
  unknown130 = 0;

  @c.field(s_hopper_query_configuration, { count: 4 })
  query_configurations = Array.from(
    { length: 4 },
    () => new s_hopper_query_configuration()
  );

  @c.field("u8")
  game_type = 0;

  @c.field(c.Bool(), { pad_after: 2 })
  is_ffa = false;

  @c.field("u32")
  minimum_player_count = 0;

  @c.field("u32")
  maximum_player_count = 0;

  @c.field("u32")
  ffa_model_override = 0;

  @c.field("u32")
  minimum_team_count = 0;

  @c.field("u32")
  maximum_team_count = 0;

  @c.field(s_hopper_configuration_per_team_data, { count: 8 })
  per_team_data = Array.from(
    { length: 8 },
    () => new s_hopper_configuration_per_team_data()
  );

  @c.field("u32")
  maximum_team_imbalance = 0;

  @c.field("u32")
  big_squad_size_threshold = 0;

  @c.field("u32")
  unknown424 = 0;

  @c.field("u32")
  unknown428 = 0;

  @c.field("u32")
  undersized_party_split_permissions = 0;

  @c.field("u32")
  jackpot_minimum_time_seconds = 0;

  @c.field(s_hopper_jackpot_configuration, { count: 3 })
  jackpot_configurations = Array.from(
    { length: 3 },
    () => new s_hopper_jackpot_configuration()
  );
}
