import { c } from "@craftycodie/cstruct";
import { describe, expect, it } from "vitest";
import { find_chunk, getBlfChunkMeta } from "../../../blf_chunk";
import {
  build_arena_hopper_stats_file,
  s_blf_chunk_arena_hopper_stats,
} from "./s_blf_chunk_arena_hopper_stats";

function make_arhs(): s_blf_chunk_arena_hopper_stats {
  const arhs = new s_blf_chunk_arena_hopper_stats();
  arhs.season = 1;
  arhs.unknown04 = 2;
  arhs.unknown08 = 3;
  arhs.unknown0C = 4;
  arhs.unknown10 = 5;
  arhs.unknown14 = 6;
  return arhs;
}

describe("s_blf_chunk_arena_hopper_stats", () => {
  it("is arhs 3.1 with a 22-byte payload", () => {
    const meta = getBlfChunkMeta(new s_blf_chunk_arena_hopper_stats());
    expect(meta.signature).toBe("arhs");
    expect(meta.major).toBe(3);
    expect(meta.minor).toBe(1);
    expect(c.sizeof(s_blf_chunk_arena_hopper_stats.schema)).toBe(22);
  });

  it("round-trips", () => {
    const parsed = new s_blf_chunk_arena_hopper_stats();
    parsed.read_body(make_arhs().write_body("big"), "big");
    expect({ ...parsed }).toEqual({ ...make_arhs() });
  });

  it("build_arena_hopper_stats_file writes arhs and _eof (blf_lib bytes)", () => {
    const file = build_arena_hopper_stats_file(make_arhs());
    expect(Buffer.from(file).toString("hex")).toBe(
      "617268730000002200030001000000010000000200000003000000040000000500065f656f6600000011000100010000002200"
    );

    const parsed = new s_blf_chunk_arena_hopper_stats();
    expect(find_chunk(file, parsed, "big")).toBe(true);
    expect(parsed.unknown14).toBe(6);
  });
});
