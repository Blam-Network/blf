import { describe, expect, it } from "vitest";
import { e_bitstream_byte_order } from "./enums";
import { BitstreamError } from "./errors";
import { c_bitstream_reader } from "./reader";

enum e_sample {
  a = 1,
  b = 2,
}

describe("c_bitstream_reader", () => {
  it("read_be", () => {
    const test_data = new Uint8Array([0b00111111, 0b11111111]);

    const sut = c_bitstream_reader.new(
      test_data,
      e_bitstream_byte_order._bitstream_byte_order_big_endian
    );
    sut.begin_reading();

    expect(sut.read_integer("test-1", 3)).toBe(0b001);
    expect(sut.read_integer("test-2", 13)).toBe(8191);
  });

  it("read_le", () => {
    const test_data = new Uint8Array([0b00111111, 0b11111111]);

    const sut = c_bitstream_reader.new(
      test_data,
      e_bitstream_byte_order._bitstream_byte_order_little_endian
    );
    sut.begin_reading();

    expect(sut.read_integer("test-1", 3)).toBe(0b001);
    expect(sut.read_integer("test-2", 13)).toBe(8191);
  });

  it("read_legacy_be", () => {
    const test_data = new Uint8Array([0b10110001, 0b00001001]);

    const sut = c_bitstream_reader.new_with_legacy_settings(
      test_data,
      e_bitstream_byte_order._bitstream_byte_order_big_endian
    );
    sut.begin_reading();

    expect(sut.read_integer("test-1", 3)).toBe(0b001);
    expect(sut.read_integer("test-2", 13)).toBe(310);
  });

  it("read_legacy_le", () => {
    const test_data = new Uint8Array([0b01001001, 0b10110000]);

    const sut = c_bitstream_reader.new_with_legacy_settings(
      test_data,
      e_bitstream_byte_order._bitstream_byte_order_little_endian
    );
    sut.begin_reading();

    expect(sut.read_integer("test-1", 3)).toBe(0b001);
    expect(sut.read_integer("test-2", 13)).toBe(310);
  });

  it("read_with_msb_to_lsb_byte_pack_direction", () => {
    const test_data = new Uint8Array([0b00011111]);

    const sut = c_bitstream_reader.new(
      test_data,
      e_bitstream_byte_order._bitstream_byte_order_big_endian
    );
    sut.begin_reading();

    expect(sut.read_integer("test-1", 3)).toBe(0b000);
    expect(sut.read_integer("test-2", 5)).toBe(0b11111);
  });

  it("read_with_lsb_to_msb_byte_pack_direction", () => {
    const test_data = new Uint8Array([0b10010100]);

    const sut = c_bitstream_reader.new_with_legacy_settings(
      test_data,
      e_bitstream_byte_order._bitstream_byte_order_big_endian
    );
    sut.begin_reading();

    expect(sut.read_integer("test-1", 6)).toBe(20);
    expect(sut.read_integer("test-2", 2)).toBe(0b10);
  });

  it("read_h3_06481_game_set_data", () => {
    const test_data = new Uint8Array([
      0b10010100, 0b00000001, 0b00000000, 0b00000000, 0b00000000, 0b10110001,
      0b00001001, 0b00000000, 0b00000000, 0b10010000, 0b10101011, 0b00000011, 0,
    ]);

    const sut = c_bitstream_reader.new_with_legacy_settings(
      test_data,
      e_bitstream_byte_order._bitstream_byte_order_big_endian
    );
    sut.begin_reading();

    expect(sut.read_integer("test-1", 6)).toBe(20);
    expect(sut.read_integer("test-2", 32)).toBe(6);
    expect(sut.read_integer("test-3", 4)).toBe(4);
    expect(sut.read_bool("test-4")).toBe(false);
    expect(sut.read_integer("test-5", 32)).toBe(310);
    expect(sut.read_string_utf8(3)).toBe("ru");
  });

  it("read_h3_12070_game_set_data", () => {
    const test_data = new Uint8Array([
      0b11011100, 0b00000000, 0b00000000, 0b00000000, 0b00000100, 0b01000000,
      0b00000000, 0b00000000, 0b00100000, 0b10000011, 0b01010101, 0b11110000, 0,
    ]);

    const sut = c_bitstream_reader.new(
      test_data,
      e_bitstream_byte_order._bitstream_byte_order_big_endian
    );
    sut.begin_reading();

    expect(sut.read_integer("test-1", 6)).toBe(55);
    expect(sut.read_integer("test-2", 32)).toBe(1);
    expect(sut.read_integer("test-3", 4)).toBe(1);
    expect(sut.read_bool("test-4")).toBe(false);
    expect(sut.read_bool("test-5")).toBe(false);
    expect(sut.read_integer("test-6", 32)).toBe(520);
    expect(sut.read_string_utf8(3)).toBe("5_");
  });

  it("read_enum rejects undeclared members by default", () => {
    const sut = c_bitstream_reader.new(
      new Uint8Array([0b10000000]),
      e_bitstream_byte_order._bitstream_byte_order_big_endian
    );
    sut.begin_reading();
    expect(() => sut.read_enum("mode", 4, e_sample)).toThrow(BitstreamError);
  });

  it("read_enum maps wire index to enum member", () => {
    const sut = c_bitstream_reader.new(
      new Uint8Array([0b00010000]),
      e_bitstream_byte_order._bitstream_byte_order_big_endian
    );
    sut.begin_reading();
    expect(sut.read_enum("mode", 4, e_sample)).toBe(e_sample.b);
  });
});
