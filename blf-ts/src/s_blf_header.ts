import { c } from "@craftycodie/cstruct";
import { BlfError } from "./error";
import { chunkSignature } from "./chunk_signature";

export function parse_blf_chunk_version(version: number | string) {
  if (typeof version === "number") {
    version = version.toString();
  }

  const [majorStr, minorStr = "0"] = version.split(".");
  const major = Number.parseInt(majorStr, 10);
  const minor = Number.parseInt(minorStr, 10);

  if (Number.isNaN(major) || Number.isNaN(minor)) {
    throw new BlfError(
      `BLF chunk version must be major.minor (e.g. "1.1"), got "${version}"`,
    );
  }

  return { major, minor };
}
@c.struct()
export class s_blf_header {
  @c.field(chunkSignature())
  signature = "";

  @c.field("u32")
  chunk_length = 0;

  @c.field("u16")
  major = 0;

  @c.field("u16")
  minor = 0;

  static create(
    signature: string,
    chunk_length: number,
    major: number,
    minor: number,
  ): s_blf_header {
    if (signature.length !== 4) {
      throw new BlfError(
        `BLF chunk signature must be exactly 4 characters, got ${signature.length}: "${signature}"`,
      );
    }
    if (!Number.isInteger(chunk_length) || chunk_length < 0) {
      throw new BlfError(
        `BLF chunk length must be a non-negative integer, got ${chunk_length}`,
      );
    }
    const header = new s_blf_header();
    header.signature = signature;
    header.chunk_length = chunk_length;
    header.major = major;
    header.minor = minor;
    return header;
  }

  /** Build from a `major.minor` float (e.g. `1.1`). */
  static from_version_float(
    signature: string,
    chunk_length: number,
    version: number,
  ): s_blf_header {
    const { major, minor } = parse_blf_chunk_version(version);
    return s_blf_header.create(signature, chunk_length, major, minor);
  }
}
