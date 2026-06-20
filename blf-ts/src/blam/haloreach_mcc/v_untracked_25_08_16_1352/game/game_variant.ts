import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../bitstream";
import { BlfError } from "../../../../error";
import { AutoMap } from "../../../../helpers/automap";
import type { s_content_item_metadata } from "../saved_games/saved_game_files";
import { c_game_engine_base_variant } from "./game_engine_default";
import { c_game_engine_sandbox_variant } from "./game_engine_sandbox";
import { c_game_engine_survival_variant } from "./game_engine_survival";
import { s_player_trait_option } from "./game_engine_traits";
import { s_game_engine_player_rating_parameters } from "./megalogamengine/game_engine_player_rating_parameters";
import { c_action } from "./megalogamengine/megalogamengine_actions";
import { c_condition } from "./megalogamengine/megalogamengine_conditions";
import { c_object_filter } from "./megalogamengine/megalogamengine_map_objects";
import { c_megalogamengine_map_permissions } from "./megalogamengine/megalogamengine_map_permissions";
import { c_megalo_game_statistic } from "./megalogamengine/megalogamengine_statistics";
import { c_trigger } from "./megalogamengine/megalogamengine_trigger";
import { s_user_defined_option } from "./megalogamengine/megalogamengine_user_defined_options";
import {
  s_variable_metadata,
  s_variable_metadata_global,
  s_variable_metadata_object,
  s_variable_metadata_player,
  s_variable_metadata_team,
} from "./megalogamengine/megalogamengine_variable_metadata";
import { c_string_table } from "./string_table";

