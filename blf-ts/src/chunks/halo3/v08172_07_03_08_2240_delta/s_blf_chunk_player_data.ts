import { c } from "@craftycodie/cstruct";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

/** Halo 3 early player data (`fupd` 1.0), also used by the 06481 alpha. */
@blf.chunk("fupd", 1.0)
export class s_blf_chunk_player_data extends CStructBLFChunk(
  c.struct({
    /** Not certain whether this is the bnet flags or the hopper access. */
    hopper_access: c.u32(),
    highest_skill: c.u32().default(1),
  })
) {}
