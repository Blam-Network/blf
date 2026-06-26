import type { BitfieldFields, BitfieldOf } from "./bitfield";
import type { c_bitstream_reader } from "./reader";
import type { c_bitstream_writer } from "./writer";

function lastWordMask(bitCount: number): number {
  const remainder = bitCount % 32;
  if (remainder === 0) {
    return 0xffffffff;
  }
  return (1 << remainder) - 1;
}

export function bigBitfieldFromWords<const F extends BitfieldFields>(
  words: number[],
  fields: F
): BitfieldOf<F> {
  const result = {} as BitfieldOf<F>;
  for (let index = 0; index < fields.length; index++) {
    const key = fields[index] as F[number];
    const word = Math.floor(index / 32);
    const bit = index % 32;
    result[key] = ((words[word] ?? 0) >> bit) & 1 ? true : false;
  }
  return result;
}

export function bigBitfieldToWords<const F extends BitfieldFields>(
  value: BitfieldOf<F>,
  fields: F
): number[] {
  const wordCount = Math.ceil(fields.length / 32);
  const words = Array.from({ length: wordCount }, () => 0);
  for (let index = 0; index < fields.length; index++) {
    if (value[fields[index] as F[number]]) {
      words[Math.floor(index / 32)] |= 1 << (index % 32);
    }
  }
  if (wordCount > 0) {
    words[wordCount - 1] &= lastWordMask(fields.length);
  }
  return words;
}

export function decodeBigBitfield<const F extends BitfieldFields>(
  bitstream: c_bitstream_reader,
  name: string,
  fields: F
): BitfieldOf<F> {
  const wordCount = Math.ceil(fields.length / 32);
  const words: number[] = [];
  for (let i = 0; i < wordCount; i++) {
    words.push(bitstream.read_integer(name, 32));
  }
  if (wordCount > 0) {
    words[wordCount - 1] &= lastWordMask(fields.length);
  }
  return bigBitfieldFromWords(words, fields);
}

export function encodeBigBitfield<const F extends BitfieldFields>(
  bitstream: c_bitstream_writer,
  value: BitfieldOf<F>,
  fields: F,
  name: string
): void {
  for (const word of bigBitfieldToWords(value, fields)) {
    bitstream.write_integer(word, 32);
    const _ = name;
  }
}

export function defaultBigBitfield<const F extends BitfieldFields>(
  fields: F
): BitfieldOf<F> {
  const result = {} as BitfieldOf<F>;
  for (const field of fields) {
    result[field as F[number]] = false;
  }
  return result;
}
