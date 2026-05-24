import { deflate, inflate } from "pako";
import { BlfError } from "../../../error";

/**
 * Zlib (RFC 1950) compression matching blf_lib `runtime_data_compress`:
 * `Compress::new_with_window_bits(Compression::new(9), true, 15)` via flate2+zlib.
 */
export function zlib_compress(data: Uint8Array): Uint8Array {
  return deflate(data, {
    level: 9,
    windowBits: 15,
    memLevel: 8,
  });
}

/** Reach runtime blob: BE `u32` uncompressed size + zlib payload. */
export function runtime_data_compress(
  source: Uint8Array,
  big_endian = true,
): Uint8Array {
  const compressed = zlib_compress(source);
  const out = new Uint8Array(4 + compressed.length);
  new DataView(out.buffer).setUint32(0, source.length, !big_endian);
  out.set(compressed, 4);
  return out;
}

/** Reach runtime blob: BE `u32` uncompressed size + zlib payload. */
export function runtime_data_decompress(
  source: Uint8Array,
  big_endian = true,
): Uint8Array {
  if (source.length < 4) {
    throw new BlfError(
      `runtime_data_decompress needs at least 4 bytes, got ${source.length}`,
    );
  }
  return inflate(source.subarray(4));
}
