import { c } from "@craftycodie/cstruct";

const player_profile_interface_flags = () =>
  c.Bitfield(
    [
      "controller_look_inverted",
      "controller_flight_stick_aircraft_controls",
      "controller_auto_center_look",
      "controller_crouch_lock_enabled",
      "female_voice_enabled",
      "controller_southpaw",
      "controller_clench_protection",
      "dirty",
      "update_from_storage_pending",
      "dirty_in_use",
      "async_io_busy",
      "title_specific_settings_previously_retrieved",
      "unsafe_to_save",
      "appearance_changed_recently",
      "settings_failed_to_save",
    ] as const,
    c.u16()
  );

const new_user_experience_flags = () =>
  c.Bitfield(
    [
      "new_user_armory",
      "initial_bonus_notified",
      "offline_grade_notice_shown",
      "campaign_completion_notice_shown",
    ] as const,
    c.u8()
  );

export const s_player_training_profile_data = c.struct({
  completion_count_low_bits: c.u32().array(2),
  completion_count_high_bits: c.u32().array(2),
  unused_completion_bits: c.u32().array(4),
});
export type s_player_training_profile_data = c.infer<
  typeof s_player_training_profile_data
>;

export const s_reward_cookie_totals = c.struct({
  cookies: c.i32(),
  cookie_award_count: c.i32(),
  commendation_progress: c.i16().array(128),
});
export type s_reward_cookie_totals = c.infer<typeof s_reward_cookie_totals>;

export const s_reward_cookie_totals_with_purchases = c.struct({
  totals: s_reward_cookie_totals,
  purchase_states: c.u8().array(256),
});
export type s_reward_cookie_totals_with_purchases = c.infer<
  typeof s_reward_cookie_totals_with_purchases
>;

export const s_profile_user_reward_state = c.struct({
  all_time: s_reward_cookie_totals_with_purchases,
  today_online: s_reward_cookie_totals_with_purchases,
  today_offline: s_reward_cookie_totals,
  day_index: c.i32(),
  cookie_checksum: c.u32(),
  owner_xuid: c.u64(),
  commendation_definition_checksum: c.u32(),
  purchase_definition_checksum: c.u32(),
  last_modification_time: c.u64(),
  security_hash: c.u64(),
  flags: c
    .Bitfield(["daily_cookie_caps_enabled", "secure_storage"] as const, c.u32())
    .padAfter(4),
});
export type s_profile_user_reward_state = c.infer<
  typeof s_profile_user_reward_state
>;

export const s_online_file_search_query = c.struct({
  file_type: c.i8(),
  game_engine_type: c.u8(),
  age: c.i8(),
  sort_criteria: c.i8(),
  map_id: c.i32(),
  use_author_filter: c.bool(),
  megalo_category: c.i8(),
  author: c.string(16, "latin1"),
  tag: c.string(24, "latin1").padAfter(2),
});
export type s_online_file_search_query = c.infer<
  typeof s_online_file_search_query
>;

export const s_player_profile_predefined_search = c.struct({
  share_id: c.u64(),
  timestamp: c.u64(),
});
export type s_player_profile_predefined_search = c.infer<
  typeof s_player_profile_predefined_search
>;

export const s_player_profile_saved_search = c.struct({
  timestamp: c.u64(),
  name: c.wstring(24),
  share_id: c.u64(),
  query: s_online_file_search_query.padAfter(4),
});
export type s_player_profile_saved_search = c.infer<
  typeof s_player_profile_saved_search
>;

export const s_lsp_leaderboard_profile_arena_hopper_data = c.struct({
  flags: c
    .Bitfield(
      ["day_rating_message_shown", "tier_message_shown"] as const,
      c.u8()
    )
    .padAfter(1),
  hopper_identifier: c.u16(),
});
export type s_lsp_leaderboard_profile_arena_hopper_data = c.infer<
  typeof s_lsp_leaderboard_profile_arena_hopper_data
>;

export const s_lsp_leaderboard_profile_arena_data = c.struct({
  flags: c.Bitfield(["arena_welcome_shown"] as const, c.u8()).padAfter(3),
  welcomed_season: c.i32(),
  hopper_count: c.i32(),
  hoppers: s_lsp_leaderboard_profile_arena_hopper_data.array(8),
});
export type s_lsp_leaderboard_profile_arena_data = c.infer<
  typeof s_lsp_leaderboard_profile_arena_data
>;

