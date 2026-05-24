import { c } from "@craftycodie/cstruct";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

/** BLF end-of-file chunk (`_eof` 1.1). `file_size` is bytes written before this chunk. */
@blf.chunk("_eof", 1.1)
@c.struct()
export class s_blf_chunk_end_of_file extends CStructBLFChunk {
  @c.field("u32")
  file_size = 0;

  @c.field("u8")
  authentication_type = 0;
}
