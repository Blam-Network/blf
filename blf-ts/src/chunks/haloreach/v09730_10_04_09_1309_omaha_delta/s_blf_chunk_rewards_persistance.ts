import { c } from "@craftycodie/cstruct";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

/** Omaha Delta rewards persistence (`rpdl` 1.1) — still emitted for TU1 clients. */
@blf.chunk("rpdl", 1.1)
export class s_blf_chunk_rewards_persistance extends CStructBLFChunk(
  c.struct({
    unknown1: c.u32(),
    unknown2: c.u8().array(0x180),
  })
) {}
