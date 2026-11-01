import { c } from "@craftycodie/cstruct";
import { network_lsp_heartbeat_response_user_file_bitfield } from "../../../blam/haloreach/v12065_11_08_24_1738_tu1actual/networking/logic/network_lsp_presence";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

/** Reach TU1 LSP presence heartbeat response (`phbr` 2.1, 0x93-byte body). */
@blf.chunk("phbr", 2.1)
export class s_blf_chunk_player_heartbeat_response extends CStructBLFChunk(
  c.struct({
    machine_file_requires_download: c.bool(),
    user_file_requires_download:
      network_lsp_heartbeat_response_user_file_bitfield,
    xuid_count: c.i32(),
    xuids: c.u64().array(16),
    session_id: c.u64(),
    ack_number: c.i32(),
    join_result: c.u8(),
  })
) {}
