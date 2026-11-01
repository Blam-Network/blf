import { c } from "@craftycodie/cstruct";
import {
  k_maximum_teams,
  s_game_results_team_data,
} from "../../../blam/haloreach/v12065_11_08_24_1738_tu1actual/game/game_results";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

@blf.chunk("mptm", 3.1)
export class s_blf_chunk_game_results_teams extends CStructBLFChunk(
  c.struct({
    teams: s_game_results_team_data.array(k_maximum_teams).padBefore(4),
  })
) {}
