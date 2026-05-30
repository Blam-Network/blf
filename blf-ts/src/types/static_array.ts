/** Fixed-length array type matching blf_lib `StaticArray<T, N>`. */
type BuildStaticArray<
  T,
  N extends number,
  Acc extends readonly T[] = [],
> = Acc["length"] extends N
  ? Acc
  : BuildStaticArray<T, N, readonly [...Acc, T]>;

export type StaticArray<T, N extends number> = BuildStaticArray<T, N>;

export function staticArray<const N extends number, T>(
  length: N,
  init: () => T
): StaticArray<T, N> {
  return Array.from({ length }, init) as StaticArray<T, N>;
}
