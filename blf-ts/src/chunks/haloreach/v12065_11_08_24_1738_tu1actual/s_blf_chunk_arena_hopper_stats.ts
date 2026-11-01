import { c } from "@craftycodie/cstruct";
import { blf, CStructBLFChunk } from "../../../blf_chunk";
import { write_blffile } from "../../../blffile";
import { s_blf_chunk_end_of_file } from "../../halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_end_of_file";

/** Reach TU1 arena hopper stats (`arhs` 3.1). Fields after `season` are not mapped yet. */
@blf.chunk("arhs", 3.1)
export class s_blf_chunk_arena_hopper_stats extends CStructBLFChunk(
  c.struct({
    season: c.u32(),
    unknown04: c.u32(),
    unknown08: c.u32(),
    unknown0C: c.u32(),
    unknown10: c.u32(),
    unknown14: c.u16(),
  })
) {}

/**
 * Arena season stats file: `arhs` then `_eof`. No `_blf` or `athr`; it is
 * unknown whether the real file has them.
 */
export function build_arena_hopper_stats_file(
  arhs: s_blf_chunk_arena_hopper_stats
): Uint8Array {
  return write_blffile("big", [arhs, new s_blf_chunk_end_of_file()]);
}
