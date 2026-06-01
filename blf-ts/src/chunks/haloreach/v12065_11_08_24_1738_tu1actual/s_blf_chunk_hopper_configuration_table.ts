import { c } from "@craftycodie/cstruct";
import { inflate } from "pako";
import {
  c_bitstream_reader,
  c_bitstream_writer,
  e_bitstream_byte_order,
} from "../../../bitstream";
import { zlib_compress } from "../../../blam/common/memory/data_compress";
import { BLFChunkBase, blf } from "../../../blf_chunk";
import { write_blffile } from "../../../blffile";
import { BlfError } from "../../../error";
import { s_blf_chunk_end_of_file } from "../../halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_end_of_file";
import { s_blf_chunk_start_of_file } from "../../halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_start_of_file";
import {
  c_hopper_configuration,
  s_game_hopper_custom_category,
} from "./hopper_configuration";

export const k_hopper_maximum_category_count = 16;
export const k_hopper_maximum_hopper_count = 32;

const MHCF_PACKED_CAPACITY = 0x8f48;
const MHCF_INNER_ENDIAN = "big" as const;
const BE_BITSTREAM = e_bitstream_byte_order._bitstream_byte_order_big_endian;

function pad_fixed_array<T>(
  items: T[],
  capacity: number,
  create_default: () => T
): T[] {
  if (items.length > capacity) {
    throw new BlfError(
      `Expected at most ${capacity} elements but got ${items.length}`
    );
  }
  const out = items.slice();
  while (out.length < capacity) {
    out.push(create_default());
  }
  return out;
}

function encode_hopper_table_inner(
  hopper_categories: s_game_hopper_custom_category[],
  hopper_configurations: c_hopper_configuration[]
): Uint8Array {
  const category_size = c.sizeof(s_game_hopper_custom_category);
  const configuration_size = c.sizeof(c_hopper_configuration);
  const encoded = new Uint8Array(
    8 +
      k_hopper_maximum_category_count * category_size +
      k_hopper_maximum_hopper_count * configuration_size
  );
  const view = new DataView(
    encoded.buffer,
    encoded.byteOffset,
    encoded.byteLength
  );
  view.setUint32(0, hopper_configurations.length, false);
  view.setUint32(4, hopper_categories.length, false);

  let offset = 8;
  for (const category of pad_fixed_array(
    hopper_categories,
    k_hopper_maximum_category_count,
    () => new s_game_hopper_custom_category()
  )) {
    encoded.set(
      c.write(s_game_hopper_custom_category, category, MHCF_INNER_ENDIAN),
      offset
    );
    offset += category_size;
  }

  for (const configuration of pad_fixed_array(
    hopper_configurations,
    k_hopper_maximum_hopper_count,
    () => new c_hopper_configuration()
  )) {
    encoded.set(
      c.write(c_hopper_configuration, configuration, MHCF_INNER_ENDIAN),
      offset
    );
    offset += configuration_size;
  }

  return encoded;
}

function decode_hopper_table_inner(data: Uint8Array): {
  hopper_categories: s_game_hopper_custom_category[];
  hopper_configurations: c_hopper_configuration[];
} {
  if (data.length < 8) {
    throw new BlfError(
      `mhcf decompressed table too short: need at least 8 bytes, got ${data.length}`
    );
  }

  const view = new DataView(data.buffer, data.byteOffset, data.byteLength);
  const hopper_configuration_count = view.getUint32(0, false);
  const hopper_category_count = view.getUint32(4, false);

  const category_size = c.sizeof(s_game_hopper_custom_category);
  const configuration_size = c.sizeof(c_hopper_configuration);
  const expected_size =
    8 +
    k_hopper_maximum_category_count * category_size +
    hopper_configuration_count * configuration_size;

  if (data.length < expected_size) {
    throw new BlfError(
      `mhcf decompressed table too short for ${hopper_configuration_count} configurations: need ${expected_size} bytes, got ${data.length}`
    );
  }

  let offset = 8;
  const hopper_categories: s_game_hopper_custom_category[] = [];
  for (let i = 0; i < k_hopper_maximum_category_count; i++) {
    const category = c.read(
      s_game_hopper_custom_category,
      data.subarray(offset, offset + category_size),
      MHCF_INNER_ENDIAN
    );
    offset += category_size;
    if (i < hopper_category_count) {
      hopper_categories.push(category);
    }
  }

  const hopper_configurations: c_hopper_configuration[] = [];
  for (let i = 0; i < hopper_configuration_count; i++) {
    hopper_configurations.push(
      c.read(
        c_hopper_configuration,
        data.subarray(offset, offset + configuration_size),
        MHCF_INNER_ENDIAN
      )
    );
    offset += configuration_size;
  }

  return { hopper_categories, hopper_configurations };
}

/**
 * Reach matchmaking hopper configuration table (`mhcf` 27.1).
 *
 * Payload is bitstream-packed and zlib-compressed; inner records are big-endian.
 */
@blf.chunk("mhcf", 27.1)
export class s_blf_chunk_hopper_configuration_table extends BLFChunkBase {
  hopper_categories: s_game_hopper_custom_category[] = [];
  hopper_configurations: c_hopper_configuration[] = [];

  read_body(_payload: Uint8Array, _endian: c.Endian): void {
    const bitstream = c_bitstream_reader.new(_payload, BE_BITSTREAM);
    bitstream.begin_reading();

    const compressed_length =
      bitstream.read_integer("compressed_length", 14) - 4;
    const decompressed_length = bitstream.read_integer(
      "decompressed_length",
      32
    );
    const compressed = bitstream.read_raw_data(compressed_length * 8);
    bitstream.finish_reading();

    const decompressed = inflate(compressed);
    if (decompressed.length !== decompressed_length) {
      throw new BlfError(
        `mhcf zlib decompressed ${decompressed.length} bytes, expected ${decompressed_length}`
      );
    }

    const table = decode_hopper_table_inner(decompressed);
    this.hopper_categories = table.hopper_categories;
    this.hopper_configurations = table.hopper_configurations;
  }

  write_body(_endian: c.Endian): Uint8Array {
    const encoded = encode_hopper_table_inner(
      this.hopper_categories,
      this.hopper_configurations
    );
    const compressed = zlib_compress(encoded);
    const compressed_length = compressed.length;

    const bitstream = c_bitstream_writer.new(
      MHCF_PACKED_CAPACITY,
      BE_BITSTREAM
    );
    bitstream.begin_writing();
    bitstream.write_integer(compressed_length + 4, 14);
    bitstream.write_integer(encoded.length, 32);
    bitstream.write_raw_data(compressed, compressed_length * 8);
    bitstream.finish_writing();

    return bitstream.get_data();
  }
}

/** BLF with `_blf`, `mhcf`, and `_eof` for Reach title-storage hopper tables. */
export function build_hopper_configuration_file(
  mhcf: s_blf_chunk_hopper_configuration_table
): Uint8Array {
  return write_blffile("big", [
    s_blf_chunk_start_of_file.create("hopper-configuration"),
    mhcf,
    new s_blf_chunk_end_of_file(),
  ]);
}

export {
  c_hopper_configuration,
  s_game_hopper_custom_category,
  s_hopper_configuration_per_team_data,
  s_hopper_jackpot_configuration,
  s_hopper_query_configuration,
  s_hopper_query_latency_desirability_configuration,
  s_hopper_voting_configuration,
} from "./hopper_configuration";
