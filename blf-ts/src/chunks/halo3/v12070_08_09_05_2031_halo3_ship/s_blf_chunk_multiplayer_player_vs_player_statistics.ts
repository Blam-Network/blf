import { c } from "@craftycodie/cstruct";
import { k_maximum_players } from "../../../blam/halo3/v12070_08_09_05_2031_halo3_ship/game/game_options";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

export const s_blf_chunk_multiplayer_player_vs_player_statistic = c.struct({
  kills: c.i16(),
  deaths: c.i16(),
});
export type s_blf_chunk_multiplayer_player_vs_player_statistic = c.infer<
  typeof s_blf_chunk_multiplayer_player_vs_player_statistic
>;

/** Halo 3 player-vs-player kills and deaths (`mps2` 2.1, 0x404 bytes), nested in `_par`. */
@blf.chunk("mps2", 2.1)
export class s_blf_chunk_multiplayer_player_vs_player_statistics extends CStructBLFChunk(
  c.struct({
    players: s_blf_chunk_multiplayer_player_vs_player_statistic
      .array(k_maximum_players)
      .array(k_maximum_players)
      .padBefore(4),
  })
) {}
