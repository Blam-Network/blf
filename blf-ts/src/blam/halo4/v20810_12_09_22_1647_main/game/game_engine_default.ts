import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../bitstream";
import { AutoMap } from "../../../../helpers/automap";
import { type StaticArray, staticArray } from "../../../../types/static_array";
import {
  content_item_metadata_decode,
  content_item_metadata_encode,
  content_item_metadata_set_defaults,
  s_content_item_display_metadata,
  s_content_item_general_metadata,
  s_content_item_history,
  s_content_item_metadata,
} from "../saved_games/saved_game_files";
import {
  c_player_traits,
  e_grenade_count_setting,
} from "./game_engine_player_traits";
import { c_string_table } from "./string_table";

export enum e_team_changing_type {
  disabled = 0,
  enabled = 1,
  balancing_only = 2,
}

export enum e_team_scoring_method {
  sum = 0,
  minimum = 1,
  maximum = 2,
}

export class e_map_override_option_flags {
  @AutoMap(() => Boolean)
  grenades_on_map = false;
  @AutoMap(() => Boolean)
  shortcuts_on_map = false;
  @AutoMap(() => Boolean)
  equipment_on_map = false;
  @AutoMap(() => Boolean)
  powerups_on_map = false;
  @AutoMap(() => Boolean)
  turrets_on_map = false;
  @AutoMap(() => Boolean)
  indestructible_vehicles = false;
  to_raw(): number {
    return (
      (this.grenades_on_map ? 1 : 0) |
      (this.shortcuts_on_map ? 1 << 1 : 0) |
      (this.equipment_on_map ? 1 << 2 : 0) |
      (this.powerups_on_map ? 1 << 3 : 0) |
      (this.turrets_on_map ? 1 << 4 : 0) |
      (this.indestructible_vehicles ? 1 << 5 : 0)
    );
  }
  static from_raw(raw: number): e_map_override_option_flags {
    const flags = new e_map_override_option_flags();
    flags.grenades_on_map = (raw & 1) !== 0;
    flags.shortcuts_on_map = (raw & (1 << 1)) !== 0;
    flags.equipment_on_map = (raw & (1 << 2)) !== 0;
    flags.powerups_on_map = (raw & (1 << 3)) !== 0;
    flags.turrets_on_map = (raw & (1 << 4)) !== 0;
    flags.indestructible_vehicles = (raw & (1 << 5)) !== 0;
    return flags;
  }
}

export class e_game_engine_social_options_flags {
  @AutoMap(() => Boolean)
  friendly_fire_enabled = false;
  @AutoMap(() => Boolean)
  betrayal_booting_enabled = false;
  @AutoMap(() => Boolean)
  enemy_voice_enabled = false;
  @AutoMap(() => Boolean)
  open_channel_voice_enabled = false;
  @AutoMap(() => Boolean)
  dead_player_voice_enabled = false;
  to_raw(): number {
    return (
      (this.friendly_fire_enabled ? 1 : 0) |
      (this.betrayal_booting_enabled ? 1 << 1 : 0) |
      (this.enemy_voice_enabled ? 1 << 2 : 0) |
      (this.open_channel_voice_enabled ? 1 << 3 : 0) |
      (this.dead_player_voice_enabled ? 1 << 4 : 0)
    );
  }
  static from_raw(raw: number): e_game_engine_social_options_flags {
    const flags = new e_game_engine_social_options_flags();
    flags.friendly_fire_enabled = (raw & 1) !== 0;
    flags.betrayal_booting_enabled = (raw & (1 << 1)) !== 0;
    flags.enemy_voice_enabled = (raw & (1 << 2)) !== 0;
    flags.open_channel_voice_enabled = (raw & (1 << 3)) !== 0;
    flags.dead_player_voice_enabled = (raw & (1 << 4)) !== 0;
    return flags;
  }
}

