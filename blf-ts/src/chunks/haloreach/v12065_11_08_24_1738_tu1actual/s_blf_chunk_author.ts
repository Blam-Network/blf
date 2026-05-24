import { c } from "@craftycodie/cstruct";
import pkg from "../../../../package.json";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

/** BLF author chunk (`athr` 3.1) — same layout as Halo 3: ODST Atlas. */
@blf.chunk("athr", 3.1)
@c.struct()
export class s_blf_chunk_author extends CStructBLFChunk {
  /** e.g. `GameData.Reach`, or blf_lib package name from `for_build`. */
  @c.field(c.String(16))
  program_name = "";

  /** e.g. 12065 */
  @c.field("u32")
  build_number = 0;

  /** e.g. 2 for Reach title storage exports */
  @c.field("u32")
  build_number_sequence = 0;

  /** e.g. `12065.11.08.24.1738.tu1actual` */
  @c.field(c.String(28))
  build_string = "";

  /** e.g. `davidav`, `dagasca` */
  @c.field(c.String(16))
  author_name = "";

  /**
   * Default author chunk for Reach TU1 exports.
   * Mirrors `s_blf_chunk_author::for_build` in blf_lib (ODST 3.1 layout).
   */
  static forBuild(options?: {
    programName?: string;
    buildString?: string;
    buildNumber?: number;
    buildNumberSequence?: number;
    authorName?: string;
  }): s_blf_chunk_author {
    const chunk = new s_blf_chunk_author();
    chunk.program_name = options?.programName ?? `blf-ts ${pkg.version}`;
    chunk.build_number = options?.buildNumber ?? 0;
    chunk.build_number_sequence = options?.buildNumberSequence ?? 0;
    chunk.build_string = (options?.buildString ?? "").slice(0, 28);
    chunk.author_name = options?.authorName ?? "";
    return chunk;
  }
}
