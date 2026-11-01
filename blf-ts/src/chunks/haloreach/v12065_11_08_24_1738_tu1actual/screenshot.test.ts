import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { reach_12065_oddball_fixture } from "../../../../tests/fixtures/paths";
import { search_for_chunk } from "../../../blf_chunk";
import { write_blffile } from "../../../blffile";
import { s_blf_chunk_compressed_data } from "../../halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_compressed_data";
import { s_blf_chunk_end_of_file } from "../../halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_end_of_file";
import { s_blf_chunk_screenshot_data as halo3_s_blf_chunk_screenshot_data } from "../../halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_screenshot_data";
import { s_blf_chunk_start_of_file } from "../../halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_start_of_file";
import { read_blind_screenshot as read_09449_blind_screenshot } from "../v09449_10_03_25_1545_omaha_beta/screenshot";
import { read_blind_screenshot as read_09730_blind_screenshot } from "../v09730_10_04_09_1309_omaha_delta/screenshot";
import { read_content_header } from "./content_header";
import { s_blf_chunk_content_header } from "./s_blf_chunk_content_header";
import { s_blf_chunk_screenshot_camera } from "./s_blf_chunk_screenshot_camera";
import { s_blf_chunk_screenshot_data } from "./s_blf_chunk_screenshot_data";
import { s_blf_chunk_server_signature } from "./s_blf_chunk_server_signature";
import { read_blind_screenshot } from "./screenshot";

const fixture = new Uint8Array(readFileSync(reach_12065_oddball_fixture));
const jpeg = new Uint8Array([0xff, 0xd8, 0xff, 0xe0, 0xff, 0xd9]);

/** The fixture is a CON package: its BLF doesn't start at offset 0. */
function fixture_chdr(): s_blf_chunk_content_header {
  const chdr = new s_blf_chunk_content_header();
  if (!search_for_chunk(fixture, chdr, "big")) {
    throw new Error("fixture has no chdr");
  }
  return chdr;
}

function release_file(): Uint8Array {
  const scnc = new s_blf_chunk_screenshot_camera();
  scnc.data = new Uint8Array(4964).fill(0x43);
  const scnd = new s_blf_chunk_screenshot_data();
  scnd.jpeg_data = jpeg;
  const ssig = new s_blf_chunk_server_signature();
  ssig.data = new Uint8Array(44).fill(0xff);

  return write_blffile("big", [
    s_blf_chunk_start_of_file.create("reach saved screenshot"),
    fixture_chdr(),
    scnc,
    scnd,
    ssig,
    new s_blf_chunk_end_of_file(),
  ]);
}

function beta_file(
  scnd: s_blf_chunk_screenshot_data | halo3_s_blf_chunk_screenshot_data
): Uint8Array {
  scnd.jpeg_data = jpeg;
  const cmp =
    scnd instanceof s_blf_chunk_screenshot_data
      ? s_blf_chunk_compressed_data.create(s_blf_chunk_screenshot_data, scnd)
      : s_blf_chunk_compressed_data.create(
          halo3_s_blf_chunk_screenshot_data,
          scnd
        );
  return write_blffile("big", [
    s_blf_chunk_start_of_file.create("reach saved screenshot"),
    fixture_chdr(),
    cmp,
    new s_blf_chunk_end_of_file(),
  ]);
}

describe("reach screenshot chunks", () => {
  it("have blf_lib sizes and versions", () => {
    expect(s_blf_chunk_screenshot_camera.schema.getSize()).toBe(4964);
    expect(s_blf_chunk_server_signature.schema.getSize()).toBe(44);

    const scnd = new s_blf_chunk_screenshot_data();
    scnd.jpeg_data = jpeg;
    const bytes = scnd.write("big");
    // scnd 1.2 header, BE length, jpeg bytes
    expect(Array.from(bytes.subarray(0, 4))).toEqual([0x73, 0x63, 0x6e, 0x64]);
    expect(new DataView(bytes.buffer).getUint16(8)).toBe(1);
    expect(new DataView(bytes.buffer).getUint16(10)).toBe(2);
    expect(new DataView(bytes.buffer).getUint32(12)).toBe(jpeg.length);
  });
});

describe("read_blind_screenshot (12065)", () => {
  it("reads a release screenshot", () => {
    const screenshot = read_blind_screenshot(release_file());
    expect(screenshot).not.toBeNull();
    expect(screenshot!._blf.name).toBe("reach saved screenshot");
    expect(screenshot!.chdr.build_number).toBe(12065);
    expect(screenshot!.chdr.metadata.name).toBe("Oddball");
    expect(screenshot!.scnc.data).toEqual(new Uint8Array(4964).fill(0x43));
    expect(screenshot!.scnd.jpeg_data).toEqual(jpeg);
    expect(screenshot!.ssig.data).toEqual(new Uint8Array(44).fill(0xff));
  });

  it("returns null for other files", () => {
    expect(read_blind_screenshot(fixture)).toBeNull();
    expect(
      read_blind_screenshot(beta_file(new s_blf_chunk_screenshot_data()))
    ).toBeNull();
  });
});

describe("read_blind_screenshot (09730)", () => {
  it("reads a public beta screenshot (_cmp of scnd 1.2)", () => {
    const screenshot = read_09730_blind_screenshot(
      beta_file(new s_blf_chunk_screenshot_data())
    );
    expect(screenshot?.chdr.metadata.name).toBe("Oddball");
    expect(screenshot?._cmp.chunk.jpeg_data).toEqual(jpeg);
  });

  it("rejects scnd 1.1 and release screenshots", () => {
    expect(
      read_09730_blind_screenshot(
        beta_file(new halo3_s_blf_chunk_screenshot_data())
      )
    ).toBeNull();
    expect(read_09730_blind_screenshot(release_file())).toBeNull();
  });
});

describe("read_blind_screenshot (09449)", () => {
  it("reads a private beta screenshot (_cmp of scnd 1.1)", () => {
    const screenshot = read_09449_blind_screenshot(
      beta_file(new halo3_s_blf_chunk_screenshot_data())
    );
    expect(screenshot?.chdr.metadata.name).toBe("Oddball");
    expect(screenshot?._cmp.chunk.jpeg_data).toEqual(jpeg);
  });

  it("rejects scnd 1.2", () => {
    expect(
      read_09449_blind_screenshot(beta_file(new s_blf_chunk_screenshot_data()))
    ).toBeNull();
  });
});

describe("read_content_header (12065)", () => {
  it("reads the chdr of a BLF file", () => {
    const chdr = read_content_header(release_file());
    expect(chdr?.build_number).toBe(12065);
    expect(chdr?.metadata.name).toBe("Oddball");
  });

  it("walks chunks from offset 0, like blf_lib's find_chunk", () => {
    // The chdr is inside a CON package, so a chunk walk doesn't reach it.
    expect(read_content_header(fixture)).toBeNull();
    expect(read_content_header(new Uint8Array(16))).toBeNull();
  });
});
