import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { reach_12065_oddball_fixture } from "../../../../tests/fixtures/paths";
import { BlfError } from "../../../error";
import { search_for_chunk } from "../../../blf_chunk";
import { write_blffile } from "../../../index";
import { e_game_mode } from "../../../blam/haloreach/v12065_11_08_24_1738_tu1actual/game/c_game_variant";
import { s_blf_chunk_game_variant } from "./s_blf_chunk_game_variant";
import { s_blf_chunk_packed_game_variant } from "./s_blf_chunk_packed_game_variant";
import {
  s_blf_chunk_start_of_file,
} from "../../halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_start_of_file";
import { s_blf_chunk_end_of_file } from "../../halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_end_of_file";

function extract_mpvr_gametype_payload(
  mpvr: s_blf_chunk_game_variant,
): Uint8Array {
  const payload = mpvr.write_body("big");
  return payload.subarray(28, 28 + mpvr.variant_length);
}

describe("s_blf_chunk_matchmaking_game_variant", () => {
  it("rejects empty payloads", () => {
    const chunk = new s_blf_chunk_packed_game_variant();
    expect(() => chunk.read_body(new Uint8Array(), "big")).toThrow(BlfError);
  });

  it("decodes gametype bytes extracted from mpvr", () => {
    const mpvr = new s_blf_chunk_game_variant();
    expect(search_for_chunk(readFileSync(reach_12065_oddball_fixture), mpvr, "big")).toBe(
      true,
    );

    const gametype_payload = extract_mpvr_gametype_payload(mpvr);
    const gvar = new s_blf_chunk_packed_game_variant();
    gvar.read_body(gametype_payload, "big");

    expect(gvar.game_variant.m_game_engine).toBe(e_game_mode.custom);
    expect(gvar.game_variant.m_custom_variant).toBeDefined();
    expect(gvar.game_variant.m_custom_variant!.m_encoding_version).toBe(107);
    expect(gvar.game_variant.m_custom_variant!.m_build_number).toBe(12065);
    expect(
      gvar.game_variant.m_custom_variant!.m_base_variant.m_metadata.name,
    ).toBe("Oddball");
  });

  it("round-trips gametype bytes byte-identically", () => {
    const mpvr = new s_blf_chunk_game_variant();
    expect(search_for_chunk(readFileSync(reach_12065_oddball_fixture), mpvr, "big")).toBe(
      true,
    );

    const gametype_payload = extract_mpvr_gametype_payload(mpvr);
    const gvar = new s_blf_chunk_packed_game_variant();
    gvar.read_body(gametype_payload, "big");

    const written = gvar.write_body("big");
    expect(Array.from(written)).toEqual(Array.from(gametype_payload));

    const roundtrip = new s_blf_chunk_packed_game_variant();
    roundtrip.read_body(written, "big");
    expect(roundtrip.game_variant.m_game_engine).toBe(
      gvar.game_variant.m_game_engine,
    );
    expect(
      roundtrip.game_variant.m_custom_variant!.m_base_variant.m_metadata.name,
    ).toBe(gvar.game_variant.m_custom_variant!.m_base_variant.m_metadata.name);
  });

  it("finds and reads a gvar chunk in a BLF file", () => {
    const mpvr = new s_blf_chunk_game_variant();
    expect(search_for_chunk(readFileSync(reach_12065_oddball_fixture), mpvr, "big")).toBe(
      true,
    );

    const gvar = s_blf_chunk_packed_game_variant.create(mpvr.game_variant);
    const blf = write_blffile("big", [
      s_blf_chunk_start_of_file.create("gvar-test"),
      gvar,
      new s_blf_chunk_end_of_file(),
    ]);

    const found = new s_blf_chunk_packed_game_variant();
    expect(search_for_chunk(blf, found, "big")).toBe(true);
    expect(found.game_variant.m_game_engine).toBe(e_game_mode.custom);
    expect(
      found.game_variant.m_custom_variant!.m_base_variant.m_metadata.name,
    ).toBe("Oddball");
  });
});