export class c_game_engine_miscellaneous_options {
  @AutoMap(() => Boolean)
  m_teams_enabled = false;
  @AutoMap(() => Boolean)
  m_round_reset_players = false;
  @AutoMap(() => Boolean)
  m_round_reset_map = false;
  @AutoMap(() => Boolean)
  m_perfection_enabled = false;
  @AutoMap(() => Number)
  m_round_time_limit_minutes = 0;
  @AutoMap(() => Number)
  m_round_limit = 0;
  @AutoMap(() => Number)
  m_early_victory_win_count = 0;
  @AutoMap(() => Boolean)
  m_killcam_enabled = false;
  @AutoMap(() => Boolean)
  m_medal_scoring_enabled = false;
  @AutoMap(() => Boolean)
  m_asymmetric_round_scoring = false;
  @AutoMap(() => Number)
  m_overshield_deplete_time = 0;
  @AutoMap(() => Boolean)
  m_mosh_enabled = false;
  @AutoMap(() => Boolean)
  m_drop_weapons_on_death = false;
  @AutoMap(() => Number)
  m_mosh_difficulty = 0;
  initialize(): void {
    this.m_teams_enabled = false;
    this.m_round_reset_players = false;
    this.m_round_reset_map = true;
    this.m_perfection_enabled = false;
    this.m_round_time_limit_minutes = 8;
    this.m_round_limit = 1;
    this.m_early_victory_win_count = 2;
    this.m_killcam_enabled = true;
    this.m_medal_scoring_enabled = true;
    this.m_asymmetric_round_scoring = false;
    this.m_overshield_deplete_time = 0;
    this.m_mosh_enabled = false;
    this.m_drop_weapons_on_death = true;
    this.m_mosh_difficulty = 0;
  }
  decode(bitstream: c_bitstream_reader): void {
    this.m_teams_enabled = bitstream.read_bool("miscellaneous-options-teams");
    this.m_round_reset_players = bitstream.read_bool(
      "miscellaneous-options-round-reset-players"
    );
    this.m_round_reset_map = bitstream.read_bool(
      "miscellaneous-options-round-reset-map"
    );
    this.m_perfection_enabled = bitstream.read_bool(
      "miscellaneous-options-perfection-enabled"
    );
    this.m_round_time_limit_minutes = bitstream.read_integer(
      "miscellaneous-options-round-time-limit-minutes",
      8
    );
    this.m_round_limit = bitstream.read_integer(
      "miscellaneous-options-round-limit",
      5
    );
    this.m_early_victory_win_count = bitstream.read_integer(
      "miscellaneous-options-early-victory-win-count",
      4
    );
    this.m_killcam_enabled = bitstream.read_bool(
      "miscellaneous-options-killcam-enabled"
    );
    this.m_medal_scoring_enabled = bitstream.read_bool(
      "miscellaneous-options-medal-scoring-enabled"
    );
    this.m_asymmetric_round_scoring = bitstream.read_bool(
      "miscellaneous-options-asymmetric-round-scoring"
    );
    this.m_overshield_deplete_time = bitstream.read_integer(
      "miscellaneous-options-overshield-deplete-time",
      8
    );
    this.m_mosh_enabled = bitstream.read_bool("miscellaneous-options-mosh");
    this.m_drop_weapons_on_death = bitstream.read_bool(
      "miscellaneous-options-drop-weapons-on-death"
    );
    this.m_mosh_difficulty = bitstream.read_integer(
      "miscellaneous-options-mosh-difficulty",
      2
    );
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_bool(this.m_teams_enabled);
    bitstream.write_bool(this.m_round_reset_players);
    bitstream.write_bool(this.m_round_reset_map);
    bitstream.write_bool(this.m_perfection_enabled);
    bitstream.write_integer(this.m_round_time_limit_minutes, 8);
    bitstream.write_integer(this.m_round_limit, 5);
    bitstream.write_integer(this.m_early_victory_win_count, 4);
    bitstream.write_bool(this.m_killcam_enabled);
    bitstream.write_bool(this.m_medal_scoring_enabled);
    bitstream.write_bool(this.m_asymmetric_round_scoring);
    bitstream.write_integer(this.m_overshield_deplete_time, 8);
    bitstream.write_bool(this.m_mosh_enabled);
    bitstream.write_bool(this.m_drop_weapons_on_death);
    bitstream.write_integer(this.m_mosh_difficulty, 2);
  }
}
export class c_game_engine_respawn_options {
  @AutoMap(() => Boolean)
  m_inherit_respawn_time = false;
  @AutoMap(() => Boolean)
  m_respawn_with_teammate = false;
  @AutoMap(() => Boolean)
  m_respawn_at_location = false;
  @AutoMap(() => Boolean)
  m_respawn_on_kills = false;
  @AutoMap(() => Boolean)
  m_early_respawn_allowed = false;
  @AutoMap(() => Number)
  m_lives_per_round = 0;
  @AutoMap(() => Number)
  m_team_lives_per_round = 0;
  @AutoMap(() => Number)
  m_min_respawn_time_seconds = 0;
  @AutoMap(() => Number)
  m_respawn_time_seconds = 0;
  @AutoMap(() => Number)
  m_suicide_penalty_seconds = 0;
  @AutoMap(() => Number)
  m_betrayal_penalty_seconds = 0;
  @AutoMap(() => Number)
  m_respawn_growth_seconds = 0;
  @AutoMap(() => Number)
  m_loadout_cam_time = 0;
  @AutoMap(() => Number)
  m_respawn_player_traits_duration_seconds = 0;
  @AutoMap(() => c_player_traits)
  m_respawn_player_traits = new c_player_traits();
  initialize(): void {
    this.m_inherit_respawn_time = false;
    this.m_respawn_with_teammate = false;
    this.m_respawn_at_location = false;
    this.m_respawn_on_kills = false;
    this.m_early_respawn_allowed = false;
    this.m_lives_per_round = 0;
    this.m_team_lives_per_round = 0;
    this.m_min_respawn_time_seconds = 0;
    this.m_respawn_time_seconds = 5;
    this.m_suicide_penalty_seconds = 5;
    this.m_betrayal_penalty_seconds = 5;
    this.m_respawn_growth_seconds = 0;
    this.m_loadout_cam_time = 10;
    this.m_respawn_player_traits_duration_seconds = 5;
    this.m_respawn_player_traits = new c_player_traits();
    this.m_respawn_player_traits.clear();
  }
  decode(bitstream: c_bitstream_reader): void {
    this.m_inherit_respawn_time = bitstream.read_bool(
      "respawn-options-inherit-respawn-time"
    );
    this.m_respawn_with_teammate = bitstream.read_bool(
      "respawn-options-respawn-with-teammate"
    );
    this.m_respawn_at_location = bitstream.read_bool(
      "respawn-options-respawn-at-location"
    );
    this.m_respawn_on_kills = bitstream.read_bool(
      "respawn-options-respawn-on-kills"
    );
    this.m_early_respawn_allowed = bitstream.read_bool(
      "respawn-options-early-respawn-allowed"
    );
    this.m_lives_per_round = bitstream.read_integer(
      "respawn-options-lives-per-round",
      6
    );
    this.m_team_lives_per_round = bitstream.read_integer(
      "respawn-options-team-lives-per-round",
      7
    );
    this.m_min_respawn_time_seconds = bitstream.read_integer(
      "respawn-options-min-respawn-time",
      8
    );
    this.m_respawn_time_seconds = bitstream.read_integer(
      "respawn-options-respawn-time",
      8
    );
    this.m_suicide_penalty_seconds = bitstream.read_integer(
      "respawn-options-suicide-time",
      8
    );
    this.m_betrayal_penalty_seconds = bitstream.read_integer(
      "respawn-options-betrayal-time",
      8
    );
    this.m_respawn_growth_seconds = bitstream.read_integer(
      "respawn-options-respawn-growth-time",
      4
    );
    this.m_loadout_cam_time = bitstream.read_integer(
      "respawn-options-initial-loadout-selection-time",
      4
    );
    this.m_respawn_player_traits_duration_seconds = bitstream.read_integer(
      "respawn-options-player-traits-duration",
      6
    );
    this.m_respawn_player_traits.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_bool(this.m_inherit_respawn_time);
    bitstream.write_bool(this.m_respawn_with_teammate);
    bitstream.write_bool(this.m_respawn_at_location);
    bitstream.write_bool(this.m_respawn_on_kills);
    bitstream.write_bool(this.m_early_respawn_allowed);
    bitstream.write_integer(this.m_lives_per_round, 6);
    bitstream.write_integer(this.m_team_lives_per_round, 7);
    bitstream.write_integer(this.m_min_respawn_time_seconds, 8);
    bitstream.write_integer(this.m_respawn_time_seconds, 8);
    bitstream.write_integer(this.m_suicide_penalty_seconds, 8);
    bitstream.write_integer(this.m_betrayal_penalty_seconds, 8);
    bitstream.write_integer(this.m_respawn_growth_seconds, 4);
    bitstream.write_integer(this.m_loadout_cam_time, 4);
    bitstream.write_integer(this.m_respawn_player_traits_duration_seconds, 6);
    this.m_respawn_player_traits.encode(bitstream);
  }
}
export class c_game_engine_social_options {
  @AutoMap(() => e_game_engine_social_options_flags)
  m_flags = new e_game_engine_social_options_flags();
  @AutoMap(() => e_team_changing_type)
  m_team_changing: e_team_changing_type = e_team_changing_type.disabled;
  @AutoMap(() => Boolean)
  m_observers_enabled = false;
  initialize(): void {
    this.m_flags = new e_game_engine_social_options_flags();
    this.m_team_changing = e_team_changing_type.enabled;
    this.m_observers_enabled = false;
    this.m_flags.friendly_fire_enabled = true;
    this.m_flags.betrayal_booting_enabled = true;
    this.m_flags.enemy_voice_enabled = true;
  }
  decode(bitstream: c_bitstream_reader): void {
    this.m_observers_enabled = bitstream.read_bool("observers-enabled");
    this.m_team_changing = bitstream.read_enum(
      "team-changing",
      2,
      e_team_changing_type
    );
    this.m_flags.friendly_fire_enabled = bitstream.read_bool(
      "social-flag-friendly-fire-enabled"
    );
    this.m_flags.betrayal_booting_enabled = bitstream.read_bool(
      "social-flag-betrayal-booting-enabled"
    );
    this.m_flags.enemy_voice_enabled = bitstream.read_bool(
      "social-flag-enemy-voice-enabled"
    );
    this.m_flags.open_channel_voice_enabled = bitstream.read_bool(
      "social-flag-open-channel-voice-enabled"
    );
    this.m_flags.dead_player_voice_enabled = bitstream.read_bool(
      "social-flag-dead-player-voice-enabled"
    );
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_bool(this.m_observers_enabled);
    bitstream.write_enum(this.m_team_changing, 2, e_team_changing_type);
    bitstream.write_bool(this.m_flags.friendly_fire_enabled);
    bitstream.write_bool(this.m_flags.betrayal_booting_enabled);
    bitstream.write_bool(this.m_flags.enemy_voice_enabled);
    bitstream.write_bool(this.m_flags.open_channel_voice_enabled);
    bitstream.write_bool(this.m_flags.dead_player_voice_enabled);
  }
}
export class PowerupEffectData {
  @AutoMap(() => c_player_traits)
  m_traits = new c_player_traits();
  @AutoMap(() => Number)
  m_duration_seconds = 0;
  decode(bitstream: c_bitstream_reader): void {
    this.m_traits.decode(bitstream);
    this.m_duration_seconds = bitstream.read_integer("effect-duration", 7);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_traits.encode(bitstream);
    bitstream.write_integer(this.m_duration_seconds, 7);
  }
}

