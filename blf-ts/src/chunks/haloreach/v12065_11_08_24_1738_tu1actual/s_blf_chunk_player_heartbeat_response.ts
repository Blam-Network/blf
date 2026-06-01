import { c } from "@craftycodie/cstruct";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

/** Reach TU1 LSP presence heartbeat response (`phbr` 2.1, 0x93-byte body). */
@blf.chunk("phbr", 2.1)
@c.struct()
export class s_blf_chunk_player_heartbeat_response extends CStructBLFChunk {
  @c.field("i8")
  machine_file_requires_download = 0;

  @c.field("i8")
  flags = 0;

  @c.field("i32")
  xuid_count = 0;

  @c.field("u64", { count: 16 })
  xuids = Array.from({ length: 16 }, () => 0n);

  @c.field("u64")
  session_id = 0n;

  @c.field("i32")
  ack_number = 0;

  @c.field("i8")
  join_result = 0;
}
