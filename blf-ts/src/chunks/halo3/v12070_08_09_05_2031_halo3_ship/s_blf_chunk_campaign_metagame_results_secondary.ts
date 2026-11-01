import { c } from "@craftycodie/cstruct";
import { blf, CStructBLFChunk } from "../../../blf_chunk";
import { s_campaign_metagame_runtime_globals_secondary } from "./s_blf_chunk_campaign_metagame_results";

/** Halo 3 secondary campaign metagame results (`cmrs` 1.1, 0x6C bytes). */
@blf.chunk("cmrs", 1.1)
export class s_blf_chunk_campaign_metagame_results_secondary extends CStructBLFChunk(
  c.struct({
    valid: c.bool().padAfter(3),
    results: s_campaign_metagame_runtime_globals_secondary,
  })
) {}
