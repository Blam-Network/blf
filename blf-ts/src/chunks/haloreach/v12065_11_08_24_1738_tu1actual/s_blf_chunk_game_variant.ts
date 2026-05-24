import type { c } from "@craftycodie/cstruct";
import {
  c_bitstream_reader,
  c_bitstream_writer,
  e_bitstream_byte_order,
} from "../../../bitstream";
import { security_calculate_hash } from "../../../blam/common/cache/security_functions";
import { c_game_variant } from "../../../blam/haloreach/v12065_11_08_24_1738_tu1actual/game/c_game_variant";
import { BLFChunkBase, blf } from "../../../blf_chunk";
import { BlfError } from "../../../error";

/** Gametype storage slot size (bytes). */
export const variant_storage_capacity = 0x5000;

/** mpvr body size: hash/header (28) + padded gametype slot. */
export const MPVR_PAYLOAD_SIZE = 28 + variant_storage_capacity;

/**
 * Reach game variant chunk (`mpvr` 54.1).
 *
 * Payload layout matches blf_lib: hash + header fields + gametype bitstream,
 * all read through one big-endian bitstream.
 */
@blf.chunk("mpvr", 54.1)
export class s_blf_chunk_game_variant extends BLFChunkBase {
  hash: Uint8Array = new Uint8Array(20);
  unknown04 = -1;
  unknown06 = 0;
  variant_length = 0;
  game_variant = new c_game_variant();

  static create(game_variant: c_game_variant): s_blf_chunk_game_variant {
    const chunk = new s_blf_chunk_game_variant();
    chunk.game_variant = game_variant;
    return chunk;
  }

  read_body(payload: Uint8Array, endian: c.Endian): void {
    if (payload.length === 0) {
      throw new BlfError("mpvr chunk payload is empty");
    }

    const byte_order =
      endian === "big"
        ? e_bitstream_byte_order._bitstream_byte_order_big_endian
        : e_bitstream_byte_order._bitstream_byte_order_little_endian;

    const bitstream = c_bitstream_reader.new(payload, byte_order);
    bitstream.begin_reading();

    this.hash = bitstream.read_raw_data(20 * 8);
    this.unknown04 = bitstream.read_signed_integer("unknown04", 16);
    this.unknown06 = bitstream.read_integer("unknown06", 16);
    this.variant_length = bitstream.read_integer("variant-length", 32);

    this.game_variant = new c_game_variant();
    this.game_variant.decode(bitstream);

    bitstream.finish_reading();
  }

  write_body(endian: c.Endian): Uint8Array {
    const byte_order =
      endian === "big"
        ? e_bitstream_byte_order._bitstream_byte_order_big_endian
        : e_bitstream_byte_order._bitstream_byte_order_little_endian;

    const gametype_bitstream = c_bitstream_writer.new(
      variant_storage_capacity,
      byte_order
    );
    gametype_bitstream.begin_writing();
    this.game_variant.encode(gametype_bitstream);
    gametype_bitstream.finish_writing();
    const gametype_data = gametype_bitstream.get_data();
    const gametype_length = gametype_data.length;

    if (gametype_length > variant_storage_capacity) {
      throw new BlfError(
        `mpvr gametype too large: ${gametype_length} bytes (max ${variant_storage_capacity})`
      );
    }

    const hashable_bitstream = c_bitstream_writer.new(
      4 + gametype_length,
      byte_order
    );
    hashable_bitstream.begin_writing();
    hashable_bitstream.write_integer(gametype_length, 32);
    hashable_bitstream.write_raw_data(gametype_data, gametype_length * 8);
    hashable_bitstream.finish_writing();
    const hash = security_calculate_hash(hashable_bitstream.get_data());

    const payload_bitstream = c_bitstream_writer.new(
      variant_storage_capacity,
      byte_order
    );
    payload_bitstream.begin_writing();
    payload_bitstream.write_raw_data(hash, hash.length * 8);
    payload_bitstream.write_signed_integer(this.unknown04, 16);
    payload_bitstream.write_integer(this.unknown06, 16);
    payload_bitstream.write_integer(gametype_length, 32);
    payload_bitstream.write_raw_data(gametype_data, gametype_length * 8);

    const pad_bytes = variant_storage_capacity - gametype_length;
    if (pad_bytes > 0) {
      payload_bitstream.seek_relative(pad_bytes * 8);
    }

    payload_bitstream.finish_writing();

    this.hash = hash;
    this.variant_length = gametype_length;

    return payload_bitstream.get_data();
  }
}
