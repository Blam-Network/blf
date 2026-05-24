import {
  type c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../bitstream";
import { BlfError } from "../../../../error";
import { c_game_engine_base_variant } from "./c_game_engine_default";
import { c_string_table } from "./c_string_table";
import { c_megalogamengine_map_permissions } from "./megalogamengine/c_megalogamengine_map_permissions";
import { s_game_engine_player_rating_parameters } from "./megalogamengine/s_game_engine_player_rating_parameters";
import { s_custom_game_engine_definition } from "./megalogamengine/s_custom_game_engine_definition";
import { c_game_engine_custom_variant_au1_settings } from "./c_game_engine_custom_variant_au1_settings";
import { s_player_trait_option } from "./c_game_engine_traits";
import { s_user_defined_option } from "./megalogamengine/s_user_defined_option";

export class c_game_engine_custom_variant {
  m_encoding_version = 0;
  m_build_number = 0;
  m_base_variant = new c_game_engine_base_variant();
  m_player_traits: s_player_trait_option[] = [];
  m_user_defined_options: s_user_defined_option[] = [];
  m_script_strings = new c_string_table(112, 0x4c00, 15, 15, 7);
  m_base_name_string_index = 0;
  m_localized_name = new c_string_table(1, 0x180, 9, 9, 1);
  m_localized_description = new c_string_table(1, 0xc00, 12, 12, 1);
  m_localized_category = new c_string_table(1, 0x180, 9, 9, 1);
  m_engine_icon = 0;
  m_engine_category = 0;
  m_map_permissions = new c_megalogamengine_map_permissions();
  m_player_ratings = new s_game_engine_player_rating_parameters();
  m_score_to_win_round = 0;
  m_fire_teams_enabled = false;
  m_symmetric_gametype = false;
  m_base_variant_parameters_locked: boolean[] = Array.from(
    { length: 1280 },
    () => false,
  );
  m_base_variant_parameters_hidden: boolean[] = Array.from(
    { length: 1280 },
    () => false,
  );
  m_user_defined_options_locked: boolean[] = Array.from({ length: 32 }, () => false);
  m_user_defined_options_hidden: boolean[] = Array.from({ length: 32 }, () => false);
  m_game_engine = new s_custom_game_engine_definition();
  m_au1_settings?: c_game_engine_custom_variant_au1_settings;

  decode(bitstream: c_bitstream_reader): void {
    this.m_encoding_version = bitstream.read_signed_integer(
      "encoding-version",
      32,
    );
    this.m_build_number = bitstream.read_signed_integer("version", 32);
    this.m_base_variant.decode(bitstream);

    const player_trait_count = bitstream.read_integer("player-trait-count", 5);
    for (let i = 0; i < player_trait_count; i++) {
      const traits = new s_player_trait_option();
      traits.decode(bitstream);
      this.m_player_traits.push(traits);
    }

    const user_defined_option_count = bitstream.read_integer(
      "user-defined-option-count",
      5,
    );
    for (let i = 0; i < user_defined_option_count; i++) {
      const option = new s_user_defined_option();
      option.decode(bitstream);
      this.m_user_defined_options.push(option);
    }

    this.m_script_strings.decode(bitstream);
    this.m_base_name_string_index = bitstream.read_integer(
      "base-name-string-index",
      7,
    );
    this.m_localized_name.decode(bitstream);
    this.m_localized_description.decode(bitstream);
    this.m_localized_category.decode(bitstream);
    this.m_engine_icon = bitstream.read_integer("engine-icon-index", 5);
    this.m_engine_category = bitstream.read_integer("engine-category", 5);
    this.m_map_permissions.decode(bitstream);
    this.m_player_ratings.decode(bitstream);
    this.m_score_to_win_round = bitstream.read_signed_integer(
      "score-to-win-round",
      16,
    );
    this.m_fire_teams_enabled = bitstream.read_bool("fire-teams-enabled");
    this.m_symmetric_gametype = bitstream.read_bool("symmetric-gametype");

    for (let i = 0; i < 1280; i++) {
      this.m_base_variant_parameters_locked[i] = bitstream.read_bool(
        "base-variant-parameters-locked",
      );
    }
    for (let i = 0; i < 1280; i++) {
      this.m_base_variant_parameters_hidden[i] = bitstream.read_bool(
        "base-variant-parameters-hidden",
      );
    }
    for (let i = 0; i < 32; i++) {
      this.m_user_defined_options_locked[i] = bitstream.read_bool(
        "user-defined-options-locked",
      );
    }
    for (let i = 0; i < 32; i++) {
      this.m_user_defined_options_hidden[i] = bitstream.read_bool(
        "user-defined-options-hidden",
      );
    }

    this.m_game_engine.decode(bitstream);

    if (this.m_encoding_version > 106) {
      const au1 = new c_game_engine_custom_variant_au1_settings();
      au1.decode(bitstream);
      this.m_au1_settings = au1;
    }
  }

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_signed_integer(this.m_encoding_version, 32);
    bitstream.write_signed_integer(this.m_build_number, 32);
    this.m_base_variant.encode(bitstream);
    bitstream.write_integer(this.m_player_traits.length, 5);
    for (const player_trait of this.m_player_traits) {
      player_trait.encode(bitstream);
    }
    bitstream.write_integer(this.m_user_defined_options.length, 5);
    for (const option of this.m_user_defined_options) {
      option.encode(bitstream);
    }
    this.m_script_strings.encode(bitstream);
    bitstream.write_integer(this.m_base_name_string_index, 7);
    this.m_localized_name.encode(bitstream);
    this.m_localized_description.encode(bitstream);
    this.m_localized_category.encode(bitstream);
    bitstream.write_integer(this.m_engine_icon, 5);
    bitstream.write_integer(this.m_engine_category, 5);
    this.m_map_permissions.encode(bitstream);
    this.m_player_ratings.encode(bitstream);
    bitstream.write_signed_integer(this.m_score_to_win_round, 16);
    bitstream.write_bool(this.m_fire_teams_enabled);
    bitstream.write_bool(this.m_symmetric_gametype);
    for (const parameter of this.m_base_variant_parameters_locked) {
      bitstream.write_bool(parameter);
    }
    for (const parameter of this.m_base_variant_parameters_hidden) {
      bitstream.write_bool(parameter);
    }
    for (const parameter of this.m_user_defined_options_locked) {
      bitstream.write_bool(parameter);
    }
    for (const parameter of this.m_user_defined_options_hidden) {
      bitstream.write_bool(parameter);
    }
    this.m_game_engine.encode(bitstream);
    if (this.m_encoding_version > 106) {
      if (!this.m_au1_settings) {
        throw new BlfError(
          "Writing v107 gametypes (and higher) requires AU1 Options to be set.",
        );
      }
      this.m_au1_settings.encode(bitstream);
    }
  }
}
