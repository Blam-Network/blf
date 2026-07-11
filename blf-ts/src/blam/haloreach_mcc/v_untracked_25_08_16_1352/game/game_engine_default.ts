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
import { c_player_traits } from "./game_engine_player_traits";
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
  @AutoMap(() => Number)
  m_sudden_death_time = 1;
  @AutoMap(() => Number)
  m_grace_period = 0;
  initialize(): void {
    this.m_teams_enabled = false;
    this.m_round_reset_players = false;
    this.m_round_reset_map = true;
    this.m_perfection_enabled = false;
    this.m_round_time_limit_minutes = 8;
    this.m_round_limit = 1;
    this.m_early_victory_win_count = 2;
    this.m_sudden_death_time = 1;
    this.m_grace_period = 0;
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
    this.m_sudden_death_time = bitstream.read_integer(
      "sudden-death-time-limit",
      7
    );
    this.m_grace_period = bitstream.read_integer("grace-period", 5);
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_bool(this.m_teams_enabled);
    bitstream.write_bool(this.m_round_reset_players);
    bitstream.write_bool(this.m_round_reset_map);
    bitstream.write_bool(this.m_perfection_enabled);
    bitstream.write_integer(this.m_round_time_limit_minutes, 8);
    bitstream.write_integer(this.m_round_limit, 5);
    bitstream.write_integer(this.m_early_victory_win_count, 4);
    bitstream.write_integer(this.m_sudden_death_time, 7);
    bitstream.write_integer(this.m_grace_period, 5);
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
  @AutoMap(() => Number)
  m_lives_per_round = 0;
  @AutoMap(() => Number)
  m_team_lives_per_round = 0;
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
    this.m_lives_per_round = 0;
    this.m_team_lives_per_round = 0;
    this.m_respawn_time_seconds = 5;
    this.m_suicide_penalty_seconds = 5;
    this.m_betrayal_penalty_seconds = 5;
    this.m_respawn_growth_seconds = 0;
    this.m_loadout_cam_time = 10;
    this.m_respawn_player_traits_duration_seconds = 5;
    this.m_respawn_player_traits = new c_player_traits();
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
    this.m_lives_per_round = bitstream.read_integer(
      "respawn-options-lives-per-round",
      6
    );
    this.m_team_lives_per_round = bitstream.read_integer(
      "respawn-options-team-lives-per-round",
      7
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
    bitstream.write_integer(this.m_lives_per_round, 6);
    bitstream.write_integer(this.m_team_lives_per_round, 7);
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
export class c_game_engine_map_override_options {
  @AutoMap(() => e_map_override_option_flags)
  m_flags = new e_map_override_option_flags();
  @AutoMap(() => c_player_traits)
  m_base_player_traits = new c_player_traits();
  @AutoMap(() => Number)
  m_weapon_set_absolute_index = 0;
  @AutoMap(() => Number)
  m_vehicle_set_absolute_index = 0;
  @AutoMap(() => c_player_traits)
  m_red_powerup_traits = new c_player_traits();
  @AutoMap(() => c_player_traits)
  m_blue_powerup_traits = new c_player_traits();
  @AutoMap(() => c_player_traits)
  m_yellow_powerup_traits = new c_player_traits();
  @AutoMap(() => Number)
  m_red_powerup_duration_seconds = 0;
  @AutoMap(() => Number)
  m_blue_powerup_duration_seconds = 0;
  @AutoMap(() => Number)
  m_yellow_powerup_duration_seconds = 0;
  initialize(): void {
    this.m_flags = new e_map_override_option_flags();
    this.m_base_player_traits = new c_player_traits();
    this.m_weapon_set_absolute_index = -2;
    this.m_vehicle_set_absolute_index = -2;
    this.m_red_powerup_traits = new c_player_traits();
    this.m_blue_powerup_traits = new c_player_traits();
    this.m_yellow_powerup_traits = new c_player_traits();
    this.m_red_powerup_duration_seconds = 5;
    this.m_blue_powerup_duration_seconds = 30;
    this.m_yellow_powerup_duration_seconds = 30;
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
    this.m_red_powerup_traits.decode(bitstream);
    this.m_blue_powerup_traits.decode(bitstream);
    this.m_yellow_powerup_traits.decode(bitstream);
    this.m_red_powerup_duration_seconds = bitstream.read_integer(
      "map-override-red-powerup-duration",
      7
    );
    this.m_blue_powerup_duration_seconds = bitstream.read_integer(
      "map-override-blue-powerup-duration",
      7
    );
    this.m_yellow_powerup_duration_seconds = bitstream.read_integer(
      "map-override-yellow-powerup-duration",
      7
    );
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_flags.to_raw(), 6);
    this.m_base_player_traits.encode(bitstream);
    bitstream.write_signed_integer(this.m_weapon_set_absolute_index, 8);
    bitstream.write_signed_integer(this.m_vehicle_set_absolute_index, 8);
    this.m_red_powerup_traits.encode(bitstream);
    this.m_blue_powerup_traits.encode(bitstream);
    this.m_yellow_powerup_traits.encode(bitstream);
    bitstream.write_integer(this.m_red_powerup_duration_seconds, 7);
    bitstream.write_integer(this.m_blue_powerup_duration_seconds, 7);
    bitstream.write_integer(this.m_yellow_powerup_duration_seconds, 7);
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

export class c_game_engine_team_options_team {
  @AutoMap(() => Boolean)
  m_team_enabled = false;
  @AutoMap(() => Boolean)
  m_override_color_armour = false;
  @AutoMap(() => Boolean)
  m_override_color_ui_text = false;
  @AutoMap(() => Boolean)
  m_override_color_ui_bitmap = false;
  @AutoMap(() => c_string_table)
  m_name = new c_string_table(1, 32, 5, 6, 1);
  @AutoMap(() => Number)
  m_team_initial_designator: e_multiplayer_team_designator =
    e_multiplayer_team_designator.none;
  @AutoMap(() => Number)
  m_model_override: e_player_model_choice = e_player_model_choice.spartan;
  @AutoMap(() => Number)
  m_team_color_override = 0;
  @AutoMap(() => Number)
  m_team_ui_text_tint_color_override = 0;
  @AutoMap(() => Number)
  m_team_ui_bitmap_tint_color_override = 0;
  @AutoMap(() => Number)
  m_fireteam_count = 0;
  initialize(team_index: number): void {
    this.m_team_enabled = true;
    this.m_override_color_armour = false;
    this.m_override_color_ui_text = false;
    this.m_override_color_ui_bitmap = false;
    this.m_name = new c_string_table(1, 32, 5, 6, 1);
    this.m_team_initial_designator = team_index + 1;
    this.m_model_override = e_player_model_choice.spartan;
    this.m_team_color_override = 0xffffffff;
    this.m_team_ui_text_tint_color_override = 0xffffffff;
    this.m_team_ui_bitmap_tint_color_override = 0xffffffff;
    this.m_fireteam_count = 1;
  }
  decode(bitstream: c_bitstream_reader): void {
    this.m_override_color_ui_bitmap = bitstream.read_bool(
      "team-override-ui-bitmap-color"
    );
    this.m_override_color_ui_text = bitstream.read_bool(
      "team-override-ui-text-color"
    );
    this.m_override_color_armour = bitstream.read_bool(
      "team-override-armour-color"
    );
    this.m_team_enabled = bitstream.read_bool("team-enabled");
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
    this.m_team_color_override = bitstream.read_integer(
      "team-color-override",
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
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_bool(this.m_override_color_ui_bitmap);
    bitstream.write_bool(this.m_override_color_ui_text);
    bitstream.write_bool(this.m_override_color_armour);
    bitstream.write_bool(this.m_team_enabled);
    this.m_name.encode(bitstream);
    bitstream.write_enum(
      this.m_team_initial_designator,
      4,
      e_multiplayer_team_designator
    );
    bitstream.write_enum(this.m_model_override, 1, e_player_model_choice);
    bitstream.write_integer(this.m_team_color_override, 32);
    bitstream.write_integer(this.m_team_ui_text_tint_color_override, 32);
    bitstream.write_integer(this.m_team_ui_bitmap_tint_color_override, 32);
    bitstream.write_integer(this.m_fireteam_count, 5);
  }
}
export class c_game_engine_team_options {
  @AutoMap(() => Number)
  m_model_override = 0;
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
    this.m_model_override = 0;
    this.m_designator_switch_type =
      e_game_engine_team_options_designator_switch_type.rotate;
    for (let i = 0; i < this.m_teams.length; i++) {
      this.m_teams[i]!.initialize(i);
    }
  }
  decode(bitstream: c_bitstream_reader): void {
    this.m_model_override = bitstream.read_integer("model-override", 3);
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
    bitstream.write_integer(this.m_model_override, 3);
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
  to_raw(): number {
    return (
      (this.spartan_loadouts_enabled ? 1 : 0) |
      (this.elite_loadouts_enabled ? 1 << 1 : 0)
    );
  }
  static from_raw(raw: number): e_game_engine_loadout_definition_flags {
    const flags = new e_game_engine_loadout_definition_flags();
    flags.spartan_loadouts_enabled = (raw & 1) !== 0;
    flags.elite_loadouts_enabled = (raw & (1 << 1)) !== 0;
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
  m_initial_grenade_count_setting = 0;
  initialize(): void {
    this.m_visible = false;
    this.m_name = -1;
    this.m_initial_primary_weapon_absolute_index = -3;
    this.m_initial_secondary_weapon_absolute_index = -3;
    this.m_initial_equipment_absolute_index = -3;
    this.m_initial_grenade_count_setting = 0;
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
    this.m_initial_grenade_count_setting = bitstream.read_integer(
      "initial-grenade-count",
      4
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
    bitstream.write_integer(this.m_initial_grenade_count_setting, 4);
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
      bitstream.read_integer("loadout-flags", 2)
    );
    for (const palette of this.m_loadout_palettes) {
      palette.decode(bitstream);
    }
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_flags.to_raw(), 2);
    for (const palette of this.m_loadout_palettes) {
      palette.encode(bitstream);
    }
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
  @AutoMap(() => c_game_engine_miscellaneous_options)
  m_miscellaneous_options = new c_game_engine_miscellaneous_options();
  @AutoMap(() => c_game_engine_respawn_options)
  m_respawn_options = new c_game_engine_respawn_options();
  @AutoMap(() => c_game_engine_social_options)
  m_social_options = new c_game_engine_social_options();
  @AutoMap(() => c_game_engine_map_override_options)
  m_map_override_options = new c_game_engine_map_override_options();
  @AutoMap(() => e_team_scoring_method)
  m_team_scoring_method: e_team_scoring_method = e_team_scoring_method.sum;
  @AutoMap(() => c_game_engine_team_options)
  m_team_options = new c_game_engine_team_options();
  @AutoMap(() => c_game_engine_loadout_traits)
  m_loadouts = new c_game_engine_loadout_traits();
  initialize(): void {
    content_item_metadata_set_defaults(this.m_metadata);
    this.m_miscellaneous_options.initialize();
    this.m_respawn_options.initialize();
    this.m_social_options.initialize();
    this.m_map_override_options.initialize();
    this.m_team_options.initialize();
    this.m_loadouts.initialize();
    this.m_built_in = false;
    this.m_team_scoring_method = e_team_scoring_method.sum;
  }
  decode(bitstream: c_bitstream_reader): void {
    content_item_metadata_decode(bitstream, this.m_metadata);
    this.m_built_in = bitstream.read_bool("variant-built-in");
    this.m_miscellaneous_options.decode(bitstream);
    this.m_respawn_options.decode(bitstream);
    this.m_social_options.decode(bitstream);
    this.m_map_override_options.decode(bitstream);
    this.m_team_scoring_method = bitstream.read_enum(
      "team-scoring-method",
      3,
      e_team_scoring_method
    );
    this.m_team_options.decode(bitstream);
    this.m_loadouts.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    content_item_metadata_encode(bitstream, this.m_metadata);
    bitstream.write_bool(this.m_built_in);
    this.m_miscellaneous_options.encode(bitstream);
    this.m_respawn_options.encode(bitstream);
    this.m_social_options.encode(bitstream);
    this.m_map_override_options.encode(bitstream);
    bitstream.write_enum(this.m_team_scoring_method, 3, e_team_scoring_method);
    this.m_team_options.encode(bitstream);
    this.m_loadouts.encode(bitstream);
  }
}
export type c_game_engine_campaign_variant = c_game_engine_base_variant;
