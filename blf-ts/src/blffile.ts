import type { c } from "@craftycodie/cstruct";
import { type BLFChunk, getBlfChunkMeta } from "./blf_chunk";

/** Concatenate chunks (each with a 12-byte BLF header) in order. */
export function write_blffile(
  endian: c.Endian,
  chunks: BLFChunk[]
): Uint8Array {
  const parts: Uint8Array[] = [];
  let offset = 0;

  for (const chunk of chunks) {
    if (
      getBlfChunkMeta(chunk).signature === "_eof" &&
      Object.getOwnPropertyNames(chunk).includes("file_size") &&
      typeof (chunk as any).file_size === "number"
    ) {
      (chunk as any).file_size = offset;
    }

    const bytes = chunk.write(endian);
    parts.push(bytes);
    offset += bytes.length;
  }

  const out = new Uint8Array(offset);
  let write_offset = 0;
  for (const part of parts) {
    out.set(part, write_offset);
    write_offset += part.length;
  }
  return out;
}
