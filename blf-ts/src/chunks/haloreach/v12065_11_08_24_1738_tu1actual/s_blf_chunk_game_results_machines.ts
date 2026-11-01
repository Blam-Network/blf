import { c } from "@craftycodie/cstruct";
import {
  k_maximum_players,
  s_blf_chunk_machines_entry,
} from "../../../blam/haloreach/v12065_11_08_24_1738_tu1actual/game/game_results";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

@blf.chunk("mpma", 6.1)
export class s_blf_chunk_game_results_machines extends CStructBLFChunk(
  c.struct({
    session_type_group: c.bool(),
    session_type_squad: c.bool().padAfter(2),
    players: s_blf_chunk_machines_entry.array(k_maximum_players),
  })
) {}