export class PowerupData {
  @AutoMap(() => [PowerupEffectData])
  m_effects: [PowerupEffectData, PowerupEffectData] = [
    new PowerupEffectData(),
    new PowerupEffectData(),
  ];
  decode(bitstream: c_bitstream_reader): void {
    for (const effect of this.m_effects) {
      effect.decode(bitstream);
    }
  }
  encode(bitstream: c_bitstream_writer): void {
    for (const effect of this.m_effects) {
      effect.encode(bitstream);
    }
  }
}

export class c_game_engine_map_override_options {
  @AutoMap(() => e_map_override_option_flags)
  m_flags = new e_map_override_option_flags();
  @AutoMap(() => c_player_traits)
  m_base_player_traits = new c_player_traits();
  @AutoMap(() => Number)
  m_weapon_set_absolute_index = 0;
  @AutoMap(() => Number)
  m_vehicle_set_absolute_index = 0;
  @AutoMap(() => Number)
  m_equipment_set_absolute_index = 0;
  @AutoMap(() => [PowerupData])
  m_powerups: [PowerupData, PowerupData, PowerupData, PowerupData] = [
    new PowerupData(),
    new PowerupData(),
    new PowerupData(),
    new PowerupData(),
  ];
  initialize(): void {
    this.m_flags = new e_map_override_option_flags();
    this.m_base_player_traits = new c_player_traits();
    this.m_base_player_traits.clear();
    this.m_weapon_set_absolute_index = -2;
    this.m_vehicle_set_absolute_index = -2;
    this.m_equipment_set_absolute_index = -2;
    this.m_flags.grenades_on_map = true;
    this.m_flags.shortcuts_on_map = true;
    this.m_flags.equipment_on_map = true;
    this.m_flags.powerups_on_map = true;
    this.m_flags.turrets_on_map = true;
  }
  decode(bitstream: c_bitstream_reader): void {
    this.m_flags = e_map_override_option_flags.from_raw(
      bitstream.read_integer("flags", 6)
    );
    this.m_base_player_traits.decode(bitstream);
    this.m_weapon_set_absolute_index = bitstream.read_signed_integer(
      "map-override-weapon-set",
      8
    );
    this.m_vehicle_set_absolute_index = bitstream.read_signed_integer(
      "map-override-vehicle-set",
      8
    );
    this.m_equipment_set_absolute_index = bitstream.read_signed_integer(
      "map-override-equipment-set",
      8
    );
    for (const powerup of this.m_powerups) {
      powerup.decode(bitstream);
    }
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_flags.to_raw(), 6);
    this.m_base_player_traits.encode(bitstream);
    bitstream.write_signed_integer(this.m_weapon_set_absolute_index, 8);
    bitstream.write_signed_integer(this.m_vehicle_set_absolute_index, 8);
    bitstream.write_signed_integer(this.m_equipment_set_absolute_index, 8);
    for (const powerup of this.m_powerups) {
      powerup.encode(bitstream);
    }
  }
}
export const k_game_variant_team_count = 8 as const;
export const k_loadout_traits_per_palette = 5 as const;
export const k_loadout_palette_count = 6 as const;
/** Matches `e_multiplayer_team_designator` in blf_lib `game_engine_team.rs`. */
export enum e_multiplayer_team_designator {
  none = -1,
  defenders = 0,
  attackers = 1,
  third_party = 2,
  fourth_party = 3,
  fifth_party = 4,
  sixth_party = 5,
  seventh_party = 6,
  eighth_party = 7,
  neutral = 8,
}
/** Matches `e_player_model_choice` in blf_lib `game_engine_team.rs`. */
export enum e_player_model_choice {
  spartan = 0,
  elite = 1,
}
/** Team designator switch mode (`m_designator_switch_type`, 2 bits). */
export enum e_game_engine_team_options_designator_switch_type {
  none = 0,
  random = 1,
  rotate = 2,
}

/** `e_game_engine_team_options_model_override_type` (ManagedMegalo 0..4), 3 bits. */
export enum e_game_engine_team_options_model_override_type {
  player_preference = 0,
  all_spartans = 1,
  all_elites = 2,
  use_team_species = 3,
  by_designator = 4,
  unknown_5 = 5,
}

