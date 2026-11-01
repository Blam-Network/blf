import { c } from "@craftycodie/cstruct";
import { s_content_item_metadata } from "../../../blam/halo3odst/v13895_09_04_27_2201_atlas_release/saved_games/saved_game_files";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

/** Halo 3: ODST content header (`chdr` 9.3, 0xFC bytes). */
@blf.chunk("chdr", 9.3)
export class s_blf_chunk_content_header extends CStructBLFChunk(
  c.struct({
    build_number: c.u16(),
    build_sequence_number: c.u16(),
    metadata: s_content_item_metadata,
  })
) {}
