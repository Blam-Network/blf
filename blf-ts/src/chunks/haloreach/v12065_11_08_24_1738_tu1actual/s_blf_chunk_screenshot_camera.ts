import { c } from "@craftycodie/cstruct";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

/**
 * Reach screenshot camera (`scnc` 4.1). Stubbed as in blf_lib: the layout is
 * unknown (it includes `s_screenshot_player_info`).
 */
@blf.chunk("scnc", 4.1)
export class s_blf_chunk_screenshot_camera extends CStructBLFChunk(
  c.struct({
    data: c.bytes(4964),
  })
) {}
