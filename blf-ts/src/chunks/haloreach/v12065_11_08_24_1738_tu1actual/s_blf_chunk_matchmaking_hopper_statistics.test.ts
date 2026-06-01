import { c } from "@craftycodie/cstruct";
import { describe, expect, it } from "vitest";
import { search_for_chunk } from "../../../blf_chunk";
import {
  build_hopper_statistics_file,
  s_blf_chunk_matchmaking_hopper_statistics,
  s_online_population_statistic,
} from "./s_blf_chunk_matchmaking_hopper_statistics";

function make_hopper(
  hopper_identifier: number,
  player_count: number
): s_online_population_statistic {
  const hopper = new s_online_population_statistic();
  hopper.hopper_identifier = hopper_identifier;
  hopper.player_count = player_count;
  return hopper;
}

describe("s_blf_chunk_matchmaking_hopper_statistics", () => {
  it("uses an 8-byte hopper_population layout", () => {
    expect(c.sizeof(s_online_population_statistic)).toBe(8);
  });

  it("rejects payloads shorter than the 14-byte header", () => {
    const chunk = new s_blf_chunk_matchmaking_hopper_statistics();
    expect(() => chunk.read_body(new Uint8Array(13), "big")).toThrow(
      /mmhs chunk payload too short/
    );
  });

  it("round-trips population fields and hoppers", () => {
    const original = new s_blf_chunk_matchmaking_hopper_statistics();
    original.total_population = 42;
    original.unknown_population_2 = 7;
    original.unknown_population_3 = 3;
    original.hoppers = [make_hopper(10, 100), make_hopper(20, 5)];

    const payload = original.write_body("big");
    expect(payload.length).toBe(14 + 2 * 8);
    expect(payload[12]).toBe(0);
    expect(payload[13]).toBe(2);

    const parsed = new s_blf_chunk_matchmaking_hopper_statistics();
    parsed.read_body(payload, "big");
    expect(parsed.total_population).toBe(42);
    expect(parsed.unknown_population_2).toBe(7);
    expect(parsed.unknown_population_3).toBe(3);
    expect(parsed.playlist_count).toBe(2);
    expect(parsed.hoppers).toHaveLength(2);
    expect(parsed.hoppers[0]!.hopper_identifier).toBe(10);
    expect(parsed.hoppers[0]!.player_count).toBe(100);
    expect(parsed.hoppers[1]!.hopper_identifier).toBe(20);
    expect(parsed.hoppers[1]!.player_count).toBe(5);
  });

  it("build_hopper_statistics_file embeds mmhs in a BLF", () => {
    const mmhs = new s_blf_chunk_matchmaking_hopper_statistics();
    mmhs.hoppers = [make_hopper(1, 12)];

    const blf = build_hopper_statistics_file(mmhs);
    const parsed = new s_blf_chunk_matchmaking_hopper_statistics();
    expect(search_for_chunk(blf, parsed, "big")).toBe(true);
    expect(parsed.hoppers).toHaveLength(1);
    expect(parsed.hoppers[0]!.hopper_identifier).toBe(1);
    expect(parsed.hoppers[0]!.player_count).toBe(12);
  });
});
