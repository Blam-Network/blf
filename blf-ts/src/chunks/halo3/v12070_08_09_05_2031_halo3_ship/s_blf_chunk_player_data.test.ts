import { describe, expect, it } from "vitest";
import { find_chunk } from "../../../blf_chunk";
import { s_blf_chunk_odst_service_record } from "../../halo3odst/v13895_09_04_27_2201_atlas_release/s_blf_chunk_odst_service_record";
import { s_blf_chunk_player_data as s_blf_chunk_player_data_v1 } from "../v08172_07_03_08_2240_delta/s_blf_chunk_player_data";
import { s_blf_chunk_player_data as s_blf_chunk_player_data_v2 } from "../v10015_07_05_14_2217_delta/s_blf_chunk_player_data";
import { s_blf_chunk_author } from "./s_blf_chunk_author";
import { s_blf_chunk_file_transfers } from "./s_blf_chunk_file_transfers";
import {
  build_user_file,
  e_bungienet_user_flags,
  s_blf_chunk_player_data,
} from "./s_blf_chunk_player_data";
import {
  Color,
  EliteArmour,
  Grade,
  PlayerModel,
  Rank,
  SpartanBody,
  SpartanHelmet,
  SpartanShoulder,
  s_blf_chunk_service_record,
} from "./s_blf_chunk_service_record";

// Expected payloads were produced by blf_lib (via @blam-network/blf_lsp) for the same inputs.
const k_fupd_3_1 =
  "000000070000000b0000000c64656661756c745f686f70706572730000000000000000000000000000000000";
const k_fupd_2_0 =
  "000000070000000c64656661756c745f686f70706572730000000000000000000000000000000000";
const k_fupd_1_0 = "000000070000000c";
const k_srid_2_1 =
  "0043006f00640069006500000000000000000000000000000000000000000000ff1e0319017f80a500011d1100000a0706080403020100420030004e00470000ffffffff00000032000003e8fffffffe0000000d000000030001e240";
const k_osri_2_0 =
  "0043006f00640069006500000000000000000000000000000000000000000000ff1e0319017f80a500011d1100000a0706080403020100420030004e00470000012a0000ffffffff00000032000003e8fffffffe0000000d000000030001e240";
const k_filq_entry =
  "0123456789abcdef000000020003000000000000000000010047007500610072006400690061006e0000000000000000000000000000000000000006ffffffff0000001f000000040000000000010000";
const k_athr_3_1_without_program_name =
  "0000000100002f2631323037302e30382e30392e30352e323033312e68616c6f335f736800000000000000000000000000000000";

const fupd = {
  hopper_access: 7,
  bungie_user_role:
    e_bungienet_user_flags.registered |
    e_bungienet_user_flags.pro_member |
    e_bungienet_user_flags.community,
  highest_skill: 12,
  hopper_directory: "default_hoppers",
};

const srid = {
  player_name: "Codie",
  appearance_flags: 0xff,
  primary_color: Color.INVALID_White,
  secondary_color: Color.Red,
  tertiary_color: Color.RubyWine,
  is_elite: PlayerModel.Elite,
  foreground_emblem: 0x7f,
  background_emblem: 0x80,
  emblem_flags: 0xa5,
  emblem_primary_color: Color.Silver,
  emblem_secondary_color: Color.Khaki,
  emblem_background_color: Color.Cyan,
  spartan_helmet: SpartanHelmet.Rogue,
  spartan_left_shoulder: SpartanShoulder.Scout,
  spartan_right_shoulder: SpartanShoulder.Marathon,
  spartan_body: SpartanBody.Bungie,
  elite_helmet: EliteArmour.Scythe,
  elite_left_shoulder: EliteArmour.Blades,
  elite_right_shoulder: EliteArmour.Raptor,
  elite_body: EliteArmour.Predator,
  service_tag: "B0NG",
  campaign_progress: -1,
  highest_skill: 50,
  total_exp: 1000,
  experience_base: -2,
  rank: Rank.General,
  grade: Grade.Grade4,
  games_completed: 123_456,
};

const osri = { ...srid, extras_portal_debug: true, vidmaster: 0x2a };

const transfer = {
  share_id: 0x01_23_45_67_89_ab_cd_efn,
  slot: 2,
  title_index: 3,
  server_id: 1n,
  file_name: "Guardian",
  file_type: 6,
  campaign_id: 0xff_ff_ff_ff,
  map_id: 31,
  game_engine_type: 4,
  size_bytes: 0x1_00_00n,
};

function hex(bytes: Uint8Array): string {
  return Buffer.from(bytes).toString("hex");
}

