/**
 * JSON round-trip for game variant class graphs.
 *
 * Uses `c.json` when a type is decorated with `@c.struct()`.
 * Bitstream-decoded game variant types fall back to a cstruct-compatible walk
 * that preserves numeric enums and serializes bigint values as strings.
 */

import { c } from "@craftycodie/cstruct";

type JsonRecord = Record<string, unknown>;
type ClassConstructor = new () => object;

function hasStructLayout(ctor: ClassConstructor): boolean {
  return c.CSTRUCT_LAYOUT in ctor;
}

function instanceToJson(value: unknown): unknown {
  if (value === null || value === undefined) {
    return value;
  }
  if (typeof value === "bigint") {
    return value.toString();
  }
  if (value instanceof Date) {
    return value.toISOString();
  }
  if (value instanceof Uint8Array) {
    return { __bytes__: Buffer.from(value).toString("hex") };
  }
  if (typeof value !== "object") {
    return value;
  }
  if (Array.isArray(value)) {
    return value.map(instanceToJson);
  }
  const out: JsonRecord = {};
  for (const key of Object.keys(value)) {
    out[key] = instanceToJson((value as JsonRecord)[key]);
  }
  return out;
}

function resolveTargetConstructor(
  templateValue: unknown,
  ctorMap: Map<ClassConstructor, ClassConstructor>
): ClassConstructor | undefined {
  if (
    templateValue === null ||
    typeof templateValue !== "object" ||
    Array.isArray(templateValue)
  ) {
    return;
  }
  const sourceCtor = templateValue.constructor;
  if (typeof sourceCtor !== "function") {
    return;
  }
  return (
    ctorMap.get(sourceCtor as ClassConstructor) ??
    (sourceCtor as ClassConstructor)
  );
}

function jsonToInstance(
  Ctor: ClassConstructor,
  json: unknown,
  template: unknown,
  ctorMap: Map<ClassConstructor, ClassConstructor>
): unknown {
  if (json === null || json === undefined) {
    return json;
  }
  if (typeof json !== "object") {
    return json;
  }
  if (Array.isArray(json)) {
    const templateArray = Array.isArray(template) ? template : [];
    return json.map((item, index) => {
      const itemTemplate = templateArray[index] ?? templateArray[0];
      const itemCtor = resolveTargetConstructor(itemTemplate, ctorMap);
      if (itemCtor && typeof item === "object" && item !== null) {
        return jsonToInstance(itemCtor, item, itemTemplate, ctorMap);
      }
      return item;
    });
  }

  const TargetCtor = ctorMap.get(Ctor) ?? Ctor;
  const instance = new TargetCtor();
  const templateRecord =
    template !== null &&
    typeof template === "object" &&
    !Array.isArray(template)
      ? (template as JsonRecord)
      : {};

  for (const key of Object.keys(json as JsonRecord)) {
    const jsonValue = (json as JsonRecord)[key];
    const templateValue = templateRecord[key];
    const nestedCtor = resolveTargetConstructor(templateValue, ctorMap);

    if (
      nestedCtor &&
      typeof jsonValue === "object" &&
      jsonValue !== null &&
      !Array.isArray(jsonValue)
    ) {
      (instance as JsonRecord)[key] = jsonToInstance(
        nestedCtor,
        jsonValue,
        templateValue,
        ctorMap
      );
      continue;
    }

    if (Array.isArray(jsonValue)) {
      (instance as JsonRecord)[key] = jsonToInstance(
        TargetCtor,
        jsonValue,
        templateValue,
        ctorMap
      );
      continue;
    }

    (instance as JsonRecord)[key] = jsonValue;
  }

  return instance;
}

/** Serialize a game variant instance to a cstruct-compatible JSON object. */
export function variantToJson<T extends object>(
  ctor: new () => T,
  instance: T
): JsonRecord {
  if (hasStructLayout(ctor)) {
    return c.json(ctor, instance) as JsonRecord;
  }
  return instanceToJson(instance) as JsonRecord;
}

/** Build a game variant instance from JSON, optionally remapping nested constructors. */
export function variantFromJson<T extends object>(
  ctor: new () => T,
  json: JsonRecord,
  template?: object,
  ctorMap: Map<ClassConstructor, ClassConstructor> = new Map()
): T {
  return jsonToInstance(ctor, json, template ?? new ctor(), ctorMap) as T;
}

/** Pair same-named exports from two version bundles for cross-build hydration. */
export function buildCrossVersionConstructorMap(
  sourceModule: Record<string, unknown>,
  targetModule: Record<string, unknown>
): Map<ClassConstructor, ClassConstructor> {
  const map = new Map<ClassConstructor, ClassConstructor>();
  for (const key of Object.keys(sourceModule)) {
    const sourceExport = sourceModule[key];
    const targetExport = targetModule[key];
    if (
      typeof sourceExport === "function" &&
      typeof targetExport === "function"
    ) {
      map.set(
        sourceExport as ClassConstructor,
        targetExport as ClassConstructor
      );
    }
  }
  return map;
}

/** Deep-clone a variant instance through JSON using cstruct-compatible encoding. */
export function cloneVariantViaJson<T extends object>(
  ctor: new () => T,
  instance: object,
  template?: object,
  ctorMap: Map<ClassConstructor, ClassConstructor> = new Map()
): T {
  return variantFromJson(
    ctor,
    variantToJson(ctor, instance as T),
    template,
    ctorMap
  );
}
