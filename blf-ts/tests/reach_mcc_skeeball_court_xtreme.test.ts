import { readFileSync } from "node:fs";
import { search_for_chunk } from "@blamnetwork/blf";
import {
  s_blf_chunk_content_header,
  s_blf_chunk_map_variant,
} from "@blamnetwork/blf/haloreach_mcc/v_untracked_25_08_16_1352";
import { describe, expect, it } from "vitest";
import { reach_mcc_skeeball_court_xtreme_fixture } from "./fixtures/paths";
import { deepSnapshot } from "./lib/snapshot";

const ENDIAN = "big" as const;

describe("skeeball_court_xtreme_031.mvar", () => {
  const file = new Uint8Array(
    readFileSync(reach_mcc_skeeball_court_xtreme_fixture)
  );

  it("reads chdr (content header)", () => {
    const chdr = new s_blf_chunk_content_header();
    expect(search_for_chunk(file, chdr, "little")).toBe(true);

    expect(deepSnapshot(chdr)).toMatchSnapshot();
  });

  it("reads mvar (map variant)", () => {
    const mvar = new s_blf_chunk_map_variant();
    expect(search_for_chunk(file, mvar, ENDIAN)).toBe(true);

    expect(mvar.map_variant.m_metadata.name).toBe("HaloBall Court X");
    expect(deepSnapshot(mvar)).toMatchSnapshot();
  });

  it("round-trips mvar body bytes", () => {
    const mvar = new s_blf_chunk_map_variant();
    expect(search_for_chunk(file, mvar, ENDIAN)).toBe(true);

    const written = mvar.write_body(ENDIAN);
    const roundtrip = new s_blf_chunk_map_variant();
    roundtrip.read_body(written, ENDIAN);

    expect(Array.from(roundtrip.hash)).toEqual(Array.from(mvar.hash));
    expect(roundtrip.packed_length).toBe(mvar.packed_length);
    expect(deepSnapshot(roundtrip.map_variant)).toEqual(
      deepSnapshot(mvar.map_variant)
    );
  });
});
