export { AUTOMAP_PROPERTIES_METADATA_KEY } from "@automapper/classes";
export { AutoMap, stage3AutoMapMetadata } from "../helpers/automap";
export { createMapWithEnumByName } from "./create_map";
export { createClassesMapper } from "./create_mapper";
export {
  convertEnumByName,
  type EnumObject,
  enumTypeConverterConfigs,
  isTypeScriptEnum,
  type ModuleExports,
  type ModulePair,
} from "./enum_by_name";
export {
  type ClassConstructor,
  installStage3AutoMapperMetadata,
  installStage3AutoMapperMetadataForCtorMap,
  installStage3AutoMapperMetadataForMany,
} from "./stage3_metadata";
