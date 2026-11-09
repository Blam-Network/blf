import { readFileSync } from "node:fs";
import { bitstream } from "@blamnetwork/blf";
import {
  c_game_engine_custom_variant,
  c_string_table,
} from "@blamnetwork/blf/haloreach_mcc/v_untracked_25_08_16_1352";
import { describe, expect, it } from "vitest";
import { megaloedit_speedpile_fixture } from "./fixtures/paths";

const { c_bitstream_reader, c_bitstream_writer, e_bitstream_byte_order } =
  bitstream;
const BE = e_bitstream_byte_order._bitstream_byte_order_big_endian;

const k_tables = [
  "m_script_strings",
  "m_localized_name",
  "m_localized_description",
  "m_localized_category",
] as const;

function decode(bytes: Uint8Array): c_game_engine_custom_variant {
  const reader = c_bitstream_reader.new(bytes, BE);
  reader.begin_reading();
  const variant = new c_game_engine_custom_variant();
  variant.decode(reader);
  return variant;
}

function encode(variant: c_game_engine_custom_variant): Uint8Array {
  const writer = c_bitstream_writer.new(0, BE);
  writer.begin_writing();
  variant.encode(writer);
  writer.finish_writing();
  return writer.get_data();
}

describe("MegaloEdit string tables (MCC)", () => {
  const bytes = new Uint8Array(readFileSync(megaloedit_speedpile_fixture));

  it("repacks assigned strings to MegaloEdit's bytes", () => {
    const variant = decode(bytes);
    for (const key of k_tables) {
      const table = variant[key];
      const repacked = new c_string_table(
        table.max_string_count,
        table.max_string_length,
        table.offset_bit_length,
        table.buffer_size_bit_length,
        table.count_bit_length
      );
      repacked.strings = table.strings;
      expect(repacked.m_offsets, key).toEqual(table.m_offsets);
      expect(repacked.m_buffer, key).toEqual(table.m_buffer);
      variant[key] = repacked;
    }
    expect(encode(variant)).toEqual(bytes);
  });
});