/** `e_player_color_index` (none=-1 .. 31), 6 bits on wire via enum index. */
export enum e_player_color_index {
  none = -1,
  color_0 = 0,
  color_1 = 1,
  color_2 = 2,
  color_3 = 3,
  color_4 = 4,
  color_5 = 5,
  color_6 = 6,
  color_7 = 7,
  color_8 = 8,
  color_9 = 9,
  color_10 = 10,
  color_11 = 11,
  color_12 = 12,
  color_13 = 13,
  color_14 = 14,
  color_15 = 15,
  color_16 = 16,
  color_17 = 17,
  color_18 = 18,
  color_19 = 19,
  color_20 = 20,
  color_21 = 21,
  color_22 = 22,
  color_23 = 23,
  color_24 = 24,
  color_25 = 25,
  color_26 = 26,
  color_27 = 27,
  color_28 = 28,
  color_29 = 29,
  color_30 = 30,
  color_31 = 31,
}

export class s_emblem_info {
  @AutoMap(() => Number)
  m_foreground_emblem_index = 0;
  @AutoMap(() => Number)
  m_background_emblem_index = 0;
  @AutoMap(() => Number)
  m_flags = 0;
  @AutoMap(() => Number)
  m_primary_color: e_player_color_index = e_player_color_index.none;
  @AutoMap(() => Number)
  m_secondary_color: e_player_color_index = e_player_color_index.none;
  @AutoMap(() => Number)
  m_background_color: e_player_color_index = e_player_color_index.none;
  decode(bitstream: c_bitstream_reader): void {
    this.m_foreground_emblem_index = bitstream.read_integer(
      "foreground-emblem-index",
      8
    );
    this.m_background_emblem_index = bitstream.read_integer(
      "background-emblem-index",
      8
    );
    this.m_flags = bitstream.read_integer("emblem-info-flags", 3);
    this.m_primary_color = bitstream.read_enum(
      "primary-color",
      6,
      e_player_color_index
    );
    this.m_secondary_color = bitstream.read_enum(
      "secondary-color",
      6,
      e_player_color_index
    );
    this.m_background_color = bitstream.read_enum(
      "background-color",
      6,
      e_player_color_index
    );
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_foreground_emblem_index, 8);
    bitstream.write_integer(this.m_background_emblem_index, 8);
    bitstream.write_integer(this.m_flags, 3);
    bitstream.write_enum(this.m_primary_color, 6, e_player_color_index);
    bitstream.write_enum(this.m_secondary_color, 6, e_player_color_index);
    bitstream.write_enum(this.m_background_color, 6, e_player_color_index);
  }
}

export class e_game_engine_team_options_team_flags {
  @AutoMap(() => Boolean)
  enabled = false;
  @AutoMap(() => Boolean)
  override_primary_color = false;
  @AutoMap(() => Boolean)
  override_secondary_color = false;
  @AutoMap(() => Boolean)
  override_ui_text_color = false;
  @AutoMap(() => Boolean)
  override_ui_bitmap_color = false;
  @AutoMap(() => Boolean)
  unknown_5 = false;
  to_raw(): number {
    return (
      (this.enabled ? 1 : 0) |
      (this.override_primary_color ? 1 << 1 : 0) |
      (this.override_secondary_color ? 1 << 2 : 0) |
      (this.override_ui_text_color ? 1 << 3 : 0) |
      (this.override_ui_bitmap_color ? 1 << 4 : 0) |
      (this.unknown_5 ? 1 << 5 : 0)
    );
  }
  static from_raw(raw: number): e_game_engine_team_options_team_flags {
    const flags = new e_game_engine_team_options_team_flags();
    flags.enabled = (raw & 1) !== 0;
    flags.override_primary_color = (raw & (1 << 1)) !== 0;
    flags.override_secondary_color = (raw & (1 << 2)) !== 0;
    flags.override_ui_text_color = (raw & (1 << 3)) !== 0;
    flags.override_ui_bitmap_color = (raw & (1 << 4)) !== 0;
    flags.unknown_5 = (raw & (1 << 5)) !== 0;
    return flags;
  }
}

