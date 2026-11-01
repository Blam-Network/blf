import { c } from "@craftycodie/cstruct";
import { type BLFChunk, getBlfChunkMeta, s_blf_header } from "./blf_chunk";
import { BlfError } from "./error";

const HEADER_SIZE = c.sizeof(s_blf_header);

/**
 * Read `chunks` in order from the start of `buffer`, like blf_lib's
 * `BlfFile::read` (the `blf_file!` structs). Every chunk must be present, in
 * order, with a matching signature and version. Bytes after the last chunk are
 * ignored.
 *
 * As in blf_lib, an `_eof` chunk's `file_size` must equal the number of bytes
 * before it.
 *
 * @throws {BlfError} when a chunk is missing, out of order, truncated or invalid.
 */
export function read_blffile(
  buffer: Uint8Array,
  endian: c.Endian,
  chunks: readonly BLFChunk[]
): void {
  let offset = 0;

  for (const chunk of chunks) {
    const meta = getBlfChunkMeta(chunk);

    if (offset + HEADER_SIZE > buffer.length) {
      throw new BlfError(
        `${meta.signature} Chunk not found: file ends at offset ${offset}`
      );
    }

    const header = c.read(
      s_blf_header,
      buffer.subarray(offset, offset + HEADER_SIZE),
      "big"
    );

    if (
      header.signature !== meta.signature ||
      header.major !== meta.major ||
      header.minor !== meta.minor
    ) {
      throw new BlfError(
        `${meta.signature} Chunk not found: expected ${meta.signature} ${meta.major}.${meta.minor} at offset ${offset}, got ${header.signature} ${header.major}.${header.minor}`
      );
    }

    if (
      header.chunk_length < HEADER_SIZE ||
      offset + header.chunk_length > buffer.length
    ) {
      throw new BlfError(
        `${meta.signature} chunk at offset ${offset} has invalid chunk_length ${header.chunk_length}`
      );
    }

    chunk.read_body(
      buffer.subarray(offset + HEADER_SIZE, offset + header.chunk_length),
      endian
    );

    const file_size = (chunk as { file_size?: unknown }).file_size;
    if (
      meta.signature === "_eof" &&
      typeof file_size === "number" &&
      file_size !== offset
    ) {
      throw new BlfError(
        `_eof has an invalid size: file_size ${file_size}, expected ${offset}`
      );
    }

    offset += header.chunk_length;
  }
}
