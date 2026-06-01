import { c } from "@craftycodie/cstruct";
import { describe, expect, it } from "vitest";
import { s_network_lsp_heartbeat_response_data } from "../../../blam/haloreach/v12065_11_08_24_1738_tu1actual/networking/logic/network_lsp_presence";
import { getBlfChunkMeta } from "../../../blf_chunk";
import { s_blf_chunk_player_heartbeat_response } from "./s_blf_chunk_player_heartbeat_response";

describe("s_blf_chunk_player_heartbeat_response", () => {
  it("is phbr 2.1 with a 0x93-byte body", () => {
    const meta = getBlfChunkMeta(new s_blf_chunk_player_heartbeat_response());
    expect(meta.signature).toBe("phbr");
    expect(meta.major).toBe(2);
    expect(meta.minor).toBe(1);
    expect(meta.version).toBe(2.1);
    expect(c.sizeof(s_blf_chunk_player_heartbeat_response)).toBe(0x93);
    expect(c.sizeof(s_network_lsp_heartbeat_response_data)).toBe(0x93);
  });

  it("round-trips a minimal payload", () => {
    const chunk = new s_blf_chunk_player_heartbeat_response();
    chunk.xuid_count = 1;
    chunk.xuids[0] = 0x000900002e3fe0b1n;
    chunk.session_id = 0xfa00002248e026ddn;
    chunk.ack_number = 42;

    const body = chunk.write_body("big");
    expect(body.length).toBe(0x93);

    const parsed = new s_blf_chunk_player_heartbeat_response();
    parsed.read_body(body, "big");

    expect(parsed.xuid_count).toBe(1);
    expect(parsed.xuids[0]).toBe(0x000900002e3fe0b1n);
    expect(parsed.session_id).toBe(0xfa00002248e026ddn);
    expect(parsed.ack_number).toBe(42);
  });
});
