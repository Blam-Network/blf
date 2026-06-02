import type { c } from "@craftycodie/cstruct";
import { BLFChunkBase, blf } from "../../../blf_chunk";
import { BlfError } from "../../../error";

/** Halo 3 screenshot JPEG payload (`scnd` 1.1): BE `u32` length + JPEG bytes. */
@blf.chunk("scnd", 1.1)
export class s_blf_chunk_screenshot_data extends BLFChunkBase {
  jpeg_data = new Uint8Array(0);

  read_body(payload: Uint8Array, _endian: c.Endian): void {
    if (payload.length < 4) {
      throw new BlfError(
        `scnd payload needs at least 4 bytes for length, got ${payload.length}`
      );
    }

    const view = new DataView(
      payload.buffer,
      payload.byteOffset,
      payload.byteLength
    );
    const length = view.getUint32(0, false);

    if (payload.length < 4 + length) {
      throw new BlfError(
        `scnd payload too short: need ${4 + length} bytes, got ${payload.length}`
      );
    }

    this.jpeg_data = new Uint8Array(payload.subarray(4, 4 + length));
  }

  write_body(_endian: c.Endian): Uint8Array {
    const out = new Uint8Array(4 + this.jpeg_data.length);
    new DataView(out.buffer).setUint32(0, this.jpeg_data.length, false);
    out.set(this.jpeg_data, 4);
    return out;
  }
}