export const s_network_banhammer_quit_probation_data = c.struct({
  last_match_ended_abnormally: c.bool().padAfter(7),
  last_abnormal_match_end_time: c.u64(),
});
export type s_network_banhammer_quit_probation_data = c.infer<
  typeof s_network_banhammer_quit_probation_data
>;

export const s_network_social_preferences = c.struct({
  motivation: c.f32(),
  tone: c.f32(),
  chattiness: c.f32(),
  teamwork: c.f32(),
});
export type s_network_social_preferences = c.infer<
  typeof s_network_social_preferences
>;

export const c_player_profile_interface = c.struct({
  flags: player_profile_interface_flags(),
  initialized: c.bool().padAfter(1),
  storage_read_retry_count: c.i32(),
  storage_read_retry_time: c.u32(),
  controller_index: c.i32(),
  achievements_report_as_obtained: c.bool().padAfter(3),
  button_preset: c.i32(),
  custom_buttons: c.i32().array(10),
  joystick_preset: c.i32(),
  look_sensitivity: c.i32(),
  camera_attached_horizontal: c.i32(),
  camera_panning: c.i32(),
  camera_look_sensitivity: c.i32(),
  camera_flying_movement: c.i32(),
  camera_flying_thrust: c.i32().padAfter(4),
  campaign_difficulty: c.u8().padAfter(1),
  last_campaign_played_absolute_index: c.i16(),
  last_campaign_map_played_absolute_index: c.i16(),
  map_completed_at_difficulty_levels: c.u8().array(128).padAfter(2),
  last_campaign_played_time: c.u64().array(2),
  campaign_awarded_skull_bitvector: c.u32(),
  campaign_terminal_read_level1_bitvector: c.u16(),
  campaign_terminal_read_level2_bitvector: c.u16(),
  campaign_progress_maps: c.u32().array(2),
  campaign_progress_insertion_points: c.u32().array(24),
  primary_change_color: c.i32(),
  secondary_change_color: c.i32(),
  tertiary_change_color: c.i32(),
  player_model_choice: c.u8().padAfter(1),
  emblem_info: c.u16().array(4),
  model_customization_selections: c.i8().array(8),
  non_model_customization_selections: c.i8().array(4),
  service_tag_validated: c.wstring(5),
  desired_service_tag: c.wstring(5),
  service_tag_was_randomly_generated: c.bool(),
  service_tag_verification_failed: c.bool(),
  service_tag_verification_complete: c.bool().padAfter(3),
  voice_mute_setting: c.i32(),
  voice_output_setting: c.i32(),
  voice_matchmaking_setting: c.i32(),
  guide_voice_through_speakers: c.i32(),
  guide_voice_muted: c.bool().padAfter(3),
  film_auto_save: c.bool().array(3),
  new_user_experience_flags: new_user_experience_flags(),
  new_user_loyalty_bonus: c.i16(),
  generic_bonus: c.i16(),
  training_data: s_player_training_profile_data,
  service_tag_verify_task: c.u32().padAfter(4),
  reward_state: s_profile_user_reward_state,
  viewed_purchase_checksum: c.u32(),
  viewed_purchase_flags: c.u32().array(8),
  notified_purchase_flags: c.u32().array(8).padAfter(4),
  file_share_timestamp: c.u64(),
  recommendations_timestamp: c.u64(),
  predefined_search_count: c.i32().padAfter(4),
  predefined_searches: s_player_profile_predefined_search.array(10),
  saved_search_count: c.i32().padAfter(4),
  saved_searches: s_player_profile_saved_search.array(5),
  lsp_arena_data: s_lsp_leaderboard_profile_arena_data,
  last_global_nag_message_id: c.i32(),
  last_per_user_nag_message_id: c.i32(),
  last_global_nag_message_view_count: c.i32(),
  last_per_user_nag_message_view_count: c.i32().padAfter(4),
  quit_probation_data: s_network_banhammer_quit_probation_data,
  recently_advanced_grades: c.bool(),
  recently_advanced_sub_grades: c.bool().padAfter(2),
  last_viewed_daily_challenge_set_id: c.i32(),
  last_viewed_weekly_challenge_set_id: c.i32(),
  last_viewed_player_grade: c.i32(),
  last_viewed_player_sub_grade: c.i32(),
  waypoint_integration_flags: c.Bitfield(
    ["waypoint_stats_written"] as const,
    c.u32()
  ),
  social_preferences: s_network_social_preferences,
});
export type c_player_profile_interface = c.infer<
  typeof c_player_profile_interface
>;
