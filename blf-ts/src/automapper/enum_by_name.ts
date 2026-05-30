/**
 * AutoMapper enum conversion by member name (not ordinal).
 *
 * TypeScript numeric enums on the wire are plain numbers; AutoMapper copies them
 * as-is unless a `typeConverter` is registered. Use these helpers when paired
 * enums share member names but differ in numeric values.
 */

import { type MappingConfiguration, typeConverter } from "@automapper/core";

export type EnumObject = Record<string, number | string>;
export type ModuleExports = Record<string, unknown>;
export type ModulePair = readonly [ModuleExports, ModuleExports];

const enumTypeConverter = typeConverter as unknown as (
  source: EnumObject,
  dest: EnumObject,
  converter: (value: number) => number
) => MappingConfiguration;

export function isTypeScriptEnum(value: unknown): value is EnumObject {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    return false;
  }
  let hasNamedMember = false;
  const record = value as Record<string, unknown>;
  for (const key of Object.keys(record)) {
    if (Number.isNaN(Number(key))) {
      hasNamedMember = typeof record[key] === "number";
    }
  }
  return hasNamedMember;
}

export function convertEnumByName(
  sourceEnum: EnumObject,
  destEnum: EnumObject,
  ordinal: number
): number {
  const name = sourceEnum[ordinal];
  if (typeof name !== "string") {
    return ordinal;
  }
  const destOrdinal = destEnum[name];
  return typeof destOrdinal === "number" ? destOrdinal : ordinal;
}

/** Build shared enum `typeConverter` configs from one or more module export pairs. */
export function enumTypeConverterConfigs(
  ...modulePairs: ModulePair[]
): MappingConfiguration[] {
  const configs: MappingConfiguration[] = [];
  const registered = new Set<string>();

  for (const [sourceModule, destModule] of modulePairs) {
    for (const key of Object.keys(sourceModule)) {
      const sourceEnum = sourceModule[key];
      const destEnum = destModule[key];
      if (!(isTypeScriptEnum(sourceEnum) && isTypeScriptEnum(destEnum))) {
        continue;
      }
      if (registered.has(key)) {
        continue;
      }
      registered.add(key);
      configs.push(
        enumTypeConverter(sourceEnum, destEnum, (value) =>
          convertEnumByName(sourceEnum, destEnum, value)
        )
      );
    }
  }

  return configs;
}