export class c_game_engine_team_options_team {
  @AutoMap(() => e_game_engine_team_options_team_flags)
  m_flags = new e_game_engine_team_options_team_flags();
  @AutoMap(() => c_string_table)
  m_name = new c_string_table(1, 544, 10, 10, 1);
  @AutoMap(() => Number)
  m_team_initial_designator: e_multiplayer_team_designator =
    e_multiplayer_team_designator.none;
  @AutoMap(() => Number)
  m_model_override: e_player_model_choice = e_player_model_choice.spartan;
  @AutoMap(() => Number)
  m_primary_color_override = 0;
  @AutoMap(() => Number)
  m_secondary_color_override = 0;
  @AutoMap(() => Number)
  m_team_ui_text_tint_color_override = 0;
  @AutoMap(() => Number)
  m_team_ui_bitmap_tint_color_override = 0;
  @AutoMap(() => Number)
  m_fireteam_count = 0;
  @AutoMap(() => s_emblem_info)
  m_emblem = new s_emblem_info();
  initialize(team_index: number): void {
    this.m_flags = new e_game_engine_team_options_team_flags();
    this.m_flags.enabled = true;
    this.m_name = new c_string_table(1, 544, 10, 10, 1);
    this.m_team_initial_designator = team_index;
    this.m_model_override = e_player_model_choice.spartan;
    this.m_primary_color_override = 0xffffffff;
    this.m_secondary_color_override = 0xffffffff;
    this.m_team_ui_text_tint_color_override = 0xffffffff;
    this.m_team_ui_bitmap_tint_color_override = 0xffffffff;
    this.m_fireteam_count = 1;
    this.m_emblem = new s_emblem_info();
  }
  decode(bitstream: c_bitstream_reader): void {
    this.m_flags = e_game_engine_team_options_team_flags.from_raw(
      bitstream.read_integer("team-flags", 6)
    );
    this.m_name.decode(bitstream);
    this.m_team_initial_designator = bitstream.read_enum(
      "team-initial-designator",
      4,
      e_multiplayer_team_designator
    );
    this.m_model_override = bitstream.read_enum(
      "team-model-override",
      1,
      e_player_model_choice
    );
    this.m_primary_color_override = bitstream.read_integer(
      "primary-color-override",
      32
    );
    this.m_secondary_color_override = bitstream.read_integer(
      "secondary-color-override",
      32
    );
    this.m_team_ui_text_tint_color_override = bitstream.read_integer(
      "team-ui-text-tint-color-override",
      32
    );
    this.m_team_ui_bitmap_tint_color_override = bitstream.read_integer(
      "team-ui-bitmap-tint-color-override",
      32
    );
    this.m_fireteam_count = bitstream.read_integer("fireteam-count", 5);
    this.m_emblem.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_flags.to_raw(), 6);
    this.m_name.encode(bitstream);
    bitstream.write_enum(
      this.m_team_initial_designator,
      4,
      e_multiplayer_team_designator
    );
    bitstream.write_enum(this.m_model_override, 1, e_player_model_choice);
    bitstream.write_integer(this.m_primary_color_override, 32);
    bitstream.write_integer(this.m_secondary_color_override, 32);
    bitstream.write_integer(this.m_team_ui_text_tint_color_override, 32);
    bitstream.write_integer(this.m_team_ui_bitmap_tint_color_override, 32);
    bitstream.write_integer(this.m_fireteam_count, 5);
    this.m_emblem.encode(bitstream);
  }
}
export class c_game_engine_team_options {
  @AutoMap(() => Number)
  m_model_override: e_game_engine_team_options_model_override_type =
    e_game_engine_team_options_model_override_type.player_preference;
  @AutoMap(() => e_game_engine_team_options_designator_switch_type)
  m_designator_switch_type: e_game_engine_team_options_designator_switch_type =
    e_game_engine_team_options_designator_switch_type.none;
  @AutoMap(() => [c_game_engine_team_options_team])
  m_teams: StaticArray<
    c_game_engine_team_options_team,
    typeof k_game_variant_team_count
  > = staticArray(
    k_game_variant_team_count,
    () => new c_game_engine_team_options_team()
  );
  initialize(): void {
    this.m_model_override =
      e_game_engine_team_options_model_override_type.player_preference;
    this.m_designator_switch_type =
      e_game_engine_team_options_designator_switch_type.rotate;
    for (let i = 0; i < this.m_teams.length; i++) {
      this.m_teams[i]!.initialize(i);
    }
  }
  decode(bitstream: c_bitstream_reader): void {
    this.m_model_override = bitstream.read_enum(
      "model-override",
      3,
      e_game_engine_team_options_model_override_type
    );
    this.m_designator_switch_type = bitstream.read_enum(
      "designator-switch-type",
      2,
      e_game_engine_team_options_designator_switch_type
    );
    for (const team of this.m_teams) {
      team.decode(bitstream);
    }
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_enum(
      this.m_model_override,
      3,
      e_game_engine_team_options_model_override_type
    );
    bitstream.write_enum(
      this.m_designator_switch_type,
      2,
      e_game_engine_team_options_designator_switch_type
    );
    for (const team of this.m_teams) {
      team.encode(bitstream);
    }
  }
}
export class e_game_engine_loadout_definition_flags {
  @AutoMap(() => Boolean)
  spartan_loadouts_enabled = false;
  @AutoMap(() => Boolean)
  elite_loadouts_enabled = false;
  @AutoMap(() => Boolean)
  flag2 = false;
  @AutoMap(() => Boolean)
  flag3 = false;
  to_raw(): number {
    return (
      (this.spartan_loadouts_enabled ? 1 : 0) |
      (this.elite_loadouts_enabled ? 1 << 1 : 0) |
      (this.flag2 ? 1 << 2 : 0) |
      (this.flag3 ? 1 << 3 : 0)
    );
  }
  static from_raw(raw: number): e_game_engine_loadout_definition_flags {
    const flags = new e_game_engine_loadout_definition_flags();
    flags.spartan_loadouts_enabled = (raw & 1) !== 0;
    flags.elite_loadouts_enabled = (raw & (1 << 1)) !== 0;
    flags.flag2 = (raw & (1 << 2)) !== 0;
    flags.flag3 = (raw & (1 << 3)) !== 0;
    return flags;
  }
}
export class c_loadout_traits {
  @AutoMap(() => Boolean)
  m_visible = false;
  @AutoMap(() => Number)
  m_name = 0;
  @AutoMap(() => Number)
  m_initial_primary_weapon_absolute_index = 0;
  @AutoMap(() => Number)
  m_initial_secondary_weapon_absolute_index = 0;
  @AutoMap(() => Number)
  m_initial_equipment_absolute_index = 0;
  @AutoMap(() => Number)
  m_initial_tactical_package_absolute_index = 0;
  @AutoMap(() => Number)
  m_initial_support_upgrade_absolute_index = 0;
  @AutoMap(() => e_grenade_count_setting)
  m_initial_grenade_count_setting: e_grenade_count_setting =
    e_grenade_count_setting.unchanged;
  @AutoMap(() => Number)
  m_initial_primary_weapon_variant = 0;
  @AutoMap(() => Number)
  m_initial_secondary_weapon_variant = 0;
  initialize(): void {
    this.m_visible = false;
    this.m_name = -1;
    this.m_initial_primary_weapon_absolute_index = -3;
    this.m_initial_secondary_weapon_absolute_index = -3;
    this.m_initial_equipment_absolute_index = -3;
    this.m_initial_tactical_package_absolute_index = -3;
    this.m_initial_support_upgrade_absolute_index = -3;
    this.m_initial_grenade_count_setting = e_grenade_count_setting.unchanged;
    this.m_initial_primary_weapon_variant = 0;
    this.m_initial_secondary_weapon_variant = 0;
  }
  decode(bitstream: c_bitstream_reader): void {
    this.m_visible = bitstream.read_bool("flags");
    this.m_name = bitstream.read_index("name", 128, 7);
    this.m_initial_primary_weapon_absolute_index =
      bitstream.read_signed_integer("initial-primary-weapon", 8);
    this.m_initial_secondary_weapon_absolute_index =
      bitstream.read_signed_integer("initial-secondary-weapon", 8);
    this.m_initial_equipment_absolute_index = bitstream.read_signed_integer(
      "initial-equipment",
      8
    );
    this.m_initial_tactical_package_absolute_index =
      bitstream.read_signed_integer("initial-tactical-package", 8);
    this.m_initial_support_upgrade_absolute_index =
      bitstream.read_signed_integer("initial-support-upgrade", 8);
    this.m_initial_grenade_count_setting = bitstream.read_enum(
      "initial-grenade-count",
      5,
      e_grenade_count_setting
    );
    this.m_initial_primary_weapon_variant = bitstream.read_integer(
      "initial-primary-weapon-variant",
      3
    );
    this.m_initial_secondary_weapon_variant = bitstream.read_integer(
      "initial-secondary-weapon-variant",
      3
    );
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_bool(this.m_visible);
    bitstream.write_index(this.m_name, 128, 7);
    bitstream.write_signed_integer(
      this.m_initial_primary_weapon_absolute_index,
      8
    );
    bitstream.write_signed_integer(
      this.m_initial_secondary_weapon_absolute_index,
      8
    );
    bitstream.write_signed_integer(this.m_initial_equipment_absolute_index, 8);
    bitstream.write_signed_integer(
      this.m_initial_tactical_package_absolute_index,
      8
    );
    bitstream.write_signed_integer(
      this.m_initial_support_upgrade_absolute_index,
      8
    );
    bitstream.write_enum(
      this.m_initial_grenade_count_setting,
      5,
      e_grenade_count_setting
    );
    bitstream.write_integer(this.m_initial_primary_weapon_variant, 3);
    bitstream.write_integer(this.m_initial_secondary_weapon_variant, 3);
  }
}
export class c_loadout_palette_traits {
  @AutoMap(() => [c_loadout_traits])
  m_loadouts: StaticArray<
    c_loadout_traits,
    typeof k_loadout_traits_per_palette
  > = staticArray(k_loadout_traits_per_palette, () => new c_loadout_traits());
  initialize(): void {
    for (const loadout of this.m_loadouts) {
      loadout.initialize();
    }
  }
  decode(bitstream: c_bitstream_reader): void {
    for (const loadout of this.m_loadouts) {
      loadout.decode(bitstream);
    }
  }
  encode(bitstream: c_bitstream_writer): void {
    for (const loadout of this.m_loadouts) {
      loadout.encode(bitstream);
    }
  }
}
export class c_game_engine_loadout_traits {
  @AutoMap(() => e_game_engine_loadout_definition_flags)
  m_flags = new e_game_engine_loadout_definition_flags();
  @AutoMap(() => [c_loadout_palette_traits])
  m_loadout_palettes: StaticArray<
    c_loadout_palette_traits,
    typeof k_loadout_palette_count
  > = staticArray(
    k_loadout_palette_count,
    () => new c_loadout_palette_traits()
  );
  initialize(): void {
    this.m_flags = new e_game_engine_loadout_definition_flags();
    for (const palette of this.m_loadout_palettes) {
      palette.initialize();
    }
  }
  decode(bitstream: c_bitstream_reader): void {
    this.m_flags = e_game_engine_loadout_definition_flags.from_raw(
      bitstream.read_integer("loadout-flags", 4)
    );
    for (const palette of this.m_loadout_palettes) {
      palette.decode(bitstream);
    }
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_flags.to_raw(), 4);
    for (const palette of this.m_loadout_palettes) {
      palette.encode(bitstream);
    }
  }
}

