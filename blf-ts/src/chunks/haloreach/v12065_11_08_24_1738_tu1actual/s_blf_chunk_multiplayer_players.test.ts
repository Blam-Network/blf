import { readFileSync } from "node:fs";
import { c } from "@craftycodie/cstruct";
import { describe, expect, it } from "vitest";
import { reach_carnage_report_1779581359260_fixture } from "../../../../tests/fixtures/paths";
import {
  s_player_appearance,
  s_player_configuration_from_client,
  s_player_configuration_from_host,
} from "../../../blam/haloreach/v12065_11_08_24_1738_tu1actual/game/players";
import { find_chunk, getBlfChunkMeta } from "../../../blf_chunk";
import {
  s_blf_chunk_multiplayer_players,
  s_blf_chunk_multiplayer_players_player,
} from "./s_blf_chunk_multiplayer_players";

describe("s_blf_chunk_multiplayer_players", () => {
  it("is mppl 8.1 with a 0x1404-byte body", () => {
    const meta = getBlfChunkMeta(new s_blf_chunk_multiplayer_players());
    expect(meta.signature).toBe("mppl");
    expect(meta.major).toBe(8);
    expect(meta.minor).toBe(1);
    expect(meta.version).toBe(8.1);
    expect(c.sizeof(s_blf_chunk_multiplayer_players)).toBe(0x1404);
    expect(c.sizeof(s_blf_chunk_multiplayer_players_player)).toBe(0x140);
    expect(c.sizeof(s_player_configuration_from_client)).toBe(0xb8);
    expect(c.sizeof(s_player_configuration_from_host)).toBe(0x30);
    expect(c.sizeof(s_player_appearance)).toBe(0x28);
  });

  it("parses mppl from a captured Reach carnage report upload", () => {
    const buffer = new Uint8Array(
      readFileSync(reach_carnage_report_1779581359260_fixture)
    );

    const mppl = new s_blf_chunk_multiplayer_players();
    expect(find_chunk(buffer, mppl, "big")).toBe(true);

    const active = mppl.players.filter((p) => p.player_exists);
    expect(active.length).toBeGreaterThan(0);
    expect(
      active[0].player_configuration_from_client.desired_name.length
    ).toBeGreaterThan(0);
  });
});
