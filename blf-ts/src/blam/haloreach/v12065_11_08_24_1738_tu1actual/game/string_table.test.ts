import { describe, expect, it } from "vitest";
import {
  c_bitstream_reader,
  c_bitstream_writer,
  e_bitstream_byte_order,
} from "../../../../bitstream";
import { runtime_data_decompress } from "../../../common/memory/data_compress";
import { c_string_table, k_language_count } from "./string_table";

const BE = e_bitstream_byte_order._bitstream_byte_order_big_endian;

function encodeTable(table: c_string_table): Uint8Array {
  const writer = c_bitstream_writer.new(0x4000, BE);
  writer.begin_writing();
  table.encode(writer);
  writer.finish_writing();
  return writer.get_data();
}

function decompressStringBlob(encoded: Uint8Array): Uint8Array {
  const reader = c_bitstream_reader.new(encoded, BE);
  reader.begin_reading();
  reader.read_integer("string-count", 1);
  for (let i = 0; i < k_language_count; i++) {
    reader.read_bool("exists");
    reader.read_integer("index", 9);
  }
  const size = reader.read_integer("size", 9);
  const compressed = reader.read_bool("compressed");
  if (!compressed) {
    const raw = reader.read_raw_data(size * 8);
    reader.finish_reading();
    return raw;
  }
  const compressedSize = reader.read_integer("compressed-size", 9);
  const raw = reader.read_raw_data(compressedSize * 8);
  reader.finish_reading();
  return runtime_data_decompress(raw);
}

describe("c_string_table", () => {
  it("round-trips UTF-8 script strings", () => {
    const table = new c_string_table(1, 32, 9, 9, 1);
    table.strings = Array.from({ length: k_language_count }, () => [
      "Abkühlungseigenschaften",
    ]);

    const encoded = encodeTable(table);

    const reader = c_bitstream_reader.new(encoded, BE);
    reader.begin_reading();
    const roundtrip = new c_string_table(1, 32, 9, 9, 1);
    roundtrip.decode(reader);
    reader.finish_reading();

    expect(roundtrip.strings[0]![0]).toBe("Abkühlungseigenschaften");

    const stringData = decompressStringBlob(encoded);
    expect(stringData[0]).toBe(0x41);
    expect(stringData[3]).toBe(0xc3);
    expect(stringData[4]).toBe(0xbc);
    expect(stringData[24]).toBe(0);
  });

  it("round-trips non-ASCII UTF-8 bytes (U+00A6)", () => {
    const table = new c_string_table(1, 32, 9, 9, 1);
    table.strings = Array.from({ length: k_language_count }, () => ["\u00a6"]);

    const encoded = encodeTable(table);

    const reader = c_bitstream_reader.new(encoded, BE);
    reader.begin_reading();
    const roundtrip = new c_string_table(1, 32, 9, 9, 1);
    roundtrip.decode(reader);
    reader.finish_reading();

    expect(roundtrip.strings[0]![0]).toBe("\u00a6");

    const stringData = decompressStringBlob(encoded);
    expect(stringData[0]).toBe(0xc2);
    expect(stringData[1]).toBe(0xa6);
    expect(stringData[2]).toBe(0);
  });

  it("uses uncompressed wire format for small string buffers", () => {
    const table = new c_string_table(1, 0x180, 9, 9, 1);
    table.strings = Array.from({ length: k_language_count }, () => ["Slayer"]);

    const encoded = encodeTable(table);

    const reader = c_bitstream_reader.new(encoded, BE);
    reader.begin_reading();
    const roundtrip = new c_string_table(1, 0x180, 9, 9, 1);
    roundtrip.decode(reader);
    reader.finish_reading();

    expect(roundtrip.m_buffer_is_compressed).toBe(false);
    expect(roundtrip.strings[0]![0]).toBe("Slayer");
    expect(Array.from(encoded)).toEqual(Array.from(encodeTable(roundtrip)));
  });

  it("uses compressed wire format for large string buffers", () => {
    const table = new c_string_table(1, 0x4c00, 15, 15, 7);
    const longString = "x".repeat(200);
    table.strings = Array.from({ length: k_language_count }, () => [
      longString,
    ]);

    const encoded = encodeTable(table);

    const reader = c_bitstream_reader.new(encoded, BE);
    reader.begin_reading();
    const roundtrip = new c_string_table(1, 0x4c00, 15, 15, 7);
    roundtrip.decode(reader);
    reader.finish_reading();

    expect(roundtrip.m_buffer_is_compressed).toBe(true);
    expect(roundtrip.strings[0]![0]).toBe(longString);
  });
});
