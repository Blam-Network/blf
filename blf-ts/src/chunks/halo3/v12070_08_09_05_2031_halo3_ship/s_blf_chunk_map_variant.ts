import { c } from "@craftycodie/cstruct";
import { c_map_variant } from "../../../blam/halo3/v12070_08_09_05_2031_halo3_ship/saved_games/scenario_map_variant";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

/** Halo 3 unpacked map variant (`mapv` 12.1, 0xE098 bytes). */
@blf.chunk("mapv", 12.1)
export class s_blf_chunk_map_variant extends CStructBLFChunk(
  c.struct({
    map_variant: c_map_variant.padBefore(4).padAfter(4),
  })
) {
  static create(map_variant: c_map_variant): s_blf_chunk_map_variant {
    const chunk = new s_blf_chunk_map_variant();
    chunk.map_variant = map_variant;
    return chunk;
  }
}
