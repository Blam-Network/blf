import { c } from "@craftycodie/cstruct";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

/** Reach TU1 challenge progress (`chpr` 2.1). */
@blf.chunk("chpr", 2.1)
export class s_blf_chunk_challenge_progress extends CStructBLFChunk(
  c.struct({
    active_challenge_set_1: c.u32(),
    active_challenge_set_2: c.u32(),
    chalenge_set_1_progress: c.i32().array(10),
    chalenge_set_2_progress: c.i32().array(10),
  })
) {}
