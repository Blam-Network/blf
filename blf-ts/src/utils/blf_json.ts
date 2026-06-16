import type { AutoMapOptions } from "@automapper/classes";
import { stage3AutoMapMetadata } from "../helpers/automap";

function isTsEnum(obj: unknown): obj is Record<string, string | number> {
  if (typeof obj !== "object" || obj === null) {
    return false;
  }
  const vals = Object.values(obj);
  return vals.length > 0 && vals.some((v) => typeof v === "number");
}

function enumToString(
  enumObj: Record<string, string | number>,
  value: number
): string {
  const name = (enumObj as Record<number, string>)[value];
  return name ?? String(value);
}

type ResolvedMapType =
  | { kind: "enum"; enumObj: Record<string, string | number> }
  | { kind: "class"; ctor: new (...args: unknown[]) => unknown }
  | { kind: "class[]"; ctor: new (...args: unknown[]) => unknown }
  | {
      kind: "boolean" | "number" | "string" | "boolean[]" | "number[]" | "unknown";
    };

function resolveMapType(typeFn: () => unknown): ResolvedMapType {
  const typeRef = typeFn();
  if (Array.isArray(typeRef)) {
    const elem = typeRef[0];
    if (elem === Boolean) {
      return { kind: "boolean[]" };
    }
    if (elem === Number) {
      return { kind: "number[]" };
    }
    if (typeof elem === "function") {
      return {
        kind: "class[]",
        ctor: elem as new (...args: unknown[]) => unknown,
      };
    }
    return { kind: "unknown" };
  }
  if (typeRef === Boolean) {
    return { kind: "boolean" };
  }
  if (typeRef === Number) {
    return { kind: "number" };
  }
  if (typeRef === String) {
    return { kind: "string" };
  }
  if (isTsEnum(typeRef)) {
    return { kind: "enum", enumObj: typeRef };
  }
  if (typeof typeRef === "function") {
    return { kind: "class", ctor: typeRef as new (...args: unknown[]) => unknown };
  }
  return { kind: "unknown" };
}

function serializeField(value: unknown, mapType: ResolvedMapType): unknown {
  if (value === undefined) {
    return undefined;
  }
  switch (mapType.kind) {
    case "enum":
      return typeof value === "number"
        ? enumToString(mapType.enumObj, value)
        : value;
    case "class":
      return blfToJson(value);
    case "class[]":
      return Array.isArray(value) ? value.map((item) => blfToJson(item)) : value;
    case "boolean[]":
    case "number[]":
    case "boolean":
    case "number":
    case "string":
      return value;
    default:
      return blfToJson(value);
  }
}

function blfToJsonFromMetadata(
  instance: Record<string, unknown>,
  meta: [string, AutoMapOptions][]
): Record<string, unknown> {
  const out: Record<string, unknown> = {};
  for (const [fieldName, options] of meta) {
    const fieldValue = instance[fieldName];
    if (fieldValue === undefined) {
      continue;
    }
    const typeFn = options.type;
    if (!typeFn) {
      continue;
    }
    const serialized = serializeField(fieldValue, resolveMapType(typeFn));
    if (serialized !== undefined) {
      out[fieldName] = serialized;
    }
  }
  return out;
}

/** Internal JSON-friendly serialization driven by `@AutoMap` metadata (test snapshots). */
export function blfToJson(value: unknown): unknown {
  if (value === null || value === undefined) {
    return undefined;
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

  const ctor = (value as object).constructor;
  if (typeof ctor === "function" && ctor !== Object) {
    const meta = stage3AutoMapMetadata(ctor as new (...args: unknown[]) => unknown);
    if (meta.length > 0) {
      return blfToJsonFromMetadata(value as Record<string, unknown>, meta);
    }
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
