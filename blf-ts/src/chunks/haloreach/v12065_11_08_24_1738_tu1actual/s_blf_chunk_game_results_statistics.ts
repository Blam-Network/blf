import type { c } from "@craftycodie/cstruct";
import { BLFChunkBase, blf } from "../../../blf_chunk";
import { BlfError } from "../../../error";
import { s_blf_chunk_game_results_player_statistics } from "./s_blf_chunk_game_results_player_statistics";
import { s_blf_chunk_game_results_player_vs_ai_statistics } from "./s_blf_chunk_game_results_player_vs_ai_statistics";
import { s_blf_chunk_game_results_player_vs_player_statistics } from "./s_blf_chunk_game_results_player_vs_player_statistics";
import { s_blf_chunk_game_results_team_statistics } from "./s_blf_chunk_game_results_team_statistics";

const BLF_HEADER_SIZE = 12;

function read_child(
  payload: Uint8Array,
  offset: number,
  chunk:
    | s_blf_chunk_game_results_player_statistics
    | s_blf_chunk_game_results_player_vs_player_statistics
    | s_blf_chunk_game_results_player_vs_ai_statistics
    | s_blf_chunk_game_results_team_statistics,
  endian: c.Endian
): number {
  if (offset + BLF_HEADER_SIZE > payload.length) {
    throw new BlfError(`_par chunk truncated at child offset ${offset}`);
  }
  const view = new DataView(
    payload.buffer,
    payload.byteOffset,
    payload.byteLength
  );
  const chunk_length = view.getUint32(offset + 4, false);
  chunk.read(payload.subarray(offset, offset + chunk_length), endian);
  return offset + chunk_length;
}

@blf.chunk("_par", 2.1)
export class s_blf_chunk_game_results_statistics extends BLFChunkBase {
  chunk_count = 4;
  player_statistics = new s_blf_chunk_game_results_player_statistics();
  player_vs_player_statistics =
    new s_blf_chunk_game_results_player_vs_player_statistics();
  player_vs_ai_statistics =
    new s_blf_chunk_game_results_player_vs_ai_statistics();
  team_statistics = new s_blf_chunk_game_results_team_statistics();

  read_body(payload: Uint8Array, endian: c.Endian): void {
    if (payload.length < 4) {
      throw new BlfError(
        `_par chunk payload too short: need at least 4 bytes, got ${payload.length}`
      );
    }
    const view = new DataView(
      payload.buffer,
      payload.byteOffset,
      payload.byteLength
    );
    this.chunk_count = view.getUint32(0, endian === "little");
    let offset = 4;
    offset = read_child(payload, offset, this.player_statistics, endian);
    offset = read_child(
      payload,
      offset,
      this.player_vs_player_statistics,
      endian
    );
    offset = read_child(payload, offset, this.player_vs_ai_statistics, endian);
    read_child(payload, offset, this.team_statistics, endian);
  }

  write_body(endian: c.Endian): Uint8Array {
    const children = [
      this.player_statistics.write(endian),
      this.player_vs_player_statistics.write(endian),
      this.player_vs_ai_statistics.write(endian),
      this.team_statistics.write(endian),
    ];
    const size = 4 + children.reduce((total, child) => total + child.length, 0);
    const out = new Uint8Array(size);
    new DataView(out.buffer).setUint32(
      0,
      this.chunk_count,
      endian === "little"
    );
    let offset = 4;
    for (const child of children) {
      out.set(child, offset);
      offset += child.length;
    }
    return out;
  }
}
