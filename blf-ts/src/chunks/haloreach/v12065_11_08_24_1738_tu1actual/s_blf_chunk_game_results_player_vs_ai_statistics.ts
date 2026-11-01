import { c } from "@craftycodie/cstruct";
import { s_game_results_player_vs_ai_statistics } from "../../../blam/haloreach/v12065_11_08_24_1738_tu1actual/game/game_results";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

@blf.chunk("mps3", 5.1)
export class s_blf_chunk_game_results_player_vs_ai_statistics extends CStructBLFChunk(
  c.struct({
    players: s_game_results_player_vs_ai_statistics.array(4).padBefore(4),
  })
) {}
