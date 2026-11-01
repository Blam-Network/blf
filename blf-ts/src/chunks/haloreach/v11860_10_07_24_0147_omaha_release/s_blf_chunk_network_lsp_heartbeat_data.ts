import { c } from "@craftycodie/cstruct";
import {
  s_network_lsp_heartbeat_player_data,
  s_network_lsp_heartbeat_session_data,
} from "../../../blam/haloreach/v12065_11_08_24_1738_tu1actual/networking/logic/network_lsp_presence";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

/** Reach release LSP presence heartbeat upload (`phbt` 5.1, 0x1BB-byte body). */
@blf.chunk("phbt", 5.1)
export class s_blf_chunk_network_lsp_heartbeat_data extends CStructBLFChunk(
  c.struct({
    has_players: c.u8(),
    local_player_count: c.u8(),
    players: s_network_lsp_heartbeat_player_data.array(4),
    machine_id: c.u64(),
    squad_session_id: c.u64(),
    session_data: s_network_lsp_heartbeat_session_data,
  })
) {}
