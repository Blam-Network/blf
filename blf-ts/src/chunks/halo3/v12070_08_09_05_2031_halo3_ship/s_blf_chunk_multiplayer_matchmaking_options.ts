import { c } from "@craftycodie/cstruct";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

/** Halo 3 game results matchmaking options (`mpmo` 2.1, 0x5C bytes). */
@blf.chunk("mpmo", 2.1)
export class s_blf_chunk_multiplayer_matchmaking_options extends CStructBLFChunk(
  c.struct({
    hopper_identifier: c.i16(),
    hopper_name: c.wstring(0x20),
    hopper_ranked: c.bool(),
    hopper_team_based: c.bool(),
    xlast_index: c.u8(),
    draw_probability: c.u32(),
    beta: c.f32(),
    tau: c.f32(),
    experience_base_increment: c.u32(),
    experience_penalty_decrement: c.u32().padAfter(3),
  })
) {}
