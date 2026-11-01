import { c } from "@craftycodie/cstruct";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

export const k_maximum_local_players = 4;

export enum e_controller_index {
  _controller0 = 0,
  _controller1 = 1,
  _controller2 = 2,
  _controller3 = 3,
}

export enum e_button_presets {
  _button_preset_standard = 0,
  _button_preset_south_paw = 1,
  _button_preset_boxer = 2,
  _button_preset_green_thumb = 3,
  _button_preset_professional = 4,
  _button_preset_walkie_talkie = 5,
  _button_preset_universal_recon = 6,
  _button_preset_universal_reach = 7,
  _button_preset_universal_heretic = 8,
  _button_preset_universal_hell_jumper = 9,
  _button_preset_universal_green_thumb = 10,
}

export enum e_joystick_presets {
  _joystick_preset_standard = 0,
  _joystick_preset_south_paw = 1,
  _joystick_preset_legacy = 2,
  _joystick_preset_legacy_south_paw = 3,
}

export enum e_look_sensitivity {
  _look_sensitivity0 = 0,
  _look_sensitivity1 = 1,
  _look_sensitivity2 = 2,
  _look_sensitivity3 = 3,
  _look_sensitivity4 = 4,
  _look_sensitivity5 = 5,
  _look_sensitivity6 = 6,
  _look_sensitivity7 = 7,
  _look_sensitivity8 = 8,
  _look_sensitivity9 = 9,
}

export enum e_camera_attached_horizontal {
  _camera_attached_horizontal_normal = 0,
  _camera_attached_horizontal_inverted = 1,
}

export enum e_camera_panning {
  _camera_panning_hold_to_pan = 0,
  _camera_panning_toggle = 1,
}

export enum e_camera_look_sensitivity {
  _camera_look_sensitivity_slow = 0,
  _camera_look_sensitivity_normal = 1,
  _camera_look_sensitivity_fast = 2,
  _camera_look_sensitivity_very_fast = 3,
}

export enum e_camera_flying_movement {
  _camera_flying_movement_slow = 0,
  _camera_flying_movement_normal = 1,
  _camera_flying_movement_fast = 2,
  _camera_flying_movement_faster = 3,
}

export enum e_camera_flying_thrust {
  _camera_flying_thrust_slow = 0,
  _camera_flying_thrust_normal = 1,
  _camera_flying_thrust_fast = 2,
}

export enum e_campaign_difficulty_level {
  _campaign_difficulty_level_easy = 0,
  _campaign_difficulty_level_normal = 1,
  _campaign_difficulty_level_hard = 2,
  _campaign_difficulty_level_impossible = 3,
}

export enum e_experience_milestone {
  _experience_milestone_none = 0,
  _experience_milestone_recruit = 1,
  _experience_milestone_training = 2,
  _experience_milestone_spartan = 3,
  _experience_milestone_officer = 4,
}

export enum e_player_color_index {
  _player_color_00 = 0,
  _player_color_01 = 1,
  _player_color_02 = 2,
  _player_color_03 = 3,
  _player_color_04 = 4,
  _player_color_05 = 5,
  _player_color_06 = 6,
  _player_color_07 = 7,
  _player_color_08 = 8,
  _player_color_09 = 9,
  _player_color_10 = 10,
  _player_color_11 = 11,
  _player_color_12 = 12,
  _player_color_13 = 13,
  _player_color_14 = 14,
  _player_color_15 = 15,
  _player_color_16 = 16,
  _player_color_17 = 17,
  _player_color_18 = 18,
  _player_color_19 = 19,
  _player_color_20 = 20,
  _player_color_21 = 21,
  _player_color_22 = 22,
  _player_color_23 = 23,
  _player_color_24 = 24,
  _player_color_25 = 25,
  _player_color_26 = 26,
  _player_color_27 = 27,
  _player_color_28 = 28,
  _player_color_29 = 29,
  _player_color_30 = 30,
  _player_color_31 = 31,
  _player_color_none = -1,
}

export enum e_player_model_choice {
  _player_model_choice_spartan = 0,
  _player_model_choice_elite = 1,
}

export enum e_voice_mute_setting {
  _voice_mute_setting_off = 0,
  _voice_mute_setting_on = 1,
}

export enum e_voice_output_setting {
  _voice_output_setting_proximity_speakers_team_headset = 0,
  _voice_output_setting_headset = 1,
}

