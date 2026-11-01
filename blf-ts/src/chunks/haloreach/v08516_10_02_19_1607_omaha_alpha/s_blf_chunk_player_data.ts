import { c } from "@craftycodie/cstruct";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

/**
 * Reach alpha player data (`fupd` 6.0). Same fields as TU1 `fupd` 7.0, but
 * `hopper_access` and `bungie_user_role` are 32-bit. TU1 user files carry both.
 */
@blf.chunk("fupd", 6.0)
export class s_blf_chunk_player_data extends CStructBLFChunk(
  c.struct({
    hopper_access: c.u32(),
    bungie_user_role: c.u32(),
    hopper_directory: c.string(32, "utf8").default("default_hoppers"),
    /** 59 achievements, capacity of 256 bits. */
    unlock_achievements: c.u8().array(32),
    extras_portal_debug: c.bool(),
  })
) {}
