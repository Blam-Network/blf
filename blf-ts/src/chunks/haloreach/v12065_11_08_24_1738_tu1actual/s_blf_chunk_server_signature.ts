import { c } from "@craftycodie/cstruct";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

/** Reach `ssig` 1.1 (name is a guess, as in blf_lib). */
@blf.chunk("ssig", 1.1)
export class s_blf_chunk_server_signature extends CStructBLFChunk(
  c.struct({
    data: c.bytes(44),
  })
) {}
