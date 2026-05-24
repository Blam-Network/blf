import { c } from "@craftycodie/cstruct";
import { s_content_item_metadata } from "../../../blam/haloreach/v12065_11_08_24_1738_tu1actual/saved_games/saved_game_files";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

@blf.chunk("chdr", 10.2)
@c.struct()
export class s_blf_chunk_content_header extends CStructBLFChunk {
  @c.field("u16")
  build_number!: number;

  @c.field("u16")
  map_minor_version!: number;

  @c.field(s_content_item_metadata)
  metadata!: s_content_item_metadata;
}
