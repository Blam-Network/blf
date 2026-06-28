import { readFileSync } from "node:fs";
import { search_for_chunk } from "@blamnetwork/blf";
import { s_blf_chunk_game_variant } from "@blamnetwork/blf/haloreach_mcc/v_untracked_25_08_16_1352";
import { describe, expect, it } from "vitest";

const HOPPER =
  process.env.REACH_HOPPER_BIN ??
  String.raw`C:\Program Files (x86)\Steam\steamapps\common\Halo The Master Chief Collection\haloreach\hopper_game_variants\assault_one_bomb_anniversary_054.bin`;

describe("assault_one_bomb_anniversary hopper", () => {
  it("reads TU1 settings", () => {
    const file = new Uint8Array(readFileSync(HOPPER));
    const mpvr = new s_blf_chunk_game_variant();
    expect(search_for_chunk(file, mpvr, "big")).toBe(true);
    const custom = mpvr.game_variant.m_custom_variant!;
    const tu1 = custom.m_tu1_settings;
    console.log("tu1 flags", tu1.m_flags);
    console.log("precision_bloom", tu1.m_precision_bloom);
    console.log("magnum_damage", tu1.m_magnum_damage);
    expect(tu1.m_flags.always_spillover_damage).toBe(true);
    expect(tu1.m_precision_bloom).toBeCloseTo(0.8464566929133858, 5);
  });
});
