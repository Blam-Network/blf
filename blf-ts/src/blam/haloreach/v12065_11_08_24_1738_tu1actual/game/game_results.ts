import { c } from "@craftycodie/cstruct";
import {
  integer_statistic,
  integer_statistic_array,
  s_real_point3d,
} from "../memory/integer_statistic";
import {
  c_player_profile_interface,
  s_network_social_preferences,
} from "../saved_games/player_profile";
import { game_variant_field } from "./game_variant";
import {
  s_player_configuration_from_client,
  s_player_configuration_from_host,
} from "./players";

export const k_maximum_players = 16;

export const k_maximum_teams = 16;

export const k_maximum_machines = 16;

export const k_game_results_statistic_count = 19;

export const k_game_results_player_rewards_statistic_count = 142;

export const k_game_results_medal_count = 128;

export const k_game_results_incident_count = 50;

export const k_damage_reporting_type_count = 82;

export const k_game_results_damage_statistic_count = 5;

export const k_game_results_griefing_statistic_count = 6;

export const k_campaign_metagame_bucket_type_count = 35;

export const k_campaign_metagame_bucket_class_count = 8;

export const k_game_results_event_count = 2000;

export const k_game_results_medal_event_count = 2000;

export enum e_game_results_statistic {
  games_played = 0,
  games_completed = 1,
  games_won = 2,
  games_tied = 3,
  rounds_completed = 4,
  rounds_won = 5,
  in_round_score = 6,
  in_game_total_score = 7,
  kills = 8,
  assists = 9,
  deaths = 10,
  betrayals = 11,
  suicides = 12,
  most_kills_in_a_row = 13,
  seconds_alive = 14,
  custom_1 = 15,
  custom_2 = 16,
  custom_3 = 17,
  custom_4 = 18,
}

export enum e_game_results_reward_statistic {
  cookies = 0,
  commendation = 1,
}

export enum e_game_results_damage_statistic {
  kills = 0,
  deaths = 1,
  betrayals = 2,
  suicides = 3,
  headshots = 4,
}

export enum e_game_results_griefing_statistic {
  shield_damage = 0,
  body_damage = 1,
  betrayals = 2,
  shield_damage_penalty = 3,
  body_damage_penalty = 4,
  betrayal_penalty = 5,
}

export enum e_game_results_player_vs_player_statistic {
  kills = 0,
  deaths = 1,
}

export enum e_game_results_player_vs_ai_statistic {
  kills = 0,
  betrayals = 1,
}

export enum e_game_results_event_type {
  none = 0,
  kill = 1,
  ai_kill = 2,
  betrayal = 3,
  ai_betrayal = 4,
  ai_death = 5,
  carry = 6,
  score = 7,
  medal = 8,
}

export const s_game_results_game_description = c.struct({
  game_instance: c.u64(),
  team_game: c.bool(),
  campaign_metagame_enabled: c.bool().padAfter(2),
  game_mode: c.i32(),
  initial_skulls: c.u32(),
  campaign_difficulty: c.i32(),
  zone_set_name: c.string(128, "latin1"),
  game_variant: game_variant_field().padAfter(4),
  game_variant_unique_id: c.u64(),
  round_time_limit_seconds: c.i32(),
  score_to_win_round: c.i32(),
  number_of_rounds: c.i32(),
  custom_statistic_count: c.i32(),
  custom_statistic_name_string_offsets: c.i32().array(4),
  map_id: c.i32().padAfter(4),
  map_variant_unique_id: c.u64(),
  map_variant_name: c.wstring(32),
  scenario_path: c.string(260, "latin1"),
  campaign_metagame_time_ticks: c.i32(),
  campaign_metagame_time_multiplier: c.f32(),
  campaign_metagame_final_score: c.f32(),
  scenario_par_score: c.f32().padAfter(4),
  survival_sets_completed: c.i32(),
  survival_full_rounds_completed: c.i32(),
  survival_waves_completed: c.i32(),
  survival_highest_skull_combo: c.u32(),
});

