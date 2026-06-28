import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../bitstream";
import {
  decodeBigBitfield,
  defaultBigBitfield,
  encodeBigBitfield,
} from "../../../../bitstream";
import { BlfError } from "../../../../error";
import { AutoMap } from "../../../../helpers/automap";
import { c_game_engine_base_variant } from "../../v12065_11_08_24_1738_tu1actual/game/game_engine_default";
import { c_game_engine_sandbox_variant } from "../../v12065_11_08_24_1738_tu1actual/game/game_engine_sandbox";
import { c_game_engine_survival_variant } from "../../v12065_11_08_24_1738_tu1actual/game/game_engine_survival";
import { s_player_trait_option } from "../../v12065_11_08_24_1738_tu1actual/game/game_engine_traits";
import {
  e_game_mode,
  e_game_variant_parameter,
  k_game_variant_parameter_flags,
  s_custom_game_engine_definition,
  type s_game_variant_parameter_flags,
} from "../../v12065_11_08_24_1738_tu1actual/game/game_variant";
import { s_game_engine_player_rating_parameters } from "../../v12065_11_08_24_1738_tu1actual/game/megalogamengine/game_engine_player_rating_parameters";
import { c_megalogamengine_map_permissions } from "../../v12065_11_08_24_1738_tu1actual/game/megalogamengine/megalogamengine_map_permissions";
import { s_user_defined_option } from "../../v12065_11_08_24_1738_tu1actual/game/megalogamengine/megalogamengine_user_defined_options";
import { c_string_table } from "../../v12065_11_08_24_1738_tu1actual/game/string_table";
import type { s_content_item_metadata } from "../../v12065_11_08_24_1738_tu1actual/saved_games/saved_game_files";

export {
  type s_game_variant_parameter_flags,
  e_game_mode,
  e_game_variant_parameter,
  k_game_variant_parameter_flags,
  s_custom_game_engine_definition,
};

export const k_game_engine_custom_variant_encoding_version = 106;

/** Release (pre-TU1) custom variant layout — TU1 v107 fields without TU1 settings. */
export class c_game_engine_custom_variant {
  @AutoMap(() => Number)
  m_encoding_version = 0;
  @AutoMap(() => Number)
  m_build_number = 0;
  @AutoMap(() => c_game_engine_base_variant)
  m_base_variant = new c_game_engine_base_variant();
  @AutoMap(() => [s_player_trait_option])
  m_player_traits: s_player_trait_option[] = [];
  @AutoMap(() => [s_user_defined_option])
  m_user_defined_options: s_user_defined_option[] = [];
  @AutoMap(() => c_string_table)
  m_script_strings = new c_string_table(112, 0x4c00, 15, 15, 7);
  @AutoMap(() => Number)
  m_base_name_string_index = 0;
  @AutoMap(() => c_string_table)
  m_localized_name = new c_string_table(1, 0x180, 9, 9, 1);
  @AutoMap(() => c_string_table)
  m_localized_description = new c_string_table(1, 0xc00, 12, 12, 1);
  @AutoMap(() => c_string_table)
  m_localized_category = new c_string_table(1, 0x180, 9, 9, 1);
  @AutoMap(() => Number)
  m_engine_icon = 0;
  @AutoMap(() => Number)
  m_engine_category = 0;
  @AutoMap(() => c_megalogamengine_map_permissions)
  m_map_permissions = new c_megalogamengine_map_permissions();
  @AutoMap(() => s_game_engine_player_rating_parameters)
  m_player_ratings = new s_game_engine_player_rating_parameters();
  @AutoMap(() => Number)
  m_score_to_win_round = 0;
  @AutoMap(() => Boolean)
  m_fire_teams_enabled = false;
  @AutoMap(() => Boolean)
  m_symmetric_gametype = false;
  @AutoMap(() => Object)
  m_base_variant_parameters_locked: s_game_variant_parameter_flags =
    defaultBigBitfield(k_game_variant_parameter_flags);
  @AutoMap(() => Object)
  m_base_variant_parameters_hidden: s_game_variant_parameter_flags =
    defaultBigBitfield(k_game_variant_parameter_flags);
  @AutoMap(() => [Boolean])
  m_user_defined_options_locked: boolean[] = Array.from(
    { length: 32 },
    () => false
  );
  @AutoMap(() => [Boolean])
  m_user_defined_options_hidden: boolean[] = Array.from(
    { length: 32 },
    () => false
  );
  @AutoMap(() => s_custom_game_engine_definition)
  m_game_engine = new s_custom_game_engine_definition();

  initialize(): void {
    Object.assign(this, new c_game_engine_custom_variant());
    this.m_encoding_version = k_game_engine_custom_variant_encoding_version;
    this.m_base_variant.initialize();
    this.m_base_variant.m_miscellaneous_options.m_round_reset_map = true;
    this.m_base_variant.m_miscellaneous_options.m_round_reset_players = true;
    this.m_player_ratings.initialize_to_default();
    this.m_map_permissions.initialize();
  }

