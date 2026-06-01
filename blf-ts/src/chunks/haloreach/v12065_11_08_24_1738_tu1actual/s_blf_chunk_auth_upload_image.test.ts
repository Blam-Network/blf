import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { reach_spartan_render_auiu_cmp_fixture } from "../../../../tests/fixtures/paths";
import { find_chunk, getBlfChunkMeta } from "../../../blf_chunk";
import { s_blf_chunk_compressed_data } from "../../halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_compressed_data";
import {
  AUIU_HEADER_SIZE,
  s_blf_chunk_auth_upload_image,
} from "./s_blf_chunk_auth_upload_image";

describe("s_blf_chunk_auth_upload_image", () => {
  it("is auiu 1.2", () => {
    const meta = getBlfChunkMeta(new s_blf_chunk_auth_upload_image());
    expect(meta.signature).toBe("auiu");
    expect(meta.major).toBe(1);
    expect(meta.minor).toBe(2);
  });

  it("round-trips a small BGRA payload", () => {
    const original = new s_blf_chunk_auth_upload_image();
    original.width = 2;
    original.height = 2;
    original.stride = 2;
    original.pixels = new Uint8Array([
      255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 0,
    ]);
    original.data_size = original.pixels.length;

    const body = original.write_body("big");
    expect(body.length).toBe(AUIU_HEADER_SIZE + 16);

    const parsed = new s_blf_chunk_auth_upload_image();
    parsed.read_body(body, "big");
    expect(parsed.width).toBe(2);
    expect(parsed.height).toBe(2);
    expect(parsed.stride).toBe(2);
    expect(parsed.data_size).toBe(16);
    expect(parsed.pixels).toEqual(original.pixels);
  });

  it("reads auiu from a _cmp-wrapped Spartan render upload", () => {
    const file = new Uint8Array(
      readFileSync(reach_spartan_render_auiu_cmp_fixture)
    );
    const cmp = new s_blf_chunk_compressed_data(s_blf_chunk_auth_upload_image);
    expect(find_chunk(file, cmp, "big")).toBe(true);
    expect(cmp.chunk.width).toBe(320);
    expect(cmp.chunk.height).toBe(704);
    expect(cmp.chunk.stride).toBe(320);
    expect(cmp.chunk.data_size).toBe(901_120);
    expect(cmp.chunk.pixels.length).toBe(901_120);
  });
});
