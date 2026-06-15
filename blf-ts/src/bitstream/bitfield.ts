/** Field names in declaration order; bit 0 is the first field. */
export type BitfieldFields = readonly string[];

export type BitfieldOf<F extends BitfieldFields> = { [K in F[number]]: boolean };

export function bitfieldFromRaw<const F extends BitfieldFields>(
  raw: number,
  fields: F
): BitfieldOf<F> {
  const result = {} as BitfieldOf<F>;
  for (let i = 0; i < fields.length; i++) {
    const key = fields[i] as F[number];
    result[key] = ((raw >> i) & 1) !== 0;
  }
  return result;
}

export function bitfieldToRaw<const F extends BitfieldFields>(
  value: BitfieldOf<F>,
  fields: F
): number {
  let raw = 0;
  for (let i = 0; i < fields.length; i++) {
    if (value[fields[i] as F[number]]) {
      raw |= 1 << i;
    }
  }
  return raw;
}
