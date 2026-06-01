import { c } from "@craftycodie/cstruct";
import {
  c_bitstream_reader,
  c_bitstream_writer,
  e_bitstream_byte_order,
} from "../../../bitstream";
import { BLFChunkBase, blf } from "../../../blf_chunk";
import { write_blffile } from "../../../blffile";
import { BlfError } from "../../../error";
import { s_blf_chunk_end_of_file } from "../../halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_end_of_file";
import { s_blf_chunk_start_of_file } from "../../halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_start_of_file";

const MMHS_HEADER_SIZE = 14;

function bitstream_byte_order(endian: c.Endian): e_bitstream_byte_order {
  return endian === "little"
    ? e_bitstream_byte_order._bitstream_byte_order_little_endian
    : e_bitstream_byte_order._bitstream_byte_order_big_endian;
}

/** Per-hopper population entry in `mmhs` 4.1 (`hopper_population`). */
@c.struct()
export class s_online_population_statistic {
  @c.field("u8", { pad_after: 1 })
  presence_type = 0;

  @c.field("i16")
  hopper_identifier = 0;

  @c.field("u32")
  player_count = 0;
}

/** Reach matchmaking hopper statistics (`mmhs` 4.1). */
@blf.chunk("mmhs", 4.1)
export class s_blf_chunk_matchmaking_hopper_statistics extends BLFChunkBase {
  total_population = 0;
  unknown_population_2 = 0;
  unknown_population_3 = 0;
  playlist_count = 0;
  hoppers: s_online_population_statistic[] = [];

  read_body(payload: Uint8Array, endian: c.Endian): void {
    if (payload.length < MMHS_HEADER_SIZE) {
      throw new BlfError(
        `mmhs chunk payload too short: need at least ${MMHS_HEADER_SIZE} bytes, got ${payload.length}`
      );
    }

    const bitstream = c_bitstream_reader.new(
      payload,
      bitstream_byte_order(endian)
    );
    bitstream.begin_reading();
    this.total_population = bitstream.read_integer("total_population", 32);
    this.unknown_population_2 = bitstream.read_integer(
      "unknown_population_2",
      32
    );
    this.unknown_population_3 = bitstream.read_integer(
      "unknown_population_3",
      32
    );
    this.playlist_count = bitstream.read_integer("playlist_count", 16);
    bitstream.finish_reading();

    const entry_size = c.sizeof(s_online_population_statistic);
    const expected_size = MMHS_HEADER_SIZE + this.playlist_count * entry_size;
    if (payload.length < expected_size) {
      throw new BlfError(
        `mmhs chunk payload too short for ${this.playlist_count} hoppers: need ${expected_size} bytes, got ${payload.length}`
      );
    }

    this.hoppers = [];
    for (let i = 0; i < this.playlist_count; i++) {
      const offset = MMHS_HEADER_SIZE + i * entry_size;
      this.hoppers.push(
        c.read(
          s_online_population_statistic,
          payload.subarray(offset, offset + entry_size),
          endian
        )
      );
    }
  }

  write_body(endian: c.Endian): Uint8Array {
    const playlist_count = this.hoppers.length;
    const entry_size = c.sizeof(s_online_population_statistic);
    const payload_size = MMHS_HEADER_SIZE + playlist_count * entry_size;
    const byte_order = bitstream_byte_order(endian);

    const bitstream = c_bitstream_writer.new(payload_size, byte_order);
    bitstream.begin_writing();
    bitstream.write_integer(this.total_population, 32);
    bitstream.write_integer(this.unknown_population_2, 32);
    bitstream.write_integer(this.unknown_population_3, 32);
    bitstream.write_integer(playlist_count, 16);

    for (const hopper of this.hoppers) {
      bitstream.write_raw_data(
        c.write(s_online_population_statistic, hopper, endian),
        entry_size * 8
      );
    }

    bitstream.finish_writing();
    this.playlist_count = playlist_count;
    return bitstream.get_data();
  }
}

/** BLF file with `_blf`, `mmhs`, and `_eof` for Reach title storage. */
export function build_hopper_statistics_file(
  mmhs: s_blf_chunk_matchmaking_hopper_statistics
): Uint8Array {
  return write_blffile("big", [
    s_blf_chunk_start_of_file.create("hopper-statistics"),
    mmhs,
    new s_blf_chunk_end_of_file(),
  ]);
}

export { s_online_population_statistic as hopper_population };
