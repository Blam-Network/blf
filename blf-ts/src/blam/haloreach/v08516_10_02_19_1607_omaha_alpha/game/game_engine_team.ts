import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../bitstream";
import { AutoMap } from "../../../../helpers/automap";
import { type StaticArray, staticArray } from "../../../../types/static_array";
import {
  e_game_engine_team_options_designator_switch_type,
  e_multiplayer_team_designator,
  e_player_model_choice,
  k_game_variant_team_count,
} from "../../v12065_11_08_24_1738_tu1actual/game/game_engine_default";
import { c_string_table } from "../../v12065_11_08_24_1738_tu1actual/game/string_table";

/**
 * Matches blf_lib omaha_delta/alpha `c_game_engine_team_options_team`
 * (no fireteam_count — that arrives later in TU1).
 */
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
  initialize(team_index: number): void {
    this.m_team_enabled = true;
    this.m_override_color_armour = false;
    this.m_override_color_ui_text = false;
    this.m_override_color_ui_bitmap = false;
    this.m_name = new c_string_table(1, 32, 5, 6, 1);
    this.m_team_initial_designator = team_index;
    this.m_model_override = e_player_model_choice.spartan;
    this.m_team_color_override = 0xffffffff;
    this.m_team_ui_text_tint_color_override = 0xffffffff;
    this.m_team_ui_bitmap_tint_color_override = 0xffffffff;
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
