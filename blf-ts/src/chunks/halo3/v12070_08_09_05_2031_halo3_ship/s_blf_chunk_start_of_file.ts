import { c } from "@craftycodie/cstruct";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

/** Wire values for {@link s_blf_chunk_start_of_file.byte_order_mark}. */
export enum e_byte_order_mark {
  little_endian = 0xfffe,
  big_endian = 0xfeff,
}

/** BLF start-of-file chunk (`_blf` 1.2). */
@blf.chunk("_blf", 1.2)
export class s_blf_chunk_start_of_file extends CStructBLFChunk(
  c.struct({
    byte_order_mark: c
      .enum(e_byte_order_mark, c.u16())
      .default(e_byte_order_mark.little_endian),
    name: c.string(32, "latin1").padAfter(2),
  })
) {
  static create(name: string): s_blf_chunk_start_of_file {
    const chunk = new s_blf_chunk_start_of_file();
    chunk.name = name;
    return chunk;
  }
}
