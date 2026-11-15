import { describe, expect, it } from "vitest";
import { c_game_engine_slayer_variant } from "../../../blam/halo3/v12070_08_09_05_2031_halo3_ship/game/game_engine_slayer";
import { s_blf_chunk_map_variant } from "./s_blf_chunk_map_variant";
import { s_blf_chunk_saved_film_header } from "./s_blf_chunk_saved_film_header";

describe("s_blf_chunk_saved_film_header", () => {
  it("matches blf_lib sizes", () => {
    expect(s_blf_chunk_saved_film_header.schema.getSize()).toBe(0xfe60);
    expect(s_blf_chunk_map_variant.schema.getSize()).toBe(0xe098);
  });

  it("round-trips the film's variants", () => {
    const original = new s_blf_chunk_saved_film_header();
    original.build_number = "12070.08.09.05.2031.halo3_ship";
    original.is_host_film = true;
    original.length_in_ticks = 18000;
    original.options.map_variant.m_metadata.name = "Sandbox";
    original.options.map_variant.m_map_id = 30;
    original.options.multiplayer_variant.m_game_engine = 2;
    original.options.multiplayer_variant.m_slayer_variant =
      c_game_engine_slayer_variant.zero();
    original.options.multiplayer_variant.m_base_variant.m_metadata.name =
      "Slayer";

    const bytes = original.write("big");
    const parsed = new s_blf_chunk_saved_film_header();
    parsed.read(bytes, "big");

    expect(parsed.build_number).toBe(original.build_number);
    expect(parsed.is_host_film).toBe(true);
    expect(parsed.length_in_ticks).toBe(18000);
    expect(parsed.options.map_variant.m_metadata.name).toBe("Sandbox");
    expect(parsed.options.map_variant.m_map_id).toBe(30);
    expect(parsed.options.multiplayer_variant.m_game_engine).toBe(2);
    expect(parsed.options.multiplayer_variant.m_slayer_variant).toBeDefined();
    expect(
      parsed.options.multiplayer_variant.m_base_variant.m_metadata.name
    ).toBe("Slayer");

    const mapv = s_blf_chunk_map_variant.create(parsed.options.map_variant);
    const reread = new s_blf_chunk_map_variant();
    reread.read(mapv.write("big"), "big");
    expect(reread.map_variant).toEqual(parsed.options.map_variant);
  });
});
