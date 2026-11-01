import { describe, expect, it } from "vitest";
import { write_blffile } from "./blffile";
import { s_blf_chunk_end_of_file } from "./chunks/halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_end_of_file";
import { s_blf_chunk_screenshot_data } from "./chunks/halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_screenshot_data";
import { s_blf_chunk_start_of_file } from "./chunks/halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_start_of_file";
import { s_blf_chunk_screenshot_data as reach_s_blf_chunk_screenshot_data } from "./chunks/haloreach/v12065_11_08_24_1738_tu1actual/s_blf_chunk_screenshot_data";
import { read_blffile } from "./read_blffile";

const SCND_NOT_FOUND = /scnd Chunk not found/;
const EOF_NOT_FOUND = /_eof Chunk not found/;
const EOF_INVALID_SIZE = /_eof has an invalid size/;

function sample_file(): Uint8Array {
  const scnd = new s_blf_chunk_screenshot_data();
  scnd.jpeg_data = new Uint8Array([0xff, 0xd8, 0xff, 0xd9]);
  return write_blffile("big", [
    s_blf_chunk_start_of_file.create("test"),
    scnd,
    new s_blf_chunk_end_of_file(),
  ]);
}

function read_sample(bytes: Uint8Array) {
  const chunks = {
    _blf: new s_blf_chunk_start_of_file(),
    scnd: new s_blf_chunk_screenshot_data(),
    _eof: new s_blf_chunk_end_of_file(),
  };
  read_blffile(bytes, "big", [chunks._blf, chunks.scnd, chunks._eof]);
  return chunks;
}

describe("read_blffile", () => {
  it("reads chunks in order", () => {
    const bytes = sample_file();
    const { _blf, scnd, _eof } = read_sample(bytes);

    expect(_blf.name).toBe("test");
    expect(scnd.jpeg_data).toEqual(new Uint8Array([0xff, 0xd8, 0xff, 0xd9]));
    expect(_eof.file_size).toBe(bytes.length - 17);
  });

  it("ignores bytes after the last chunk", () => {
    const bytes = sample_file();
    const padded = new Uint8Array(bytes.length + 8);
    padded.set(bytes);
    expect(() => read_sample(padded)).not.toThrow();
  });

  it("rejects a chunk with the wrong version", () => {
    const bytes = sample_file();
    const chunks = [
      new s_blf_chunk_start_of_file(),
      new reach_s_blf_chunk_screenshot_data(),
      new s_blf_chunk_end_of_file(),
    ];
    expect(() => read_blffile(bytes, "big", chunks)).toThrow(SCND_NOT_FOUND);
  });

  it("rejects missing chunks", () => {
    const bytes = sample_file();
    const chunks = [
      new s_blf_chunk_start_of_file(),
      new s_blf_chunk_end_of_file(),
    ];
    expect(() => read_blffile(bytes, "big", chunks)).toThrow(EOF_NOT_FOUND);
  });

  it("rejects a truncated file", () => {
    const bytes = sample_file();
    expect(() => read_sample(bytes.subarray(0, bytes.length - 1))).toThrow();
    expect(() => read_sample(bytes.subarray(0, 40))).toThrow();
  });

  it("rejects an _eof whose file_size doesn't match", () => {
    const bytes = sample_file().slice();
    const eof_offset = bytes.length - 17;
    new DataView(bytes.buffer).setUint32(eof_offset + 12, eof_offset + 1);
    expect(() => read_sample(bytes)).toThrow(EOF_INVALID_SIZE);
  });
});
