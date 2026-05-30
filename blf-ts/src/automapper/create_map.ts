import {
  createMap,
  type Dictionary,
  type Mapper,
  type Mapping,
  type MappingConfiguration,
  type ModelIdentifier,
} from "@automapper/core";
import { enumTypeConverterConfigs, type ModulePair } from "./enum_by_name";

/** `createMap` with enum-by-name converters derived from module export pairs. */
export function createMapWithEnumByName<
  TSource extends Dictionary<TSource>,
  TDestination extends Dictionary<TDestination>,
>(
  mapper: Mapper,
  source: ModelIdentifier<TSource>,
  destination: ModelIdentifier<TDestination>,
  modulePairs: readonly ModulePair[],
  ...configurations: MappingConfiguration<TSource, TDestination>[]
): Mapping<TSource, TDestination> {
  return createMap(
    mapper,
    source,
    destination,
    ...enumTypeConverterConfigs(...modulePairs),
    ...configurations
  );
}
