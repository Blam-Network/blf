import { c } from "@craftycodie/cstruct";
import { describe, expect, it } from "vitest";
import { getBlfChunkMeta } from "../../../blf_chunk";
import { s_blf_chunk_challenge_progress } from "./s_blf_chunk_challenge_progress";
import {
  e_challenge_category,
  s_blf_chunk_challenge_state,
  s_challenge_state,
} from "./s_blf_chunk_challenge_state";

describe("s_challenge_state", () => {
  it("is 28 bytes with -1 sentinels for unset overrides", () => {
    expect(c.sizeof(s_challenge_state)).toBe(28);

    const challenge = new s_challenge_state();
    challenge.category = e_challenge_category.firefight;
    challenge.challenge = 7;
    challenge.cookie_reward = 2250;

    const bytes = c.write(s_challenge_state, challenge, "big");
    expect(bytes.length).toBe(28);
    expect(bytes[0]).toBe(e_challenge_category.firefight);
    expect(bytes[1]).toBe(7);
    expect(Buffer.from(bytes.subarray(2, 4)).readInt16BE(0)).toBe(2250);
    expect(Buffer.from(bytes.subarray(4, 8)).readInt32BE(0)).toBe(-1);

    const parsed = c.read(s_challenge_state, bytes, "big");
    expect(parsed.category).toBe(e_challenge_category.firefight);
    expect(parsed.challenge).toBe(7);
    expect(parsed.cookie_reward).toBe(2250);
    expect(parsed.required_progress).toBeNull();
    expect(parsed.skull_flags).toBeNull();
  });
});

describe("s_blf_chunk_challenge_state", () => {
  it("is dcha 3.1", () => {
    const meta = getBlfChunkMeta(new s_blf_chunk_challenge_state());
    expect(meta.signature).toBe("dcha");
    expect(meta.major).toBe(3);
    expect(meta.minor).toBe(1);
    expect(c.sizeof(s_blf_chunk_challenge_state)).toBe(0x24a);
  });

  it("round-trips challenge sets", () => {
    const chunk = new s_blf_chunk_challenge_state();
    chunk.active_challenge_set_1 = 100;
    chunk.active_challenge_set_2 = 20;
    chunk.chalenge_set_1_count = 4;
    chunk.chalenge_set_2_count = 1;
    chunk.chalenge_set_1_timestamp = new Date(Date.UTC(2026, 6, 11, 11, 0, 0));
    chunk.chalenge_set_1[0].category = e_challenge_category.bounty;
    chunk.chalenge_set_1[0].challenge = 3;
    chunk.chalenge_set_1[0].cookie_reward = 3000;

    const body = chunk.write_body("big");
    const parsed = new s_blf_chunk_challenge_state();
    parsed.read_body(body, "big");

    expect(parsed.active_challenge_set_1).toBe(100);
    expect(parsed.chalenge_set_1_count).toBe(4);
    expect(parsed.chalenge_set_1[0].category).toBe(e_challenge_category.bounty);
    expect(parsed.chalenge_set_1[0].challenge).toBe(3);
    expect(parsed.chalenge_set_1[0].cookie_reward).toBe(3000);
    expect(parsed.chalenge_set_1[1].cookie_reward).toBeNull();
  });
});

describe("s_blf_chunk_challenge_progress", () => {
  it("is chpr 2.1 with an 0x58-byte body", () => {
    const meta = getBlfChunkMeta(new s_blf_chunk_challenge_progress());
    expect(meta.signature).toBe("chpr");
    expect(meta.version).toBe(2.1);
    expect(c.sizeof(s_blf_chunk_challenge_progress)).toBe(0x58);
  });
});
