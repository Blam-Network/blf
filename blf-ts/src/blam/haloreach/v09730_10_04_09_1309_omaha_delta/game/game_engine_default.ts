import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../bitstream";
import { AutoMap } from "../../../../helpers/automap";
import {
  c_game_engine_social_options,
  e_map_override_option_flags,
  e_team_scoring_method,
} from "../../v12065_11_08_24_1738_tu1actual/game/game_engine_default";
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
import { c_game_engine_team_options } from "./game_engine_team";
import {
  c_game_engine_miscellaneous_options,
  c_game_engine_respawn_options,
} from "./game_engine_traits";

export {
  c_game_engine_social_options,
  e_game_engine_social_options_flags,
  e_game_engine_team_options_designator_switch_type,
  e_map_override_option_flags,
  e_multiplayer_team_designator,
  e_player_model_choice,
  e_team_changing_type,
  e_team_scoring_method,
  k_game_variant_team_count,
} from "../../v12065_11_08_24_1738_tu1actual/game/game_engine_default";
export {
  c_game_engine_team_options,
  c_game_engine_team_options_team,
} from "./game_engine_team";

/** Matches blf_lib omaha_delta `c_game_engine_map_override_options`. */
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
    this.m_base_player_traits.clear();
    this.m_weapon_set_absolute_index = -2;
    this.m_vehicle_set_absolute_index = -2;
    this.m_red_powerup_traits = new c_player_traits();
    this.m_red_powerup_traits.clear();
    this.m_blue_powerup_traits = new c_player_traits();
    this.m_blue_powerup_traits.clear();
    this.m_yellow_powerup_traits = new c_player_traits();
    this.m_yellow_powerup_traits.clear();
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

/**
 * Matches blf_lib omaha_delta `c_game_engine_base_variant` (no loadout block).
 */
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
  initialize(): void {
    content_item_metadata_set_defaults(this.m_metadata);
    this.m_miscellaneous_options.initialize();
    this.m_respawn_options.initialize();
    this.m_social_options.initialize();
    this.m_map_override_options.initialize();
    this.m_team_options.initialize();
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
  }
}

export type c_game_engine_campaign_variant = c_game_engine_base_variant;
