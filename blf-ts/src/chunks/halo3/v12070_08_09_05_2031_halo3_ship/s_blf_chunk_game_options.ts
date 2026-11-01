import { c } from "@craftycodie/cstruct";
import { game_options } from "../../../blam/halo3/v12070_08_09_05_2031_halo3_ship/game/game_options";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

/** Halo 3 game options (`gmop` 1.1, 0xF814 bytes), as uploaded in campaign carnage reports. */
@blf.chunk("gmop", 1.1)
export class s_blf_chunk_game_options extends CStructBLFChunk(
  c.struct({
    options: game_options.padBefore(4),
  })
) {}
