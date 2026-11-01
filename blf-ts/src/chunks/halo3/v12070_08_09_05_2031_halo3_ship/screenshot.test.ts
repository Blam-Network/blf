import { c } from "@craftycodie/cstruct";
import { describe, expect, it } from "vitest";
import { s_content_item_metadata } from "../../../blam/halo3/v12070_08_09_05_2031_halo3_ship/saved_games/saved_game_files";
import { write_blffile } from "../../../blffile";
import { read_blind_screenshot as read_11637_blind_screenshot } from "../v11637_07_08_02_2348_release/screenshot";
import { read_content_header } from "./content_header";
import { s_blf_chunk_compressed_data } from "./s_blf_chunk_compressed_data";
import { s_blf_chunk_content_header } from "./s_blf_chunk_content_header";
import { s_blf_chunk_end_of_file } from "./s_blf_chunk_end_of_file";
import {
  render_camera,
  render_projection,
  s_blf_chunk_screenshot_camera,
  s_saved_camera,
} from "./s_blf_chunk_screenshot_camera";
import { s_blf_chunk_screenshot_data } from "./s_blf_chunk_screenshot_data";
import { s_blf_chunk_start_of_file } from "./s_blf_chunk_start_of_file";
import { read_blind_screenshot, read_fileshare_screenshot } from "./screenshot";

const jpeg = new Uint8Array([0xff, 0xd8, 0xff, 0xe0, 1, 2, 3, 0xff, 0xd9]);
const date = new Date("2024-07-07T13:23:11.000Z");

function make_chunks() {
  const chdr = new s_blf_chunk_content_header();
  chdr.build_number = 12070;
  chdr.metadata.unique_id = 9991926892298367423n;
  chdr.metadata.name = "7 on the 7th";
  chdr.metadata.description = "Happy Bungie Day!";
  chdr.metadata.author = "craftycodie";
  chdr.metadata.file_type = 13;
  chdr.metadata.author_is_xuid_online = true;
  chdr.metadata.author_id = 0x0009_01fc_3fb8_fe71n;
  chdr.metadata.size_in_bytes = BigInt(jpeg.length);
  chdr.metadata.date = date;
  chdr.metadata.campaign_id = -1;
  chdr.metadata.map_id = 340;
  chdr.metadata.game_engine_type = 2;
  chdr.metadata.campaign_difficulty = 1;
  chdr.metadata.hopper_id = -1;
  chdr.metadata.game_id = 9455406705681050547n;

  const scnc = new s_blf_chunk_screenshot_camera();
  scnc.jpeg_data_length = jpeg.length;
  scnc.camera.camera.position = { x: 50.25, y: -88, z: 3.5 };
  scnc.camera.camera.render_pixel_bounds = {
    top: 0,
    left: 0,
    bottom: 640,
    right: 1152,
  };
  scnc.camera.projection.projection_matrix[3][2] = 0.5;
  scnc.game_tick = 1215;
  scnc.film_tick = 1232;

  const scnd = new s_blf_chunk_screenshot_data();
  scnd.jpeg_data = jpeg;

  return {
    _blf: s_blf_chunk_start_of_file.create("halo 3 saved screenshot"),
    chdr,
    scnc,
    scnd,
  };
}

function blind_file(): Uint8Array {
  const { _blf, chdr, scnc, scnd } = make_chunks();
  return write_blffile("big", [
    _blf,
    chdr,
    scnc,
    scnd,
    new s_blf_chunk_end_of_file(),
  ]);
}

function fileshare_file(): Uint8Array {
  const { _blf, chdr, scnc, scnd } = make_chunks();
  return write_blffile("big", [
    _blf,
    chdr,
    scnc,
    s_blf_chunk_compressed_data.create(s_blf_chunk_screenshot_data, scnd),
    new s_blf_chunk_end_of_file(),
  ]);
}

