import { describe, expect, it } from "vitest";
import { s_blf_chunk_player_data } from "./s_blf_chunk_player_data";

describe("s_blf_chunk_player_data (alpha fupd 6.0)", () => {
  it("defaults hopper_directory to default_hoppers", () => {
    expect(new s_blf_chunk_player_data().hopper_directory).toBe(
      "default_hoppers"
    );
  });

  it("round-trips 32-bit hopper_access and bungie_user_role", () => {
    const fupd = new s_blf_chunk_player_data();
    fupd.hopper_access = 0x01_02_03_04;
    fupd.bungie_user_role = 0xc0_47;
    fupd.hopper_directory = "title/hoppers";
    fupd.unlock_achievements = Array.from({ length: 32 }, (_, i) => 255 - i);
    fupd.extras_portal_debug = true;

    const bytes = fupd.write_body("big");
    expect(bytes.length).toBe(0x49);
    expect(Buffer.from(bytes).readUInt32BE(0)).toBe(0x01_02_03_04);
    expect(Buffer.from(bytes).readUInt32BE(4)).toBe(0xc0_47);

    const parsed = new s_blf_chunk_player_data();
    parsed.read_body(bytes, "big");
    expect({ ...parsed }).toEqual({ ...fupd });
  });
});