/** Halo 4 `e_CustomGameMapSize` / `MapLoadoutsToken` (0..2, 2 bits). */
export enum e_custom_game_map_size {
  small = 0,
  medium = 1,
  large = 2,
}

/** Halo 4 `MapLoadoutInfo` (IDA @ 0x82d7b150). */
export class MapLoadoutInfo {
  @AutoMap(() => e_custom_game_map_size)
  m_size: e_custom_game_map_size = e_custom_game_map_size.small;
  @AutoMap(() => c_loadout_traits)
  m_loadout = new c_loadout_traits();
  decode(bitstream: c_bitstream_reader): void {
    this.m_size = bitstream.read_enum("size", 2, e_custom_game_map_size);
    this.m_loadout.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_enum(this.m_size, 2, e_custom_game_map_size);
    this.m_loadout.encode(bitstream);
  }
}

export class c_game_engine_prototype_options {
  @AutoMap(() => Number)
  m_mode = 0;
  @AutoMap(() => Number)
  m_promethean_energy_kill = 0;
  @AutoMap(() => Number)
  m_promethean_energy_time = 0;
  @AutoMap(() => Number)
  m_promethean_energy_medal = 0;
  @AutoMap(() => Number)
  m_promethean_duration = 0;
  @AutoMap(() => Boolean)
  m_class_color_override = false;
  decode(bitstream: c_bitstream_reader): void {
    this.m_mode = bitstream.read_integer("prototype-options-mode", 2);
    this.m_promethean_energy_kill = bitstream.read_integer(
      "prototype-options-promethean-energy-kill",
      3
    );
    this.m_promethean_energy_time = bitstream.read_integer(
      "prototype-options-promethean-energy-time",
      3
    );
    this.m_promethean_energy_medal = bitstream.read_integer(
      "prototype-options-promethean-energy-medal",
      3
    );
    this.m_promethean_duration = bitstream.read_integer(
      "prototype-options-promethean-duration",
      4
    );
    this.m_class_color_override = bitstream.read_bool(
      "prototype-options-class-color-override"
    );
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_mode, 2);
    bitstream.write_integer(this.m_promethean_energy_kill, 3);
    bitstream.write_integer(this.m_promethean_energy_time, 3);
    bitstream.write_integer(this.m_promethean_energy_medal, 3);
    bitstream.write_integer(this.m_promethean_duration, 4);
    bitstream.write_bool(this.m_class_color_override);
  }
}

/** IDA `c_enum<e_requisition_sub_menu,…,0,1>` → only value 0; encoding bits = 0. */
export enum e_requisition_sub_menu {
  unknown_0 = 0,
}