describe("halo3 12070 screenshot layouts", () => {
  it("match blf_lib sizes", () => {
    expect(c.sizeof(s_content_item_metadata)).toBe(0xf8);
    expect(s_blf_chunk_content_header.schema.getSize()).toBe(0xfc);
    expect(c.sizeof(render_camera)).toBe(0x88);
    expect(c.sizeof(render_projection)).toBe(0xc0);
    expect(c.sizeof(s_saved_camera)).toBe(0x158);
    expect(s_blf_chunk_screenshot_camera.schema.getSize()).toBe(0x164);
  });

  it("place metadata fields at their blf_lib offsets", () => {
    const { chdr } = make_chunks();
    const body = chdr.write_body("big");
    const view = new DataView(body.buffer, body.byteOffset);

    expect(view.getUint16(0)).toBe(12070);
    expect(view.getBigUint64(4)).toBe(9991926892298367423n);
    expect(view.getInt32(4 + 0xb8)).toBe(13); // file_type
    expect(view.getUint8(4 + 0xbc)).toBe(1); // author_is_xuid_online
    expect(view.getBigUint64(4 + 0xc0)).toBe(0x0009_01fc_3fb8_fe71n);
    expect(view.getBigInt64(4 + 0xd0)).toBe(BigInt(date.getTime() / 1000));
    expect(view.getInt32(4 + 0xe0)).toBe(340); // map_id
    expect(view.getInt16(4 + 0xec)).toBe(-1); // hopper_id
    expect(view.getBigUint64(4 + 0xf0)).toBe(9455406705681050547n);
  });
});

describe("read_blind_screenshot (12070)", () => {
  it("reads a blind screenshot", () => {
    const screenshot = read_blind_screenshot(blind_file());
    expect(screenshot).not.toBeNull();
    const { _blf, chdr, scnc, scnd, _eof } = screenshot!;

    expect(_blf.name).toBe("halo 3 saved screenshot");
    expect(chdr.build_number).toBe(12070);
    expect(chdr.metadata.name).toBe("7 on the 7th");
    expect(chdr.metadata.author).toBe("craftycodie");
    expect(chdr.metadata.author_id).toBe(0x0009_01fc_3fb8_fe71n);
    expect(chdr.metadata.date).toEqual(date);
    expect(chdr.metadata.hopper_id).toBe(-1);
    expect(scnc.jpeg_data_length).toBe(jpeg.length);
    expect(scnc.camera.camera.position).toEqual({ x: 50.25, y: -88, z: 3.5 });
    expect(scnc.camera.camera.render_pixel_bounds).toEqual({
      top: 0,
      left: 0,
      bottom: 640,
      right: 1152,
    });
    expect(scnc.camera.projection.projection_matrix[3][2]).toBe(0.5);
    expect(scnc.game_tick).toBe(1215);
    expect(scnc.film_tick).toBe(1232);
    expect(scnd.jpeg_data).toEqual(jpeg);
    expect(_eof.file_size).toBe(blind_file().length - 17);
  });

  it("returns null for other files", () => {
    expect(read_blind_screenshot(fileshare_file())).toBeNull();
    expect(read_blind_screenshot(new Uint8Array(0))).toBeNull();
    const truncated = blind_file();
    expect(
      read_blind_screenshot(truncated.subarray(0, truncated.length - 1))
    ).toBeNull();
  });

  it("is not an 11637 blind screenshot (it has a camera chunk)", () => {
    expect(read_11637_blind_screenshot(blind_file())).toBeNull();
  });
});

describe("read_blind_screenshot (11637)", () => {
  it("reads a screenshot without a camera chunk", () => {
    const { _blf, chdr, scnd } = make_chunks();
    const bytes = write_blffile("big", [
      _blf,
      chdr,
      scnd,
      new s_blf_chunk_end_of_file(),
    ]);

    const screenshot = read_11637_blind_screenshot(bytes);
    expect(screenshot?.chdr.metadata.name).toBe("7 on the 7th");
    expect(screenshot?.scnd.jpeg_data).toEqual(jpeg);
    expect(read_blind_screenshot(bytes)).toBeNull();
  });
});

describe("read_fileshare_screenshot", () => {
  it("reads a file share screenshot with compressed jpeg data", () => {
    const screenshot = read_fileshare_screenshot(fileshare_file());
    expect(screenshot).not.toBeNull();
    expect(screenshot!.chdr.metadata.game_id).toBe(9455406705681050547n);
    expect(screenshot!.scnc.game_tick).toBe(1215);
    expect(screenshot!.scnd.chunk.jpeg_data).toEqual(jpeg);
  });

  it("returns null for a blind screenshot", () => {
    expect(read_fileshare_screenshot(blind_file())).toBeNull();
  });
});

describe("read_content_header (12070)", () => {
  it("finds chdr 9.2", () => {
    const chdr = read_content_header(fileshare_file());
    expect(chdr?.build_number).toBe(12070);
    expect(chdr?.metadata.description).toBe("Happy Bungie Day!");
  });

  it("returns null without a chdr 9.2", () => {
    const bytes = write_blffile("big", [
      s_blf_chunk_start_of_file.create("x"),
      new s_blf_chunk_end_of_file(),
    ]);
    expect(read_content_header(bytes)).toBeNull();
    expect(read_content_header(new Uint8Array(3))).toBeNull();
  });
});
