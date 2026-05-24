import { describe, expect, it } from "vitest";
import { c } from "@craftycodie/cstruct";
import { s_blf_header } from "./s_blf_header";

describe("chunkSignature", () => {
  it("keeps character order on big-endian", () => {
    const header = s_blf_header.create("test", 13, 1, 2);
    const written = c.write(s_blf_header, header, "big");
    expect(written.subarray(0, 4)).toEqual(new Uint8Array([0x74, 0x65, 0x73, 0x74]));

    const read = c.read(s_blf_header, written, "big");
    expect(read.signature).toBe("test");
  });

  it("reverses character order on little-endian", () => {
    const header = s_blf_header.create("test", 13, 1, 2);
    const written = c.write(s_blf_header, header, "little");
    expect(written.subarray(0, 4)).toEqual(new Uint8Array([0x74, 0x73, 0x65, 0x74]));

    const read = c.read(s_blf_header, written, "little");
    expect(read.signature).toBe("test");
  });

  it("rejects invalid signatures", () => {
    expect(() => s_blf_header.create("toolong", 12, 1, 1)).toThrow(
      /exactly 4 characters/,
    );
    expect(() => s_blf_header.create("ab", 12, 1, 1)).toThrow(
      /exactly 4 characters/,
    );
  });
});
