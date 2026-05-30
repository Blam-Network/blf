import { BitstreamError } from "./errors";

/** TypeScript numeric enum object passed to {@link c_bitstream_reader.read_enum}. */
export type NumericEnum = Record<string, number | string>;

export type EnumNumber<E extends NumericEnum> = Extract<E[keyof E], number>;

/** Numeric enum members in source declaration order (wire index order). */
export function enumMembersInDeclarationOrder(enumObj: NumericEnum): number[] {
  return Object.keys(enumObj)
    .filter((key) => Number.isNaN(Number(key)))
    .map((key) => enumObj[key] as number);
}

export function enumMemberFromWireIndex(
  enumObj: NumericEnum,
  index: number,
  name: string
): number {
  const members = enumMembersInDeclarationOrder(enumObj);
  if (!Number.isInteger(index) || index < 0 || index >= members.length) {
    throw new BitstreamError(`Unexpected enum value for ${name}: ${index}`);
  }
  return members[index]!;
}

export function enumWireIndexFromMember(
  enumObj: NumericEnum,
  member: number,
  name: string
): number {
  const index = enumMembersInDeclarationOrder(enumObj).indexOf(member);
  if (index === -1) {
    throw new BitstreamError(`Unexpected enum member for ${name}: ${member}`);
  }
  return index;
}

export function enumMembersIncludeValue(
  enumObj: NumericEnum,
  value: number
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
  size_in_bits: number
): void {
  const max = (1 << size_in_bits) - 1;
  if (value < 0 || value > max) {
    throw new BitstreamError(
      `Value ${value} for ${name} does not fit in ${size_in_bits} bits (max ${max})`
    );
  }
}
