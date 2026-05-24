import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { reach_12065_oddball_fixture } from "../../../../tests/fixtures/paths";
import { search_for_chunk } from "../../../blf_chunk";
import { s_blf_chunk_game_variant } from "../../haloreach/v12065_11_08_24_1738_tu1actual/s_blf_chunk_game_variant";
import { s_blf_chunk_compressed_data } from "./s_blf_chunk_compressed_data";

describe("s_blf_chunk_compressed_data", () => {
  it("round-trips an inner mpvr chunk", () => {
    const file = new Uint8Array(readFileSync(reach_12065_oddball_fixture));
    const original = new s_blf_chunk_game_variant();
    expect(search_for_chunk(file, original, "big")).toBe(true);

    const cmp = s_blf_chunk_compressed_data.create(
      s_blf_chunk_game_variant,
      original
    );
    const payload = cmp.write("big");

    const roundtrip = new s_blf_chunk_compressed_data(s_blf_chunk_game_variant);
    roundtrip.read(payload, "big");

    expect(roundtrip.compression_type).toBe(0);
    expect(roundtrip.chunk.game_variant.m_game_engine).toBe(
      original.game_variant.m_game_engine
    );
    expect(
      roundtrip.chunk.game_variant.m_custom_variant!.m_base_variant.m_metadata
        .name
    ).toBe(
      original.game_variant.m_custom_variant!.m_base_variant.m_metadata.name
    );
  });
});
