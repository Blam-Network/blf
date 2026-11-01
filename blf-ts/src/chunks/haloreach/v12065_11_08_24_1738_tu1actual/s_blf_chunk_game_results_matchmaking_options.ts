import { s_game_matchmaking_options } from "../../../blam/haloreach/v12065_11_08_24_1738_tu1actual/game/game_results";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

@blf.chunk("mpmo", 7.1)
export class s_blf_chunk_game_results_matchmaking_options extends CStructBLFChunk(
  s_game_matchmaking_options
) {}
