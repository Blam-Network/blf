import { readFileSync } from "node:fs";
import { c } from "@craftycodie/cstruct";
import { describe, expect, it } from "vitest";
import { reach_presence_heartbeat_1780307835656_fixture } from "../../../../tests/fixtures/paths";
import { s_player_appearance } from "../../../blam/haloreach/v12065_11_08_24_1738_tu1actual/game/players";
import {
  s_network_lsp_heartbeat_data,
  s_network_lsp_heartbeat_player_data,
  s_network_lsp_heartbeat_session_data,
  s_network_lsp_heartbeat_session_player_data,
} from "../../../blam/haloreach/v12065_11_08_24_1738_tu1actual/networking/logic/network_lsp_presence";
import { find_chunk, getBlfChunkMeta } from "../../../blf_chunk";
import { s_blf_chunk_network_lsp_heartbeat_data } from "./s_blf_chunk_network_lsp_heartbeat_data";

function bytesToHex(values: number[]): string {
  return values.map((b) => b.toString(16).padStart(2, "0")).join("");
}

describe("s_blf_chunk_network_lsp_heartbeat_data", () => {
  it("is phbt 5.1 with a 0x1BB-byte body", () => {
    const meta = getBlfChunkMeta(new s_blf_chunk_network_lsp_heartbeat_data());
    expect(meta.signature).toBe("phbt");
    expect(meta.major).toBe(5);
    expect(meta.minor).toBe(1);
    expect(meta.version).toBe(5.1);
    expect(c.sizeof(s_blf_chunk_network_lsp_heartbeat_data)).toBe(0x1bb);
    expect(c.sizeof(s_network_lsp_heartbeat_data)).toBe(0x1bb);
  });

  it("matches nested struct sizes from Reach TU1", () => {
    expect(c.sizeof(s_network_lsp_heartbeat_session_player_data)).toBe(9);
    expect(c.sizeof(s_network_lsp_heartbeat_player_data)).toBe(56);
    expect(c.sizeof(s_network_lsp_heartbeat_session_data)).toBe(201);
    expect(c.sizeof(s_player_appearance)).toBe(40);
  });

  it("round-trips a minimal payload", () => {
    const chunk = new s_blf_chunk_network_lsp_heartbeat_data();
    chunk.has_players = 1;
    chunk.local_player_count = 1;
    chunk.machine_id = 0xe20000000001n;
    chunk.players = Array.from(
      { length: 4 },
      () => new s_network_lsp_heartbeat_player_data()
    );
    chunk.players[0].player_xuid = 0x123456789abcdef0n;
    chunk.players[0].player_grade = 3;
    chunk.session_data = new s_network_lsp_heartbeat_session_data();
    chunk.session_data.gui_game_mode = 2;
    chunk.session_data.hopper_id = 42;

    const body = chunk.write_body("big");
    expect(body.length).toBe(0x1bb);

    const parsed = new s_blf_chunk_network_lsp_heartbeat_data();
    parsed.read_body(body, "big");

    expect(parsed.has_players).toBe(1);
    expect(parsed.local_player_count).toBe(1);
    expect(parsed.machine_id).toBe(0xe20000000001n);
    expect(parsed.players[0].player_xuid).toBe(0x123456789abcdef0n);
    expect(parsed.players[0].player_grade).toBe(3);
    expect(parsed.session_data.gui_game_mode).toBe(2);
    expect(parsed.session_data.hopper_id).toBe(42);
  });

  it("parses a captured Reach presence heartbeat upload", () => {
    const buffer = new Uint8Array(
      readFileSync(reach_presence_heartbeat_1780307835656_fixture)
    );
    expect(buffer.length).toBe(455);

    const chunk = new s_blf_chunk_network_lsp_heartbeat_data();
    expect(find_chunk(buffer, chunk, "big")).toBe(true);

    expect(chunk.has_players).toBe(1);
    expect(chunk.local_player_count).toBe(1);
    expect(chunk.machine_id).toBe(0xfa00002248e026ddn);
    expect(bytesToHex(chunk.unknowneeA)).toBe("ae00ab3a5af4a3dc");

    const local = chunk.players[0];
    expect(local.player_xuid).toBe(0x000900002e3fe0b1n);
    expect(local.player_grade).toBe(0);
    expect(local.player_sub_grade).toBe(0);
    expect(local.flags).toBe(0x3d);
    expect(local.bungienet_user_flags).toBe(0x1000);
    expect(local.player_appearance.service_tag).toBe("0000");

    const session = chunk.session_data;
    expect(session.gui_game_mode).toBe(2);
    expect(session.session_game_mode).toBe(2);
    expect(session.hopper_id).toBe(122);
    expect(session.session_piracy_mode.network_session_privacy).toBe(1);
    expect(session.session_piracy_mode.network_session_closed_status).toBe(1);
    expect(session.local_player_count).toBe(16);
    expect(session.player_count).toBe(1);
    expect(session.incoming_join_failed).toBe(0);
    expect(bytesToHex(session.unknown18f.slice(0, 4))).toBe("ffffffff");
    expect(bytesToHex(session.unknown18f.slice(28, 44))).toBe(
      "00000000000000000000000000000000"
    );

    const sessionPlayer = session.session_players[0];
    expect(bytesToHex(sessionPlayer.unknown0)).toBe("000900002e3fe0b1");
    expect(sessionPlayer.team).toBe(0);
  });
});
