import { c } from "@craftycodie/cstruct";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

/** Halo 3 local display info (`clif` 1.1, 12 bytes). */
@blf.chunk("clif", 1.1)
export class s_blf_chunk_local_info extends CStructBLFChunk(
  c.struct({
    display_mode: c.i32(),
    screen_width: c.i32(),
    screen_height: c.i32(),
  })
) {}
