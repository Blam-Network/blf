import { describe, expect, it } from "vitest";
import { e_bitstream_byte_order } from "./enums";
import { c_bitstream_reader } from "./reader";
import { c_bitstream_writer } from "./writer";

function write_then_read(
  writer: c_bitstream_writer,
  readerFactory: (data: Uint8Array) => c_bitstream_reader,
  writes: (w: c_bitstream_writer) => void,
  reads: (r: c_bitstream_reader) => void
): void {
  writer.begin_writing();
  writes(writer);
  writer.finish_writing();
  const data = writer.get_data();
  const reader = readerFactory(data);
  reader.begin_reading();
  reads(reader);
}

describe("c_bitstream_writer", () => {
  it("write_legacy_be", () => {
    const expected = new Uint8Array([0b10110_001, 0b00001001]);
    const sut = c_bitstream_writer.new_with_legacy_settings(
      2,
      e_bitstream_byte_order._bitstream_byte_order_big_endian
    );
    sut.begin_writing();
    sut.write_integer(0b001, 3);
    sut.write_integer(310, 13);
    sut.finish_writing();
    expect(sut.get_data()).toEqual(expected);
  });

  it("write_legacy_le", () => {
    const expected = new Uint8Array([0b011111_001, 0b11111111]);
    const sut = c_bitstream_writer.new_with_legacy_settings(
      2,
      e_bitstream_byte_order._bitstream_byte_order_little_endian
    );
    sut.begin_writing();
    sut.write_integer(0b001, 3);
    sut.write_integer(8191, 13);
    sut.finish_writing();
    expect(sut.get_data()).toEqual(expected);
  });

  it("write_be", () => {
    const expected = new Uint8Array([0b001_11111, 0b11111111]);
    const sut = c_bitstream_writer.new(
      expected.length,
      e_bitstream_byte_order._bitstream_byte_order_big_endian
    );
    sut.begin_writing();
    sut.write_integer(0b001, 3);
    sut.write_integer(8191, 13);
    sut.finish_writing();
    expect(sut.get_data()).toEqual(expected);
  });

  it("write_le", () => {
    const expected = new Uint8Array([0b001_10000, 0b10000001]);
    const sut = c_bitstream_writer.new(
      expected.length,
      e_bitstream_byte_order._bitstream_byte_order_little_endian
    );
    sut.begin_writing();
    sut.write_integer(0b001, 3);
    sut.write_integer(388, 13);
    sut.finish_writing();
    expect(sut.get_data()).toEqual(expected);
  });

  it("write_with_msb_to_lsb_byte_pack_direction", () => {
    const expected = new Uint8Array([0b00011111]);
    const sut = c_bitstream_writer.new(
      1,
      e_bitstream_byte_order._bitstream_byte_order_big_endian
    );
    sut.begin_writing();
    sut.write_integer(0b000, 3);
    sut.write_integer(0b11111, 5);
    sut.finish_writing();
    expect(sut.get_data()).toEqual(expected);
  });

  it("write_with_lsb_to_msb_byte_pack_direction", () => {
    const expected = new Uint8Array([0b11111000]);
    const sut = c_bitstream_writer.new_with_legacy_settings(
      1,
      e_bitstream_byte_order._bitstream_byte_order_big_endian
    );
    sut.begin_writing();
    sut.write_integer(0b000, 3);
    sut.write_integer(0b11111, 5);
    sut.finish_writing();
    expect(sut.get_data()).toEqual(expected);
  });

  it("write_9_at_2", () => {
    const expected = new Uint8Array([0b00000010, 0b01100000]);
    const sut = c_bitstream_writer.new(
      2,
      e_bitstream_byte_order._bitstream_byte_order_big_endian
    );
    sut.begin_writing();
    sut.write_integer(0b00, 2);
    sut.write_integer(19, 9);
    sut.finish_writing();
    expect(sut.get_data()).toEqual(expected);
  });

  it("write_h3_06481_game_set_data", () => {
    const expected = new Uint8Array([
      0b10010100, 0b00000001, 0b00000000, 0b00000000, 0b00000000, 0b10110001,
      0b00001001, 0b00000000, 0b00000000, 0b10010000, 0b10101011, 0b00000_011,
      0,
    ]);
    const sut = c_bitstream_writer.new_with_legacy_settings(
      13,
      e_bitstream_byte_order._bitstream_byte_order_big_endian
    );
    sut.begin_writing();
    sut.write_integer(20, 6);
    sut.write_integer(6, 32);
    sut.write_integer(4, 4);
    sut.write_bool(false);
    sut.write_integer(310, 32);
    sut.write_string_utf8("ru", 3);
    sut.finish_writing();
    expect(sut.get_data()).toEqual(expected);
  });

  it("write_h3_12070_game_set_data", () => {
    const expected = new Uint8Array([
      0b11011100, 0b00000000, 0b00000000, 0b00000000, 0b00000100, 0b01000000,
      0b00000000, 0b00000000, 0b00100000, 0b10000011, 0b01010101, 0b1111_0000,
      0,
    ]);
    const sut = c_bitstream_writer.new(
      13,
      e_bitstream_byte_order._bitstream_byte_order_big_endian
    );
    sut.begin_writing();
    sut.write_integer(55, 6);
    sut.write_integer(1, 32);
    sut.write_integer(1, 4);
    sut.write_bool(false);
    sut.write_bool(false);
    sut.write_integer(520, 32);
    sut.write_string_utf8("5_", 3);
    sut.finish_writing();
    expect(sut.get_data()).toEqual(expected);
  });

  it("write_raw_data skips leading surplus bytes when buffer is oversized (BE)", () => {
    const cap = 0x5000;
    const source = c_bitstream_writer.new(
      cap,
      e_bitstream_byte_order._bitstream_byte_order_big_endian
    );
    source.begin_writing();
    source.write_integer(2, 4);
    const [bytePos] = source.get_current_offset();
    source.write_raw_data(new Uint8Array(cap - bytePos), (cap - bytePos) * 8);
    source.finish_writing();
    const gametype_data = source.get_data();
    expect(gametype_data[0]).toBe(0x20);
    expect(gametype_data.length).toBe(cap + 1);

    const hashable = c_bitstream_writer.new(
      4 + cap,
      e_bitstream_byte_order._bitstream_byte_order_big_endian
    );
    hashable.begin_writing();
    hashable.write_integer(1, 32);
    hashable.write_raw_data(gametype_data, cap * 8);
    hashable.finish_writing();

    expect(hashable.get_data()[4]).not.toBe(gametype_data[0]);
    expect(hashable.get_data()[4]).toBe(gametype_data[1]);
  });

  it("round_trips_be integers", () => {
    const writer = c_bitstream_writer.new(
      4,
      e_bitstream_byte_order._bitstream_byte_order_big_endian
    );
    write_then_read(
      writer,
      (data) =>
        c_bitstream_reader.new(
          data,
          e_bitstream_byte_order._bitstream_byte_order_big_endian
        ),
      (w) => {
        w.write_integer(0b001, 3);
        w.write_integer(8191, 13);
        w.write_bool(true);
        w.write_signed_integer(-3, 8);
      },
      (r) => {
        expect(r.read_integer("a", 3)).toBe(0b001);
        expect(r.read_integer("b", 13)).toBe(8191);
        expect(r.read_bool("c")).toBe(true);
        expect(r.read_signed_integer("d", 8)).toBe(-3);
      }
    );
  });
});
