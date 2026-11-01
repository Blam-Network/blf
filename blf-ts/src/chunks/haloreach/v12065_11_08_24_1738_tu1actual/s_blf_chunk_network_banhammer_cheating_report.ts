import { c } from "@craftycodie/cstruct";
import { s_network_banhammer_cheating_report } from "../../../blam/haloreach/v12065_11_08_24_1738_tu1actual/game/game_results";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

@blf.chunk("chrt", 2.1)
export class s_blf_chunk_network_banhammer_cheating_report extends CStructBLFChunk(
  c.struct({
    valid: c.bool(),
    reserved: c.u8().array(3),
    report: s_network_banhammer_cheating_report,
  })
) {}
