import { c } from "@craftycodie/cstruct";
import { s_game_results_game_description } from "../../../blam/haloreach/v12065_11_08_24_1738_tu1actual/game/game_results";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

@blf.chunk("mpgd", 8.2)
export class s_blf_chunk_game_results_game_description extends CStructBLFChunk(
  c.struct({
    started: c.bool(),
    start_time: c.u32(),
    finished: c.bool(),
    finish_time: c.u32().padAfter(2),
    game_end_reason: c.u8(),
    description: s_game_results_game_description,
    ui_game_mode: c.u8(),
    game_engine_type: c.u8().padAfter(2),
    game_engine_category: c.i8().padAfter(10),
  })
) {}
