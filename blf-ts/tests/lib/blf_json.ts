/** JSON-friendly serialization of decoded BLF values (test snapshots). */
export function blfToJson(value: unknown): unknown {
  if (value === null || value === undefined) {
    return;
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
    return value.map((item) => blfToJson(item));
  }

  const out: Record<string, unknown> = {};
  for (const key of Object.keys(value)) {
    const serialized = blfToJson((value as Record<string, unknown>)[key]);
    if (serialized !== undefined) {
      out[key] = serialized;
    }
  }
  return out;
}
