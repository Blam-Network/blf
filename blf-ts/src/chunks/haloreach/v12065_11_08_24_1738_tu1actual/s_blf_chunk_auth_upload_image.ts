import type { c } from "@craftycodie/cstruct";
import { BLFChunkBase, blf } from "../../../blf_chunk";
import { BlfError } from "../../../error";

/** Fixed big-endian header before RGBA pixels in `auiu` 1.2. */
export const AUIU_HEADER_SIZE = 12;

/** Reach Spartan render upload (`auiu` 1.2): header + RGBA8 pixel buffer. */
@blf.chunk("auiu", 1.2)
export class s_blf_chunk_auth_upload_image extends BLFChunkBase {
  width = 0;
  height = 0;
  stride = 0;
  reserved = 0;
  data_size = 0;
  pixels = new Uint8Array(0);

  /** Minimum RGBA byte count implied by `stride` × `height`. */
  static expected_pixel_bytes(stride: number, height: number): number {
    return stride * height * 4;
  }

  read_body(payload: Uint8Array, _endian: c.Endian): void {
    if (payload.length < AUIU_HEADER_SIZE) {
      throw new BlfError(
        `auiu payload needs at least ${AUIU_HEADER_SIZE} bytes, got ${payload.length}`
      );
    }

    const view = new DataView(
      payload.buffer,
      payload.byteOffset,
      payload.byteLength
    );
    this.width = view.getUint16(0, false);
    this.height = view.getUint16(2, false);
    this.stride = view.getUint16(4, false);
    this.reserved = view.getUint16(6, false);
    this.data_size = view.getUint32(8, false);

    if (this.width === 0 || this.height === 0) {
      throw new BlfError(
        `auiu invalid dimensions: ${this.width}x${this.height}`
      );
    }
    if (this.stride < this.width) {
      throw new BlfError(
        `auiu stride ${this.stride} is less than width ${this.width}`
      );
    }

    const min_pixels = s_blf_chunk_auth_upload_image.expected_pixel_bytes(
      this.stride,
      this.height
    );
    if (this.data_size < min_pixels) {
      throw new BlfError(
        `auiu data_size ${this.data_size} is less than stride*height*4 (${min_pixels})`
      );
    }
    if (payload.length < AUIU_HEADER_SIZE + this.data_size) {
      throw new BlfError(
        `auiu payload too short: need ${AUIU_HEADER_SIZE + this.data_size} bytes, got ${payload.length}`
      );
    }

    this.pixels = new Uint8Array(
      payload.subarray(AUIU_HEADER_SIZE, AUIU_HEADER_SIZE + this.data_size)
    );
  }

  write_body(_endian: c.Endian): Uint8Array {
    const min_pixels = s_blf_chunk_auth_upload_image.expected_pixel_bytes(
      this.stride,
      this.height
    );
    if (this.pixels.length < min_pixels) {
      throw new BlfError(
        `auiu pixels too short: need at least ${min_pixels} bytes, got ${this.pixels.length}`
      );
    }

    const data_size = this.data_size || this.pixels.length;
    if (data_size < min_pixels) {
      throw new BlfError(
        `auiu data_size ${data_size} is less than stride*height*4 (${min_pixels})`
      );
    }

    const out = new Uint8Array(AUIU_HEADER_SIZE + data_size);
    const view = new DataView(out.buffer);
    view.setUint16(0, this.width, false);
    view.setUint16(2, this.height, false);
    view.setUint16(4, this.stride, false);
    view.setUint16(6, this.reserved, false);
    view.setUint32(8, data_size, false);
    out.set(this.pixels.subarray(0, data_size), AUIU_HEADER_SIZE);
    return out;
  }
}