export enum e_voice_matchmaking_setting {
  _voice_matchmaking_setting_everyone = 0,
  _voice_matchmaking_setting_team_only = 1,
}

export enum e_voice_mask {
  _voice_mask_none = 0,
  _voice_mask_anonymous = 1,
}

export enum e_guide_voice_through_speakers {
  _guide_voice_through_speakers_off = 0,
  _guide_voice_through_speakers_on = 1,
  _guide_voice_through_speakers_both = 2,
}

export enum e_language {
  _language_english = 0,
  _language_japanese = 1,
  _language_german = 2,
  _language_french = 3,
  _language_spanish = 4,
  _language_mexican_spanish = 5,
  _language_italian = 6,
  _language_korean = 7,
  _language_traditional_chinese = 8,
  _language_simplified_chinese = 9,
  _language_portuguese = 10,
  _language_polish = 11,
  _language_invalid = -1,
}

export enum e_subtitle_setting {
  _subtitle_setting_automatic = 0,
  _subtitle_setting_enabled = 1,
  _subtitle_setting_disabled = 2,
}

export const c_player_profile_interface_controls = c.struct({
  button_preset: c.enum(e_button_presets, c.i32()),
  joystick_preset: c.enum(e_joystick_presets, c.i32()),
  look_sensitivity: c.enum(e_look_sensitivity, c.i32()),
});
export type c_player_profile_interface_controls = c.infer<
  typeof c_player_profile_interface_controls
>;

export const c_player_profile_interface_camera = c.struct({
  attached_horizontal: c.enum(e_camera_attached_horizontal, c.i32()),
  panning: c.enum(e_camera_panning, c.i32()),
  look_sensitivity: c.enum(e_camera_look_sensitivity, c.i32()),
  flying_movement: c.enum(e_camera_flying_movement, c.i32()),
  flying_thrust: c.enum(e_camera_flying_thrust, c.i32()),
});
export type c_player_profile_interface_camera = c.infer<
  typeof c_player_profile_interface_camera
>;

export const c_player_profile_interface_campaign = c.struct({
  /** An `e_campaign_difficulty_level`, kept as a number (as in blf_lib). */
  current_difficulty: c.i32(),
  current_campaign_absolute_index: c.i16(),
  current_map_absolute_index: c.i16(),
  map_difficulties_completed: c.u8().array(2).array(32),
  last_campaign_played_time: c.Time64().array(2),
  awarded_primary_skull_bitvector: c.i32(),
  awarded_secondary_skull_bitvector: c.i32(),
  terminals_read_level1_bitvector: c.i16(),
  terminals_read_level2_bitvector: c.i16(),
  map_flags: c.u32(),
  insertion_point_flags: c.u32().array(4),
});
export type c_player_profile_interface_campaign = c.infer<
  typeof c_player_profile_interface_campaign
>;

export const s_hopper_best_rank = c.struct({
  hopper_id: c.i16(),
  best_rank_index: c.i16(),
  highest_skill: c.i32(),
});
export type s_hopper_best_rank = c.infer<typeof s_hopper_best_rank>;

export const c_player_profile_interface_multiplayer = c.struct({
  last_hopper_identifier: c.i16(),
  last_map: c.i32().padBefore(2),
  last_variant_type: c.i32(),
  last_variant_checksum: c.i32(),
  spartan_program_best_milestone: c.enum(e_experience_milestone, c.i32()),
  spartan_program_best_rank: s_hopper_best_rank.array(32),
});
export type c_player_profile_interface_multiplayer = c.infer<
  typeof c_player_profile_interface_multiplayer
>;

export const s_emblem_info = c.struct({
  foreground_emblem_index: c.u8(),
  background_emblem_index: c.u8(),
  emblem_info_flags: c.u8(),
  primary_color_index: c.u8(),
  secondary_color_index: c.u8(),
  background_color_index: c.u8().padAfter(2),
});
export type s_emblem_info = c.infer<typeof s_emblem_info>;

