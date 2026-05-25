import { describe, expect, it } from "vitest";
import { c_game_engine_survival_variant as c_game_engine_survival_variant_tu1 } from "./blam/haloreach/v12065_11_08_24_1738_tu1actual/game/c_game_engine_survival_variant";
import {
  c_game_variant as c_game_variant_tu1,
  e_game_mode as e_game_mode_tu1,
} from "./blam/haloreach/v12065_11_08_24_1738_tu1actual/game/c_game_variant";
import { e_math_operation as e_math_operation_tu1 } from "./blam/haloreach/v12065_11_08_24_1738_tu1actual/game/megalogamengine/megalogamengine_actions";
import { c_game_engine_custom_variant as c_game_engine_custom_variant_mcc } from "./blam/haloreach_mcc/v_untracked_25_08_19_1352/game/c_game_engine_custom_variant";
import { c_game_engine_survival_variant as c_game_engine_survival_variant_mcc } from "./blam/haloreach_mcc/v_untracked_25_08_19_1352/game/c_game_engine_survival_variant";
import {
  c_game_variant as c_game_variant_mcc,
  e_game_mode as e_game_mode_mcc,
} from "./blam/haloreach_mcc/v_untracked_25_08_19_1352/game/c_game_variant";
import {
  c_action,
  e_math_operation as e_math_operation_mcc,
} from "./blam/haloreach_mcc/v_untracked_25_08_19_1352/game/megalogamengine/megalogamengine_actions";
import { e_explicit_player_type } from "./blam/haloreach_mcc/v_untracked_25_08_19_1352/game/megalogamengine/megalogamengine_explicit_player";
import { c_explicit_player } from "./blam/haloreach_mcc/v_untracked_25_08_19_1352/game/megalogamengine/megalogamengine_references";
import { s_custom_game_engine_definition } from "./blam/haloreach_mcc/v_untracked_25_08_19_1352/game/megalogamengine/s_custom_game_engine_definition";
import { convert_reach_gametype } from "./convert_reach_gametype";
import { BlfError } from "./error";

const k_forge_sandbox_error = /Forge game variants are not supported/;

describe("convert_reach_gametype", () => {
  it("throws when source and target are the same build", () => {
    const variant = new c_game_variant_tu1();
    variant.m_game_engine = e_game_mode_tu1.custom;

    expect(() =>
      convert_reach_gametype(variant, new c_game_variant_tu1())
    ).toThrow(BlfError);
  });

  it("throws for forge (sandbox) variants", () => {
    const from = new c_game_variant_mcc();
    from.m_game_engine = e_game_mode_mcc.sandbox;

    expect(() =>
      convert_reach_gametype(from, new c_game_variant_tu1())
    ).toThrow(k_forge_sandbox_error);
  });

  it("copies MCC custom variant to TU1 when compatible", () => {
    const from = new c_game_variant_mcc();
    from.m_game_engine = e_game_mode_mcc.custom;
    from.m_custom_variant = new c_game_engine_custom_variant_mcc();
    from.m_custom_variant.m_build_number = 99;

    const to = new c_game_variant_tu1();
    expect(convert_reach_gametype(from, to)).toBe(true);
    expect(to.m_game_engine).toBe(e_game_mode_tu1.custom);
    expect(to.m_custom_variant?.m_build_number).toBe(99);
  });

  it("fails MCC → TU1 when a temporary player reference is used", () => {
    const from = new c_game_variant_mcc();
    from.m_game_engine = e_game_mode_mcc.custom;
    const custom = new c_game_engine_custom_variant_mcc();
    const engine = new s_custom_game_engine_definition();
    const action = new c_action();
    action.m_type = 3;
    const player = new c_explicit_player();
    player.m_explicit_player_type = e_explicit_player_type.temporary_0;
    action.m_object = { m_player: player } as never;
    engine.m_actions.push(action);
    custom.m_game_engine = engine;
    from.m_custom_variant = custom;

    const to = new c_game_variant_tu1();
    expect(convert_reach_gametype(from, to)).toBe(false);
  });

  it("remaps abs= MCC → TU1", () => {
    const from = new c_game_variant_mcc();
    from.m_game_engine = e_game_mode_mcc.custom;
    const custom = new c_game_engine_custom_variant_mcc();
    const engine = new s_custom_game_engine_definition();
    const action = new c_action();
    action.m_type = 1;
    action.m_set_score_parameters = {
      m_operation: e_math_operation_mcc.set_to_absolute,
    } as never;
    engine.m_actions.push(action);
    custom.m_game_engine = engine;
    from.m_custom_variant = custom;

    const to = new c_game_variant_tu1();
    expect(convert_reach_gametype(from, to)).toBe(true);
    const copied = to.m_custom_variant?.m_game_engine?.m_actions[0]
      ?.m_set_score_parameters as { m_operation: number } | undefined;
    expect(copied?.m_operation).toBe(e_math_operation_tu1.set_to_absolute);
  });

  it("fails MCC → TU1 when <<= is used", () => {
    const from = new c_game_variant_mcc();
    from.m_game_engine = e_game_mode_mcc.custom;
    const custom = new c_game_engine_custom_variant_mcc();
    const engine = new s_custom_game_engine_definition();
    const action = new c_action();
    action.m_type = 1;
    action.m_set_score_parameters = {
      m_operation: e_math_operation_mcc.shift_left_with,
    } as never;
    engine.m_actions.push(action);
    custom.m_game_engine = engine;
    from.m_custom_variant = custom;

    const to = new c_game_variant_tu1();
    expect(convert_reach_gametype(from, to)).toBe(false);
  });

  it("fails MCC → TU1 when survival additional_flags are present", () => {
    const from = new c_game_variant_mcc();
    from.m_game_engine = e_game_mode_mcc.survival;
    const survival = new c_game_engine_survival_variant_mcc();
    survival.m_additional_flags = 1;
    from.m_survival_variant = survival;

    const to = new c_game_variant_tu1();
    expect(convert_reach_gametype(from, to)).toBe(false);
  });

  it("sets survival campaign difficulty 1 MCC → TU1 and encoding version 2 TU1 → MCC", () => {
    const mcc = new c_game_variant_mcc();
    mcc.m_game_engine = e_game_mode_mcc.survival;
    const mccSurvival = new c_game_engine_survival_variant_mcc();
    mccSurvival.m_encoding_version = 5;
    mccSurvival.m_additional_flags = 0;
    mcc.m_survival_variant = mccSurvival;

    const tu1Target = new c_game_variant_tu1();
    expect(convert_reach_gametype(mcc, tu1Target)).toBe(true);
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
    expect(convert_reach_gametype(tu1, mccTarget)).toBe(true);
    expect(mccTarget.m_survival_variant?.m_encoding_version).toBe(2);
    expect(mccTarget.m_survival_variant?.m_additional_flags).toBe(0);
    expect(
      (mccTarget.m_survival_variant as Record<string, unknown>)
        .m_campaign_difficulty_level
    ).toBeUndefined();
  });
});
