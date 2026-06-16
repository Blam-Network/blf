/**
 * Local-only megalo roundtrip tests against Desktop gametype fixtures.
 * Not intended for CI — skips when the fixture folder is absent.
 */
import { mkdirSync, readFileSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { basename, dirname, join, relative } from "node:path";
import { c } from "@craftycodie/cstruct";
import { s_blf_chunk_packed_game_variant } from "../../src/chunks/haloreach/v12065_11_08_24_1738_tu1actual/s_blf_chunk_packed_game_variant";
import { s_blf_chunk_packed_game_variant as s_blf_chunk_packed_game_variant_mcc } from "../../src/chunks/haloreach_mcc/v_untracked_25_08_16_1352/s_blf_chunk_packed_game_variant";
import {
  type BLFChunk,
  getBlfChunkMeta,
  search_for_chunk,
} from "../../src/blf_chunk";
import { s_blf_header } from "../../src/s_blf_header";
import { describe, expect, it } from "vitest";

const GAMETYPES_ROOT = "C:/Users/codie/Desktop/gametypes";
const OUTPUT_ROOT = join(GAMETYPES_ROOT, "_roundtrip_out");
const ENDIAN = "big" as const;

const fixturesPresent = (() => {
  try {
    return statSync(join(GAMETYPES_ROOT, "release")).isDirectory();
  } catch {
    return false;
  }
})();

function collectFiles(dir: string, extension: string): string[] {
  const files: string[] = [];
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const path = join(dir, entry.name);
    if (entry.isDirectory()) {
      files.push(...collectFiles(path, extension));
      continue;
    }
    if (entry.name.toLowerCase().endsWith(`.${extension}`)) {
      files.push(path);
    }
  }
  return files.sort();
}

function findChunkBodyRange(
  buffer: Uint8Array,
  signature: string,
  major: number,
  minor: number
): { bodyStart: number; bodyEnd: number } | undefined {
  const headerSize = c.sizeof(s_blf_header);

  for (let offset = 0; offset + headerSize <= buffer.length; offset++) {
    let header: s_blf_header;
    try {
      header = c.read(
        s_blf_header,
        buffer.subarray(offset, offset + headerSize),
        ENDIAN
      );
    } catch {
      continue;
    }

    if (
      header.signature !== signature ||
      header.major !== major ||
      header.minor !== minor
    ) {
      continue;
    }

    const chunkEnd = offset + header.chunk_length;
    if (chunkEnd > buffer.length) {
      continue;
    }

    return {
      bodyStart: offset + headerSize,
      bodyEnd: chunkEnd,
    };
  }

  return undefined;
}

function roundtripBlfGvar(
  original: Uint8Array,
  ChunkType: new () => BLFChunk
): Uint8Array {
  const chunk = new ChunkType();
  expect(search_for_chunk(original, chunk, ENDIAN)).toBe(true);

  const meta = getBlfChunkMeta(chunk);
  const range = findChunkBodyRange(
    original,
    meta.signature,
    meta.major,
    meta.minor
  );
  expect(range).toBeDefined();

  const rewrittenBody = chunk.write_body(ENDIAN);
  const originalBody = original.subarray(range!.bodyStart, range!.bodyEnd);
  expect(rewrittenBody.length).toBe(originalBody.length);

  const output = new Uint8Array(original);
  output.set(rewrittenBody, range!.bodyStart);
  return output;
}

function roundtripFile(
  inputPath: string,
  inputRoot: string,
  outputRoot: string,
  roundtrip: (bytes: Uint8Array) => Uint8Array
): void {
  const original = new Uint8Array(readFileSync(inputPath));
  const rewritten = roundtrip(original);
  expect(Array.from(rewritten)).toEqual(Array.from(original));

  const relativePath = relative(inputRoot, inputPath);
  const outputPath = join(outputRoot, relativePath);
  mkdirSync(dirname(outputPath), { recursive: true });
  writeFileSync(outputPath, rewritten);
}

describe.skipIf(!fixturesPresent)("local megalo roundtrip fixtures", () => {
  it(
    "round-trips release BLF gvar chunks (TU1)",
    () => {
    const inputRoot = join(GAMETYPES_ROOT, "release");
    const outputRoot = join(OUTPUT_ROOT, "release");
    const files = collectFiles(inputRoot, "bin");
    expect(files.length).toBeGreaterThan(0);

    const failures: string[] = [];
    let passed = 0;

    for (const file of files) {
      try {
        roundtripFile(file, inputRoot, outputRoot, (bytes) =>
          roundtripBlfGvar(bytes, s_blf_chunk_packed_game_variant)
        );
        passed++;
      } catch (error) {
        failures.push(`${basename(file)}: ${String(error)}`);
      }
    }

    if (failures.length > 0) {
      throw new Error(
        `${failures.length} release TU1 failures (${passed} passed)\n${failures.slice(0, 20).join("\n")}`
      );
    }
  },
    120_000
  );

  it(
    "round-trips release BLF gvar chunks (MCC codec, same files)",
    () => {
    const inputRoot = join(GAMETYPES_ROOT, "release");
    const outputRoot = join(OUTPUT_ROOT, "release-mcc");
    const files = collectFiles(inputRoot, "bin");
    expect(files.length).toBeGreaterThan(0);

    const failures: string[] = [];
    let passed = 0;

    for (const file of files) {
      try {
        roundtripFile(file, inputRoot, outputRoot, (bytes) =>
          roundtripBlfGvar(bytes, s_blf_chunk_packed_game_variant_mcc)
        );
        passed++;
      } catch (error) {
        failures.push(`${basename(file)}: ${String(error)}`);
      }
    }

    if (failures.length > 0) {
      throw new Error(
        `${failures.length} release MCC failures (${passed} passed)\n${failures.slice(0, 20).join("\n")}`
      );
    }
  },
    120_000
  );
});