export class c_game_engine_requisition_palette_item {
  @AutoMap(() => Number)
  m_global_palette_index = 0;
  @AutoMap(() => Boolean)
  m_locked = false;
  @AutoMap(() => Number)
  m_designer_id = 0;
  @AutoMap(() => e_requisition_sub_menu)
  m_sub_menu: e_requisition_sub_menu = e_requisition_sub_menu.unknown_0;
  @AutoMap(() => Number)
  m_max_instances = 0;
  @AutoMap(() => Number)
  m_price = 0;
  @AutoMap(() => Number)
  m_model_variant_name = 0;
  @AutoMap(() => Number)
  m_starting_ammo = 0;
  @AutoMap(() => Number)
  m_warm_up = 0;
  @AutoMap(() => Number)
  m_purchase_frequency_player = 0;
  @AutoMap(() => Number)
  m_purchase_frequency_team = 0;
  @AutoMap(() => Number)
  m_price_increase_factor = 0;
  @AutoMap(() => Number)
  m_max_buy_player = 0;
  @AutoMap(() => Number)
  m_max_buy_team = 0;
  decode(bitstream: c_bitstream_reader): void {
    this.m_global_palette_index = bitstream.read_integer(
      "requisition-item-global-palette-index",
      6
    );
    this.m_locked = bitstream.read_bool("requisition-item-locked");
    this.m_designer_id = bitstream.read_integer(
      "requisition-item-designer-id",
      32
    );
    // Wire: 0 bits (required_encoding_bits_for_enum returns 0). Always value 0.
    this.m_sub_menu = e_requisition_sub_menu.unknown_0;
    this.m_max_instances = bitstream.read_integer(
      "requisition-item-max-instances",
      30
    );
    this.m_price = bitstream.read_float("requisition-item-price", 32);
    this.m_model_variant_name = bitstream.read_integer(
      "requisition-item-model-variant-name",
      30
    );
    this.m_starting_ammo = bitstream.read_integer(
      "requisition-item-starting-ammo",
      30
    );
    this.m_warm_up = bitstream.read_float("requisition-item-warm-up", 32);
    this.m_purchase_frequency_player = bitstream.read_float(
      "requisition-item-purchase-frequency-player",
      32
    );
    this.m_purchase_frequency_team = bitstream.read_float(
      "requisition-item-purchase-frequency-team",
      32
    );
    this.m_price_increase_factor = bitstream.read_float(
      "requisition-item-price-increase-factor",
      32
    );
    this.m_max_buy_player = bitstream.read_integer(
      "requisition-item-max-buy-player",
      8
    );
    this.m_max_buy_team = bitstream.read_integer(
      "requisition-item-max-buy-team",
      8
    );
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_global_palette_index, 6);
    bitstream.write_bool(this.m_locked);
    bitstream.write_integer(this.m_designer_id, 32);
    // Wire: 0 bits for e_requisition_sub_menu (count 1).
    bitstream.write_integer(this.m_max_instances, 30);
    bitstream.write_float(this.m_price, 32);
    bitstream.write_integer(this.m_model_variant_name, 30);
    bitstream.write_integer(this.m_starting_ammo, 30);
    bitstream.write_float(this.m_warm_up, 32);
    bitstream.write_float(this.m_purchase_frequency_player, 32);
    bitstream.write_float(this.m_purchase_frequency_team, 32);
    bitstream.write_float(this.m_price_increase_factor, 32);
    bitstream.write_integer(this.m_max_buy_player, 8);
    bitstream.write_integer(this.m_max_buy_team, 8);
  }
}

export class c_game_engine_requisition_options {
  @AutoMap(() => Number)
  m_player_frequency = 0;
  @AutoMap(() => Number)
  m_initial_currency = 0;
  @AutoMap(() => [c_game_engine_requisition_palette_item])
  m_items: c_game_engine_requisition_palette_item[] = [];
  decode(bitstream: c_bitstream_reader): void {
    this.m_player_frequency = bitstream.read_float(
      "requisition-options-player-frequency",
      32
    );
    this.m_initial_currency = bitstream.read_integer(
      "requisition-options-initial-game-currency",
      32
    );
    const count = bitstream.read_integer("requisition-item-count", 7);
    for (let i = 0; i < count; i++) {
      const item = new c_game_engine_requisition_palette_item();
      item.decode(bitstream);
      this.m_items.push(item);
    }
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_float(this.m_player_frequency, 32);
    bitstream.write_integer(this.m_initial_currency, 32);
    bitstream.write_integer(this.m_items.length, 7);
    for (const item of this.m_items) {
      item.encode(bitstream);
    }
  }
}

/** Halo 4 ordnance weight/cost: 30-bit quantized real, exact endpoints only. */
const k_ordnance_quantized_bits = 30;
const k_ordnance_quantized_max = 10000;

export class GameEngineOrdnanceSlotItem {
  @AutoMap(() => String)
  m_name = "";
  @AutoMap(() => Number)
  m_weight = 0;
  decode(bitstream: c_bitstream_reader): void {
    // Halo `c_bitstream::write_string` is raw 8-bit (not UTF-8); 0xFE appears in retail .mglo.
    this.m_name = bitstream.read_string_extended_ascii(32);
    this.m_weight = bitstream.read_quantized_real(
      0,
      k_ordnance_quantized_max,
      k_ordnance_quantized_bits,
      false,
      true
    );
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_string_extended_ascii(this.m_name, 32);
    bitstream.write_quantized_real(
      this.m_weight,
      0,
      k_ordnance_quantized_max,
      k_ordnance_quantized_bits,
      false,
      true
    );
  }
}

