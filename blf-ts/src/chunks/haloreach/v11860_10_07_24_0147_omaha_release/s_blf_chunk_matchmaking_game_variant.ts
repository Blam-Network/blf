import type { c } from "@craftycodie/cstruct";
import {
  c_bitstream_reader,
  c_bitstream_writer,
  e_bitstream_byte_order,
} from "../../../bitstream";
import { c_game_variant } from "../../../blam/haloreach/v11860_10_07_24_0147_omaha_release/game/game_variant";
import { BLFChunkBase, blf } from "../../../blf_chunk";
import { BlfError } from "../../../error";

/**
 * Reach matchmaking game variant chunk (`gvar` 54.1).
 *
 * Payload is the gametype bitstream only (no mpvr hash/header wrapper).
 */
@blf.chunk("gvar", 54.1)
export class s_blf_chunk_matchmaking_game_variant extends BLFChunkBase {
  game_variant = new c_game_variant();

  static create(
    game_variant: c_game_variant
  ): s_blf_chunk_matchmaking_game_variant {
    const chunk = new s_blf_chunk_matchmaking_game_variant();
    chunk.game_variant = game_variant;
    return chunk;
  }

  read_body(payload: Uint8Array, _endian: c.Endian): void {
    if (payload.length === 0) {
      throw new BlfError("gvar chunk payload is empty");
    }

    const bitstream = c_bitstream_reader.new(
      payload,
      e_bitstream_byte_order._bitstream_byte_order_big_endian
    );
    bitstream.begin_reading();
    this.game_variant = new c_game_variant();
    this.game_variant.decode(bitstream);
    bitstream.finish_reading();
  }

  write_body(): Uint8Array {
    const bitstream = c_bitstream_writer.new(
      0x5028,
      e_bitstream_byte_order._bitstream_byte_order_big_endian
    );
    bitstream.begin_writing();
    this.game_variant.encode(bitstream);
    bitstream.finish_writing();
    return bitstream.get_data();
  }
}
