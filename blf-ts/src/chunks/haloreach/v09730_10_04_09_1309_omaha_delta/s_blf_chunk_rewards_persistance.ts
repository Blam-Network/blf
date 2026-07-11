import { c } from "@craftycodie/cstruct";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

/** Omaha Delta rewards persistence (`rpdl` 1.1) — still emitted for TU1 clients. */
@blf.chunk("rpdl", 1.1)
@c.struct()
export class s_blf_chunk_rewards_persistance extends CStructBLFChunk {
  @c.field("u32")
  unknown1 = 0;

  @c.field("u8", { count: 0x180 })
  unknown2 = Array.from({ length: 0x180 }, () => 0);
}
