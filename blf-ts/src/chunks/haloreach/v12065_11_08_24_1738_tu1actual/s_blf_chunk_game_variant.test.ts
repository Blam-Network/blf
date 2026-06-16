import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { reach_12065_oddball_fixture } from "../../../../tests/fixtures/paths";
import { security_calculate_hash } from "../../../blam/common/cache/security_functions";
import { e_game_mode } from "../../../blam/haloreach/v12065_11_08_24_1738_tu1actual/game/game_variant";
import { search_for_chunk } from "../../../blf_chunk";
import {
  MPVR_PAYLOAD_SIZE,
  s_blf_chunk_game_variant,
} from "./s_blf_chunk_game_variant";

describe("s_blf_chunk_game_variant", () => {
  it("decodes mpvr header fields and game-engine enum", () => {
    const gametype_data = new Uint8Array([0x20]);
    const hashable = new Uint8Array(4 + gametype_data.length);
    new DataView(hashable.buffer).setUint32(0, gametype_data.length, false);
    hashable.set(gametype_data, 4);
    const hash = security_calculate_hash(hashable);

    const payload = new Uint8Array(28 + gametype_data.length);
    payload.set(hash, 0);
    const view = new DataView(payload.buffer, payload.byteOffset + 20);
    view.setInt16(0, -1, false);
    view.setUint16(2, 0, false);
    view.setUint32(4, gametype_data.length, false);
    payload.set(gametype_data, 28);

    const chunk = new s_blf_chunk_game_variant();
    expect(() => chunk.read_body(payload, "big")).toThrow();

    expect(chunk.hash).toEqual(hash);
    expect(chunk.unknown04).toBe(-1);
    expect(chunk.unknown06).toBe(0);
    expect(chunk.variant_length).toBe(1);
    expect(chunk.game_variant.m_game_engine).toBe(e_game_mode.custom);
  });

  it("round-trips the Reach MPVR fixture (big-endian)", () => {
    const file = new Uint8Array(readFileSync(reach_12065_oddball_fixture));
    const original = new s_blf_chunk_game_variant();
    expect(search_for_chunk(file, original, "big")).toBe(true);

    const written = original.write_body("big");
    expect(written.length).toBe(MPVR_PAYLOAD_SIZE);
    const roundtrip = new s_blf_chunk_game_variant();
    roundtrip.read_body(written, "big");

    expect(roundtrip.hash).toEqual(original.hash);
    expect(roundtrip.variant_length).toBe(original.variant_length);
    expect(roundtrip.game_variant.m_game_engine).toBe(
      original.game_variant.m_game_engine
    );
    expect(
      roundtrip.game_variant.m_custom_variant!.m_base_variant.m_metadata.name
    ).toBe(
      original.game_variant.m_custom_variant!.m_base_variant.m_metadata.name
    );
  });

  it("finds and reads the Reach MPVR chunk (big-endian)", () => {
    const file = new Uint8Array(readFileSync(reach_12065_oddball_fixture));

    const mpvr = new s_blf_chunk_game_variant();
    expect(search_for_chunk(file, mpvr, "big")).toBe(true);

    expect(mpvr.game_variant.m_game_engine).toBe(e_game_mode.custom);
    expect(mpvr.game_variant.m_custom_variant).toBeDefined();
    expect(mpvr.game_variant.m_custom_variant!.m_encoding_version).toBe(107);
    expect(mpvr.game_variant.m_custom_variant!.m_build_number).toBe(12065);
    expect(
      mpvr.game_variant.m_custom_variant!.m_base_variant.m_metadata.name
    ).toBe("Oddball");
    expect(
      mpvr.game_variant.m_custom_variant!.m_game_engine.m_conditions.length
    ).toBeGreaterThan(0);
    expect(
      mpvr.game_variant.m_custom_variant!.m_game_engine.m_triggers.length
    ).toBeGreaterThan(0);
  });
});
