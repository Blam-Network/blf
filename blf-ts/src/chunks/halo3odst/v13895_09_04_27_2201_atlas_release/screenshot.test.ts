import { c } from "@craftycodie/cstruct";
import { describe, expect, it } from "vitest";
import {
  find_file_chunk,
  read_odst_blind_screenshot as read_blind_screenshot,
  read_halo3_blind_screenshot,
} from "../../../../tests/lib/blf_files";
import { s_content_item_metadata } from "../../../blam/halo3odst/v13895_09_04_27_2201_atlas_release/saved_games/saved_game_files";
import { write_blffile } from "../../../blffile";
import { s_blf_chunk_content_header as halo3_s_blf_chunk_content_header } from "../../halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_content_header";
import { s_blf_chunk_end_of_file } from "../../halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_end_of_file";
import { s_blf_chunk_screenshot_camera } from "../../halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_screenshot_camera";
import { s_blf_chunk_screenshot_data } from "../../halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_screenshot_data";
import { s_blf_chunk_start_of_file } from "../../halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_start_of_file";
import { s_blf_chunk_content_header } from "./s_blf_chunk_content_header";

const read_content_header = (buffer: Uint8Array) =>
  find_file_chunk(buffer, new s_blf_chunk_content_header());
const read_halo3_content_header = (buffer: Uint8Array) =>
  find_file_chunk(buffer, new halo3_s_blf_chunk_content_header());

const jpeg = new Uint8Array([0xff, 0xd8, 0xff, 0xd9]);

function make_file(): Uint8Array {
  const chdr = new s_blf_chunk_content_header();
  chdr.build_number = 13895;
  chdr.build_sequence_number = 7;
  chdr.metadata.name = "UPLIFT RESERVE";
  chdr.metadata.description = "UPLIFT RESERVE on Normal, 06/01/26";
  chdr.metadata.author = "craftycodie";
  chdr.metadata.author_id = 2535457702870641n;
  chdr.metadata.date = new Date("2026-05-31T16:47:03.000Z");
  chdr.metadata.campaign_id = 1;
  chdr.metadata.map_id = 6110;
  chdr.metadata.campaign_insertion_point = 3;
  chdr.metadata.campaign_survival_enabled = true;
  chdr.metadata.game_id = 3639935025416717465n;

  const scnc = new s_blf_chunk_screenshot_camera();
  scnc.jpeg_data_length = jpeg.length;
  const scnd = new s_blf_chunk_screenshot_data();
  scnd.jpeg_data = jpeg;

  return write_blffile("big", [
    s_blf_chunk_start_of_file.create("odst saved screenshot"),
    chdr,
    scnc,
    scnd,
    new s_blf_chunk_end_of_file(),
  ]);
}

describe("odst content header", () => {
  it("matches the blf_lib layout", () => {
    expect(c.sizeof(s_content_item_metadata)).toBe(0xf8);
    expect(s_blf_chunk_content_header.schema.getSize()).toBe(0xfc);

    const chdr = new s_blf_chunk_content_header();
    chdr.metadata.campaign_insertion_point = -2;
    chdr.metadata.campaign_survival_enabled = true;
    chdr.metadata.game_id = 0x1122_3344_5566_7788n;
    const body = chdr.write_body("big");
    const view = new DataView(body.buffer, body.byteOffset);
    expect(view.getInt8(4 + 0xec)).toBe(-2);
    expect(view.getUint8(4 + 0xed)).toBe(1);
    expect(view.getBigUint64(4 + 0xf0)).toBe(0x1122_3344_5566_7788n);
  });
});

describe("read_blind_screenshot (odst)", () => {
  it("reads an ODST blind screenshot", () => {
    const screenshot = read_blind_screenshot(make_file());
    expect(screenshot).not.toBeNull();
    expect(screenshot!.chdr.build_number).toBe(13895);
    expect(screenshot!.chdr.build_sequence_number).toBe(7);
    expect(screenshot!.chdr.metadata.campaign_insertion_point).toBe(3);
    expect(screenshot!.chdr.metadata.campaign_survival_enabled).toBe(true);
    expect(screenshot!.scnc.jpeg_data_length).toBe(jpeg.length);
    expect(screenshot!.scnd.jpeg_data).toEqual(jpeg);
  });

  it("isn't a Halo 3 screenshot (chdr 9.3)", () => {
    expect(read_halo3_blind_screenshot(make_file())).toBeNull();
  });
});

describe("read_content_header (odst)", () => {
  it("finds chdr 9.3", () => {
    const chdr = read_content_header(make_file());
    expect(chdr?.metadata.name).toBe("UPLIFT RESERVE");
    expect(chdr?.metadata.date).toEqual(new Date("2026-05-31T16:47:03.000Z"));
    expect(read_halo3_content_header(make_file())).toBeNull();
  });
});
