import { c } from "@craftycodie/cstruct";
import { describe, expect, it } from "vitest";
import { BLFChunkBase, blf, search_for_chunk } from "./blf_chunk";
import { s_blf_chunk_end_of_file } from "./chunks/halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_end_of_file";
import {
  e_byte_order_mark,
  s_blf_chunk_start_of_file,
} from "./chunks/halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_start_of_file";
import { write_blffile } from "./index";
import { s_blf_header } from "./s_blf_header";

@blf.chunk("test", 1.0)
class s_blf_chunk_test_stub extends BLFChunkBase {
  value = 0;

  read_body(payload: Uint8Array, endian: c.Endian): void {
    const view = new DataView(
      payload.buffer,
      payload.byteOffset,
      payload.byteLength
    );
    this.value = view.getUint32(0, endian === "little");
  }

  write_body(endian: c.Endian): Uint8Array {
    const out = new Uint8Array(4);
    new DataView(out.buffer).setUint32(0, this.value, endian === "little");
    return out;
  }
}

describe("s_blf_chunk_start_of_file", () => {
  it("round-trips _blf 1.2 (36-byte body)", () => {
    const original = s_blf_chunk_start_of_file.create("reach-test");
    original.byte_order_mark = e_byte_order_mark.little_endian;
    const bytes = original.write("big");

    expect(bytes.length).toBe(c.size(s_blf_header) + 36);

    const parsed = new s_blf_chunk_start_of_file();
    parsed.read(bytes, "big");
    expect(parsed.byte_order_mark).toBe(e_byte_order_mark.little_endian);
    expect(parsed.name).toBe("reach-test");
  });
});

describe("write_blffile", () => {
  it("writes _blf then _eof with patched file_size", () => {
    const start = s_blf_chunk_start_of_file.create("file");
    const eof = new s_blf_chunk_end_of_file();
    const file = write_blffile("big", [start, eof]);

    const blf = new s_blf_chunk_start_of_file();
    expect(search_for_chunk(file, blf, "big")).toBe(true);
    expect(blf.name).toBe("file");

    const eof_off = c.size(s_blf_header) + 36;
    const parsed_eof = new s_blf_chunk_end_of_file();
    parsed_eof.read(file.subarray(eof_off), "big");
    expect(parsed_eof.file_size).toBe(eof_off);
  });

  it("sets _eof file_size to bytes written before the eof chunk", () => {
    const body = new s_blf_chunk_test_stub();
    body.value = 0xdeadbeef;
    const eof = new s_blf_chunk_end_of_file();

    const file = write_blffile("big", [body, eof]);
    const expected_eof_offset = c.size(s_blf_header) + 4;

    const parsed = new s_blf_chunk_end_of_file();
    parsed.read(file.subarray(expected_eof_offset), "big");

    expect(parsed.file_size).toBe(expected_eof_offset);
    expect(file.length).toBe(expected_eof_offset + c.size(s_blf_header) + 5);
  });
});
