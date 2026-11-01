import { c } from "@craftycodie/cstruct";
import { blf, CStructBLFChunk } from "../../blf_chunk";

@blf.chunk("_fsm", 1.1)
export class s_blf_chunk_fileshare_metadata extends CStructBLFChunk(
  c.struct({
    unknown0: c.u64(),
    unknown8: c.u8().array(32),
    unknown28: c.string(36, "latin1"),
    unknown4c: c.u64(),
    unknown54: c.u8().array(32),
    unknown74: c.string(36, "latin1"),
    unknown98: c.string(40, "latin1"),
    file_signature: c.u8().array(256).padAfter(4),
  })
) {}
