import { inflate } from "pako";
import {
  blf,
  BLFChunk,
  BLFChunkBase,
  BLFChunkConstructor,
  createBlfChunk,
  getBlfChunkMeta,
} from "../../../blf_chunk";
import { find_chunk } from "../../../blf_chunk";
import type { c } from "@craftycodie/cstruct";
import { BlfError } from "../../../error";
import { zlib_compress } from "../../../blam/common/memory/data_compress";

/**
 * Halo 3 `_cmp` (1.1): zlib-compressed wrapper around another BLF chunk.
 *
 * Payload layout (big-endian): `u8 compression_type`, `u32 uncompressed_size`,
 * zlib bytes (RFC 1950). Uncompressed data is a full inner BLF chunk (12-byte
 * header + payload).
 */
@blf.chunk("_cmp", 1.1)
export class s_blf_chunk_compressed_data<T extends BLFChunk = BLFChunk>
  extends BLFChunkBase
{
  readonly InnerCtor: BLFChunkConstructor<T>;
  compression_type = 0;
  chunk: T;

  constructor(InnerCtor: BLFChunkConstructor<T>, chunk?: T) {
    super();
    this.InnerCtor = InnerCtor;
    this.chunk = chunk ?? createBlfChunk(InnerCtor);
  }

  static create<C extends BLFChunk>(
    InnerCtor: BLFChunkConstructor<C>,
    chunk: C,
  ): s_blf_chunk_compressed_data<C> {
    const wrapper = new s_blf_chunk_compressed_data(InnerCtor, chunk);
    wrapper.compression_type = 0;
    return wrapper;
  }

  /** Decompress `_cmp` body bytes (type + BE size + zlib) without parsing the inner chunk. */
  static decompress_payload(payload: Uint8Array): Uint8Array {
    if (payload.length < 5) {
      throw new BlfError(
        `_cmp payload needs at least 5 bytes, got ${payload.length}`,
      );
    }

    const uncompressed_size = new DataView(
      payload.buffer,
      payload.byteOffset,
    ).getUint32(1, false);
    const compressed = payload.subarray(5);
    const uncompressed = inflate(compressed);
    if (uncompressed.length !== uncompressed_size) {
      throw new BlfError(
        `_cmp zlib decompressed ${uncompressed.length} bytes, expected ${uncompressed_size}`,
      );
    }
    return uncompressed;
  }

  read_body(payload: Uint8Array, endian: c.Endian): void {
    if (payload.length < 5) {
      throw new BlfError(
        `_cmp payload needs at least 5 bytes, got ${payload.length}`,
      );
    }

    this.compression_type = payload[0]!;
    const uncompressed = s_blf_chunk_compressed_data.decompress_payload(payload);

    const chunk = createBlfChunk(this.InnerCtor);
    if (!find_chunk(uncompressed, chunk, endian)) {
      const expected = getBlfChunkMeta(this.InnerCtor);
      throw new BlfError(
        `Unexpected compressed chunk: expected ${expected.signature} ${expected.major}.${expected.minor} not found in decompressed body`,
      );
    }
    this.chunk = chunk;
  }

  write_body(endian: c.Endian): Uint8Array {
    const inner_bytes = this.chunk.write(endian);
    const compressed = zlib_compress(inner_bytes);

    const payload = new Uint8Array(1 + 4 + compressed.length);
    payload[0] = this.compression_type;
    new DataView(payload.buffer).setUint32(1, inner_bytes.length, false);
    payload.set(compressed, 5);
    return payload;
  }
}
