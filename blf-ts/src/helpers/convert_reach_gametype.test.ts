import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { reach_mcc_3nvasion_dlc_fixture } from "../../tests/fixtures/paths";
import { c_game_engine_survival_variant as c_game_engine_survival_variant_tu1 } from "../blam/haloreach/v12065_11_08_24_1738_tu1actual/game/game_engine_survival";
import {
  c_game_engine_custom_variant as c_game_engine_custom_variant_tu1,
  c_game_variant as c_game_variant_tu1,
  e_game_mode as e_game_mode_tu1,
} from "../blam/haloreach/v12065_11_08_24_1738_tu1actual/game/game_variant";
import { e_math_operation as e_math_operation_tu1 } from "../blam/haloreach/v12065_11_08_24_1738_tu1actual/game/megalogamengine/megalogamengine_actions";
import { e_explicit_player_type as e_explicit_player_type_tu1 } from "../blam/haloreach/v12065_11_08_24_1738_tu1actual/game/megalogamengine/megalogamengine_explicit_player";
import { e_custom_variable_type as e_custom_variable_type_tu1 } from "../blam/haloreach/v12065_11_08_24_1738_tu1actual/game/megalogamengine/megalogamengine_references";
import { c_game_engine_survival_variant as c_game_engine_survival_variant_mcc } from "../blam/haloreach_mcc/v_untracked_25_08_16_1352/game/game_engine_survival";
import {
  c_game_engine_custom_variant as c_game_engine_custom_variant_mcc,
  c_game_variant as c_game_variant_mcc,
  e_game_mode as e_game_mode_mcc,
  s_custom_game_engine_definition,
} from "../blam/haloreach_mcc/v_untracked_25_08_16_1352/game/game_variant";
import {
  c_action,
  e_action_type,
  e_math_operation as e_math_operation_mcc,
  s_action_object_get_orientation_parameters,
  s_action_set_score_parameters,
} from "../blam/haloreach_mcc/v_untracked_25_08_16_1352/game/megalogamengine/megalogamengine_actions";
import { e_explicit_player_type as e_explicit_player_type_mcc } from "../blam/haloreach_mcc/v_untracked_25_08_16_1352/game/megalogamengine/megalogamengine_explicit_player";
import {
  c_custom_variable_reference,
  c_explicit_player,
  c_object_reference,
  e_custom_variable_type as e_custom_variable_type_mcc,
  e_object_reference_type,
} from "../blam/haloreach_mcc/v_untracked_25_08_16_1352/game/megalogamengine/megalogamengine_references";
import { search_for_chunk } from "../blf_chunk";
import { s_blf_chunk_game_variant as s_blf_chunk_game_variant_mcc } from "../chunks/haloreach_mcc/v_untracked_25_08_16_1352/s_blf_chunk_game_variant";
import { BlfError } from "../error";
import {
  convert_reach_gametype,
  e_reach_gametype_conversion_error,
} from "./convert_reach_gametype";

function mcc_custom_variant_with_action(action: c_action): c_game_variant_mcc {
  const from = new c_game_variant_mcc();
  from.m_game_engine = e_game_mode_mcc.custom;
  const custom = new c_game_engine_custom_variant_mcc();
  const engine = new s_custom_game_engine_definition();
  engine.m_actions.push(action);
  custom.m_game_engine = engine;
  from.m_custom_variant = custom;
  return from;
}

