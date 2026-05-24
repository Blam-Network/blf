/** JSON-friendly value for vitest snapshots (Dates, bigints, byte arrays). */
export function deepSnapshot(value: unknown): unknown {
  if (value === null || value === undefined) {
    return value;
  }
  if (value instanceof Uint8Array) {
    return { $bytes: Buffer.from(value).toString("hex") };
  }
  if (typeof value === "bigint") {
    return { $bigint: value.toString() };
  }
  if (value instanceof Date) {
    return { $date: value.toISOString() };
  }
  if (typeof value !== "object") {
    return value;
  }
  if (Array.isArray(value)) {
    return value.map(deepSnapshot);
  }
  const out: Record<string, unknown> = {};
  for (const key of Object.keys(value)) {
    out[key] = deepSnapshot((value as Record<string, unknown>)[key]);
  }
  return out;
}
