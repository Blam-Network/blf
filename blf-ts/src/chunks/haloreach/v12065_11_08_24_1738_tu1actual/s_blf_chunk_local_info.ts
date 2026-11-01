import { c } from "@craftycodie/cstruct";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

@blf.chunk("clif", 2.1)
export class s_blf_chunk_local_info extends CStructBLFChunk(
  c.struct({
    is_widescreen: c.bool(),
    is_interlaced: c.bool(),
    is_high_definition: c.bool(),
    is_pal: c.bool(),
    is_ntsc_m: c.bool(),
    is_ntsc_j: c.bool().padAfter(2),
    display_width: c.i32(),
    display_height: c.i32(),
    vblank_rate: c.f32(),
  })
) {}
