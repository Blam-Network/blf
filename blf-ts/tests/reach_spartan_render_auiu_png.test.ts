import { readFileSync } from "node:fs";
import { find_chunk } from "@blamnetwork/blf";
import {
  s_blf_chunk_auth_upload_image,
  s_blf_chunk_compressed_data,
} from "@blamnetwork/blf/haloreach/v12065_11_08_24_1738_tu1actual";
import { describe, expect, it } from "vitest";
import { auiu_chunk_to_png } from "../src/helpers/auiu_to_png";
import {
  reach_spartan_render_auiu_cmp_fixture,
  reach_spartan_render_auiu_png_snapshot,
} from "./fixtures/paths";
import { expectBinarySnapshot } from "./lib/binary_snapshot";

describe("reach_spartan_render_auiu_cmp.blf", () => {
  it("matches committed PNG snapshot", () => {
    const file = new Uint8Array(
      readFileSync(reach_spartan_render_auiu_cmp_fixture)
    );
    const cmp = new s_blf_chunk_compressed_data(s_blf_chunk_auth_upload_image);
    expect(find_chunk(file, cmp, "big")).toBe(true);
    expectBinarySnapshot(
      auiu_chunk_to_png(cmp.chunk),
      reach_spartan_render_auiu_png_snapshot
    );
  });

  it("reads auiu 1.2 inside _cmp", () => {
    const file = new Uint8Array(
      readFileSync(reach_spartan_render_auiu_cmp_fixture)
    );
    const cmp = new s_blf_chunk_compressed_data(s_blf_chunk_auth_upload_image);
    expect(find_chunk(file, cmp, "big")).toBe(true);
    expect(cmp.chunk.width).toBe(320);
    expect(cmp.chunk.height).toBe(704);
    expect(auiu_chunk_to_png(cmp.chunk).length).toBeGreaterThan(10_000);
  });
});
