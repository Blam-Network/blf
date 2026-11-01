import { c } from "@craftycodie/cstruct";
import { describe, expect, it } from "vitest";
import { find_chunk, getBlfChunkMeta } from "../../../blf_chunk";
import { s_blf_chunk_player_data as s_blf_chunk_player_data_alpha } from "../v08516_10_02_19_1607_omaha_alpha/s_blf_chunk_player_data";
import { s_blf_chunk_challenge_progress } from "./s_blf_chunk_challenge_progress";
import {
  s_blf_chunk_file_transfers,
  s_files_user_auto_download_queue_item,
} from "./s_blf_chunk_file_transfers";
import {
  build_user_file,
  e_bungienet_user_flags,
  s_blf_chunk_player_data,
} from "./s_blf_chunk_player_data";
import { s_blf_chunk_service_record } from "./s_blf_chunk_service_record";
import { s_blf_chunk_user_messaging_data } from "./s_blf_chunk_user_messaging_data";

const hex = (bytes: Uint8Array) => Buffer.from(bytes).toString("hex");

/** Chunk signatures and versions in a BLF file, in order. */
function chunk_list(file: Uint8Array): string[] {
  const view = Buffer.from(file);
  const out: string[] = [];
  let offset = 0;
  while (offset + 12 <= view.length) {
    const length = view.readUInt32BE(offset + 4);
    out.push(
      `${view.toString("latin1", offset, offset + 4)} ${view.readUInt16BE(offset + 8)}.${view.readUInt16BE(offset + 10)}`
    );
    offset += length;
  }
  return out;
}

function make_fupd(): s_blf_chunk_player_data {
  const fupd = new s_blf_chunk_player_data();
  fupd.hopper_access = 3;
  fupd.bungie_user_role =
    e_bungienet_user_flags.registered |
    e_bungienet_user_flags.pro_member |
    e_bungienet_user_flags.staff |
    e_bungienet_user_flags.debug_enabled |
    e_bungienet_user_flags.nameplate_allstar;
  fupd.hopper_directory = "hoppers";
  fupd.unlock_achievements = Array.from({ length: 32 }, (_, i) => i);
  fupd.extras_portal_debug = true;
  return fupd;
}

describe("s_blf_chunk_player_data (TU1)", () => {
  it("is fupd 7.0 with a 0x45-byte payload", () => {
    const meta = getBlfChunkMeta(new s_blf_chunk_player_data());
    expect(meta.signature).toBe("fupd");
    expect(meta.major).toBe(7);
    expect(meta.minor).toBe(0);
    expect(c.sizeof(s_blf_chunk_player_data.schema)).toBe(0x45);
  });

  it("defaults hopper_directory to default_hoppers", () => {
    const fupd = new s_blf_chunk_player_data();
    expect(fupd.hopper_directory).toBe("default_hoppers");
    expect(fupd.bungie_user_role).toBe(0);
  });

  it("round-trips with a pad byte after hopper_access", () => {
    const bytes = make_fupd().write_body("big");
    expect(bytes[0]).toBe(3);
    expect(bytes[1]).toBe(0);
    expect(Buffer.from(bytes).readUInt16BE(2)).toBe(0xc0_47);

    const parsed = new s_blf_chunk_player_data();
    parsed.read_body(bytes, "big");
    expect({ ...parsed }).toEqual({ ...make_fupd() });
  });
});

describe("s_blf_chunk_player_data (alpha)", () => {
  it("is fupd 6.0 with 32-bit access and role", () => {
    const meta = getBlfChunkMeta(new s_blf_chunk_player_data_alpha());
    expect(meta.signature).toBe("fupd");
    expect(meta.major).toBe(6);
    expect(meta.minor).toBe(0);
    expect(c.sizeof(s_blf_chunk_player_data_alpha.schema)).toBe(0x49);
  });
});

describe("s_blf_chunk_user_messaging_data", () => {
  it("is umsg 1.0, 12 bytes, and round-trips", () => {
    const meta = getBlfChunkMeta(new s_blf_chunk_user_messaging_data());
    expect(meta.signature).toBe("umsg");
    expect(meta.major).toBe(1);
    expect(meta.minor).toBe(0);
    expect(c.sizeof(s_blf_chunk_user_messaging_data.schema)).toBe(12);

    const umsg = new s_blf_chunk_user_messaging_data();
    umsg.message_index = 300;
    umsg.expires_at = new Date(1_700_000_000_000);
    const parsed = new s_blf_chunk_user_messaging_data();
    parsed.read_body(umsg.write_body("big"), "big");
    expect(parsed.message_index).toBe(300);
    expect(parsed.expires_at.getTime()).toBe(1_700_000_000_000);
  });
});

describe("build_user_file", () => {
  it("writes a default fupd when none is given (blf_lib bytes)", () => {
    expect(hex(build_user_file())).toBe(
      "5f626c660000003000010002fffe6f6d61686120757365720000000000000000000000000000000000000000000000006675706400000051000700000000000064656661756c745f686f707065727300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000005f656f6600000011000100010000008100"
    );
  });

  it("writes TU1 and alpha fupd plus umsg (blf_lib bytes)", () => {
    const umsg = new s_blf_chunk_user_messaging_data();
    umsg.message_index = 300;
    umsg.expires_at = new Date(1_700_000_000_500);
    expect(hex(build_user_file(make_fupd(), null, null, umsg))).toBe(
      "5f626c660000003000010002fffe6f6d61686120757365720000000000000000000000000000000000000000000000006675706400000051000700000300c047686f707065727300000000000000000000000000000000000000000000000000000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f01667570640000005500060000000000030000c047686f707065727300000000000000000000000000000000000000000000000000000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f01756d736700000018000100000000012c000000006553f1005f656f660000001100010001000000ee00"
    );
  });

  it("orders optional chunks as chpr, umsg, srid, filq", () => {
    const filq = new s_blf_chunk_file_transfers();
    filq.transfers = [s_files_user_auto_download_queue_item.zero()];
    const file = build_user_file(
      make_fupd(),
      new s_blf_chunk_challenge_progress(),
      new s_blf_chunk_service_record(),
      new s_blf_chunk_user_messaging_data(),
      filq
    );
    expect(chunk_list(file)).toEqual([
      "_blf 1.2",
      "fupd 7.0",
      "fupd 6.0",
      "chpr 2.1",
      "umsg 1.0",
      "srid 7.1",
      "filq 3.0",
      "_eof 1.1",
    ]);

    const fupd = new s_blf_chunk_player_data();
    expect(find_chunk(file, fupd, "big")).toBe(true);
    expect({ ...fupd }).toEqual({ ...make_fupd() });
  });
});
