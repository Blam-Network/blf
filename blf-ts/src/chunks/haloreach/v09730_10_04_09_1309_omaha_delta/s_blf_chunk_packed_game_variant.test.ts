import { describe, expect, it } from "vitest";
import { BlfError } from "../../../error";
import { s_blf_chunk_packed_game_variant } from "./s_blf_chunk_packed_game_variant";

describe("s_blf_chunk_matchmaking_game_variant (omaha_delta)", () => {
  it("rejects empty payloads", () => {
    const chunk = new s_blf_chunk_packed_game_variant();
    expect(() => chunk.read_body(new Uint8Array(), "big")).toThrow(BlfError);
  });
});
