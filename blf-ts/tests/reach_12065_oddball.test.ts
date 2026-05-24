import { readFileSync } from "node:fs";
import { search_for_chunk } from "@blamnetwork/blf";
import {
  s_blf_chunk_content_header,
  s_blf_chunk_game_variant,
} from "@blamnetwork/blf/haloreach/v12065_11_08_24_1738_tu1actual";
import { describe, expect, it } from "vitest";
import { reach_12065_oddball_fixture } from "./fixtures/paths";
import { deepSnapshot } from "./lib/snapshot";

const ENDIAN = "big" as const;

describe("haloreach_gea0_map.blf", () => {
  const file = new Uint8Array(readFileSync(reach_12065_oddball_fixture));

  it("reads chdr (content header)", () => {
    const chdr = new s_blf_chunk_content_header();
    expect(search_for_chunk(file, chdr, ENDIAN)).toBe(true);

    expect(deepSnapshot(chdr)).toMatchSnapshot();
  });

  it("reads mpvr (game variant)", () => {
    const mpvr = new s_blf_chunk_game_variant();
    expect(search_for_chunk(file, mpvr, ENDIAN)).toBe(true);

    expect(deepSnapshot(mpvr)).toMatchSnapshot();
  });
});