/** Payloads of every chunk in a BLF file, keyed `sig major.minor`. */
function payloads(file: Uint8Array): Record<string, string> {
  const view = new DataView(file.buffer, file.byteOffset, file.byteLength);
  const out: Record<string, string> = {};
  let offset = 0;
  while (offset + 12 <= file.length) {
    const sig = Buffer.from(file.subarray(offset, offset + 4)).toString(
      "latin1"
    );
    const length = view.getUint32(offset + 4);
    const version = `${view.getUint16(offset + 8)}.${view.getUint16(offset + 10)}`;
    out[`${sig} ${version}`] = hex(file.subarray(offset + 12, offset + length));
    offset += length;
  }
  return out;
}

describe("Halo 3 user file chunks", () => {
  it("have blf_lib's sizes", () => {
    expect(s_blf_chunk_player_data.schema.getSize()).toBe(0x2c);
    expect(s_blf_chunk_player_data_v2.schema.getSize()).toBe(0x28);
    expect(s_blf_chunk_player_data_v1.schema.getSize()).toBe(8);
    expect(s_blf_chunk_service_record.schema.getSize()).toBe(0x5c);
    expect(s_blf_chunk_odst_service_record.schema.getSize()).toBe(0x60);
    expect(s_blf_chunk_author.schema.getSize()).toBe(0x44);
    expect(new s_blf_chunk_file_transfers().write_body("big")).toHaveLength(
      0x2_80
    );
  });

  it("defaults hopper_directory to default_hoppers", () => {
    expect(new s_blf_chunk_player_data().hopper_directory).toBe(
      "default_hoppers"
    );
    expect(new s_blf_chunk_player_data_v2().hopper_directory).toBe(
      "default_hoppers"
    );
    expect(new s_blf_chunk_player_data_v1().highest_skill).toBe(1);
  });

  it("build_user_file writes blf_lib's chunks in blf_lib's order", () => {
    const file = build_user_file(fupd, srid, { transfers: [transfer] }, osri);
    const chunks = payloads(file);

    expect(Object.keys(chunks)).toEqual([
      "_blf 1.2",
      "athr 3.1",
      "fupd 3.1",
      "fupd 2.0",
      "fupd 1.0",
      "srid 2.1",
      "filq 1.0",
      "osri 2.0",
      "_eof 1.1",
    ]);
    expect(file).toHaveLength(1137);
    expect(chunks["athr 3.1"]!.slice(32)).toBe(k_athr_3_1_without_program_name);
    expect(chunks["fupd 3.1"]).toBe(k_fupd_3_1);
    expect(chunks["fupd 2.0"]).toBe(k_fupd_2_0);
    expect(chunks["fupd 1.0"]).toBe(k_fupd_1_0);
    expect(chunks["srid 2.1"]).toBe(k_srid_2_1);
    expect(chunks["osri 2.0"]).toBe(k_osri_2_0);
    expect(chunks["filq 1.0"]!.slice(0, 160)).toBe(k_filq_entry);
    expect(chunks["filq 1.0"]!.slice(160)).toBe("0".repeat(0x2_80 * 2 - 160));
    expect(chunks["_eof 1.1"]).toBe("0000046000");
  });

  it("build_user_file skips missing chunks and defaults fupd", () => {
    const file = build_user_file(undefined, undefined, undefined, undefined);
    const chunks = payloads(file);
    expect(Object.keys(chunks)).toEqual([
      "_blf 1.2",
      "athr 3.1",
      "fupd 3.1",
      "fupd 2.0",
      "fupd 1.0",
      "_eof 1.1",
    ]);
    const parsed = new s_blf_chunk_player_data();
    expect(find_chunk(file, parsed, "big")).toBe(true);
    expect(parsed.hopper_directory).toBe("default_hoppers");
    expect(chunks["fupd 1.0"]).toBe("0000000000000000");
  });

  it("reads back what it writes", () => {
    const file = build_user_file(fupd, srid, { transfers: [transfer] }, osri);

    const player_data = new s_blf_chunk_player_data();
    expect(find_chunk(file, player_data, "big")).toBe(true);
    expect({ ...player_data }).toEqual(fupd);

    const service_record = new s_blf_chunk_service_record();
    expect(find_chunk(file, service_record, "big")).toBe(true);
    expect({ ...service_record }).toEqual(srid);

    const odst_service_record = new s_blf_chunk_odst_service_record();
    expect(find_chunk(file, odst_service_record, "big")).toBe(true);
    expect({ ...odst_service_record }).toEqual(osri);

    const file_transfers = new s_blf_chunk_file_transfers();
    expect(find_chunk(file, file_transfers, "big")).toBe(true);
    expect(file_transfers.transfers).toHaveLength(8);
    expect(file_transfers.transfers[0]).toEqual(transfer);
    expect(file_transfers.transfers[1]!.share_id).toBe(0n);
  });

  it("rejects more than 8 transfers", () => {
    const chunk = new s_blf_chunk_file_transfers();
    chunk.transfers = Array.from({ length: 9 }, () => transfer);
    expect(() => chunk.write_body("big")).toThrow(/at most 8/);
  });
});
