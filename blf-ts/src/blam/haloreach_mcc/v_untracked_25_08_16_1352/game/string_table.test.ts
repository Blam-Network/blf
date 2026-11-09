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

  it("skips compression when the zlib blob would be larger", () => {
    // Multilingual category-sized payload just over the 128-byte threshold
    // where zlib+header expands (MegaloEdit / pre_game parity).
    const table = new c_string_table(1, 0x180, 9, 9, 1);
    table.strings = [
      ["Pre-Game"],
      ["プレゲーム"],
      ["VOR DEM SPIEL"],
      ["ÉCHAUFFEMENT"],
      ["PREÁMBULO RECREATIVO"],
      ["PREVIO AL JUEGO"],
      ["PREPARTITA"],
      ["프리게임"],
      ["사전"],
      [null],
      ["Pré-Jogo"],
      [null],
    ];

    const encoded = encodeTable(table);

    const reader = c_bitstream_reader.new(encoded, BE);
    reader.begin_reading();
    const roundtrip = new c_string_table(1, 0x180, 9, 9, 1);
    roundtrip.decode(reader);
    reader.finish_reading();

    expect(roundtrip.m_buffer_is_compressed).toBe(false);
    expect(roundtrip.strings[0]![0]).toBe("Pre-Game");
  });
});

describe("c_string_table MegaloEdit packing", () => {
  function packed(strings: (string | null)[][]): c_string_table {
    const table = new c_string_table(4, 64, 15, 15, 3);
    table.strings = strings;
    return table;
  }

  function buffer_text(table: c_string_table): string {
    return new TextDecoder().decode(table.m_buffer);
  }

  it("stores a string shared by every language once", () => {
    const table = packed(Array.from({ length: k_language_count }, () => ["a"]));
    expect(buffer_text(table)).toBe("a\0");
    expect(table.m_offsets).toEqual(
      Array.from({ length: k_language_count }, () => [0])
    );
  });

  it("stores the same text at two indices twice", () => {
    const table = packed(
      Array.from({ length: k_language_count }, () => ["a", "a"])
    );
    expect(buffer_text(table)).toBe("a\0a\0");
    expect(table.m_offsets).toEqual(
      Array.from({ length: k_language_count }, () => [0, 2])
    );
  });

  it("stores each language separately when only some match", () => {
    const strings = [...new Array<string>(10).fill("a"), "b", null];
    const table = packed(strings.map((string) => [string]));
    expect(buffer_text(table)).toBe(`${"a\0".repeat(10)}b\0`);
    expect(table.m_offsets).toEqual([
      ...Array.from({ length: 11 }, (_, language) => [language * 2]),
      [-1],
    ]);
  });

  it("appends in index order, then language order", () => {
    const table = packed(
      Array.from({ length: k_language_count }, (_, language) => [
        language === 0 ? "x" : "y",
        "z",
      ])
    );
    expect(buffer_text(table)).toBe(`x\0${"y\0".repeat(11)}z\0`);
    expect(table.m_offsets[0]).toEqual([0, 24]);
    expect(table.m_offsets[11]).toEqual([22, 24]);
  });
});
