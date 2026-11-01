import { c } from "@craftycodie/cstruct";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

/** Reach TU1 user nag message (`umsg` 1.0). */
@blf.chunk("umsg", 1.0)
export class s_blf_chunk_user_messaging_data extends CStructBLFChunk(
  c.struct({
    message_index: c.u32(),
    expires_at: c.Time64(),
  })
) {}
