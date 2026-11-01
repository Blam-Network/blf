import { c } from "@craftycodie/cstruct";
import { k_maximum_teams } from "../../../blam/halo3/v12070_08_09_05_2031_halo3_ship/game/game_options";
import { blf, CStructBLFChunk } from "../../../blf_chunk";
import { s_game_results_statistics } from "./s_blf_chunk_multiplayer_player_statistics";

/** Halo 3 per-team statistics (`mps3` 2.1, 0x664 bytes), nested in `_par`. */
@blf.chunk("mps3", 2.1)
export class s_blf_chunk_multiplayer_team_statistics extends CStructBLFChunk(
  c.struct({
    teams: s_game_results_statistics.array(k_maximum_teams).padBefore(4),
  })
) {}
