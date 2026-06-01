import { c } from "@craftycodie/cstruct";
import {
  s_network_lsp_heartbeat_player_data,
  s_network_lsp_heartbeat_session_data,
} from "../../../blam/haloreach/v12065_11_08_24_1738_tu1actual/networking/logic/network_lsp_presence";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

/** Reach release LSP presence heartbeat upload (`phbt` 5.1, 0x1BB-byte body). */
@blf.chunk("phbt", 5.1)
@c.struct()
export class s_blf_chunk_network_lsp_heartbeat_data extends CStructBLFChunk {
  @c.field("u8")
  has_players = 0;

  @c.field("u8")
  local_player_count = 0;

  @c.field(s_network_lsp_heartbeat_player_data, { count: 4 })
  players = Array.from(
    { length: 4 },
    () => new s_network_lsp_heartbeat_player_data()
  );

  @c.field("u64")
  machine_id = 0n;

  @c.field("u8", { count: 8 })
  unknowneeA = Array.from({ length: 8 }, () => 0);

  @c.field(s_network_lsp_heartbeat_session_data)
  session_data = new s_network_lsp_heartbeat_session_data();
}
