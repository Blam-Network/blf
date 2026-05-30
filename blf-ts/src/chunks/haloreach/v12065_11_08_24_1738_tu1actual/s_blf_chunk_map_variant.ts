import type { c as CStruct } from "@craftycodie/cstruct";
import {
  c_bitstream_reader,
  c_bitstream_writer,
  e_bitstream_byte_order,
} from "../../../bitstream";
import { security_calculate_hash } from "../../../blam/common/cache/security_functions";
import { c_map_variant } from "../../../blam/haloreach/v12065_11_08_24_1738_tu1actual/saved_games/scenario_map_variant";
import { BLFChunkBase, blf } from "../../../blf_chunk";
import { BlfError } from "../../../error";

/** Reach map variant chunk packed bitstream storage (bytes). */
export const map_variant_storage_capacity = 0x7000;

/**
 * Reach map variant chunk (`mvar` 31.1).
 *
 * Payload layout: SHA1 hash + BE `packed_length` + 0x7000 packed storage + 4 zero pad.
 * Hash covers only `packed_length` (4 BE bytes) + the packed bitstream bytes (not 0x7000
 * storage slack or the 4-byte tail pad).
 */
@blf.chunk("mvar", 31.1)
export class s_blf_chunk_map_variant extends BLFChunkBase {
  hash: Uint8Array = new Uint8Array(20);
  packed_length = 0;
  map_variant = new c_map_variant();

  static create(map_variant: c_map_variant): s_blf_chunk_map_variant {
    const chunk = new s_blf_chunk_map_variant();
    chunk.map_variant = map_variant;
    return chunk;
  }

  read_body(payload: Uint8Array, _endian: CStruct.Endian): void {
    if (payload.length < 24) {
      throw new BlfError("mvar chunk payload is too short");
    }

    this.hash = payload.subarray(0, 20);
    this.packed_length = new DataView(
      payload.buffer,
      payload.byteOffset + 20,
      4
    ).getUint32(0, false);

    const packed_on_disk = payload.subarray(24);
    const decode_length =
      this.packed_length > 0
        ? Math.min(
            this.packed_length,
            map_variant_storage_capacity,
            packed_on_disk.length
          )
        : Math.min(map_variant_storage_capacity, packed_on_disk.length);
    const packed_data = packed_on_disk.subarray(0, decode_length);
    const bitstream = c_bitstream_reader.new(
      packed_data,
      e_bitstream_byte_order._bitstream_byte_order_big_endian
    );
    bitstream.begin_reading();

    this.map_variant = new c_map_variant();
    this.map_variant.decode(bitstream);
    bitstream.finish_reading();
  }

  write_body(_endian: CStruct.Endian): Uint8Array {
    const bitstream = c_bitstream_writer.new(
      map_variant_storage_capacity,
      e_bitstream_byte_order._bitstream_byte_order_big_endian
    );
    bitstream.begin_writing();
    this.map_variant.encode(bitstream);
    bitstream.finish_writing();
    const packed_data = bitstream.get_data();
    if (packed_data.length > map_variant_storage_capacity) {
      throw new BlfError(
        `packed map variant exceeds storage capacity (${packed_data.length} > ${map_variant_storage_capacity})`
      );
    }

    const packed_storage = new Uint8Array(map_variant_storage_capacity);
    packed_storage.set(packed_data, 0);

    const length_bytes = new Uint8Array(4);
    new DataView(length_bytes.buffer).setUint32(0, packed_data.length, false);

    const hashable = new Uint8Array(4 + packed_data.length);
    hashable.set(length_bytes, 0);
    hashable.set(packed_data, 4);
    const hash = security_calculate_hash(hashable);

    const payload = new Uint8Array(20 + 4 + map_variant_storage_capacity + 4);
    payload.set(hash, 0);
    payload.set(length_bytes, 20);
    payload.set(packed_storage, 24);

    this.hash = hash;
    this.packed_length = packed_data.length;

    return payload;
  }
}