export class s_custom_game_engine_definition {
  @AutoMap(() => [c_condition])
  m_conditions: c_condition[] = [];
  @AutoMap(() => [c_action])
  m_actions: c_action[] = [];
  @AutoMap(() => [c_trigger])
  m_triggers: c_trigger[] = [];
  @AutoMap(() => [c_megalo_game_statistic])
  m_statistics: c_megalo_game_statistic[] = [];
  @AutoMap(() => s_variable_metadata)
  m_global_variable_metadata = s_variable_metadata_global();
  @AutoMap(() => s_variable_metadata)
  m_player_variable_metadata = s_variable_metadata_player();
  @AutoMap(() => s_variable_metadata)
  m_object_variable_metadata = s_variable_metadata_object();
  @AutoMap(() => s_variable_metadata)
  m_team_variable_metadata = s_variable_metadata_team();
  @AutoMap(() => [Number])
  m_hud_widgets: number[] = [];
  @AutoMap(() => Number)
  m_initialization_trigger_index = 0;
  @AutoMap(() => Number)
  m_local_initialization_trigger_index = 0;
  @AutoMap(() => Number)
  m_host_migration_trigger_index = 0;
  @AutoMap(() => Number)
  m_double_migration_trigger_index = 0;
  @AutoMap(() => Number)
  m_object_death_event_trigger_index = 0;
  @AutoMap(() => Number)
  m_local_trigger_index = 0;
  @AutoMap(() => Number)
  m_pregame_trigger_index = 0;
  @AutoMap(() => [Boolean])
  m_objects_used: boolean[] = Array.from({ length: 2048 }, () => false);
  @AutoMap(() => [c_object_filter])
  m_object_filters: c_object_filter[] = [];
  decode(bitstream: c_bitstream_reader): void {
    const condition_count = bitstream.read_integer("condition-count", 10);
    for (let i = 0; i < condition_count; i++) {
      const condition = new c_condition();
      condition.decode(bitstream);
      this.m_conditions.push(condition);
    }
    const action_count = bitstream.read_integer("action-count", 11);
    for (let i = 0; i < action_count; i++) {
      const action = new c_action();
      action.decode(bitstream);
      this.m_actions.push(action);
    }
    const trigger_count = bitstream.read_integer("trigger-count", 9);
    for (let i = 0; i < trigger_count; i++) {
      const trigger = new c_trigger();
      trigger.decode(bitstream);
      this.m_triggers.push(trigger);
    }
    const statistic_count = bitstream.read_integer("game-statistic-count", 3);
    for (let i = 0; i < statistic_count; i++) {
      const statistic = new c_megalo_game_statistic();
      statistic.decode(bitstream);
      this.m_statistics.push(statistic);
    }
    this.m_global_variable_metadata.decode(bitstream);
    this.m_player_variable_metadata.decode(bitstream);
    this.m_object_variable_metadata.decode(bitstream);
    this.m_team_variable_metadata.decode(bitstream);
    const widget_count = bitstream.read_integer("hud-widget-count", 3);
    for (let i = 0; i < widget_count; i++) {
      this.m_hud_widgets.push(bitstream.read_integer("position", 4));
    }
    this.m_initialization_trigger_index = bitstream.read_integer(
      "initial-trigger-index",
      9
    );
    this.m_local_initialization_trigger_index = bitstream.read_integer(
      "local-initialization-trigger-index",
      9
    );
    this.m_host_migration_trigger_index = bitstream.read_integer(
      "host-migration-trigger-index",
      9
    );
    this.m_double_migration_trigger_index = bitstream.read_integer(
      "double-migration-trigger-index",
      9
    );
    this.m_object_death_event_trigger_index = bitstream.read_integer(
      "death-event-trigger-index",
      9
    );
    this.m_local_trigger_index = bitstream.read_integer(
      "local-trigger-index",
      9
    );
    this.m_pregame_trigger_index = bitstream.read_integer(
      "pregame-trigger-index",
      9
    );
    for (let i = 0; i < 2048; i++) {
      this.m_objects_used[i] = bitstream.read_bool("object-types-used");
    }
    const object_filter_count = bitstream.read_integer(
      "object-filter-count",
      5
    );
    for (let i = 0; i < object_filter_count; i++) {
      const filter = new c_object_filter();
      filter.decode(bitstream);
      this.m_object_filters.push(filter);
    }
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_conditions.length, 10);
    for (const condition of this.m_conditions) {
      condition.encode(bitstream);
    }
    bitstream.write_integer(this.m_actions.length, 11);
    for (const action of this.m_actions) {
      action.encode(bitstream);
    }
    bitstream.write_integer(this.m_triggers.length, 9);
    for (const trigger of this.m_triggers) {
      trigger.encode(bitstream);
    }
    bitstream.write_integer(this.m_statistics.length, 3);
    for (const statistic of this.m_statistics) {
      statistic.encode(bitstream);
    }
    this.m_global_variable_metadata.encode(bitstream);
    this.m_player_variable_metadata.encode(bitstream);
    this.m_object_variable_metadata.encode(bitstream);
    this.m_team_variable_metadata.encode(bitstream);
    bitstream.write_integer(this.m_hud_widgets.length, 3);
    for (const widget of this.m_hud_widgets) {
      bitstream.write_integer(widget, 4);
    }
    bitstream.write_integer(this.m_initialization_trigger_index, 9);
    bitstream.write_integer(this.m_local_initialization_trigger_index, 9);
    bitstream.write_integer(this.m_host_migration_trigger_index, 9);
    bitstream.write_integer(this.m_double_migration_trigger_index, 9);
    bitstream.write_integer(this.m_object_death_event_trigger_index, 9);
    bitstream.write_integer(this.m_local_trigger_index, 9);
    bitstream.write_integer(this.m_pregame_trigger_index, 9);
    for (const used of this.m_objects_used) {
      bitstream.write_bool(used);
    }
    bitstream.write_integer(this.m_object_filters.length, 5);
    for (const filter of this.m_object_filters) {
      filter.encode(bitstream);
    }
  }
}

