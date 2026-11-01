import { c } from "@craftycodie/cstruct";
import {
  k_game_results_event_count,
  k_game_results_medal_event_count,
  s_game_results_event,
  s_game_results_medal_event,
} from "../../../blam/haloreach/v12065_11_08_24_1738_tu1actual/game/game_results";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

@blf.chunk("mpev", 5.1)
export class s_blf_chunk_game_results_events extends CStructBLFChunk(
  c.struct({
    events: s_game_results_event.array(k_game_results_event_count).padBefore(4),
    medal_events: s_game_results_medal_event.array(
      k_game_results_medal_event_count
    ),
  })
) {}
