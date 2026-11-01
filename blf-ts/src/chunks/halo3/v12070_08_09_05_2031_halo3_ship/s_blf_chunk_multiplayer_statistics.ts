import type { c } from "@craftycodie/cstruct";
import { BLFChunkBase, blf } from "../../../blf_chunk";
import { write_blffile } from "../../../blffile";
import { BlfError } from "../../../error";
import { read_blffile } from "../../../read_blffile";
import { s_blf_chunk_multiplayer_player_statistics } from "./s_blf_chunk_multiplayer_player_statistics";
import { s_blf_chunk_multiplayer_player_vs_player_statistics } from "./s_blf_chunk_multiplayer_player_vs_player_statistics";
import { s_blf_chunk_multiplayer_team_statistics } from "./s_blf_chunk_multiplayer_team_statistics";

/**
 * Halo 3 game results statistics (`_par` 1.1, 0x4C14 bytes): a chunk count,
 * then the nested `mps1`, `mps2` and `mps3` chunks, in that order.
 */
@blf.chunk("_par", 1.1)
export class s_blf_chunk_multiplayer_statistics extends BLFChunkBase {
  mps1 = new s_blf_chunk_multiplayer_player_statistics();
  mps2 = new s_blf_chunk_multiplayer_player_vs_player_statistics();
  mps3 = new s_blf_chunk_multiplayer_team_statistics();

  read_body(payload: Uint8Array, endian: c.Endian): void {
    if (payload.length < 4) {
      throw new BlfError(
        `Cannot read _par chunk: need at least 4 bytes, got ${payload.length}`
      );
    }
    // Skip the chunk count, as blf_lib does.
    read_blffile(payload.subarray(4), endian, [
      this.mps1,
      this.mps2,
      this.mps3,
    ]);
  }

  write_body(endian: c.Endian): Uint8Array {
    const chunks = write_blffile(endian, [this.mps1, this.mps2, this.mps3]);
    const out = new Uint8Array(4 + chunks.length);
    new DataView(out.buffer).setUint32(0, 3, false);
    out.set(chunks, 4);
    return out;
  }
}
