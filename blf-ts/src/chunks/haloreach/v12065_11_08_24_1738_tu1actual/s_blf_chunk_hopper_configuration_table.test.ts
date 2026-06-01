import { readFileSync } from "node:fs";
import { c } from "@craftycodie/cstruct";
import { describe, expect, it } from "vitest";
import {
  c_hopper_configuration,
  s_blf_chunk_hopper_configuration_table,
  s_game_hopper_custom_category,
  s_hopper_query_configuration,
} from "./s_blf_chunk_hopper_configuration_table";

const MATCHMAKING_HOPPER_FIXTURE =
  "C:/Users/codie/Projects/Blam-Title-Storage/fixtures/reach/matchmaking_hopper_027.bin";

describe("hopper configuration structs", () => {
  it("matches blf_lib layout sizes", () => {
    expect(c.sizeof(s_game_hopper_custom_category)).toBe(0x44);
    expect(c.sizeof(s_hopper_query_configuration)).toBe(0x94);
    expect(c.sizeof(c_hopper_configuration)).toBe(0x458);
  });
});

describe("s_blf_chunk_hopper_configuration_table", () => {
  it("round-trips an empty table", () => {
    const original = new s_blf_chunk_hopper_configuration_table();
    const written = original.write_body("big");
    const parsed = new s_blf_chunk_hopper_configuration_table();
    parsed.read_body(written, "big");

    expect(parsed.hopper_categories).toEqual([]);
    expect(parsed.hopper_configurations).toEqual([]);
  });

  it("round-trips categories and configurations", () => {
    const original = new s_blf_chunk_hopper_configuration_table();

    const category = new s_game_hopper_custom_category();
    category.category_identifier = 7;
    category.category_name = "Competitive";
    original.hopper_categories = [category];

    const hopper = new c_hopper_configuration();
    hopper.hopper_name = "Arena Slayer";
    hopper.identifier = 101;
    hopper.category_identifier = 7;
    hopper.minimum_player_count = 8;
    hopper.maximum_player_count = 8;
    original.hopper_configurations = [hopper];

    const written = original.write_body("big");
    const parsed = new s_blf_chunk_hopper_configuration_table();
    parsed.read_body(written, "big");

    expect(parsed.hopper_categories).toHaveLength(1);
    expect(parsed.hopper_categories[0]!.category_identifier).toBe(7);
    expect(parsed.hopper_categories[0]!.category_name).toBe("Competitive");

    expect(parsed.hopper_configurations).toHaveLength(1);
    expect(parsed.hopper_configurations[0]!.hopper_name).toBe("Arena Slayer");
    expect(parsed.hopper_configurations[0]!.identifier).toBe(101);
    expect(parsed.hopper_configurations[0]!.minimum_player_count).toBe(8);
  });

  it("parses a title-storage hopper fixture when present", () => {
    let file: Uint8Array;
    try {
      file = new Uint8Array(readFileSync(MATCHMAKING_HOPPER_FIXTURE));
    } catch {
      return;
    }

    const chunk = new s_blf_chunk_hopper_configuration_table();
    chunk.read(file, "big");

    expect(chunk.hopper_configurations.length).toBeGreaterThan(0);
    expect(chunk.hopper_categories.length).toBeGreaterThan(0);
    expect(chunk.hopper_configurations[0]!.hopper_name.length).toBeGreaterThan(
      0
    );

    const roundTrip = chunk.write("big");
    const reparsed = new s_blf_chunk_hopper_configuration_table();
    reparsed.read(roundTrip, "big");

    expect(reparsed.hopper_configurations.length).toBe(
      chunk.hopper_configurations.length
    );
    expect(reparsed.hopper_categories.length).toBe(
      chunk.hopper_categories.length
    );
    expect(reparsed.hopper_configurations[0]!.identifier).toBe(
      chunk.hopper_configurations[0]!.identifier
    );
  });
});
