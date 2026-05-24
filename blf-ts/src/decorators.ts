import type { BLFChunk, BLFChunkConstructor, s_blf_header } from "./blf_chunk";
import { BlfError } from "./error";
import { parse_blf_chunk_version } from "./s_blf_header";

type StringLength<
  S extends string,
  Acc extends unknown[] = [],
> = S extends `${string}${infer Rest}`
  ? StringLength<Rest, [...Acc, unknown]>
  : Acc["length"];

/** Exactly four characters (FourCC). */
export type BLFSignature<T extends string = string> = string extends T
  ? string
  : StringLength<T> extends 4
    ? T
    : never;

/** Major.minor version as a float literal, e.g. `1.1` (major 1, minor 1). */
export type BLFVersion<T extends number = number> = T;

/** Metadata attached by `@blf.chunk(sig, version)`. */
export type ChunkMetadata<
  TSignature extends BLFSignature<string> = BLFSignature,
  TVersion extends BLFVersion = BLFVersion,
> = {
  readonly signature: TSignature;
  readonly version: TVersion;
  readonly major: number;
  readonly minor: number;
};

export const BLF_CHUNK_META = Symbol.for("blf.chunk.meta");

export type BLFChunkInfo = ChunkMetadata;

/** Constructor after `@blf.chunk` — instances include {@link ChunkMetadata}. */
export type ChunkDecorator<
  T extends abstract new (
    ...args: any[]
  ) => BLFChunk,
  TSignature extends BLFSignature<string>,
  TVersion extends BLFVersion,
> = (abstract new (
  ...args: ConstructorParameters<T>
) => InstanceType<T> & ChunkMetadata<TSignature, TVersion>) &
  T & {
    readonly [BLF_CHUNK_META]: BLFChunkInfo & {
      signature: TSignature;
      version: TVersion;
    };
  };

/** Instance or constructor for a class decorated with `@blf.chunk`. */
export type BLFChunkLookup = BLFChunkConstructor | BLFChunk;

function attach_chunk_metadata(
  target: abstract new (...args: any[]) => BLFChunk,
  info: BLFChunkInfo
): void {
  Object.defineProperty(target, BLF_CHUNK_META, {
    value: info,
    enumerable: false,
    configurable: false,
  });

  for (const key of ["signature", "version", "major", "minor"] as const) {
    Object.defineProperty(target.prototype, key, {
      get(): number | string {
        return info[key];
      },
      enumerable: true,
      configurable: true,
    });
  }
}

function chunk_info<
  TSignature extends BLFSignature<string>,
  TVersion extends BLFVersion,
>(signature: TSignature, version: TVersion) {
  if (signature.length !== 4) {
    throw new BlfError(
      `BLF chunk signature must be exactly 4 characters, got ${signature.length}: "${signature}"`
    );
  }
  const parsed = parse_blf_chunk_version(version);
  return {
    signature,
    version,
    major: parsed.major,
    minor: parsed.minor,
  } as BLFChunkInfo & { signature: TSignature; version: TVersion };
}

/**
 * Registers BLF signature/version on the class (runtime getters + constructor meta).
 * Implement {@link BLFChunk} `read_body` / `write_body`, or extend {@link CStructBLFChunk} with `@c.struct`.
 */
export function chunk<
  TSignature extends BLFSignature<string>,
  TVersion extends BLFVersion,
>(signature: TSignature, version: TVersion) {
  const info = chunk_info(signature, version);

  return <T extends abstract new (...args: any[]) => BLFChunk>(
    target: T,
    _context: ClassDecoratorContext
  ): ChunkDecorator<T, TSignature, TVersion> => {
    attach_chunk_metadata(target, info);
    return target as ChunkDecorator<T, TSignature, TVersion>;
  };
}

/** `@blf.chunk` decorator and namespace. */
export const blf = { chunk } as const;

export function getBlfChunkMeta(from: BLFChunkLookup): BLFChunkInfo {
  if (typeof from === "function") {
    const meta = (
      from as BLFChunkConstructor & { [BLF_CHUNK_META]?: BLFChunkInfo }
    )[BLF_CHUNK_META];
    if (!meta) {
      throw new BlfError(`${from.name} is not decorated with @blf.chunk`);
    }
    return meta;
  }

  const inst = from as BLFChunk & ChunkMetadata;
  return {
    signature: inst.signature,
    version: inst.version,
    major: inst.major,
    minor: inst.minor,
  };
}

export function createBlfChunk<T extends new (...args: any[]) => object>(
  ctor: T
): InstanceType<T> {
  return new ctor() as InstanceType<T>;
}

export function chunkMatches(
  header: s_blf_header,
  meta: BLFChunkInfo
): boolean {
  return (
    header.signature === meta.signature &&
    header.major === meta.major &&
    header.minor === meta.minor
  );
}
