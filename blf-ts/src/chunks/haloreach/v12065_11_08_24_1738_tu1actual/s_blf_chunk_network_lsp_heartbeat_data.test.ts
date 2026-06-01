import { c } from "@craftycodie/cstruct";
import { describe, expect, it } from "vitest";
import { s_player_appearance } from "../../../blam/haloreach/v12065_11_08_24_1738_tu1actual/game/players";
import {
  s_network_lsp_heartbeat_data,
  s_network_lsp_heartbeat_player_data,
  s_network_lsp_heartbeat_session_data,
  s_network_lsp_heartbeat_session_player_data,
} from "../../../blam/haloreach/v12065_11_08_24_1738_tu1actual/networking/logic/network_lsp_presence";
import { getBlfChunkMeta } from "../../../blf_chunk";
import { s_blf_chunk_network_lsp_heartbeat_data } from "./s_blf_chunk_network_lsp_heartbeat_data";

describe("s_blf_chunk_network_lsp_heartbeat_data", () => {
  it("is phbt 6.0 with a 0x667-byte body", () => {
    const meta = getBlfChunkMeta(new s_blf_chunk_network_lsp_heartbeat_data());
    expect(meta.signature).toBe("phbt");
    expect(meta.major).toBe(6);
    expect(meta.minor).toBe(0);
    expect(meta.version).toBe(6);
    expect(c.sizeof(s_blf_chunk_network_lsp_heartbeat_data)).toBe(0x667);
    expect(c.sizeof(s_network_lsp_heartbeat_data)).toBe(0x1bb);
  });

  it("matches nested struct sizes from Reach TU1", () => {
    expect(c.sizeof(s_network_lsp_heartbeat_session_player_data)).toBe(9);
    expect(c.sizeof(s_network_lsp_heartbeat_player_data)).toBe(56);
    expect(c.sizeof(s_network_lsp_heartbeat_session_data)).toBe(201);
    expect(c.sizeof(s_player_appearance)).toBe(40);
  });

  it("round-trips a minimal payload including the trailing extra bytes", () => {
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
    chunk.extra[0] = 0xab;
    chunk.extra[0x4ab] = 0xcd;

    const body = chunk.write_body("big");
    expect(body.length).toBe(0x667);

    const parsed = new s_blf_chunk_network_lsp_heartbeat_data();
    parsed.read_body(body, "big");

    expect(parsed.has_players).toBe(1);
    expect(parsed.local_player_count).toBe(1);
    expect(parsed.machine_id).toBe(0xe20000000001n);
    expect(parsed.players[0].player_xuid).toBe(0x123456789abcdef0n);
    expect(parsed.players[0].player_grade).toBe(3);
    expect(parsed.session_data.gui_game_mode).toBe(2);
    expect(parsed.session_data.hopper_id).toBe(42);
    expect(parsed.extra[0]).toBe(0xab);
    expect(parsed.extra[0x4ab]).toBe(0xcd);
  });
});