export type s_game_results_game_description = c.infer<
  typeof s_game_results_game_description
>;

export const s_game_set_replicated_data = c.struct({
  flags: c
    .Bitfield(["cookie_scaling_factors_valid"] as const, c.u8())
    .padAfter(1),
  game_set_entry_index: c.i16(),
  leaderboard_column_index: c.i32(),
  cookie_earn_scaling_factor: c.f32(),
  cookie_winner_bonus_scaling_factor: c.f32(),
  cookie_performance_scaling_factor: c.f32(),
});

export type s_game_set_replicated_data = c.infer<
  typeof s_game_set_replicated_data
>;

export const s_game_matchmaking_options = c.struct({
  flags: c
    .Bitfield(
      [
        "team_game",
        "ranked",
        "uses_arena_lsp_stats",
        "uses_high_score_leaderboard",
        "uses_ffa_scoring_for_leaderboard_writes",
        "auto_upload_film",
      ] as const,
      c.u8()
    )
    .padAfter(1),
  hopper_identifier: c.u16(),
  xlast_index: c.u8().padAfter(1),
  hopper_name: c.wstring(32).padAfter(2),
  trueskill_draw_probability: c.i32(),
  trueskill_beta_performance_variation: c.f32(),
  trueskill_tau_dynamics_factor: c.f32(),
  player_investment_category: c.u8().padAfter(3),
  posse_formation: c.i32(),
  jackpot_cookies: c.i32(),
  jackpot_minimum_time_seconds: c.i32(),
  valid_team_mask: c.u32(),
  team_model_overrides: c.i32().array(8),
  ffa_model_override: c.i32(),
  arena_season: c.i32(),
  game_set_replicated_data: s_game_set_replicated_data,
});

export type s_game_matchmaking_options = c.infer<
  typeof s_game_matchmaking_options
>;

export const s_game_results_player_data = c.struct({
  player_exists: c.bool(),
  machine_identifier: c.u8().array(6),
  player_identifier: c.u64().padAfter(1),
  configuration_from_client: s_player_configuration_from_client,
  configuration_from_host: s_player_configuration_from_host,
  standing: c.i8().padAfter(35),
  voluntary_mute_mask: c.u32(),
  player_rating: c.i16(),
  score: c.i16(),
  campaign_metagame_final_score: c.i32(),
  campaign_metagame_best_kill_victim_player_index: c.i32(),
  campaign_metagame_best_kill_bucket_type: c.i32(),
  campaign_metagame_best_kill_bucket_class: c.i32(),
  campaign_metagame_best_kill_points: c.i32(),
  ticks_played: c.u32().padAfter(4),
});

export type s_game_results_player_data = c.infer<
  typeof s_game_results_player_data
>;

export const s_game_results_team_data = c.struct({
  team_exists: c.bool(),
  standing: c.i8(),
  score: c.i16(),
});

export type s_game_results_team_data = c.infer<typeof s_game_results_team_data>;

export const s_game_results_machine_data = c.struct({
  machine_identifier: c.u8().array(6),
  machine_exists: c.bool(),
  machine_connected: c.bool(),
  machine_host: c.bool(),
  machine_initial_host: c.bool(),
  machine_leave_reason: c.u8(),
  only_machine_with_full_connectivity: c.bool(),
  machine_on_only_team_with_full_connectivity: c.bool(),
  bandwidth_events: c.u8().array(5).padAfter(6),
});

export type s_game_results_machine_data = c.infer<
  typeof s_game_results_machine_data
>;

export const s_transport_secure_address = c.struct({
  ina: c.u8().array(4),
  ina_online: c.u8().array(4),
  port_online: c.u16(),
  enet_address: c.u8().array(6),
  online_address: c.u8().array(20),
});

export type s_transport_secure_address = c.infer<
  typeof s_transport_secure_address
