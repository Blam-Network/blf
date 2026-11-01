import { c } from "@craftycodie/cstruct";
import { s_game_local_player } from "../../../blam/haloreach/v12065_11_08_24_1738_tu1actual/game/game_results";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

@blf.chunk("glcp", 3.1)
export class s_blf_chunk_game_local_players extends CStructBLFChunk(
  c.struct({
    local_players: s_game_local_player.array(4).padBefore(4),
  })
) {}
