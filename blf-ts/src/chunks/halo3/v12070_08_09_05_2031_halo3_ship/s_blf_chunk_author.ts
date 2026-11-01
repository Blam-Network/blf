import { c } from "@craftycodie/cstruct";
import pkg from "../../../../package.json";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

const k_build_number = 12_070;
const k_build_string = "12070.08.09.05.2031.halo3_ship";

/** Halo 3 author chunk (`athr` 3.1, 0x44 bytes). */
@blf.chunk("athr", 3.1)
export class s_blf_chunk_author extends CStructBLFChunk(
  c.struct({
    /** e.g. `GameData.Halo3` */
    program_name: c.string(16, "latin1"),
    /** e.g. 1 */
    build_number_sequence: c.i32(),
    /** e.g. 12070 */
    build_number: c.i32(),
    /** e.g. `12070.08.09.05.2031.halo3_s` */
    build_string: c.string(28, "latin1"),
    /** e.g. `sameling` */
    author_name: c.string(16, "latin1"),
  })
) {
  /**
   * Author chunk for files written by this library for Halo 3 (12070).
   * Mirrors `s_blf_chunk_author::for_build` in blf_lib, except the default
   * program name names blf-ts instead of blf_lib.
   */
  static forBuild(options?: {
    programName?: string;
    buildString?: string;
    buildNumber?: number;
    buildNumberSequence?: number;
    authorName?: string;
  }): s_blf_chunk_author {
    const chunk = new s_blf_chunk_author();
    chunk.program_name = (
      options?.programName ?? `blf-ts ${pkg.version}`
    ).slice(0, 16);
    chunk.build_number = options?.buildNumber ?? k_build_number;
    chunk.build_number_sequence = options?.buildNumberSequence ?? 1;
    chunk.build_string = (options?.buildString ?? k_build_string).slice(0, 28);
    chunk.author_name = options?.authorName ?? "";
    return chunk;
  }
}
