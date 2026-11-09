use crate::blf::chunks::BlfChunkHooks;
use binrw::{binrw, BinRead, BinWrite};
use crate::types::array::StaticArray;
use crate::{BlfChunk, TestSize};
use serde::{Deserialize, Serialize};
use crate::types::bool::Bool;
use serde_hex::{SerHex,StrictCap};
use crate::types::string::StaticWcharString;
use crate::types::time::time64_t;
use crate::types::u64::Unsigned64;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("glcp", 1.1)]
#[Size(0x2A64)]
#[brw(big)]
pub struct s_blf_chunk_game_local_players {
    #[brw(pad_before = 4)]
    pub players: StaticArray<s_blf_chunk_game_local_players_player, 4>,
}

impl BlfChunkHooks for s_blf_chunk_game_local_players {}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(repr = i32)]
#[repr(i32)]
pub enum e_controller_index {
    #[default]
    _controller0 = 0x0,
    _controller1 = 0x1,
    _controller2 = 0x2,
    _controller3 = 0x3,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(repr = i32)]
#[repr(i32)]
pub enum e_button_presets {
    #[default]
    _button_preset_standard = 0x0,
    _button_preset_south_paw = 0x1,
    _button_preset_boxer = 0x2,
    _button_preset_green_thumb = 0x3,
    _button_preset_professional = 0x4,
    _button_preset_walkie_talkie = 0x5,
    _button_preset_universal_recon = 0x6,
    _button_preset_universal_reach = 0x7,
    _button_preset_universal_heretic = 0x8,
    _button_preset_universal_hell_jumper = 0x9,
    _button_preset_universal_green_thumb = 0xA,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(repr = i32)]
#[repr(i32)]
pub enum e_joystick_presets {
    #[default]
    _joystick_preset_standard = 0x0,
    _joystick_preset_south_paw = 0x1,
    _joystick_preset_legacy = 0x2,
    _joystick_preset_legacy_south_paw = 0x3,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(repr = i32)]
