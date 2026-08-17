import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import {
  halo4_21117_ctf_mglo,
  halo4_21117_dominion_mglo,
  halo4_21117_regicide_mglo,
  halo4_21117_slayer_mglo,
} from "../../../../tests/fixtures/paths";
import {
  c_bitstream_reader,
  c_bitstream_writer,
  e_bitstream_byte_order,
} from "../../../bitstream";
import {
  c_game_engine_custom_variant,
  c_game_variant,
  e_game_engine_type,
  k_game_engine_custom_variant_encoding_version,
} from "../../../blam/halo4/v20810_12_09_22_1647_main/game/game_variant";
import { s_blf_chunk_packed_game_variant } from "./s_blf_chunk_packed_game_variant";

/** `.mglo` is MegaloEdit `SerializedEngine`: custom-variant encode only (no 4-bit engine index). */
function decode_mglo(path: string): c_game_engine_custom_variant {
  const data = new Uint8Array(readFileSync(path));
  const bitstream = c_bitstream_reader.new(
    data,
    e_bitstream_byte_order._bitstream_byte_order_big_endian
  );
  bitstream.begin_reading();
  const variant = new c_game_engine_custom_variant();
  variant.decode(bitstream);
  bitstream.finish_reading();
  return variant;
}

function encode_custom_variant(
  variant: c_game_engine_custom_variant
): Uint8Array {
  const bitstream = c_bitstream_writer.new(
    31744,
    e_bitstream_byte_order._bitstream_byte_order_big_endian
  );
  bitstream.begin_writing();
  variant.encode(bitstream);
  bitstream.finish_writing();
  return bitstream.get_data();
}

function wrap_as_game_variant(
  custom: c_game_engine_custom_variant
): c_game_variant {
  const game = new c_game_variant();
  game.m_game_engine = e_game_engine_type.megalogamengine;
  game.m_custom_variant = custom;
  return game;
}

describe.each([
  ["h4_slayer.mglo", halo4_21117_slayer_mglo, "Slayer"],
  ["h4_ctf.mglo", halo4_21117_ctf_mglo, "Flag"],
  ["h4_dominion.mglo", halo4_21117_dominion_mglo, "Dominion"],
  ["h4_regicide.mglo", halo4_21117_regicide_mglo, "Regicide"],
])("Halo 4 %s SerializedEngine", (_label, fixture, expectedNameHint) => {
  it("decodes encoding version, build, name, and non-empty megalo", () => {
    const custom = decode_mglo(fixture);
    expect(custom.m_encoding_version).toBe(
      k_game_engine_custom_variant_encoding_version
    );
    expect(custom.m_build_number).toBe(21117);
    expect(custom.m_base_variant.m_metadata.name.length).toBeGreaterThan(0);
    expect(custom.m_base_variant.m_metadata.name).toMatch(
      new RegExp(expectedNameHint, "i")
    );
    expect(custom.m_game_engine.m_conditions.length).toBeGreaterThan(0);
    expect(custom.m_game_engine.m_actions.length).toBeGreaterThan(0);
    expect(custom.m_game_engine.m_triggers.length).toBeGreaterThan(0);
  });

  it("round-trips the bitstream payload bit-identically", () => {
    const original = new Uint8Array(readFileSync(fixture));
    const custom = decode_mglo(fixture);
    const written = encode_custom_variant(custom);
    expect(written).toEqual(original);
  });

  it("round-trips through packed gvar with engine-index prefix", () => {
    const custom = decode_mglo(fixture);
    const chunk = s_blf_chunk_packed_game_variant.create(
      wrap_as_game_variant(custom)
    );
    const written = chunk.write_body("big");
    const roundtrip = new s_blf_chunk_packed_game_variant();
    roundtrip.read_body(written, "big");
    expect(roundtrip.game_variant.m_game_engine).toBe(
      e_game_engine_type.megalogamengine
    );
    expect(roundtrip.game_variant.m_custom_variant!.m_encoding_version).toBe(
      k_game_engine_custom_variant_encoding_version
    );
    expect(
      roundtrip.game_variant.m_custom_variant!.m_base_variant.m_metadata.name
    ).toBe(custom.m_base_variant.m_metadata.name);
    expect(
      roundtrip.game_variant.m_custom_variant!.m_game_engine.m_actions.length
    ).toBe(custom.m_game_engine.m_actions.length);
  });
});
