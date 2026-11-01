import { c } from "@craftycodie/cstruct";
import { BLFChunkBase, blf } from "../../../blf_chunk";
import { BlfError } from "../../../error";

/** Reach TU1 list of XUIDs (`idls` 1.1), e.g. a user's friends. */
@blf.chunk("idls", 1.1)
export class s_blf_chunk_xuid_list extends BLFChunkBase {
  xuids: bigint[] = [];

  read_body(payload: Uint8Array, endian: c.Endian): void {
    if (payload.length < 4) {
      throw new BlfError(
        `idls chunk payload too short: need at least 4 bytes, got ${payload.length}`
      );
    }
    const count = c.read(c.struct({ count: c.u32() }), payload, endian).count;
    this.xuids = c.read(c.u64().array(count), payload, 4, endian);
  }

  write_body(endian: c.Endian): Uint8Array {
    return c.write(
      c.struct({ count: c.u32(), xuids: c.u64().array(this.xuids.length) }),
      { count: this.xuids.length, xuids: this.xuids },
      endian
    );
  }
}