export class GameEngineOrdnanceOptions {
  @AutoMap(() => Boolean)
  m_initial_enabled = false;
  @AutoMap(() => Boolean)
  m_random_enabled = false;
  @AutoMap(() => Boolean)
  m_objective_enabled = false;
  @AutoMap(() => Boolean)
  m_player_enabled = false;
  @AutoMap(() => Boolean)
  m_custom_player_ordnance_enabled = false;
  @AutoMap(() => Boolean)
  m_non_player_drop_enabled = false;
  @AutoMap(() => Number)
  m_random_drop_count = 0;
  @AutoMap(() => Number)
  m_random_drop_delay_min = 0;
  @AutoMap(() => Number)
  m_random_drop_delay_max = 0;
  @AutoMap(() => Number)
  m_random_drop_fanfare_duration = 0;
  @AutoMap(() => String)
  m_initial_drop_name = "";
  @AutoMap(() => Number)
  m_initial_drop_delay = 0;
  @AutoMap(() => Number)
  m_initial_drop_fanfare_duration = 0;
  @AutoMap(() => String)
  m_normal_drop_name = "";
  @AutoMap(() => String)
  m_player_drop_name = "";
  @AutoMap(() => String)
  m_remapping_table_name = "";
  @AutoMap(() => [GameEngineOrdnanceSlotItem])
  m_custom_banks: GameEngineOrdnanceSlotItem[][] = Array.from(
    { length: 4 },
    () => Array.from({ length: 8 }, () => new GameEngineOrdnanceSlotItem())
  );
  @AutoMap(() => Number)
  m_cost = 0;
  @AutoMap(() => Number)
  m_cost_multiplier = 0;
  decode(bitstream: c_bitstream_reader): void {
    this.m_initial_enabled = bitstream.read_bool("ordnanceInitialEnabled");
    this.m_random_enabled = bitstream.read_bool("ordnanceRandomEnabled");
    this.m_objective_enabled = bitstream.read_bool("ordnanceObjectiveEnabled");
    this.m_player_enabled = bitstream.read_bool("ordnancePlayerEnabled");
    this.m_non_player_drop_enabled = bitstream.read_bool(
      "nonPlayerDropEnabled"
    );
    this.m_random_drop_count = bitstream.read_signed_integer(
      "randomDropCount",
      8
    );
    this.m_random_drop_delay_min = bitstream.read_signed_integer(
      "randomDropDelayMin",
      16
    );
    this.m_random_drop_delay_max = bitstream.read_signed_integer(
      "randomDropDelayMax",
      16
    );
    this.m_random_drop_fanfare_duration = bitstream.read_signed_integer(
      "randomDropFanfareDuration",
      16
    );
    this.m_initial_drop_name = bitstream.read_string_extended_ascii(32);
    this.m_initial_drop_delay = bitstream.read_signed_integer(
      "initialDropDelay",
      16
    );
    this.m_initial_drop_fanfare_duration = bitstream.read_signed_integer(
      "initialDropFanfareDuration",
      16
    );
    this.m_normal_drop_name = bitstream.read_string_extended_ascii(32);
    this.m_player_drop_name = bitstream.read_string_extended_ascii(32);
    this.m_remapping_table_name = bitstream.read_string_extended_ascii(32);
    this.m_custom_player_ordnance_enabled = bitstream.read_bool(
      "customPlayerOrdnanceEnabled"
    );
    for (let bank = 0; bank < 4; bank++) {
      for (let slot = 0; slot < 8; slot++) {
        this.m_custom_banks[bank]![slot]!.decode(bitstream);
      }
    }
    this.m_cost = bitstream.read_quantized_real(
      0,
      k_ordnance_quantized_max,
      k_ordnance_quantized_bits,
      false,
      true
    );
    this.m_cost_multiplier = bitstream.read_quantized_real(
      0,
      k_ordnance_quantized_max,
      k_ordnance_quantized_bits,
      false,
      true
    );
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_bool(this.m_initial_enabled);
    bitstream.write_bool(this.m_random_enabled);
    bitstream.write_bool(this.m_objective_enabled);
    bitstream.write_bool(this.m_player_enabled);
    bitstream.write_bool(this.m_non_player_drop_enabled);
    bitstream.write_signed_integer(this.m_random_drop_count, 8);
    bitstream.write_signed_integer(this.m_random_drop_delay_min, 16);
    bitstream.write_signed_integer(this.m_random_drop_delay_max, 16);
    bitstream.write_signed_integer(this.m_random_drop_fanfare_duration, 16);
    bitstream.write_string_extended_ascii(this.m_initial_drop_name, 32);
    bitstream.write_signed_integer(this.m_initial_drop_delay, 16);
    bitstream.write_signed_integer(this.m_initial_drop_fanfare_duration, 16);
    bitstream.write_string_extended_ascii(this.m_normal_drop_name, 32);
    bitstream.write_string_extended_ascii(this.m_player_drop_name, 32);
    bitstream.write_string_extended_ascii(this.m_remapping_table_name, 32);
    bitstream.write_bool(this.m_custom_player_ordnance_enabled);
    for (let bank = 0; bank < 4; bank++) {
      for (let slot = 0; slot < 8; slot++) {
        this.m_custom_banks[bank]![slot]!.encode(bitstream);
      }
    }
    bitstream.write_quantized_real(
      this.m_cost,
      0,
      k_ordnance_quantized_max,
      k_ordnance_quantized_bits,
      false,
      true
    );
    bitstream.write_quantized_real(
      this.m_cost_multiplier,
      0,
      k_ordnance_quantized_max,
      k_ordnance_quantized_bits,
      false,
      true
    );
  }
}

export class c_game_engine_base_variant {
  @AutoMap(() => s_content_item_metadata)
  m_metadata = (() => {
    const metadata = new s_content_item_metadata();
    metadata.general = new s_content_item_general_metadata();
    metadata.display = new s_content_item_display_metadata();
    metadata.creation_history = new s_content_item_history();
    metadata.modification_history = new s_content_item_history();
    return metadata;
  })();
  @AutoMap(() => Boolean)
  m_built_in = false;
  @AutoMap(() => Boolean)
  m_user_created = false;
  @AutoMap(() => c_game_engine_miscellaneous_options)
  m_miscellaneous_options = new c_game_engine_miscellaneous_options();
  @AutoMap(() => c_game_engine_prototype_options)
  m_prototype_options = new c_game_engine_prototype_options();
  @AutoMap(() => c_game_engine_respawn_options)
  m_respawn_options = new c_game_engine_respawn_options();
  @AutoMap(() => c_game_engine_social_options)
  m_social_options = new c_game_engine_social_options();
  @AutoMap(() => c_game_engine_map_override_options)
  m_map_override_options = new c_game_engine_map_override_options();
  @AutoMap(() => c_game_engine_requisition_options)
  m_requisition_options = new c_game_engine_requisition_options();
  @AutoMap(() => Number)
  m_infinity_mission_id = 0;
  @AutoMap(() => c_game_engine_team_options)
  m_team_options = new c_game_engine_team_options();
  @AutoMap(() => c_game_engine_loadout_traits)
  m_loadouts = new c_game_engine_loadout_traits();
  @AutoMap(() => GameEngineOrdnanceOptions)
  m_ordnance_options = new GameEngineOrdnanceOptions();
  initialize(): void {
    content_item_metadata_set_defaults(this.m_metadata);
    this.m_miscellaneous_options.initialize();
    this.m_respawn_options.initialize();
    this.m_social_options.initialize();
    this.m_map_override_options.initialize();
    this.m_team_options.initialize();
    this.m_loadouts.initialize();
    this.m_built_in = false;
    this.m_user_created = false;
    this.m_infinity_mission_id = 0;
  }
  decode(bitstream: c_bitstream_reader): void {
    content_item_metadata_decode(bitstream, this.m_metadata);
    this.m_built_in = bitstream.read_bool("variant-built-in");
    this.m_user_created = bitstream.read_bool("variant-user-created");
    this.m_miscellaneous_options.decode(bitstream);
    this.m_prototype_options.decode(bitstream);
    this.m_respawn_options.decode(bitstream);
    this.m_social_options.decode(bitstream);
    this.m_map_override_options.decode(bitstream);
    this.m_requisition_options.decode(bitstream);
    this.m_infinity_mission_id = bitstream.read_integer(
      "infinity-mission-id",
      32
    );
    this.m_team_options.decode(bitstream);
    this.m_loadouts.decode(bitstream);
    this.m_ordnance_options.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    content_item_metadata_encode(bitstream, this.m_metadata);
    bitstream.write_bool(this.m_built_in);
    bitstream.write_bool(this.m_user_created);
    this.m_miscellaneous_options.encode(bitstream);
    this.m_prototype_options.encode(bitstream);
    this.m_respawn_options.encode(bitstream);
    this.m_social_options.encode(bitstream);
    this.m_map_override_options.encode(bitstream);
    this.m_requisition_options.encode(bitstream);
    bitstream.write_integer(this.m_infinity_mission_id, 32);
    this.m_team_options.encode(bitstream);
    this.m_loadouts.encode(bitstream);
    this.m_ordnance_options.encode(bitstream);
  }
}
export type c_game_engine_campaign_variant = c_game_engine_base_variant;
