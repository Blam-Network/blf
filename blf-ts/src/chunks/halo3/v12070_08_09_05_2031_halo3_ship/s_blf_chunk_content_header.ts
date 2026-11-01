import { c } from "@craftycodie/cstruct";
import { s_content_item_metadata } from "../../../blam/halo3/v12070_08_09_05_2031_halo3_ship/saved_games/saved_game_files";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

/** Halo 3 content header (`chdr` 9.2, 0xFC bytes). */
@blf.chunk("chdr", 9.2)
export class s_blf_chunk_content_header extends CStructBLFChunk(
  c.struct({
    build_number: c.u16(),
    build_sequence_number: c.u16(),
    metadata: s_content_item_metadata,
  })
) {}
