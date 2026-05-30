/**
 * Stage-3 `@AutoMap` for Reach gametype classes.
 * Compatible with `@craftycodie/cstruct` field decorators in the same project.
 */

import type { AutoMapOptions } from "@automapper/classes";
import { AUTOMAP_PROPERTIES_METADATA_KEY as automapPropertiesKey } from "@automapper/classes";
import type { Constructor } from "@automapper/core";

export { AUTOMAP_PROPERTIES_METADATA_KEY } from "@automapper/classes";

function resolveOptions(
  typeFnOrOptions?: (() => unknown) | AutoMapOptions
): AutoMapOptions {
  if (typeFnOrOptions === undefined) {
    throw new Error(
      "@AutoMap requires an explicit type, e.g. @AutoMap(() => MyType)"
    );
  }
  if (typeof typeFnOrOptions === "function") {
    return {
      type: typeFnOrOptions as () => Constructor | [Constructor],
      depth: 1,
    };
  }
  return { depth: 1, ...typeFnOrOptions };
}

export function AutoMap(
  typeFnOrOptions: (() => unknown) | AutoMapOptions
): (value: undefined, context: ClassFieldDecoratorContext) => void {
  const options = resolveOptions(typeFnOrOptions);
  return (_value, context) => {
    if (context.kind !== "field") {
      return;
    }
    const metadata = context.metadata as Record<
      string,
      [string, AutoMapOptions][]
    >;
    const list = metadata[automapPropertiesKey] ?? [];
    list.push([String(context.name), options]);
    metadata[automapPropertiesKey] = list;
  };
}

export function stage3AutoMapMetadata(
  ctor: new (...args: unknown[]) => unknown
): [string, AutoMapOptions][] {
  const classMetadata = (ctor as unknown as Record<symbol, unknown>)[
    Symbol.metadata
  ] as Record<string, [string, AutoMapOptions][]> | undefined;
  return classMetadata?.[automapPropertiesKey] ?? [];
}
