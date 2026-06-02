import { describe, expect, it } from "vitest";
import { s_blf_chunk_screenshot_data } from "./s_blf_chunk_screenshot_data";

describe("s_blf_chunk_screenshot_data", () => {
  it("round-trips jpeg bytes", () => {
    const original = new s_blf_chunk_screenshot_data();
    original.jpeg_data = new Uint8Array([0xff, 0xd8, 0xff, 0xd9]);

    const bytes = original.write("big");
    const parsed = new s_blf_chunk_screenshot_data();
    parsed.read(bytes, "big");

    expect(parsed.jpeg_data).toEqual(original.jpeg_data);
  });
});
