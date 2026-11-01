import { c } from "@craftycodie/cstruct";
import { BLFChunkBase, blf } from "../../../blf_chunk";
import { write_blffile } from "../../../blffile";
import { BlfError } from "../../../error";
import { s_blf_chunk_author } from "./s_blf_chunk_author";
import { s_blf_chunk_end_of_file } from "./s_blf_chunk_end_of_file";
import { s_blf_chunk_start_of_file } from "./s_blf_chunk_start_of_file";

/** One recent player (10 bytes, unaligned: the xuid follows the u16 directly). */
export const s_files_user_recent_players_entry = c.struct({
  hopper_identifier: c.u16(),
  xuid: c.u64(),
});
export type s_files_user_recent_players_entry = c.infer<
  typeof s_files_user_recent_players_entry
>;

const s_user_recent_players_header = c.struct({ player_count: c.u32() });
const k_player_count_size = s_user_recent_players_header.getSize();

/** Halo 3 recent players (`furp` 2.1): a u32 count, then that many entries. */
@blf.chunk("furp", 2.1)
export class s_blf_chunk_user_recent_players extends BLFChunkBase {
  players: s_files_user_recent_players_entry[] = [];

  read_body(payload: Uint8Array, endian: c.Endian): void {
    if (payload.length < k_player_count_size) {
      throw new BlfError(
        `furp chunk payload too short: need at least ${k_player_count_size} bytes, got ${payload.length}`
      );
    }
    const { player_count } = s_user_recent_players_header.read(
      payload.subarray(0, k_player_count_size),
      endian
    );
    const entry_size = s_files_user_recent_players_entry.getSize();
    const expected_size = k_player_count_size + player_count * entry_size;
    if (payload.length < expected_size) {
      throw new BlfError(
        `furp chunk payload too short for ${player_count} players: need ${expected_size} bytes, got ${payload.length}`
      );
    }
    this.players = c.read(
      s_files_user_recent_players_entry.array(player_count),
      payload.subarray(k_player_count_size, expected_size),
      endian
    );
  }

  write_body(endian: c.Endian): Uint8Array {
    const entry_size = s_files_user_recent_players_entry.getSize();
    const out = new Uint8Array(
      k_player_count_size + this.players.length * entry_size
    );
    out.set(
      s_user_recent_players_header.write(
        { player_count: this.players.length },
        endian
      ),
      0
    );
    this.players.forEach((player, i) => {
      out.set(
        s_files_user_recent_players_entry.write(player, endian),
        k_player_count_size + i * entry_size
      );
    });
    return out;
  }
}

/** Halo 3 recent players file: `_blf` "halo3 recent players", `athr`, `furp`, `_eof`. */
export function build_recent_players_file(furp: {
  players: s_files_user_recent_players_entry[];
}): Uint8Array {
  const chunk = new s_blf_chunk_user_recent_players();
  chunk.players = furp.players;
  return write_blffile("big", [
    s_blf_chunk_start_of_file.create("halo3 recent players"),
    s_blf_chunk_author.forBuild(),
    chunk,
    new s_blf_chunk_end_of_file(),
  ]);
}
