import { blfToJson } from "../../src/utils/blf_json";

/** JSON-friendly value for vitest snapshots (Dates, bigints, byte arrays). */
export function deepSnapshot(value: unknown): unknown {
  return blfToJson(value);
}