describe("convert_reach_gametype", () => {
  it("throws when source and target are the same build", () => {
    const variant = new c_game_variant_tu1();
    variant.m_game_engine = e_game_mode_tu1.custom;

    expect(() =>
      convert_reach_gametype(variant, new c_game_variant_tu1())
    ).toThrow(BlfError);
  });

  it("returns forge_variant for sandbox variants", () => {
    const from = new c_game_variant_mcc();
    from.m_game_engine = e_game_mode_mcc.sandbox;

    expect(convert_reach_gametype(from, new c_game_variant_tu1())).toBe(
      e_reach_gametype_conversion_error.forge_variant
    );
  });

  it("returns campaign_variant for campaign variants", () => {
    const from = new c_game_variant_mcc();
    from.m_game_engine = e_game_mode_mcc.campaign;

    expect(convert_reach_gametype(from, new c_game_variant_tu1())).toBe(
      e_reach_gametype_conversion_error.campaign_variant
    );
  });

  it("copies MCC custom variant to TU1 when compatible", () => {
    const from = new c_game_variant_mcc();
    from.m_game_engine = e_game_mode_mcc.custom;
    from.m_custom_variant = new c_game_engine_custom_variant_mcc();
    from.m_custom_variant.m_build_number = 99;

    const to = new c_game_variant_tu1();
    expect(convert_reach_gametype(from, to)).toBe(
      e_reach_gametype_conversion_error.ok
    );
    expect(to.m_game_engine).toBe(e_game_mode_tu1.custom);
    expect(to.m_custom_variant?.m_build_number).toBe(99);
  });

  it("relocates a temporary player reference to a global slot MCC → TU1", () => {
    const action = new c_action();
    action.m_type = e_action_type.object_get_orientation;
    const player = new c_explicit_player();
    player.m_explicit_player_type = e_explicit_player_type_mcc.temporary_0;
    const object = new c_object_reference();
    object.m_type = e_object_reference_type.player_object;
    object.m_player = player;
    const parameters = new s_action_object_get_orientation_parameters();
    parameters.m_object = object;
    action.m_object_get_orientation_parameters = parameters;

    const from = mcc_custom_variant_with_action(action);
    const to = new c_game_variant_tu1();

    expect(convert_reach_gametype(from, to)).toBe(
      e_reach_gametype_conversion_error.ok
    );

    const copiedPlayer = to.m_custom_variant?.m_game_engine?.m_actions[0]
      ?.m_object_get_orientation_parameters?.m_object?.m_player as
      | c_explicit_player
      | undefined;
    expect(copiedPlayer?.m_explicit_player_type).toBe(
      e_explicit_player_type_tu1.global_0
    );
    expect(
      to.m_custom_variant?.m_game_engine?.m_global_variable_metadata
        .m_player_variables
    ).toHaveLength(1);
  });

  it("relocates a temporary_number reference to a global slot MCC → TU1", () => {
    const action = new c_action();
    action.m_type = e_action_type.set_score;
    const parameters = new s_action_set_score_parameters();
    const variable = new c_custom_variable_reference();
    variable.m_type = e_custom_variable_type_mcc.temporary_number;
    variable.m_variable_index = 0;
    parameters.m_variable = variable;
    action.m_set_score_parameters = parameters;

    const from = mcc_custom_variant_with_action(action);
    const to = new c_game_variant_tu1();

    expect(convert_reach_gametype(from, to)).toBe(
      e_reach_gametype_conversion_error.ok
    );

    const copiedVariable =
      to.m_custom_variant?.m_game_engine?.m_actions[0]?.m_set_score_parameters
        ?.m_variable;
    expect(copiedVariable?.m_type).toBe(
      e_custom_variable_type_tu1.global_number
    );
    expect(copiedVariable?.m_variable_index).toBe(0);
    expect(
      to.m_custom_variant?.m_game_engine?.m_global_variable_metadata
        .m_numeric_variables
    ).toHaveLength(1);
  });

  it("returns insufficient_global_slots when player globals are exhausted", () => {
    const from = new c_game_variant_mcc();
    from.m_game_engine = e_game_mode_mcc.custom;
    const custom = new c_game_engine_custom_variant_mcc();
    const engine = new s_custom_game_engine_definition();
    engine.m_global_variable_metadata.m_player_variables = Array.from(
      { length: 8 },
      () => 0
    );

    const action = new c_action();
    action.m_type = e_action_type.object_get_orientation;
    const player = new c_explicit_player();
    player.m_explicit_player_type = e_explicit_player_type_mcc.temporary_0;
    const object = new c_object_reference();
    object.m_type = e_object_reference_type.player_object;
    object.m_player = player;
    const parameters = new s_action_object_get_orientation_parameters();
    parameters.m_object = object;
    action.m_object_get_orientation_parameters = parameters;
    engine.m_actions.push(action);

    custom.m_game_engine = engine;
    from.m_custom_variant = custom;

    const to = new c_game_variant_tu1();
    expect(convert_reach_gametype(from, to)).toBe(
      e_reach_gametype_conversion_error.insufficient_global_slots
    );
  });

  it("remaps set_to_absolute by enum name MCC → TU1", () => {
    const action = new c_action();
    action.m_type = 1;
    action.m_set_score_parameters = {
      m_operation: e_math_operation_mcc.set_to_absolute,
    } as never;

    const from = mcc_custom_variant_with_action(action);
    const to = new c_game_variant_tu1();

    expect(convert_reach_gametype(from, to)).toBe(
      e_reach_gametype_conversion_error.ok
    );
    expect(
      to.m_custom_variant?.m_game_engine?.m_actions[0]?.m_set_score_parameters
        ?.m_operation
    ).toBe(e_math_operation_tu1.set_to_absolute);
  });

  it("returns mcc_exclusive_math_operator when <<= is used", () => {
    const action = new c_action();
    action.m_type = 1;
    action.m_set_score_parameters = {
      m_operation: e_math_operation_mcc.shift_left_with,
    } as never;

    const from = mcc_custom_variant_with_action(action);
    const to = new c_game_variant_tu1();

    expect(convert_reach_gametype(from, to)).toBe(
      e_reach_gametype_conversion_error.mcc_exclusive_math_operator
    );
  });

  it("returns mcc_exclusive_action when an MCC-only action type is used", () => {
    const action = new c_action();
    action.m_type = e_action_type.hide_object;

    const from = mcc_custom_variant_with_action(action);
    const to = new c_game_variant_tu1();

    expect(convert_reach_gametype(from, to)).toBe(
      e_reach_gametype_conversion_error.mcc_exclusive_action
    );
  });

  it("returns mcc_survival_additional_flags when survival additional_flags are present", () => {
    const from = new c_game_variant_mcc();
    from.m_game_engine = e_game_mode_mcc.survival;
    const survival = new c_game_engine_survival_variant_mcc();
    survival.m_additional_flags = 1;
    from.m_survival_variant = survival;

    const to = new c_game_variant_tu1();
    expect(convert_reach_gametype(from, to)).toBe(
      e_reach_gametype_conversion_error.mcc_survival_additional_flags
    );
  });

  it("sets survival campaign difficulty 1 MCC → TU1 and encoding version 2 TU1 → MCC", () => {
    const mcc = new c_game_variant_mcc();
    mcc.m_game_engine = e_game_mode_mcc.survival;
    const mccSurvival = new c_game_engine_survival_variant_mcc();
    mccSurvival.m_encoding_version = 5;
    mccSurvival.m_additional_flags = 0;
    mcc.m_survival_variant = mccSurvival;

    const tu1Target = new c_game_variant_tu1();
    expect(convert_reach_gametype(mcc, tu1Target)).toBe(
      e_reach_gametype_conversion_error.ok
    );
    expect(tu1Target.m_survival_variant?.m_campaign_difficulty_level).toBe(1);
    expect(
      (tu1Target.m_survival_variant as Record<string, unknown>)
        .m_encoding_version
    ).toBeUndefined();

    const tu1 = new c_game_variant_tu1();
    tu1.m_game_engine = e_game_mode_tu1.survival;
    const tu1Survival = new c_game_engine_survival_variant_tu1();
    tu1Survival.m_campaign_difficulty_level = 3;
    tu1.m_survival_variant = tu1Survival;

    const mccTarget = new c_game_variant_mcc();
    expect(convert_reach_gametype(tu1, mccTarget)).toBe(
      e_reach_gametype_conversion_error.ok
    );
    expect(mccTarget.m_survival_variant?.m_encoding_version).toBe(2);
    expect(mccTarget.m_survival_variant?.m_additional_flags).toBe(0);
    expect(
      (mccTarget.m_survival_variant as Record<string, unknown>)
        .m_campaign_difficulty_level
    ).toBeUndefined();
  });

  it("always allows TU1 → MCC for custom variants", () => {
    const from = new c_game_variant_tu1();
    from.m_game_engine = e_game_mode_tu1.custom;
    from.m_custom_variant = new c_game_engine_custom_variant_tu1();

    expect(convert_reach_gametype(from, new c_game_variant_mcc())).toBe(
      e_reach_gametype_conversion_error.ok
    );
  });

  it("resolves $hr_gvar_ metadata from localized strings MCC → TU1", () => {
    const from = new c_game_variant_mcc();
    from.m_game_engine = e_game_mode_mcc.custom;
    const custom = new c_game_engine_custom_variant_mcc();
    custom.m_base_variant.m_metadata.name = "$hr_gvar_TU_Headhunter";
    custom.m_base_variant.m_metadata.description =
      "$hr_gvar_TU_Headhunter_desc";
    custom.m_localized_name.strings[0] = ["Team Headhunter"];
    custom.m_localized_description.strings[0] = [
      "Collect skulls from fallen foes.",
    ];
    from.m_custom_variant = custom;

    const to = new c_game_variant_tu1();
    expect(convert_reach_gametype(from, to)).toBe(
      e_reach_gametype_conversion_error.ok
    );
    expect(to.m_custom_variant?.m_base_variant.m_metadata.name).toBe(
      "Team Headhunter"
    );
    expect(to.m_custom_variant?.m_base_variant.m_metadata.description).toBe(
      "Collect skulls from fallen foes."
    );
  });

  it("falls back when $hr_gvar_ localized strings are missing MCC → TU1", () => {
    const from = new c_game_variant_mcc();
    from.m_game_engine = e_game_mode_mcc.custom;
    const custom = new c_game_engine_custom_variant_mcc();
    custom.m_base_variant.m_metadata.name = "$hr_gvar_TU_Headhunter";
    custom.m_base_variant.m_metadata.description =
      "$hr_gvar_TU_Headhunter_desc";
    from.m_custom_variant = custom;

    const to = new c_game_variant_tu1();
    expect(convert_reach_gametype(from, to)).toBe(
      e_reach_gametype_conversion_error.ok
    );
    expect(to.m_custom_variant?.m_base_variant.m_metadata.name).toBe(
      "TU Headhunter"
    );
    expect(to.m_custom_variant?.m_base_variant.m_metadata.description).toBe("");
  });

  it("converts the MCC 3nvasion DLC gametype to TU1 with proper class instances", () => {
    const file = new Uint8Array(readFileSync(reach_mcc_3nvasion_dlc_fixture));
    const mpvr = new s_blf_chunk_game_variant_mcc();
    expect(search_for_chunk(file, mpvr, "big")).toBe(true);

    const from = mpvr.game_variant;
    const to = new c_game_variant_tu1();
    expect(convert_reach_gametype(from, to)).toBe(
      e_reach_gametype_conversion_error.ok
    );

    expect(to.m_game_engine).toBe(e_game_mode_tu1.custom);
    expect(to.m_custom_variant?.m_base_variant.m_metadata.name).toBe(
      "INVASION: BREAKPOINT"
    );
    expect(to.m_custom_variant?.m_base_variant.m_metadata.description).toBe(
      "Spartans, defend the data core at all costs."
    );
    expect(typeof to.m_custom_variant?.encode).toBe("function");
    expect(to.m_custom_variant?.m_game_engine?.m_actions[0]?.encode).toBeTypeOf(
      "function"
    );
  });
});
