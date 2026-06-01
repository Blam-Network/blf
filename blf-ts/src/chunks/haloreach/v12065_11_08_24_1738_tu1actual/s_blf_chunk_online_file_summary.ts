import { c } from "@craftycodie/cstruct";
import {
  c_bitstream_reader,
  c_bitstream_writer,
  e_bitstream_byte_order,
} from "../../../bitstream";
import { s_online_file_summary_listing_entry } from "../../../blam/haloreach/v12065_11_08_24_1738_tu1actual/networking/online/files/online_file_summary_listing";
import { BLFChunkBase, blf } from "../../../blf_chunk";
import { BlfError } from "../../../error";

/** Reach online file summary chunk (`finf` 1.0). */
@blf.chunk("finf", 1.0)
export class s_blf_chunk_online_file_summary extends BLFChunkBase {
  entry_count = 0;
  entries: s_online_file_summary_listing_entry[] = [];

  read_body(payload: Uint8Array, endian: c.Endian): void {
    if (payload.length < 4) {
      throw new BlfError(
        `finf chunk payload too short: need at least 4 bytes, got ${payload.length}`
      );
    }

    const bitstream = c_bitstream_reader.new(
      payload,
      endian === "little"
        ? e_bitstream_byte_order._bitstream_byte_order_little_endian
        : e_bitstream_byte_order._bitstream_byte_order_big_endian
    );
    bitstream.begin_reading();
    this.entry_count = bitstream.read_integer("entry_count", 16);
    bitstream.seek_relative(16);
    bitstream.finish_reading();

    const entry_size = c.sizeof(s_online_file_summary_listing_entry);
    const expected_size = 4 + this.entry_count * entry_size;
    if (payload.length < expected_size) {
      throw new BlfError(
        `finf chunk payload too short for ${this.entry_count} entries: need ${expected_size} bytes, got ${payload.length}`
      );
    }

    this.entries = [];
    for (let i = 0; i < this.entry_count; i++) {
      const offset = 4 + i * entry_size;
      this.entries.push(
        c.read(
          s_online_file_summary_listing_entry,
          payload.subarray(offset, offset + entry_size),
          endian
        )
      );
    }
  }

  write_body(endian: c.Endian): Uint8Array {
    const entry_count = this.entries.length;
    const entry_size = c.sizeof(s_online_file_summary_listing_entry);
    const payload_size = 4 + entry_count * entry_size;

    const bitstream = c_bitstream_writer.new(
      payload_size,
      endian === "little"
        ? e_bitstream_byte_order._bitstream_byte_order_little_endian
        : e_bitstream_byte_order._bitstream_byte_order_big_endian
    );
    bitstream.begin_writing();
    bitstream.write_integer(entry_count, 16);
    bitstream.seek_relative(16);

    for (const entry of this.entries) {
      bitstream.write_raw_data(
        c.write(s_online_file_summary_listing_entry, entry, endian),
        entry_size * 8
      );
    }

    bitstream.finish_writing();
    this.entry_count = entry_count;
    return bitstream.get_data();
  }
}
