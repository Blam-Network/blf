import { blf } from "../../../blf_chunk";
import { CStructBLFChunk } from "../../../blf_chunk";
import { c } from "@craftycodie/cstruct";

/** Wire values for {@link s_blf_chunk_start_of_file.byte_order_mark}. */
export enum e_byte_order_mark {
  little_endian = 0xfffe,
  big_endian = 0xfeff,
}

/** BLF start-of-file chunk (`_blf` 1.2). */
@blf.chunk("_blf", 1.2)
@c.struct()
export class s_blf_chunk_start_of_file extends CStructBLFChunk {
  @c.field(c.enum("u16", e_byte_order_mark))
  byte_order_mark = e_byte_order_mark.little_endian;

  @c.field(c.String(32), { pad_before: 2 })
  name = "";

  static create(name: string): s_blf_chunk_start_of_file {
    const chunk = new s_blf_chunk_start_of_file();
    chunk.name = name;
    return chunk;
  }
}
