import { c } from "@craftycodie/cstruct";
import { blf, CStructBLFChunk } from "../../blf_chunk";

@blf.chunk("_fsm", 1.1)
@c.struct()
export class s_blf_chunk_fileshare_metadata extends CStructBLFChunk {
  @c.field("u64")
  unknown0 = 0n;

  @c.field("u8", { count: 32 })
  unknown8 = new Uint8Array(32);

  @c.field(c.String(36))
  unknown28 = "";

  @c.field("u64")
  unknown4c = 0n;

  @c.field("u8", { count: 32 })
  unknown54 = new Uint8Array(32);

  @c.field(c.String(36))
  unknown74 = "";

  @c.field(c.String(40))
  unknown98 = "";

  @c.field("u8", { count: 256, pad_after: 4 })
  attestation_signature = new Uint8Array(256);
}
