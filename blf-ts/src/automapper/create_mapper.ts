import { classes } from "@automapper/classes";
import {
  type CreateMapperOptions,
  createMapper,
  type Mapper,
} from "@automapper/core";

type ClassesMapperOptions = Omit<CreateMapperOptions, "strategyInitializer">;

/** Standard BLF mapper using the `@automapper/classes` strategy. */
export function createClassesMapper(
  options: ClassesMapperOptions = {}
): Mapper {
  return createMapper({
    ...options,
    strategyInitializer: (mapper) => classes()(mapper),
  });
}