export class c_game_engine_custom_variant_au1_settings {
  @AutoMap(() => Number)
  m_flags = 0;
  @AutoMap(() => Number)
  m_precision_bloom = 0;
  @AutoMap(() => Number)
  m_active_camo_energy_curve_min = 0;
  @AutoMap(() => Number)
  m_active_camo_energy_curve_max = 0;
  @AutoMap(() => Number)
  m_armor_lock_damage_drain = 0;
  @AutoMap(() => Number)
  m_armor_lock_damage_drain_limit = 0;
  @AutoMap(() => Number)
  m_magnum_damage = 0;
  @AutoMap(() => Number)
  m_magnum_fire_delay = 0;
  decode(bitstream: c_bitstream_reader): void {
    this.m_flags = bitstream.read_integer("flags", 32);
    this.m_precision_bloom = bitstream.read_quantized_real(
      0,
      2,
      8,
      false,
      true
    );
    this.m_active_camo_energy_curve_min = bitstream.read_quantized_real(
      0,
      2,
      8,
      false,
      true
    );
    this.m_active_camo_energy_curve_max = bitstream.read_quantized_real(
      0,
      2,
      8,
      false,
      true
    );
    this.m_armor_lock_damage_drain = bitstream.read_quantized_real(
      0,
      2,
      8,
      false,
      true
    );
    this.m_armor_lock_damage_drain_limit = bitstream.read_quantized_real(
      0,
      2,
      8,
      false,
      true
    );
    this.m_magnum_damage = bitstream.read_quantized_real(0, 10, 8, false, true);
    this.m_magnum_fire_delay = bitstream.read_quantized_real(
      0,
      10,
      8,
      false,
      true
    );
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_flags, 32);
    bitstream.write_quantized_real(
      this.m_precision_bloom,
      0,
      2,
      8,
      false,
      true
    );
    bitstream.write_quantized_real(
      this.m_active_camo_energy_curve_min,
      0,
      2,
      8,
      false,
      true
    );
    bitstream.write_quantized_real(
      this.m_active_camo_energy_curve_max,
      0,
      2,
      8,
      false,
      true
    );
    bitstream.write_quantized_real(
      this.m_armor_lock_damage_drain,
      0,
      2,
      8,
      false,
      true
    );
    bitstream.write_quantized_real(
      this.m_armor_lock_damage_drain_limit,
      0,
      2,
      8,
      false,
      true
    );
    bitstream.write_quantized_real(this.m_magnum_damage, 0, 10, 8, false, true);
    bitstream.write_quantized_real(
      this.m_magnum_fire_delay,
      0,
      10,
      8,
      false,
      true
    );
  }
}

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
  @AutoMap(() => [Boolean])
  m_base_variant_parameters_locked: boolean[] = Array.from(
    { length: 1280 },
    () => false
  );
  @AutoMap(() => [Boolean])
  m_base_variant_parameters_hidden: boolean[] = Array.from(
    { length: 1280 },
    () => false
  );
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
  @AutoMap(() => c_game_engine_custom_variant_au1_settings)
  m_au1_settings?: c_game_engine_custom_variant_au1_settings;
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
    for (let i = 0; i < 1280; i++) {
      this.m_base_variant_parameters_locked[i] = bitstream.read_bool(
        "base-variant-parameters-locked"
      );
    }
    for (let i = 0; i < 1280; i++) {
      this.m_base_variant_parameters_hidden[i] = bitstream.read_bool(
        "base-variant-parameters-hidden"
      );
    }
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
          "Writing v107 gametypes (and higher) requires AU1 Options to be set."
        );
      }
      this.m_au1_settings.encode(bitstream);
    }
  }
}

/** Matches `e_game_mode` in blf_lib. */
export enum e_game_mode {
  none = 0,
  sandbox = 1,
  megalogamengine = 2,
  campaign = 3,
  survival = 4,
}
/**
 * Halo Reach game variant gametype bitstream body (after the `mpvr` hash header).
 * Mirrors `c_game_variant` in blf_lib — decode-only.
 */
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
