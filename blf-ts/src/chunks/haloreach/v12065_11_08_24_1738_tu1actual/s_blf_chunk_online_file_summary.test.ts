import { c } from "@craftycodie/cstruct";
import { describe, expect, it } from "vitest";
import { s_online_file_summary_listing_entry } from "../../../blam/haloreach/v12065_11_08_24_1738_tu1actual/networking/online/files/online_file_summary_listing";
import { search_for_chunk } from "../../../blf_chunk";
import { write_blffile } from "../../../index";
import { s_blf_chunk_end_of_file } from "../../halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_end_of_file";
import { s_blf_chunk_start_of_file } from "../../halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_start_of_file";
import { s_blf_chunk_online_file_summary } from "./s_blf_chunk_online_file_summary";

function make_entry(
  share_id: bigint,
  screenshots: number,
  films: number,
  game_variants: number,
  map_variants: number,
  new_items: number
): s_online_file_summary_listing_entry {
  const entry = new s_online_file_summary_listing_entry();
  entry.share_id = share_id;
  entry.screenshots_count = screenshots;
  entry.films_count = films;
  entry.game_variants_count = game_variants;
  entry.map_variants_count = map_variants;
  entry.new_items_count = new_items;
  entry.unknown1C = 0;
  entry.unknown20 = 0;
  return entry;
}

describe("s_blf_chunk_online_file_summary", () => {
  it("uses a 36-byte entry layout", () => {
    expect(c.sizeof(s_online_file_summary_listing_entry)).toBe(36);
  });

  it("rejects payloads shorter than the 4-byte header", () => {
    const chunk = new s_blf_chunk_online_file_summary();
    expect(() => chunk.read_body(new Uint8Array(3), "big")).toThrow(
      /finf chunk payload too short/
    );
  });

  it("round-trips entry_count, pad, and entries", () => {
    const original = new s_blf_chunk_online_file_summary();
    original.entries = [
      make_entry(0x0123_4567_89ab_cdefn, 1, 2, 3, 4, 5),
      make_entry(0xfedcba98_76543210n, 10, 20, 30, 40, 50),
    ];

    const payload = original.write_body("big");
    expect(payload.length).toBe(4 + 2 * 36);
    expect(payload[0]).toBe(0);
    expect(payload[1]).toBe(2);
    expect(payload[2]).toBe(0);
    expect(payload[3]).toBe(0);

    const parsed = new s_blf_chunk_online_file_summary();
    parsed.read_body(payload, "big");

    expect(parsed.entry_count).toBe(2);
    expect(parsed.entries).toHaveLength(2);
    expect(parsed.entries[0].share_id).toBe(0x0123_4567_89ab_cdefn);
    expect(parsed.entries[0].screenshots_count).toBe(1);
    expect(parsed.entries[0].films_count).toBe(2);
    expect(parsed.entries[0].game_variants_count).toBe(3);
    expect(parsed.entries[0].map_variants_count).toBe(4);
    expect(parsed.entries[0].new_items_count).toBe(5);
    expect(parsed.entries[1].share_id).toBe(0xfedcba98_76543210n);
    expect(parsed.entries[1].screenshots_count).toBe(10);
    expect(parsed.entries[1].game_variants_count).toBe(30);
    expect(parsed.write_body("big")).toEqual(payload);
  });

  it("finds and reads finf in a BLF file", () => {
    const summary = new s_blf_chunk_online_file_summary();
    summary.entries = [make_entry(42n, 7, 0, 1, 2, 3)];

    const blf = write_blffile("big", [
      s_blf_chunk_start_of_file.create("finf-test"),
      summary,
      new s_blf_chunk_end_of_file(),
    ]);

    const found = new s_blf_chunk_online_file_summary();
    expect(search_for_chunk(blf, found, "big")).toBe(true);
    expect(found.entry_count).toBe(1);
    expect(found.entries[0].share_id).toBe(42n);
    expect(found.entries[0].screenshots_count).toBe(7);
  });
});