#[repr(i32)]
pub enum e_look_sensitivity {
    #[default]
    _look_sensitivity0 = 0x0,
    _look_sensitivity1 = 0x1,
    _look_sensitivity2 = 0x2,
    _look_sensitivity3 = 0x3,
    _look_sensitivity4 = 0x4,
    _look_sensitivity5 = 0x5,
    _look_sensitivity6 = 0x6,
    _look_sensitivity7 = 0x7,
    _look_sensitivity8 = 0x8,
    _look_sensitivity9 = 0x9,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
pub struct c_player_profile_interface_controls {
    pub button_preset: e_button_presets,
    pub joystick_preset: e_joystick_presets,
    pub look_sensitivity: e_look_sensitivity,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(repr = i32)]
#[repr(i32)]
pub enum e_camera_attached_horizontal {
    #[default]
    _camera_attached_horizontal_normal = 0x0,
    _camera_attached_horizontal_inverted = 0x1,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(repr = i32)]
#[repr(i32)]
pub enum e_camera_panning {
    #[default]
    _camera_panning_hold_to_pan = 0x0,
    _camera_panning_toggle = 0x1,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(repr = i32)]
#[repr(i32)]
pub enum e_camera_look_sensitivity {
    #[default]
    _camera_look_sensitivity_slow = 0x0,
    _camera_look_sensitivity_normal = 0x1,
    _camera_look_sensitivity_fast = 0x2,
    _camera_look_sensitivity_very_fast = 0x3,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(repr = i32)]
#[repr(i32)]
pub enum e_camera_flying_movement {
    #[default]
    _camera_flying_movement_slow = 0x0,
    _camera_flying_movement_normal = 0x1,
    _camera_flying_movement_fast = 0x2,
    _camera_flying_movement_faster = 0x3,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(repr = i32)]
#[repr(i32)]
pub enum e_camera_flying_thrust {
    #[default]
    _camera_flying_thrust_slow = 0x0,
    _camera_flying_thrust_normal = 0x1,
    _camera_flying_thrust_fast = 0x2,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
pub struct c_player_profile_interface_camera {
    pub attached_horizontal: e_camera_attached_horizontal,
    pub panning: e_camera_panning,
    pub look_sensitivity: e_camera_look_sensitivity,
    pub flying_movement: e_camera_flying_movement,
    pub flying_thrust: e_camera_flying_thrust,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(repr = i32)]
#[repr(i32)]
pub enum e_campaign_difficulty_level {
    #[default]
    _campaign_difficulty_level_easy = 0x0,
    _campaign_difficulty_level_normal = 0x1,
    _campaign_difficulty_level_hard = 0x2,
    _campaign_difficulty_level_impossible = 0x3,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
pub struct c_player_profile_interface_campaign {
    pub current_difficulty: i32, //: e_campaign_difficulty_level,
    pub current_campaign_absolute_index: i16,
    pub current_map_absolute_index: i16,
    pub map_difficulties_completed: StaticArray<StaticArray<u8, 2>, 32>,
    pub last_campaign_played_time: StaticArray<time64_t, 2>, // definitely OK
    pub awarded_primary_skull_bitvector: i32,
    pub awarded_secondary_skull_bitvector: i32,
    pub terminals_read_level1_bitvector: i16,
    // #[brw(pad_before = 2)]
    pub terminals_read_level2_bitvector: i16,
    // #[brw(pad_before = 2)]
    pub map_flags: u32,
    pub insertion_point_flags: StaticArray<u32, 4>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(repr = i32)]
#[repr(i32)]
pub enum e_experience_milestone {
    #[default]
    _experience_milestone_none = 0x0,
    _experience_milestone_recruit = 0x1,
    _experience_milestone_training = 0x2,
    _experience_milestone_spartan = 0x3,
    _experience_milestone_officer = 0x4,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
pub struct s_hopper_best_rank {
    pub hopper_id: i16,
    pub best_rank_index: i16,
    pub highest_skill: i32,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
pub struct c_player_profile_interface_multiplayer {
    pub last_hopper_identifier: i16,
    #[brw(pad_before = 2)]
    pub last_map: i32,
    pub last_variant_type: i32,
    pub last_variant_checksum: i32,
    pub spartan_program_best_milestone: e_experience_milestone,
    pub spartan_program_best_rank: StaticArray<s_hopper_best_rank, 32>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(repr = i32)]
#[repr(i32)]
pub enum e_player_color_index {
    #[default]
    _player_color_00 = 0x0,
    _player_color_01 = 0x1,
    _player_color_02 = 0x2,
    _player_color_03 = 0x3,
    _player_color_04 = 0x4,
    _player_color_05 = 0x5,
    _player_color_06 = 0x6,
    _player_color_07 = 0x7,
    _player_color_08 = 0x8,
    _player_color_09 = 0x9,
    _player_color_10 = 0xA,
    _player_color_11 = 0xB,
    _player_color_12 = 0xC,
    _player_color_13 = 0xD,
    _player_color_14 = 0xE,
    _player_color_15 = 0xF,
    _player_color_16 = 0x10,
    _player_color_17 = 0x11,
    _player_color_18 = 0x12,
    _player_color_19 = 0x13,
    _player_color_20 = 0x14,
    _player_color_21 = 0x15,
    _player_color_22 = 0x16,
    _player_color_23 = 0x17,
    _player_color_24 = 0x18,
    _player_color_25 = 0x19,
    _player_color_26 = 0x1A,
    _player_color_27 = 0x1B,
    _player_color_28 = 0x1C,
    _player_color_29 = 0x1D,
    _player_color_30 = 0x1E,
    _player_color_31 = 0x1F,
    _player_color_none = -1,
}


#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(repr = i32)]
#[repr(i32)]
pub enum e_player_model_choice {
    #[default]
    _player_model_choice_spartan = 0x0,
    _player_model_choice_elite = 0x1,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
pub struct s_emblem_info {
    pub foreground_emblem_index: u8,
    pub background_emblem_index: u8,
    pub emblem_info_flags: u8,
    pub primary_color_index: u8,
    pub secondary_color_index: u8,
    #[brw(pad_after = 2)]
    pub background_color_index: u8,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
pub struct c_player_profile_interface_appearance {
    pub primary_color: e_player_color_index,
    pub secondary_color: e_player_color_index,
    pub tertiary_color: e_player_color_index,
    pub player_model_choice: e_player_model_choice,
    pub emblem: s_emblem_info,
    pub model_area_selections: StaticArray<StaticArray<u8, 4>, 2>,
    pub last_known_good_service_tag: StaticWcharString<4>,
    pub desired_service_tag: StaticWcharString<4>,
    pub service_tag_was_randomly_generated: Bool,
    #[brw(pad_after = 2)]
    pub service_tag_failed_verification: Bool,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(repr = i32)]
#[repr(i32)]
pub enum e_voice_mute_setting {
    #[default]
    _voice_mute_setting_off = 0x0,
    _voice_mute_setting_on = 0x1,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(repr = i32)]
#[repr(i32)]
pub enum e_voice_output_setting {
    #[default]
    _voice_output_setting_proximity_speakers_team_headset = 0x0,
    _voice_output_setting_headset = 0x1,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(repr = i32)]
#[repr(i32)]
pub enum e_voice_matchmaking_setting {
    #[default]
    _voice_matchmaking_setting_everyone = 0x0,
    _voice_matchmaking_setting_team_only = 0x1,
}


#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(repr = i32)]
#[repr(i32)]
pub enum e_voice_mask {
    #[default]
    _voice_mask_none = 0x0,
    _voice_mask_anonymous = 0x1,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(repr = i32)]
#[repr(i32)]
pub enum e_guide_voice_through_speakers {
    #[default]
    _guide_voice_through_speakers_off = 0x0,
    _guide_voice_through_speakers_on = 0x1,
    _guide_voice_through_speakers_both = 0x2,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
pub struct c_player_profile_interface_voice {
    pub mute_setting: e_voice_mute_setting,
    pub output_setting: e_voice_output_setting,
    pub matchmaking_setting: e_voice_matchmaking_setting,
    pub voice_mask: e_voice_mask,
    pub guide_voice_through_speakers: e_guide_voice_through_speakers,
    #[brw(pad_after = 3)]
    pub guide_voice_muted: Bool,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
pub struct c_player_profile_interface_online {
    pub last_shown_popup_message_index: StaticArray<i32, 2>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
pub struct s_player_training_profile_data {
    pub training_blob_bitvector_low: StaticArray<i32, 2>,
    pub training_blob_bitvector_high: StaticArray<i32, 2>,
    pub expansion: StaticArray<i32, 4>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(repr = i32)]
#[repr(i32)]
pub enum e_language {
    #[default]
    _language_english = 0x0,
    _language_japanese = 0x1,
    _language_german = 0x2,
    _language_french = 0x3,
    _language_spanish = 0x4,
    _language_mexican_spanish = 0x5,
    _language_italian = 0x6,
    _language_korean = 0x7,
    _language_traditional_chinese = 0x8,
    _language_simplified_chinese = 0x9,
    _language_portuguese = 0xA,
    _language_polish = 0xB,
    _language_invalid = -1,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
pub struct c_string_verify_task {
    // #[brw(pad_before=4)]
    #[serde(with = "SerHex::<StrictCap>")]
    pub _vftable: u32,
    #[brw(pad_before=4)]
    pub flags: u16,
    #[brw(pad_before=6)]
    pub state: u16,
    #[serde(with = "SerHex::<StrictCap>")]
    #[brw(pad_before=6)]
    pub m_file_name_ptr: u32,
    #[brw(pad_before=4)]
    pub m_line: u32,
    #[brw(pad_before=4)]
    pub m_language: e_language,
    #[brw(pad_before=4)]
    pub m_strings_to_verify: StaticArray<StaticWcharString<0x100>, 4>,
    pub m_strings_to_verify_count: i32,
    #[brw(pad_before=4)]
    pub m_results: StaticArray<Bool, 4>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(repr = i32)]
#[repr(i32)]
pub enum e_subtitle_setting {
    #[default]
    _subtitle_setting_automatic = 0x0,
    _subtitle_setting_enabled = 0x1,
    _subtitle_setting_disabled = 0x2,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
pub struct c_player_profile_interface {
    pub m_flags: i32,
    pub m_settings_read_retry_count: i32,
    pub m_next_settings_read_retry_time_milliseconds: u32,
    pub m_controller_index: e_controller_index,
    pub m_achievements_report_as_obtained: Bool,
    #[brw(pad_before = 3)]
    pub m_controls: c_player_profile_interface_controls,
    pub m_camera: c_player_profile_interface_camera,
    #[brw(pad_before = 4)]
    pub m_campaign: c_player_profile_interface_campaign,
    pub m_multiplayer: c_player_profile_interface_multiplayer,
    pub m_appearance: c_player_profile_interface_appearance,
    pub m_voice: c_player_profile_interface_voice,
    pub m_online: c_player_profile_interface_online,
    pub m_film_auto_save: StaticArray<Bool, 3>,
    #[brw(pad_before = 1)]
    pub m_gamer_region: i32,
    pub m_gamer_zone: i32,
    pub m_training_data: s_player_training_profile_data,
    pub m_string_verify_task: c_string_verify_task,
    pub m_subtitle_setting: e_subtitle_setting,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(0xA98)]
pub struct s_blf_chunk_game_local_players_player {
    pub in_use: Bool,
    pub is_unsigned_in_user: Bool,
    pub has_voice: Bool,
    pub is_signed_in_to_machine: Bool,
    pub is_signed_in_to_live: Bool,
    pub is_silver_or_gold_live: Bool,
    pub is_guest: Bool,
    #[serde(with = "SerHex::<StrictCap>")]
    #[brw(pad_before = 1)]
    pub identifier: Unsigned64,
    #[brw(pad_after = 4)]
    pub profile: c_player_profile_interface,
}
