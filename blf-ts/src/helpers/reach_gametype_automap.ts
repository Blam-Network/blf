/**
 * Registers AutoMapper cross-version profiles for Reach gametype conversion.
 * Mapping metadata comes from `@AutoMap()` on game classes.
 */

import {
  type ClassConstructor,
  createClassesMapper,
  createMapWithEnumByName,
  installStage3AutoMapperMetadataForCtorMap,
  type ModuleExports,
  type ModulePair,
} from "../automapper";
import * as tu1_game from "../blam/haloreach/v12065_11_08_24_1738_tu1actual/game";
import { c_game_variant as c_game_variant_tu1 } from "../blam/haloreach/v12065_11_08_24_1738_tu1actual/game/game_variant";
import * as mcc_game from "../blam/haloreach_mcc/v_untracked_25_08_16_1352/game";
import { c_game_variant as c_game_variant_mcc } from "../blam/haloreach_mcc/v_untracked_25_08_16_1352/game/game_variant";

function isClassConstructor(value: unknown): value is ClassConstructor {
  return (
    typeof value === "function" &&
    value.prototype !== undefined &&
    value.prototype.constructor === value
  );
}

function buildClassConstructorMap(
  sourceModule: ModuleExports,
  targetModule: ModuleExports
): Map<ClassConstructor, ClassConstructor> {
  const map = new Map<ClassConstructor, ClassConstructor>();
  for (const key of Object.keys(sourceModule)) {
    const sourceExport = sourceModule[key];
    const targetExport = targetModule[key];
    if (
      isClassConstructor(sourceExport) &&
      isClassConstructor(targetExport) &&
      sourceExport !== targetExport
    ) {
      map.set(sourceExport, targetExport);
    }
  }
  return map;
}

export const MCC_TO_TU1_CTORS = buildClassConstructorMap(mcc_game, tu1_game);
export const TU1_TO_MCC_CTORS = buildClassConstructorMap(tu1_game, mcc_game);

export const mccToTu1Mapper = createClassesMapper();
export const tu1ToMccMapper = createClassesMapper();

installStage3AutoMapperMetadataForCtorMap(
  MCC_TO_TU1_CTORS,
  c_game_variant_mcc,
  c_game_variant_tu1
);
installStage3AutoMapperMetadataForCtorMap(
  TU1_TO_MCC_CTORS,
  c_game_variant_tu1,
  c_game_variant_mcc
);

function registerCrossVersionProfiles(
  mapper: ReturnType<typeof createClassesMapper>,
  ctorMap: Map<ClassConstructor, ClassConstructor>,
  rootSource: ClassConstructor,
  rootDest: ClassConstructor,
  modulePairs: readonly ModulePair[]
): void {
  const pairs = new Map<ClassConstructor, ClassConstructor>();
  for (const [sourceCtor, destCtor] of ctorMap) {
    pairs.set(sourceCtor, destCtor);
  }
  pairs.set(rootSource, rootDest);

  for (const [source, dest] of pairs) {
    createMapWithEnumByName(mapper, source, dest, modulePairs);
  }
}

registerCrossVersionProfiles(
  mccToTu1Mapper,
  MCC_TO_TU1_CTORS,
  c_game_variant_mcc,
  c_game_variant_tu1,
  [[mcc_game, tu1_game]]
);
registerCrossVersionProfiles(
  tu1ToMccMapper,
  TU1_TO_MCC_CTORS,
  c_game_variant_tu1,
  c_game_variant_mcc,
  [[tu1_game, mcc_game]]
);