export const c_player_profile_interface_appearance = c.struct({
  primary_color: c.enum(e_player_color_index, c.i32()),
  secondary_color: c.enum(e_player_color_index, c.i32()),
  tertiary_color: c.enum(e_player_color_index, c.i32()),
  player_model_choice: c.enum(e_player_model_choice, c.i32()),
  emblem: s_emblem_info,
  model_area_selections: c.u8().array(4).array(2),
  last_known_good_service_tag: c.wstring(4),
  desired_service_tag: c.wstring(4),
  service_tag_was_randomly_generated: c.bool(),
  service_tag_failed_verification: c.bool().padAfter(2),
});
export type c_player_profile_interface_appearance = c.infer<
  typeof c_player_profile_interface_appearance
>;

export const c_player_profile_interface_voice = c.struct({
  mute_setting: c.enum(e_voice_mute_setting, c.i32()),
  output_setting: c.enum(e_voice_output_setting, c.i32()),
  matchmaking_setting: c.enum(e_voice_matchmaking_setting, c.i32()),
  voice_mask: c.enum(e_voice_mask, c.i32()),
  guide_voice_through_speakers: c.enum(e_guide_voice_through_speakers, c.i32()),
  guide_voice_muted: c.bool().padAfter(3),
});
export type c_player_profile_interface_voice = c.infer<
  typeof c_player_profile_interface_voice
>;

export const c_player_profile_interface_online = c.struct({
  last_shown_popup_message_index: c.i32().array(2),
});
export type c_player_profile_interface_online = c.infer<
  typeof c_player_profile_interface_online
>;

export const s_player_training_profile_data = c.struct({
  training_blob_bitvector_low: c.i32().array(2),
  training_blob_bitvector_high: c.i32().array(2),
  expansion: c.i32().array(4),
});
export type s_player_training_profile_data = c.infer<
  typeof s_player_training_profile_data
>;

export const c_string_verify_task = c.struct({
  _vftable: c.u32(),
  flags: c.u16().padBefore(4),
  state: c.u16().padBefore(6),
  m_file_name_ptr: c.u32().padBefore(6),
  m_line: c.u32().padBefore(4),
  m_language: c.enum(e_language, c.i32()).padBefore(4),
  m_strings_to_verify: c.wstring(0x100).array(4).padBefore(4),
  m_strings_to_verify_count: c.i32(),
  m_results: c.bool().array(4).padBefore(4),
});
export type c_string_verify_task = c.infer<typeof c_string_verify_task>;

export const c_player_profile_interface = c.struct({
  m_flags: c.i32(),
  m_settings_read_retry_count: c.i32(),
  m_next_settings_read_retry_time_milliseconds: c.u32(),
  m_controller_index: c.enum(e_controller_index, c.i32()),
  m_achievements_report_as_obtained: c.bool(),
  m_controls: c_player_profile_interface_controls.padBefore(3),
  m_camera: c_player_profile_interface_camera,
  m_campaign: c_player_profile_interface_campaign.padBefore(4),
  m_multiplayer: c_player_profile_interface_multiplayer,
  m_appearance: c_player_profile_interface_appearance,
  m_voice: c_player_profile_interface_voice,
  m_online: c_player_profile_interface_online,
  m_film_auto_save: c.bool().array(3),
  m_gamer_region: c.i32().padBefore(1),
  m_gamer_zone: c.i32(),
  m_training_data: s_player_training_profile_data,
  m_string_verify_task: c_string_verify_task,
  m_subtitle_setting: c.enum(e_subtitle_setting, c.i32()),
});
export type c_player_profile_interface = c.infer<
  typeof c_player_profile_interface
>;

/** 0xA98 bytes. */
export const s_blf_chunk_game_local_players_player = c.struct({
  in_use: c.bool(),
  is_unsigned_in_user: c.bool(),
  has_voice: c.bool(),
  is_signed_in_to_machine: c.bool(),
  is_signed_in_to_live: c.bool(),
  is_silver_or_gold_live: c.bool(),
  is_guest: c.bool(),
  identifier: c.u64().padBefore(1),
  profile: c_player_profile_interface.padAfter(4),
});
export type s_blf_chunk_game_local_players_player = c.infer<
  typeof s_blf_chunk_game_local_players_player
>;

/** Halo 3 local players and their profiles (`glcp` 1.1, 0x2A64 bytes). */
@blf.chunk("glcp", 1.1)
export class s_blf_chunk_game_local_players extends CStructBLFChunk(
  c.struct({
    players: s_blf_chunk_game_local_players_player
      .array(k_maximum_local_players)
      .padBefore(4),
  })
) {}
