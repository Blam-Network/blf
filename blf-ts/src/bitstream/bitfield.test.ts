import { describe, expect, it } from "vitest";
import { bitfieldFromRaw, bitfieldToRaw } from "./bitfield";

const fields = ["object_type", "team", "user_data"] as const;

describe("bitfield", () => {
  it("roundtrips through raw", () => {
    const value = { object_type: true, team: false, user_data: true };
    expect(bitfieldFromRaw(bitfieldToRaw(value, fields), fields)).toEqual(value);
  });

  it("assigns bits in field order", () => {
    expect(bitfieldToRaw({ object_type: true, team: true, user_data: false }, fields)).toBe(
      0b011
    );
    expect(
      bitfieldFromRaw(0b101, fields)
    ).toEqual({ object_type: true, team: false, user_data: true });
  });
});
