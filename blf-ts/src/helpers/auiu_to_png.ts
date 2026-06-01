// biome-ignore-all lint/suspicious/noBitwiseOperators: PNG CRC32 requires bitwise arithmetic
// biome-ignore-all lint/style/noNonNullAssertion: buffer indices are bounded by stride/width/height checks
// biome-ignore-all lint/style/useForOf: CRC inner bit loop is not a simple array walk
import { deflate } from "pako";
import type { s_blf_chunk_auth_upload_image } from "../chunks/haloreach/v12065_11_08_24_1738_tu1actual/s_blf_chunk_auth_upload_image";
import { BlfError } from "../error";

const PNG_SIGNATURE = new Uint8Array([137, 80, 78, 71, 13, 10, 26, 10]);

/**
 * Pack `auiu` scanlines into tightly packed RGBA.
 *
 * Reach stores pixels as RGBA8 in byte order (R, G, B, A per pixel).
 */
export function auiu_pixels_to_rgba(
  width: number,
  height: number,
  stride: number,
  pixels: Uint8Array
): Uint8Array {
  if (stride < width) {
    throw new BlfError(`auiu stride ${stride} is less than width ${width}`);
  }

  const rgba = new Uint8Array(width * height * 4);
  for (let y = 0; y < height; y++) {
    const row = y * stride * 4;
    for (let x = 0; x < width; x++) {
      const src = row + x * 4;
      const dst = (y * width + x) * 4;
      rgba[dst] = pixels[src]!;
      rgba[dst + 1] = pixels[src + 1]!;
      rgba[dst + 2] = pixels[src + 2]!;
      rgba[dst + 3] = pixels[src + 3]!;
    }
  }
  return rgba;
}

/** Encode tightly packed RGBA8 scanlines as a PNG (color type 6). */
export function rgba_to_png(
  width: number,
  height: number,
  rgba: Uint8Array
): Uint8Array {
  const expected = width * height * 4;
  if (rgba.length < expected) {
    throw new BlfError(
      `rgba_to_png needs at least ${expected} bytes, got ${rgba.length}`
    );
  }

  const raw = new Uint8Array(height * (1 + width * 4));
  let off = 0;
  for (let y = 0; y < height; y++) {
    raw[off++] = 0;
    const row = y * width * 4;
    for (let x = 0; x < width * 4; x++) {
      raw[off++] = rgba[row + x]!;
    }
  }

  const ihdr = new Uint8Array(13);
  const ihdrView = new DataView(ihdr.buffer);
  ihdrView.setUint32(0, width, false);
  ihdrView.setUint32(4, height, false);
  ihdr[8] = 8;
  ihdr[9] = 6;
  ihdr[10] = 0;
  ihdr[11] = 0;
  ihdr[12] = 0;

  const idat = deflate(raw, { level: 9 });
  const parts = [
    PNG_SIGNATURE,
    write_png_chunk("IHDR", ihdr),
    write_png_chunk("IDAT", idat),
    write_png_chunk("IEND", new Uint8Array(0)),
  ];

  const total = parts.reduce((sum, part) => sum + part.length, 0);
  const png = new Uint8Array(total);
  let writeOff = 0;
  for (const part of parts) {
    png.set(part, writeOff);
    writeOff += part.length;
  }
  return png;
}

/** Encode `auiu` RGBA8 pixels as PNG. */
export function auiu_pixels_to_png(
  width: number,
  height: number,
  stride: number,
  pixels: Uint8Array
): Uint8Array {
  const rgba = auiu_pixels_to_rgba(width, height, stride, pixels);
  return rgba_to_png(width, height, rgba);
}

/** Encode a parsed `auiu` 1.2 chunk as PNG. */
export function auiu_chunk_to_png(
  chunk: s_blf_chunk_auth_upload_image
): Uint8Array {
  return auiu_pixels_to_png(
    chunk.width,
    chunk.height,
    chunk.stride,
    chunk.pixels
  );
}

function write_png_chunk(type: string, data: Uint8Array): Uint8Array {
  const typeBytes = new TextEncoder().encode(type);
  const chunk = new Uint8Array(4 + 4 + data.length + 4);
  const view = new DataView(chunk.buffer);
  view.setUint32(0, data.length, false);
  chunk.set(typeBytes, 4);
  chunk.set(data, 8);
  const crc = crc32(chunk.subarray(4, 8 + data.length));
  view.setUint32(8 + data.length, crc, false);
  return chunk;
}

function crc32(data: Uint8Array): number {
  let crc = 0xffffffff;
  for (let i = 0; i < data.length; i++) {
    crc ^= data[i]!;
    for (let bit = 0; bit < 8; bit++) {
      const mask = -(crc & 1);
      crc = (crc >>> 1) ^ (0xedb8_8320 & mask);
    }
  }
  return (crc ^ 0xffffffff) >>> 0;
}
