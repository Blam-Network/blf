/**
 * Stage-3 `@AutoMap` for Reach gametype classes.
 * Compatible with `@craftycodie/cstruct` field decorators in the same project.
 */

/** TS stage-3 emit stores field metadata on `ctor[Symbol.metadata]`; polyfill when missing (Node < 22). */
function ensureSymbolMetadata(): void {
  if (typeof Symbol === "undefined") {
    return;
  }
  const symbolCtor = Symbol as SymbolConstructor & {
    metadata?: symbol;
  };
  if (symbolCtor.metadata !== undefined) {
    return;
  }
  try {
    Object.defineProperty(Symbol, "metadata", {
      value: Symbol.for("Symbol.metadata"),
      writable: false,
      enumerable: false,
      configurable: true,
    });
  } catch {
    // already defined by another copy of this module
  }
}

ensureSymbolMetadata();

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
    const metadata = context.metadata as
      | Record<string, [string, AutoMapOptions][]>
      | undefined;
    if (!metadata) {
      throw new Error(
        "@AutoMap stage-3 metadata is unavailable (Symbol.metadata missing). " +
          "Import from @blamnetwork/blf/helpers/automap before decorated classes."
      );
    }
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
