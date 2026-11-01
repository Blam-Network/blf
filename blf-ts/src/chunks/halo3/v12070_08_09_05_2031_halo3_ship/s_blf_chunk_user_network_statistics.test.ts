import { describe, expect, it } from "vitest";
import { find_chunk } from "../../../blf_chunk";
import { s_blf_chunk_author } from "./s_blf_chunk_author";
import {
  build_hopper_statistics_file,
  s_blf_chunk_matchmaking_hopper_statistics,
} from "./s_blf_chunk_matchmaking_hopper_statistics";
import {
  build_machine_file,
  s_blf_chunk_user_network_statistics,
} from "./s_blf_chunk_user_network_statistics";
import {
  build_recent_players_file,
  s_blf_chunk_user_recent_players,
} from "./s_blf_chunk_user_recent_players";

// Expected bytes were produced by blf_lib (via @blam-network/blf_lsp) for the same inputs.
const k_funs_1_1 =
  "000000000000000100000000000000020000000000000003000000000000000400000000000000050000000000000006000000000000000700000000000000080000000000000009000000000000000a000000030000000a000000140000001e00000028000000320000003c0000004600000050000000080000000100000002000000030000000400000005000000060000000700000008fffffffffffffffefffffffdfffffffcfffffffbfffffffafffffff9fffffff8fffffffb00000000";
const k_furp_2_1 = "0000000200030009000000000001fffffedcba9876543210";
const k_mmhs_file = `5f626c660000003000010002fffe${"00".repeat(34)}6d6d687300000110000300010000000c0000ffff00000005000000c800000007${"00".repeat(30 * 8)}5f656f6600000011000100010000014000`;

const funs = {
  bandwidth_data: {
    qos_sample_count: 3,
    qos_sample_bps: [10, 20, 30, 40, 50, 60, 70, 80],
    bandwidth_measurement_count: 8,
    bandwidth_measurement_successful_bps: [1, 2, 3, 4, 5, 6, 7, 8],
    bandwidth_measurement_unsafe_bps: [-1, -2, -3, -4, -5, -6, -7, -8],
    bandwidth_dispute_count: -5,
  },
  session: [
    { client_badness_history: [1n, 2n], host_badness_history: [3n, 4n] },
    { client_badness_history: [5n, 6n], host_badness_history: [7n, 8n] },
  ],
  connection_history: [9n, 10n],
} as const;

const players = [
  { hopper_identifier: 3, xuid: 0x00_09_00_00_00_00_00_01n },
  { hopper_identifier: 0xff_ff, xuid: 0xfe_dc_ba_98_76_54_32_10n },
];

function hex(bytes: Uint8Array): string {
  return Buffer.from(bytes).toString("hex");
}

/** File bytes after `_blf` and `athr` (both 0x30 + 0x50 bytes long). */
function after_author(file: Uint8Array): string {
  return hex(file.subarray(0x30 + 0x50));
}

describe("Halo 3 machine, recent players and hopper statistics files", () => {
  it("build_machine_file matches blf_lib", () => {
    const file = build_machine_file(funs);
    expect(hex(file.subarray(0, 0x30))).toBe(
      `5f626c660000003000010002fffe${Buffer.from("halo3 machine").toString("hex")}${"00".repeat(32 - 13)}0000`
    );
    expect(after_author(file)).toBe(
      `66756e73000000cc00010001${k_funs_1_1}5f656f6600000011000100010000014c00`
    );

    const parsed = new s_blf_chunk_user_network_statistics();
    expect(find_chunk(file, parsed, "big")).toBe(true);
    expect({ ...parsed }).toEqual(funs);

    const author = new s_blf_chunk_author();
    expect(find_chunk(file, author, "big")).toBe(true);
    expect(author.build_number).toBe(12_070);
    expect(author.build_number_sequence).toBe(1);
    expect(author.build_string).toBe("12070.08.09.05.2031.halo3_sh");
  });

  it("build_recent_players_file matches blf_lib", () => {
    const file = build_recent_players_file({ players });
    expect(after_author(file)).toBe(
      `667572700000002400020001${k_furp_2_1}5f656f660000001100010001000000a400`
    );

    const parsed = new s_blf_chunk_user_recent_players();
    expect(find_chunk(file, parsed, "big")).toBe(true);
    expect(parsed.players).toEqual(players);
  });

  it("recent players round-trips an empty list", () => {
    const chunk = new s_blf_chunk_user_recent_players();
    expect(hex(chunk.write_body("big"))).toBe("00000000");
    chunk.read_body(new Uint8Array(4), "big");
    expect(chunk.players).toEqual([]);
  });

  it("recent players rejects a truncated payload", () => {
    const chunk = new s_blf_chunk_user_recent_players();
    expect(() =>
      chunk.read_body(Uint8Array.from([0, 0, 0, 1, 0, 3]), "big")
    ).toThrow(/too short for 1 players/);
  });

  it("build_hopper_statistics_file matches blf_lib", () => {
    const data = [
      { hopper_identifier: -1, player_count: 5 },
      { hopper_identifier: 200, player_count: 7 },
    ];
    const file = build_hopper_statistics_file({ player_count: 12, data });
    expect(hex(file)).toBe(k_mmhs_file);

    const parsed = new s_blf_chunk_matchmaking_hopper_statistics();
    expect(find_chunk(file, parsed, "big")).toBe(true);
    expect(parsed.player_count).toBe(12);
    expect(parsed.data).toHaveLength(32);
    expect(parsed.data.slice(0, 2)).toEqual(data);
  });

  it("hopper statistics rejects more than 32 hoppers", () => {
    const chunk = new s_blf_chunk_matchmaking_hopper_statistics();
    chunk.data = Array.from({ length: 33 }, () => ({
      hopper_identifier: 1,
      player_count: 1,
    }));
    expect(() => chunk.write_body("big")).toThrow(/at most 32/);
  });
});
