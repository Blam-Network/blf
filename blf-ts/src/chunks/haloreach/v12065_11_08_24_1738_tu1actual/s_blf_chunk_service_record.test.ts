import { c } from "@craftycodie/cstruct";
import { describe, expect, it } from "vitest";
import { find_chunk, getBlfChunkMeta } from "../../../blf_chunk";
import {
  build_service_record_file,
  s_blf_chunk_service_record,
  s_blf_chunk_service_record_arena_hopper_stats,
  s_blf_chunk_service_record_arena_season_stats,
  s_blf_chunk_service_record_campaign_difficulty_stats,
  s_blf_chunk_service_record_commendation,
  s_blf_chunk_service_record_firefight_difficulty_stats,
  s_blf_chunk_service_record_matchmaking_category_stats,
} from "./s_blf_chunk_service_record";

function make_srid(): s_blf_chunk_service_record {
  const srid = new s_blf_chunk_service_record();
  srid.player_name = "Spartän 117";
  srid.player_info_available = true;
  srid.armour_primary_color = 0x11;
  srid.player_model_choice = 1;
  srid.emblem_secondary = true;
  srid.service_tag = "B117";
  srid.credits = 123_456;
  srid.campaign_completed_at = new Date(1_300_000_000_000);
  srid.campaign_difficulty_stats[2]!.highest_skull_multiplier = 7;
  srid.campaign_commendations_count = 1;
  srid.campaign_commendations[0] = { commendation: 5, progress: 99 };
  srid.matchmaking_category_stats[4]!.assists = 42;
  srid.arena_season_stats_count = 1;
  const hopper = srid.arena_season_stats[0]!.hopper_stats[7]!;
  hopper.hopper_name = "Arena Doubles";
  hopper.last_7_days_kill_death_ratio = 1.5;
  srid.halo3_first_played_time = new Date(1_190_000_000_000);
  srid.unknown5 = 0xab;
  return srid;
}

describe("s_blf_chunk_service_record", () => {
  it("is srid 7.1", () => {
    const meta = getBlfChunkMeta(new s_blf_chunk_service_record());
    expect(meta.signature).toBe("srid");
    expect(meta.major).toBe(7);
    expect(meta.minor).toBe(1);
  });

  it("has blf_lib's struct sizes", () => {
    expect(c.sizeof(s_blf_chunk_service_record.schema)).toBe(0xd3c);
    expect(c.sizeof(s_blf_chunk_service_record_campaign_difficulty_stats)).toBe(
      0x18
    );
    expect(
      c.sizeof(s_blf_chunk_service_record_firefight_difficulty_stats)
    ).toBe(0x1c);
    expect(
      c.sizeof(s_blf_chunk_service_record_matchmaking_category_stats)
    ).toBe(0x14);
    expect(c.sizeof(s_blf_chunk_service_record_arena_season_stats)).toBe(0x348);
    expect(c.sizeof(s_blf_chunk_service_record_arena_hopper_stats)).toBe(0x68);
    expect(c.sizeof(s_blf_chunk_service_record_commendation)).toBe(8);
  });

  it("places fields at blf_lib's offsets", () => {
    const bytes = Buffer.from(make_srid().write_body("big"));
    expect(bytes.readUInt16BE(0)).toBe("S".charCodeAt(0));
    expect(bytes[0x25]).toBe(0x11);
    expect(bytes[0x28]).toBe(1);
    expect(bytes.readUInt16BE(0x40)).toBe("B".charCodeAt(0));
    expect(bytes.readInt32BE(0x4e)).toBe(123_456);
    expect(bytes.readUInt32BE(0x53)).toBe(1_300_000_000);
    expect(bytes.readUInt32BE(0xaf)).toBe(1);
    expect(bytes.readUInt32BE(0xb3)).toBe(5);
    expect(bytes[0xd3b]).toBe(0xab);
  });

  it("round-trips", () => {
    const parsed = new s_blf_chunk_service_record();
    parsed.read_body(make_srid().write_body("big"), "big");
    expect({ ...parsed }).toEqual({ ...make_srid() });
  });

  it("build_service_record_file writes only the srid chunk", () => {
    const file = build_service_record_file(make_srid());
    expect(file.length).toBe(12 + 0xd3c);
    expect(Buffer.from(file).toString("latin1", 0, 4)).toBe("srid");

    const parsed = new s_blf_chunk_service_record();
    expect(find_chunk(file, parsed, "big")).toBe(true);
    expect(parsed.player_name).toBe("Spartän 117");
  });
});
