import { readFileSync } from "node:fs";
import { search_for_chunk } from "@blamnetwork/blf";
import {
  s_blf_chunk_content_header,
  s_blf_chunk_game_variant,
} from "@blamnetwork/blf/haloreach_mcc/v_untracked_25_08_16_1352";
import { describe, expect, it } from "vitest";
import { reach_mcc_3nvasion_dlc_fixture } from "./fixtures/paths";
import { deepSnapshot } from "./lib/snapshot";

const ENDIAN = "big" as const;

describe("3nvasion_dlc_054.bin", () => {
  const file = new Uint8Array(readFileSync(reach_mcc_3nvasion_dlc_fixture));

  it("reads content header", () => {
    const contentHeader = new s_blf_chunk_content_header();
    expect(search_for_chunk(file, contentHeader, "little")).toBe(true);
    expect(deepSnapshot(contentHeader)).toMatchSnapshot();
  });

  it("reads mpvr (game variant)", () => {
    const mpvr = new s_blf_chunk_game_variant();
    expect(search_for_chunk(file, mpvr, ENDIAN)).toBe(true);

    expect(
      mpvr.game_variant.m_custom_variant!.m_base_variant.m_metadata.name
    ).toBe("INVASION: BREAKPOINT");
    expect(deepSnapshot(mpvr)).toMatchSnapshot();
  });

  it("round-trips mpvr body bytes", () => {
    const mpvr = new s_blf_chunk_game_variant();
    expect(search_for_chunk(file, mpvr, ENDIAN)).toBe(true);

    const written = mpvr.write_body(ENDIAN);
    const roundtrip = new s_blf_chunk_game_variant();
    roundtrip.read_body(written, ENDIAN);

    expect(Array.from(roundtrip.hash)).toEqual(Array.from(mpvr.hash));
    expect(roundtrip.variant_length).toBe(mpvr.variant_length);
    expect(deepSnapshot(roundtrip.game_variant)).toEqual(
      deepSnapshot(mpvr.game_variant)
    );
  });
});
