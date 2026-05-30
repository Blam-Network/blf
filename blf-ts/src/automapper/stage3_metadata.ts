import {
  AUTOMAPPER_METADATA_FACTORY_KEY,
  type AutoMapOptions,
} from "@automapper/classes";
import { stage3AutoMapMetadata } from "../helpers/automap";

type ClassConstructor = new (...args: unknown[]) => object;
type MetadataEntry = [string, AutoMapOptions];

function mergeMetadataEntries(
  existing: MetadataEntry[],
  extra: MetadataEntry[]
): MetadataEntry[] {
  const merged = new Map<string, MetadataEntry>();
  for (const entry of existing) {
    merged.set(entry[0], entry);
  }
  for (const entry of extra) {
    merged.set(entry[0], entry);
  }
  return [...merged.values()];
}

/** Bridge stage-3 `@AutoMap()` metadata into `@automapper/classes` factory hook. */
export function installStage3AutoMapperMetadata(ctor: ClassConstructor): void {
  const entries = stage3AutoMapMetadata(ctor);
  if (entries.length === 0) {
    return;
  }
  const record = ctor as unknown as Record<string, unknown>;
  const existingFactory = record[AUTOMAPPER_METADATA_FACTORY_KEY] as
    | (() => MetadataEntry[])
    | undefined;
  record[AUTOMAPPER_METADATA_FACTORY_KEY] = () => {
    const existing = existingFactory?.() ?? [];
    return mergeMetadataEntries(existing, entries);
  };
}

export function installStage3AutoMapperMetadataForMany(
  ...ctors: ClassConstructor[]
): void {
  for (const ctor of ctors) {
    installStage3AutoMapperMetadata(ctor);
  }
}

export function installStage3AutoMapperMetadataForCtorMap(
  ctorMap: Map<ClassConstructor, ClassConstructor>,
  ...extra: ClassConstructor[]
): void {
  const ctors = new Set<ClassConstructor>(extra);
  for (const [source, dest] of ctorMap) {
    ctors.add(source);
    ctors.add(dest);
  }
  installStage3AutoMapperMetadataForMany(...ctors);
}

export type { ClassConstructor };
