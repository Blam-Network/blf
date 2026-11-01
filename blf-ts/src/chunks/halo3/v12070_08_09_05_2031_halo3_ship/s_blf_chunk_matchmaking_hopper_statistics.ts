import { c } from "@craftycodie/cstruct";
import { BLFChunkBase, blf } from "../../../blf_chunk";
import { write_blffile } from "../../../blffile";
import { BlfError } from "../../../error";
import { s_blf_chunk_end_of_file } from "./s_blf_chunk_end_of_file";
import { s_blf_chunk_start_of_file } from "./s_blf_chunk_start_of_file";

const k_max_hoppers = 32;

/** Per-hopper population entry in `mmhs` 3.1 (8 bytes). */
export const hopper_population = c.struct({
  hopper_identifier: c.i16().padBefore(2),
  player_count: c.u32(),
});
export type hopper_population = c.infer<typeof hopper_population>;

const s_matchmaking_hopper_statistics_header = c.struct({
  player_count: c.u32(),
});

const s_matchmaking_hopper_statistics_body =
  s_matchmaking_hopper_statistics_header.extend({
    data: hopper_population.array(k_max_hoppers),
  });

/**
 * Halo 3 matchmaking hopper statistics (`mmhs` 3.1, 0x104 bytes).
 * Holds up to 32 hoppers; fewer are padded with empty entries on write
 * (as blf_lib's `StaticArray` does). Reading always yields 32 entries.
 */
@blf.chunk("mmhs", 3.1)
export class s_blf_chunk_matchmaking_hopper_statistics extends BLFChunkBase {
  player_count = 0;
  data: hopper_population[] = [];

  read_body(payload: Uint8Array, endian: c.Endian): void {
    const size = s_matchmaking_hopper_statistics_body.getSize();
    if (payload.length < size) {
      throw new BlfError(
        `mmhs chunk payload too short: need ${size} bytes, got ${payload.length}`
      );
    }
    const body = s_matchmaking_hopper_statistics_body.read(payload, endian);
    this.player_count = body.player_count;
    this.data = body.data;
  }

  write_body(endian: c.Endian): Uint8Array {
    if (this.data.length > k_max_hoppers) {
      throw new BlfError(
        `mmhs holds at most ${k_max_hoppers} hoppers, got ${this.data.length}`
      );
    }
    const out = new Uint8Array(s_matchmaking_hopper_statistics_body.getSize());
    s_matchmaking_hopper_statistics_header.writeInto(
      out,
      { player_count: this.player_count },
      0,
      endian
    );
    const header_size = s_matchmaking_hopper_statistics_header.getSize();
    const entry_size = hopper_population.getSize();
    this.data.forEach((hopper, i) => {
      hopper_population.writeInto(
        out,
        hopper,
        header_size + i * entry_size,
        endian
      );
    });
    return out;
  }
}

/** Halo 3 hopper statistics file: `_blf` (unnamed), `mmhs`, `_eof`. */
export function build_hopper_statistics_file(mmhs: {
  player_count: number;
  data: hopper_population[];
}): Uint8Array {
  const chunk = new s_blf_chunk_matchmaking_hopper_statistics();
  chunk.player_count = mmhs.player_count;
  chunk.data = mmhs.data;
  return write_blffile("big", [
    new s_blf_chunk_start_of_file(),
    chunk,
    new s_blf_chunk_end_of_file(),
  ]);
}
