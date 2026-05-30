import { c } from "@craftycodie/cstruct";
import type { ChunkMetadata } from "./decorators";
import { chunkMatches, getBlfChunkMeta } from "./decorators";
import { BlfError } from "./error";
import { parse_blf_chunk_version, s_blf_header } from "./s_blf_header";

export {
  BLF_CHUNK_META,
  type BLFChunkInfo,
  type BLFSignature,
  type BLFVersion,
  blf,
  type ChunkDecorator,
  type ChunkMetadata,
  chunk,
  createBlfChunk,
  getBlfChunkMeta,
} from "./decorators";
export { parse_blf_chunk_version, s_blf_header };

/** BLF chunk headers are always big-endian. */
const BLF_HEADER_ENDIAN = "big" as const;

/** BLF chunk I/O; metadata from `@blf.chunk`. */
export interface BLFChunk {
  /** Parse header + payload. `endian` is payload only; the 12-byte header is always big-endian. */
  read(bytes: Uint8Array, endian: c.Endian): void;
  /** Parse chunk payload only (no 12-byte BLF header). */
  read_body(payload: Uint8Array, endian: c.Endian): void;
  /** Serialize header + payload. `endian` is payload only; the 12-byte header is always big-endian. */
  write(endian: c.Endian): Uint8Array;
  /** Serialize chunk payload only. */
  write_body(endian: c.Endian): Uint8Array;
}

/** Default {@link BLFChunk.read} / {@link BLFChunk.write}. */
export abstract class BLFChunkBase implements BLFChunk {
  abstract read_body(payload: Uint8Array, endian: c.Endian): void;
  abstract write_body(endian: c.Endian): Uint8Array;

  read(bytes: Uint8Array, endian: c.Endian): void {
    const header = c.read(
      s_blf_header,
      bytes.subarray(0, c.sizeof(s_blf_header)),
      BLF_HEADER_ENDIAN
    );
    const meta = getBlfChunkMeta(this);

    if (
      header.signature !== meta.signature ||
      header.major !== meta.major ||
      header.minor !== meta.minor
    ) {
      throw new BlfError(
        `BLF chunk header mismatch: expected ${meta.signature} ${meta.major}.${meta.minor}, got ${header.signature} ${header.major}.${header.minor}`
      );
    }

    if (bytes.length < header.chunk_length) {
      throw new BlfError(
        `BLF chunk buffer too short: chunk_length ${header.chunk_length}, got ${bytes.length}`
      );
    }

    const payload = bytes.subarray(c.sizeof(s_blf_header), header.chunk_length);
    this.read_body(payload, endian);
  }

  write(endian: c.Endian): Uint8Array {
    const body = this.write_body(endian);
    const { signature, major, minor } = getBlfChunkMeta(this);
    const header_bytes = c.write(
      s_blf_header,
      s_blf_header.create(
        signature,
        body.length + c.sizeof(s_blf_header),
        major,
        minor
      ),
      BLF_HEADER_ENDIAN
    );

    const out = new Uint8Array(header_bytes.length + body.length);
    out.set(header_bytes, 0);
    out.set(body, header_bytes.length);
    return out;
  }
}

/**
 * Default {@link BLFChunk} read/write for classes decorated with `@c.struct`.
 * Apply `@blf.chunk` above `@c.struct` and extend this base class.
 */
export abstract class CStructBLFChunk extends BLFChunkBase {
  read_body(payload: Uint8Array, endian: c.Endian): void {
    const ctor = this.constructor as new () => object;
    const info = this as unknown as IBLFChunk;
    const size = c.sizeof(ctor);

    if (payload.length < size) {
      throw new BlfError(
        `Cannot read ${info.signature} chunk: need at least ${size} bytes, got ${payload.length}`
      );
    }

    Object.assign(this, c.read(ctor, payload, endian));
  }

  write_body(endian: c.Endian): Uint8Array {
    const ctor = this.constructor as new () => object;
    return c.write(ctor, this, endian);
  }
}