>;

export const s_network_quality_estimated = c.struct({
  estimated_upstream_bps: c.u32(),
  estimated_upstream_bps_reliability: c.f32(),
  estimated_upstream_congestion_variance: c.f32(),
  estimated_latency_ms: c.u32(),
  estimated_latency_variance_ms: c.u32(),
  estimated_packet_loss_pct: c.f32(),
  connectivity_badness_pct: c.f32(),
  host_badness_pct: c.f32(),
  client_badness_pct: c.f32(),
});

export type s_network_quality_estimated = c.infer<
  typeof s_network_quality_estimated
>;

export const s_machine_locality = c.struct({
  x: c.f32(),
  y: c.f32(),
  z: c.f32(),
});

export type s_machine_locality = c.infer<typeof s_machine_locality>;

export const s_network_session_peer_properties = c.struct({
  state_token: c.u32(),
  peer_name: c.wstring(32),
  peer_session_name: c.wstring(32).padAfter(4),
  available_map_mask: c.u64(),
  available_dlc_map_pack_mask: c.u8().padAfter(3),
  peer_map_id: c.i32(),
  peer_map_status: c.i32(),
  peer_map_progress_percentage: c.i32(),
  peer_game_instance: c.u64(),
  ready_hopper_id: c.u32(),
  game_start_warning: c.i8(),
  game_start_error: c.i8(),
  has_hard_drive: c.bool(),
  installed_to_hard_drive: c.bool(),
  hard_drive_space: c.u8(),
  hard_drive_free_space: c.u8().padAfter(2),
  estimated_network_quality: s_network_quality_estimated,
  nat_type: c.u32(),
  locality: s_machine_locality,
  peer_connectivity_mask: c.u32(),
  peer_probe_mask: c.u32(),
  language: c.u32(),
  social_preferences: s_network_social_preferences,
  determinism_version: c.u32(),
  determinism_compatible_version: c.u32(),
  flags: c.u32(),
  matchmaking_team_nonce: c.u64(),
  online_party_player_count: c.i8(),
  online_party_players_on_enemy_team: c.bool(),
  squad_session_friend_count: c.i8().padAfter(1),
  party_host_peer_masks: c.u32().array(2),
});

export type s_network_session_peer_properties = c.infer<
  typeof s_network_session_peer_properties
>;

export const s_network_session_peer = c.struct({
  secure_address: s_transport_secure_address,
  connection_state: c.u32(),
  network_version_number: c.u32(),
  creation_timestamp: c.u32(),
  properties: s_network_session_peer_properties.padAfter(4),
  join_nonce: c.u64(),
  player_count: c.i32(),
  player_indices: c.i32().array(4).padAfter(4),
});

export type s_network_session_peer = c.infer<typeof s_network_session_peer>;

export const s_blf_chunk_machines_entry = c.struct({
  machine: s_game_results_machine_data,
  peer: s_network_session_peer,
});

export type s_blf_chunk_machines_entry = c.infer<
  typeof s_blf_chunk_machines_entry
>;

export const s_game_results_damage_statistics = c.struct({
  valid: c.bool().padAfter(1),
  statistics: integer_statistic_array(k_game_results_damage_statistic_count),
});

export type s_game_results_damage_statistics = c.infer<
  typeof s_game_results_damage_statistics
>;

export const s_game_results_player_statistics = c.struct({
  statistics: integer_statistic_array(k_game_results_statistic_count),
  reward_statistics: integer_statistic_array(
    k_game_results_player_rewards_statistic_count
  ),
  medals: integer_statistic_array(k_game_results_medal_count),
  incidents: integer_statistic_array(k_game_results_incident_count).padAfter(2),
  achievements: c.u32().array(8),
  damage: s_game_results_damage_statistics.array(k_damage_reporting_type_count),
  griefing: c.f32().array(k_game_results_griefing_statistic_count),
});

