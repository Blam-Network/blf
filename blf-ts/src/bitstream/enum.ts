import { BitstreamError } from "./errors";

/** TypeScript numeric enum object passed to {@link c_bitstream_reader.read_enum}. */
export type NumericEnum = Record<string, number | string>;

export type EnumNumber<E extends NumericEnum> = Extract<
  E[keyof E],
  number
>;

export function isNumericEnumValue(
  enumObj: NumericEnum,
  value: number,
): boolean {
  for (const member of Object.values(enumObj)) {
    if (typeof member === "number" && member === value) {
      return true;
    }
  }
  return false;
}

export function assertFitsInBits(
  name: string,
  value: number,
  size_in_bits: number,
): void {
  const max = (1 << size_in_bits) - 1;
  if (value < 0 || value > max) {
    throw new BitstreamError(
      `Value ${value} for ${name} does not fit in ${size_in_bits} bits (max ${max})`,
    );
  }
}
