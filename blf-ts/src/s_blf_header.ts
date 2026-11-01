import { c } from "@craftycodie/cstruct";
import { chunkSignature } from "./chunk_signature";
import { BlfError } from "./error";

export function parse_blf_chunk_version(version: number | string) {
  if (typeof version === "number") {
    version = version.toString();
  }

  const [majorStr, minorStr = "0"] = version.split(".");
  const major = Number.parseInt(majorStr, 10);
  const minor = Number.parseInt(minorStr, 10);

  if (Number.isNaN(major) || Number.isNaN(minor)) {
    throw new BlfError(
      `BLF chunk version must be major.minor (e.g. "1.1"), got "${version}"`
    );
  }

  return { major, minor };
}
export const s_blf_header = c.struct({
  signature: chunkSignature(),
  chunk_length: c.u32(),
  major: c.u16(),
  minor: c.u16(),
});
export type s_blf_header = c.infer<typeof s_blf_header>;

export function create_blf_header(
  signature: string,
  chunk_length: number,
  major: number,
  minor: number
): s_blf_header {
  if (signature.length !== 4) {
    throw new BlfError(
      `BLF chunk signature must be exactly 4 characters, got ${signature.length}: "${signature}"`
    );
  }
  if (!Number.isInteger(chunk_length) || chunk_length < 0) {
    throw new BlfError(
      `BLF chunk length must be a non-negative integer, got ${chunk_length}`
    );
  }
  return { signature, chunk_length, major, minor };
}

/** Build from a `major.minor` float (e.g. `1.1`). */
export function blf_header_from_version_float(
  signature: string,
  chunk_length: number,
  version: number
): s_blf_header {
  const { major, minor } = parse_blf_chunk_version(version);
  return create_blf_header(signature, chunk_length, major, minor);
}
