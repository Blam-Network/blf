import { c } from "@craftycodie/cstruct";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

/** Reach TU1 challenge progress (`chpr` 2.1). */
@blf.chunk("chpr", 2.1)
@c.struct()
export class s_blf_chunk_challenge_progress extends CStructBLFChunk {
  @c.field("u32")
  active_challenge_set_1 = 0;

  @c.field("u32")
  active_challenge_set_2 = 0;

  @c.field("i32", { count: 10 })
  chalenge_set_1_progress = Array.from({ length: 10 }, () => 0);

  @c.field("i32", { count: 10 })
  chalenge_set_2_progress = Array.from({ length: 10 }, () => 0);
}