/** Full chunk instance (I/O + header metadata). */
export interface IBLFChunk extends BLFChunk, ChunkMetadata {}

/** Class decorated with `@blf.chunk` (instance type must implement {@link BLFChunk}). */
export type BLFChunkConstructor<T extends BLFChunk = BLFChunk> = new (
  ...args: any[]
) => T;

function tryReadHeader(
  buffer: Uint8Array,
  byte_offset: number
): s_blf_header | null {
  if (byte_offset + c.sizeof(s_blf_header) > buffer.length) {
    return null;
  }
  try {
    return c.read(
      s_blf_header,
      buffer.subarray(byte_offset, byte_offset + c.sizeof(s_blf_header)),
      BLF_HEADER_ENDIAN
    );
  } catch {
    return null;
  }
}

function isChunkLengthValid(
  chunk_length: number,
  byte_offset: number,
  buffer_length: number
): boolean {
  return (
    Number.isInteger(chunk_length) &&
    chunk_length >= c.sizeof(s_blf_header) &&
    byte_offset + chunk_length <= buffer_length
  );
}

function loadChunkAt(
  buffer: Uint8Array,
  byte_offset: number,
  header: s_blf_header,
  chunk: BLFChunk,
  endian: c.Endian
): void {
  const chunk_end = byte_offset + header.chunk_length;
  const payload = buffer.subarray(
    byte_offset + c.sizeof(s_blf_header),
    chunk_end
  );
  chunk.read_body(payload, endian);
}

/** Walk a BLF file sequentially until `chunk` is found, then read it in place. */
export function find_chunk(
  buffer: Uint8Array,
  chunk: BLFChunk,
  endian: c.Endian
): boolean {
  const meta = getBlfChunkMeta(chunk);
  let offset = 0;

  while (offset + c.sizeof(s_blf_header) <= buffer.length) {
    let header: s_blf_header;
    try {
      header = c.read(
        s_blf_header,
        buffer.subarray(offset, offset + c.sizeof(s_blf_header)),
        BLF_HEADER_ENDIAN
      );
    } catch {
      return false;
    }

    if (chunkMatches(header, meta)) {
      if (!isChunkLengthValid(header.chunk_length, offset, buffer.length)) {
        throw new BlfError(
          `Matched chunk "${header.signature}" at offset ${offset} has invalid chunk_length ${header.chunk_length}`
        );
      }
      loadChunkAt(buffer, offset, header, chunk, endian);
      return true;
    }

    if (!isChunkLengthValid(header.chunk_length, offset, buffer.length)) {
      return false;
    }

    offset += header.chunk_length;
  }

  return false;
}

/** Scan every byte offset for `chunk`, then read it in place when found. */
export function search_for_chunk(
  buffer: Uint8Array,
  chunk: BLFChunk,
  endian: c.Endian
): boolean {
  const meta = getBlfChunkMeta(chunk);
  const last_offset = buffer.length - c.sizeof(s_blf_header);
  const sig = meta.signature;

  for (let offset = 0; offset <= last_offset; offset++) {
    if (
      buffer[offset] !== sig.charCodeAt(0) ||
      buffer[offset + 1] !== sig.charCodeAt(1) ||
      buffer[offset + 2] !== sig.charCodeAt(2) ||
      buffer[offset + 3] !== sig.charCodeAt(3)
    ) {
      continue;
    }

    const header = tryReadHeader(buffer, offset);

    if (header === null || !chunkMatches(header, meta)) {
      continue;
    }

    if (!isChunkLengthValid(header.chunk_length, offset, buffer.length)) {
      continue;
    }

    try {
      // We've encoutered files that have fragmeneted chunks before the actual blf file starts,
      // so we ignore read errors so we can try the next matching header.
      loadChunkAt(buffer, offset, header, chunk, endian);
    } catch {
      continue;
    }

    return true;
  }

  return false;
}
