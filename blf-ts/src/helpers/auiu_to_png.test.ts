import { describe, expect, it } from "vitest";
import { auiu_pixels_to_png } from "./auiu_to_png";

const PNG_SIGNATURE = new Uint8Array([137, 80, 78, 71, 13, 10, 26, 10]);

describe("auiu_to_png", () => {
  it("encodes a 2x2 checker as a valid PNG", () => {
    const rgba = new Uint8Array([
      255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 0,
    ]);
    const png = auiu_pixels_to_png(2, 2, 2, rgba);
    expect(png.subarray(0, 8)).toEqual(PNG_SIGNATURE);
    expect(png.length).toBeGreaterThan(40);
  });
});