export type s_game_results_player_statistics = c.infer<
  typeof s_game_results_player_statistics
>;

export const s_game_results_player_vs_player_statistic = c.struct({
  kills: integer_statistic(),
  deaths: integer_statistic(),
});

export type s_game_results_player_vs_player_statistic = c.infer<
  typeof s_game_results_player_vs_player_statistic
>;

export const s_game_results_player_vs_player_statistics = c.struct({
  opponents: s_game_results_player_vs_player_statistic.array(k_maximum_players),
});

export type s_game_results_player_vs_player_statistics = c.infer<
  typeof s_game_results_player_vs_player_statistics
>;

export const s_game_results_player_vs_ai_statistic = c.struct({
  kills: integer_statistic(),
  betrayals: integer_statistic(),
});

export type s_game_results_player_vs_ai_statistic = c.infer<
  typeof s_game_results_player_vs_ai_statistic
>;

export const s_game_results_player_vs_ai_bucket_type = c.struct({
  bucket_classes: s_game_results_player_vs_ai_statistic.array(
    k_campaign_metagame_bucket_class_count
  ),
});

export type s_game_results_player_vs_ai_bucket_type = c.infer<
  typeof s_game_results_player_vs_ai_bucket_type
>;

export const s_game_results_player_vs_ai_statistics = c.struct({
  bucket_types: s_game_results_player_vs_ai_bucket_type.array(
    k_campaign_metagame_bucket_type_count
  ),
});

export type s_game_results_player_vs_ai_statistics = c.infer<
  typeof s_game_results_player_vs_ai_statistics
>;

export const s_game_results_team_statistics = c.struct({
  statistics: integer_statistic_array(k_game_results_statistic_count),
});

export type s_game_results_team_statistics = c.infer<
  typeof s_game_results_team_statistics
>;

export const s_game_results_event = c.struct({
  event_type: c.i8(),
  player_index: c.u8(),
  victim_player_index_or_bucket_type: c.u8(),
  bucket_class: c.u8(),
  player_position: s_real_point3d,
  victim_position: s_real_point3d,
  damage_reporting_type: c.u8(),
  victim_weapon_damage_reporting_type: c.u8(),
  points: c.i16(),
  time: c.u32(),
});

export type s_game_results_event = c.infer<typeof s_game_results_event>;

export const s_game_results_medal_event = c.struct({
  event_type: c.i8(),
  player_index: c.u8().padAfter(2),
  medal: c.i32(),
  player_position: s_real_point3d,
  time: c.u32(),
});

export type s_game_results_medal_event = c.infer<
  typeof s_game_results_medal_event
>;

export const s_game_local_player = c.struct({
  in_use: c.bool(),
  is_unsigned_in_user: c.bool(),
  has_voice: c.bool(),
  signed_in_to_machine: c.bool(),
  signed_in_to_live: c.bool(),
  silver_or_gold_live: c.bool(),
  is_guest: c.bool().padAfter(1),
  player_identifier: c.u64(),
  player_profile: c_player_profile_interface,
});

export type s_game_local_player = c.infer<typeof s_game_local_player>;

const network_banhammer_cheat_flags = () =>
  c.Bitfield(
    [
      "modified_content",
      "webstats_upload_failures",
      "machine_file_upload_failures",
      "reward_sync_upload_failures",
      "arena_upload_failures",
      "title_checksum_failed",
    ] as const,
    c.u8()
  );

export const s_network_banhammer_cheating_report = c.struct({
  machine_cheat_flags: network_banhammer_cheat_flags().padAfter(3),
  controller_cheat_flags: network_banhammer_cheat_flags().padAfter(3),
  report_flag_0: c.bool(),
  report_flag_1: c.bool(),
  report_data: c.u8().array(256).padAfter(2),
  report_identifier: c.i32(),
});

export type s_network_banhammer_cheating_report = c.infer<
  typeof s_network_banhammer_cheating_report
>;
