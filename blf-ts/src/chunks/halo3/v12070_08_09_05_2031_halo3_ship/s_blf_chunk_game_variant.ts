import { c } from "@craftycodie/cstruct";
import { c_game_variant } from "../../../blam/halo3/v12070_08_09_05_2031_halo3_ship/game/game_engine_variant";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

/** Halo 3 unpacked game variant (`mpvr` 3.1, 0x264 bytes). */
@blf.chunk("mpvr", 3.1)
export class s_blf_chunk_game_variant extends CStructBLFChunk(
  c.struct({
    game_variant: c_game_variant(),
  })
) {}
