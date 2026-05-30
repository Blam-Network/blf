import { describe, expect, it } from "vitest";
import {
  c_bitstream_reader,
  c_bitstream_writer,
  e_bitstream_byte_order,
} from "../../../bitstream";
import { global_up3d } from "../../../bitstream/math";
import { get_unit_vector_encoding_constants } from "../../../blam/common/math/unit_vector_quantization";
import { e_file_type } from "../../../blam/haloreach/v12065_11_08_24_1738_tu1actual/saved_games/saved_game_files";
import {
  c_map_variant,
  s_variant_object_datum,
} from "../../../blam/haloreach/v12065_11_08_24_1738_tu1actual/saved_games/scenario_map_variant";
import { search_for_chunk } from "../../../blf_chunk";
import {
  map_variant_storage_capacity,
  s_blf_chunk_map_variant,
} from "./s_blf_chunk_map_variant";

function create_minimal_map_variant(): c_map_variant {
  const map_variant = new c_map_variant();
  map_variant.m_metadata.general.file_type = e_file_type.MapVariant;
  map_variant.m_metadata.general.size_in_bytes = 0;
  map_variant.m_metadata.general.unique_id = 1n;
  map_variant.m_metadata.general.parent_unique_id = 1n;
  map_variant.m_metadata.general.root_unique_id = 1n;
  map_variant.m_metadata.general.game_id = 0n;
  map_variant.m_metadata.general.activity = 0;
  map_variant.m_metadata.general.game_mode = 0;
  map_variant.m_metadata.general.game_engine_type = 0;
  map_variant.m_metadata.general.map_id = 1020;
  map_variant.m_metadata.display.megalo_category_index = -1;
  map_variant.m_metadata.creation_history.timestamp = new Date(0);
  map_variant.m_metadata.creation_history.xuid = 0n;
  map_variant.m_metadata.creation_history.name = "test";
  map_variant.m_metadata.modification_history.timestamp = new Date(0);
  map_variant.m_metadata.modification_history.xuid = 0n;
  map_variant.m_metadata.modification_history.name = "test";
  map_variant.m_metadata.name = "Test Map";
  map_variant.m_metadata.description = "Test Description";
  map_variant.m_map_variant_version = 31;
  map_variant.m_map_id = 1020;
  map_variant.m_number_of_placeable_object_quotas = 1;
  map_variant.m_world_bounds = {
    x: { lower: -100, upper: 100 },
    y: { lower: -100, upper: 100 },
    z: { lower: -10, upper: 10 },
  };
  map_variant.m_maximum_budget = 2000;
  map_variant.m_spent_budget = 100;
  map_variant.m_string_table.strings = ["slayer"];

  const object = new s_variant_object_datum();
  object.flags = 1;
  object.variant_quota_index = 0;
  object.variant_index = 0;
  object.position = { x: 0, y: 0, z: 0 };
  object.forward = { i: 0, j: 0, k: 1 };
  object.up = { i: 0, j: 1, k: 0 };
  object.spawn_relative_to = -1;
  map_variant.m_variant_objects[0] = object;

  map_variant.m_quotas[0]!.minimum_count = 0;
  map_variant.m_quotas[0]!.maximum_count = 16;
  map_variant.m_quotas[0]!.placed_on_map = 1;

  return map_variant;
}

describe("unit vector encoding constants", () => {
  it("matches blf_lib Reach constants for 20-bit axes", () => {
    const constants = get_unit_vector_encoding_constants(20);
    expect(constants.actual_per_axis_max_count).toBe(0x2aaaa);
    expect(constants.quantized_value_count).toBe(0x1a1);
  });
});

describe("c_map_variant", () => {
  it("round-trips a minimal packed map variant bitstream", () => {
    const original = create_minimal_map_variant();

    const writer = c_bitstream_writer.new(
      0x4000,
      e_bitstream_byte_order._bitstream_byte_order_big_endian
    );
    writer.begin_writing();
    original.encode(writer);
    writer.finish_writing();

    const reader = c_bitstream_reader.new(
      writer.get_data(),
      e_bitstream_byte_order._bitstream_byte_order_big_endian
    );
    reader.begin_reading();
    const roundtrip = new c_map_variant();
    roundtrip.decode(reader);
    reader.finish_reading();

    expect(roundtrip.m_metadata.name).toBe("Test Map");
    expect(roundtrip.m_map_variant_version).toBe(31);
    expect(roundtrip.m_map_id).toBe(1020);
    expect(roundtrip.m_number_of_placeable_object_quotas).toBe(1);
    expect(roundtrip.m_string_table.strings).toEqual(["slayer"]);
    expect(roundtrip.m_variant_objects[0]!.flags).toBe(1);
    expect(roundtrip.m_variant_objects[0]!.position.x).toBeCloseTo(0, 1);
    expect(roundtrip.m_variant_objects[0]!.forward.k).toBeCloseTo(1, 3);
    expect(roundtrip.m_variant_objects[0]!.up.j).toBeCloseTo(1, 3);
    expect(roundtrip.m_quotas[0]!.maximum_count).toBe(16);
  });

  it("preserves global up when encoding default axes", () => {
    const map_variant = create_minimal_map_variant();
    map_variant.m_variant_objects[0]!.up = { ...global_up3d };

    const writer = c_bitstream_writer.new(
      0x4000,
      e_bitstream_byte_order._bitstream_byte_order_big_endian
    );
    writer.begin_writing();
    map_variant.encode(writer);
    writer.finish_writing();

    const reader = c_bitstream_reader.new(
      writer.get_data(),
      e_bitstream_byte_order._bitstream_byte_order_big_endian
    );
    reader.begin_reading();
    const roundtrip = new c_map_variant();
    roundtrip.decode(reader);
    reader.finish_reading();

    expect(roundtrip.m_variant_objects[0]!.up.j).toBeCloseTo(1, 3);
  });
});

describe("s_blf_chunk_map_variant", () => {
  it("writes fixed-size packed storage with zero padding", () => {
    const chunk = new s_blf_chunk_map_variant();
    chunk.map_variant = create_minimal_map_variant();

    const payload = chunk.write_body("big");
    expect(map_variant_storage_capacity).toBe(0x7000);
    expect(payload.length).toBe(20 + 4 + map_variant_storage_capacity);
    expect(chunk.packed_length).toBeLessThan(map_variant_storage_capacity);
    expect(
      payload.subarray(24 + chunk.packed_length).every((b) => b === 0)
    ).toBe(true);
  });

  it("round-trips a minimal mvar chunk payload", () => {
    const original = new s_blf_chunk_map_variant();
    original.map_variant = create_minimal_map_variant();

    const written = original.write_body("big");
    const roundtrip = new s_blf_chunk_map_variant();
    roundtrip.read_body(written, "big");

    expect(roundtrip.hash).toEqual(original.hash);
    expect(roundtrip.packed_length).toBe(original.packed_length);
    expect(roundtrip.map_variant.m_metadata.name).toBe("Test Map");
    expect(roundtrip.map_variant.m_map_id).toBe(1020);
    expect(roundtrip.map_variant.m_variant_objects[0]!.flags).toBe(1);
  });

  it("writes a full BLF chunk that can be found via search_for_chunk", () => {
    const chunk = new s_blf_chunk_map_variant();
    chunk.map_variant = create_minimal_map_variant();

    const blf_bytes = chunk.write("big");
    const found = new s_blf_chunk_map_variant();
    expect(search_for_chunk(blf_bytes, found, "big")).toBe(true);
    expect(found.map_variant.m_metadata.name).toBe("Test Map");
  });
});
