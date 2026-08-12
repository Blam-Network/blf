import { describe, expect, it } from "vitest";
import { e_bitstream_byte_order } from "./enums";
import { c_bitstream_reader } from "./reader";
import { c_bitstream_writer } from "./writer";

describe("c_bitstream write_big_flags / read_big_flags", () => {
  it("round-trips indices with dword packing", () => {
    const flags = Array.from({ length: 2048 }, () => false);
    for (const i of [48, 49, 76, 141]) {
      flags[i] = true;
    }

    const writer = c_bitstream_writer.new(
      256,
      e_bitstream_byte_order._bitstream_byte_order_big_endian
    );
    writer.begin_writing();
    writer.write_big_flags(flags, "object-types-used");
    writer.finish_writing();

    const reader = c_bitstream_reader.new(
      writer.get_data(),
      e_bitstream_byte_order._bitstream_byte_order_big_endian
    );
    reader.begin_reading();
    const decoded = reader.read_big_flags("object-types-used", 2048);
    expect(decoded.map((v, i) => (v ? i : -1)).filter((i) => i >= 0)).toEqual([
      48, 49, 76, 141,
    ]);
  });

  it("does not look like sequential write_bool within a dword", () => {
    // Sequential bools would place set(0) at stream bit 0; big_flags places it
    // as LSB of the first dword (stream bit 31 under MSB-first packing).
    const flags = Array.from({ length: 32 }, () => false);
    flags[0] = true;

    const big = c_bitstream_writer.new(
      4,
      e_bitstream_byte_order._bitstream_byte_order_big_endian
    );
    big.begin_writing();
    big.write_big_flags(flags, "t");
    big.finish_writing();

    const sequential = c_bitstream_writer.new(
      4,
      e_bitstream_byte_order._bitstream_byte_order_big_endian
    );
    sequential.begin_writing();
    for (const f of flags) {
      sequential.write_bool(f);
    }
    sequential.finish_writing();

    expect(big.get_data()).not.toEqual(sequential.get_data());
  });
});