  decode(bitstream: c_bitstream_reader): void {
    this.m_encoding_version = bitstream.read_signed_integer(
      "encoding-version",
      32
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
      5
    );
    for (let i = 0; i < user_defined_option_count; i++) {
      const option = new s_user_defined_option();
      option.decode(bitstream);
      this.m_user_defined_options.push(option);
    }
    this.m_script_strings.decode(bitstream);
    this.m_base_name_string_index = bitstream.read_integer(
      "base-name-string-index",
      7
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
      16
    );
    this.m_fire_teams_enabled = bitstream.read_bool("fire-teams-enabled");
    this.m_symmetric_gametype = bitstream.read_bool("symmetric-gametype");
    this.m_base_variant_parameters_locked = decodeBigBitfield(
      bitstream,
      "base-variant-parameters-locked",
      k_game_variant_parameter_flags
    );
    this.m_base_variant_parameters_hidden = decodeBigBitfield(
      bitstream,
      "base-variant-parameters-hidden",
      k_game_variant_parameter_flags
    );
    for (let i = 0; i < 32; i++) {
      this.m_user_defined_options_locked[i] = bitstream.read_bool(
        "user-defined-options-locked"
      );
    }
    for (let i = 0; i < 32; i++) {
      this.m_user_defined_options_hidden[i] = bitstream.read_bool(
        "user-defined-options-hidden"
      );
    }
    this.m_game_engine.decode(bitstream);
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
    encodeBigBitfield(
      bitstream,
      this.m_base_variant_parameters_locked,
      k_game_variant_parameter_flags,
      "base-variant-parameters-locked"
    );
    encodeBigBitfield(
      bitstream,
      this.m_base_variant_parameters_hidden,
      k_game_variant_parameter_flags,
      "base-variant-parameters-hidden"
    );
    for (const parameter of this.m_user_defined_options_locked) {
      bitstream.write_bool(parameter);
    }
    for (const parameter of this.m_user_defined_options_hidden) {
      bitstream.write_bool(parameter);
    }
    this.m_game_engine.encode(bitstream);
  }
}

export class c_game_variant {
  @AutoMap(() => e_game_mode)
  m_game_engine: e_game_mode = e_game_mode.megalogamengine;
  @AutoMap(() => c_game_engine_base_variant)
  m_campaign_variant?: c_game_engine_base_variant;
  @AutoMap(() => c_game_engine_custom_variant)
  m_custom_variant?: c_game_engine_custom_variant;
  @AutoMap(() => c_game_engine_survival_variant)
  m_survival_variant?: c_game_engine_survival_variant;
  @AutoMap(() => c_game_engine_sandbox_variant)
  m_sandbox_variant?: c_game_engine_sandbox_variant;

  decode(bitstream: c_bitstream_reader): void {
    this.m_game_engine = bitstream.read_enum("game-engine", 4, e_game_mode);
    switch (this.m_game_engine) {
      case e_game_mode.sandbox: {
        const sandbox = new c_game_engine_sandbox_variant();
        sandbox.decode(bitstream);
        this.m_sandbox_variant = sandbox;
        break;
      }
      case e_game_mode.megalogamengine: {
        const custom = new c_game_engine_custom_variant();
        custom.decode(bitstream);
        this.m_custom_variant = custom;
        break;
      }
      case e_game_mode.campaign: {
        const campaign = new c_game_engine_base_variant();
        campaign.decode(bitstream);
        this.m_campaign_variant = campaign;
        break;
      }
      case e_game_mode.survival: {
        const survival = new c_game_engine_survival_variant();
        survival.decode(bitstream);
        this.m_survival_variant = survival;
        break;
      }
      default:
        throw new BlfError(`Unrecognized game engine ${this.m_game_engine}`);
    }
  }

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_enum(this.m_game_engine, 4, e_game_mode);
    switch (this.m_game_engine) {
      case e_game_mode.sandbox:
        if (!this.m_sandbox_variant) {
          throw new BlfError("m_sandbox_variant does not exist");
        }
        this.m_sandbox_variant.encode(bitstream);
        break;
      case e_game_mode.megalogamengine:
        if (!this.m_custom_variant) {
          throw new BlfError("m_custom_variant does not exist");
        }
        this.m_custom_variant.encode(bitstream);
        break;
      case e_game_mode.campaign:
        if (!this.m_campaign_variant) {
          throw new BlfError("m_campaign_variant does not exist");
        }
        this.m_campaign_variant.encode(bitstream);
        break;
      case e_game_mode.survival:
        if (!this.m_survival_variant) {
          throw new BlfError("m_survival_variant does not exist");
        }
        this.m_survival_variant.encode(bitstream);
        break;
      default:
        throw new BlfError(`Unrecognized game engine ${this.m_game_engine}`);
    }
  }

  get_metadata(): s_content_item_metadata {
    switch (this.m_game_engine) {
      case e_game_mode.sandbox:
        if (!this.m_sandbox_variant) {
          throw new BlfError("m_sandbox_variant does not exist");
        }
        return this.m_sandbox_variant.m_custom_variant.m_base_variant
          .m_metadata;
      case e_game_mode.megalogamengine:
        if (!this.m_custom_variant) {
          throw new BlfError("m_custom_variant does not exist");
        }
        return this.m_custom_variant.m_base_variant.m_metadata;
      case e_game_mode.campaign:
        if (!this.m_campaign_variant) {
          throw new BlfError("m_campaign_variant does not exist");
        }
        return this.m_campaign_variant.m_metadata;
      case e_game_mode.survival:
        if (!this.m_survival_variant) {
          throw new BlfError("m_survival_variant does not exist");
        }
        return this.m_survival_variant.m_base_variant.m_metadata;
      default:
        throw new BlfError(`Unrecognized game engine ${this.m_game_engine}`);
    }
  }
}
