import { c } from "@craftycodie/cstruct";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

/** Halo 3 delta player data (`fupd` 2.0). No `bungie_user_role` yet. */
@blf.chunk("fupd", 2.0)
export class s_blf_chunk_player_data extends CStructBLFChunk(
  c.struct({
    hopper_access: c.u32(),
    highest_skill: c.u32(),
    hopper_directory: c.string(32, "latin1").default("default_hoppers"),
  })
) {}
