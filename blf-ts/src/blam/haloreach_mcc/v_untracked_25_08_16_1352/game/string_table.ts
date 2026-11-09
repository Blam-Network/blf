import {
  type c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../bitstream";
import {
  runtime_data_compress,
  runtime_data_decompress,
} from "../../../common/memory/data_compress";
export const k_language_count = 12;
/** Null-terminated UTF-8 bytes (matches blf_lib `write_string_utf8` / `String::as_bytes()`). */
function read_null_terminated_string(data: Uint8Array, offset: number): string {
  let end = offset;
  while (end < data.length && data[end] !== 0) {
    end++;
  }
  return new TextDecoder("utf-8").decode(data.subarray(offset, end));
}
function write_null_terminated_string(
  writer: c_bitstream_writer,
  value: string,
  max_length: number
): void {
  writer.write_string_utf8(value, max_length);
}
/** Wire byte length (UTF-8 encoded, excluding NUL). */
function string_byte_length(value: string): number {
  return new TextEncoder().encode(value).length;
}
/** Reach `c_string_buffer::encode` only attempts compression at this size or above. */
const k_string_buffer_compression_threshold = 128;

function write_buffer_blob(
  bitstream: c_bitstream_writer,
  buffer: Uint8Array,
  buffer_size_bit_length: number
): void {
  bitstream.write_integer(buffer.length, buffer_size_bit_length);
  if (buffer.length >= k_string_buffer_compression_threshold) {
    const compressed = runtime_data_compress(buffer, true);
    if (compressed.length <= buffer.length) {
      bitstream.write_bool(true);
      bitstream.write_integer(compressed.length, buffer_size_bit_length);
      bitstream.write_raw_data(compressed, compressed.length * 8);
      return;
    }
  }
  bitstream.write_bool(false);
  bitstream.write_raw_data(buffer, buffer.length * 8);
}
export class c_single_language_string_table {
  strings: string[] = [];
  m_buffer_is_compressed = true;
  constructor(
    readonly max_string_count: number,
    readonly max_string_length: number,
    readonly offset_bit_length: number,
    readonly buffer_size_bit_length: number,
    readonly count_bit_length: number
  ) {}
  decode(bitstream: c_bitstream_reader): void {
    this.strings = [];
    const string_count = bitstream.read_integer(
      "string-count",
      this.count_bit_length
    );
    if (string_count === 0) {
      return;
    }
    const offsets: number[] = [];
    for (let i = 0; i < string_count; i++) {
      if (bitstream.read_bool("exists")) {
        offsets.push(bitstream.read_integer("offset", this.offset_bit_length));
      } else {
        offsets.push(0);
      }
    }
    const buffer_size = bitstream.read_integer(
      "size",
      this.buffer_size_bit_length
    );
    this.m_buffer_is_compressed = bitstream.read_bool("compressed");
    const string_data = this.m_buffer_is_compressed
      ? runtime_data_decompress(
          bitstream.read_raw_data(
            bitstream.read_integer(
              "compressed-buffer-size",
              this.buffer_size_bit_length
            ) * 8
          )
        )
      : bitstream.read_raw_data(buffer_size * 8);
    for (const offset of offsets) {
      this.strings.push(read_null_terminated_string(string_data, offset));
    }
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.strings.length, this.count_bit_length);
    if (this.strings.length === 0) {
      return;
    }
    const string_writer = c_bitstream_writer.new_from_instance(
      this.max_string_count * this.max_string_length,
      bitstream
    );
    string_writer.begin_writing();
    let offset = 0;
    for (const string of this.strings) {
      bitstream.write_bool(true);
      bitstream.write_integer(offset, this.offset_bit_length);
      write_null_terminated_string(
        string_writer,
        string,
        this.max_string_length
      );
      offset += string_byte_length(string) + 1;
    }
    string_writer.finish_writing();
    const buffer = string_writer.get_data();
    write_buffer_blob(bitstream, buffer, this.buffer_size_bit_length);
  }
}
export class c_string_table {
  /** Per language, each string's byte offset into {@link m_buffer}, or -1 when it has none. */
  m_offsets: number[][] = Array.from({ length: k_language_count }, () => []);
  m_buffer: Uint8Array = new Uint8Array(0);
  m_buffer_is_compressed = true;
  constructor(
    readonly max_string_count: number,
    readonly max_string_length: number,
    readonly offset_bit_length: number,
    readonly buffer_size_bit_length: number,
    readonly count_bit_length: number
  ) {}
  get string_count(): number {
    return this.m_offsets[0]?.length ?? 0;
  }
  get_string(language: number, index: number): string | null {
    const offset = this.m_offsets[language]?.[index] ?? -1;
    return offset < 0
      ? null
      : read_null_terminated_string(this.m_buffer, offset);
  }
  /**
   * Sets one string, reusing an identical string already in the buffer. Strings this replaces
   * stay in the buffer; assign {@link strings} to repack in MegaloEdit's layout.
   */
  set_string(language: number, index: number, value: string | null): void {
    if (index >= this.max_string_count) {
      throw new RangeError(
        `string table holds ${this.max_string_count} strings, can't set string ${index}`
      );
    }
    for (const offsets of this.m_offsets) {
      while (offsets.length <= index) {
        offsets.push(-1);
      }
    }
    this.m_offsets[language]![index] = value === null ? -1 : this.store(value);
  }
  /** Every language's strings, read from the buffer. Frozen: assign a new array to change them. */
  get strings(): readonly (readonly (string | null)[])[] {
    return Object.freeze(
      this.m_offsets.map((offsets, language) =>
        Object.freeze(
          offsets.map((_, index) => this.get_string(language, index))
        )
      )
    );
  }
  /**
   * Repacks the buffer in MegaloEdit's layout: strings in index then language order, a string
   * that's the same in every language stored once, any other string stored once per language.
   */
  set strings(value: readonly (readonly (string | null)[])[]) {
    const count = value[0]?.length ?? 0;
    if (count > this.max_string_count) {
      throw new RangeError(
        `string table holds ${this.max_string_count} strings, got ${count}`
      );
    }
    this.m_buffer = new Uint8Array(0);
    this.m_offsets = Array.from({ length: k_language_count }, () => []);
    for (let index = 0; index < count; index++) {
      const row = Array.from(
        { length: k_language_count },
        (_, language) => value[language]?.[index] ?? null
      );
      const shared =
        row[0] !== null && row.every((string) => string === row[0]);
      if (shared) {
        const offset = this.append(row[0]!);
        for (const offsets of this.m_offsets) {
          offsets.push(offset);
        }
        continue;
      }
      for (const [language, string] of row.entries()) {
        this.m_offsets[language]!.push(
          string === null ? -1 : this.append(string)
        );
      }
    }
  }
  // The offset of `value` in the buffer, appending it unless it's already there.
  private store(value: string): number {
    for (const offsets of this.m_offsets) {
      for (const offset of offsets) {
        if (
          offset >= 0 &&
          read_null_terminated_string(this.m_buffer, offset) === value
        ) {
          return offset;
        }
      }
    }
    return this.append(value);
  }
  private append(value: string): number {
    const encoded = new TextEncoder().encode(value);
    const offset = this.m_buffer.length;
    if (offset + encoded.length + 1 > this.max_string_length) {
      throw new RangeError(
        `string table buffer holds ${this.max_string_length} bytes, can't add ${encoded.length + 1} more to ${offset}`
      );
    }
    const buffer = new Uint8Array(offset + encoded.length + 1);
    buffer.set(this.m_buffer);
    buffer.set(encoded, offset);
    this.m_buffer = buffer;
    return offset;
  }
  decode(bitstream: c_bitstream_reader): void {
    this.m_offsets = Array.from({ length: k_language_count }, () => []);
    this.m_buffer = new Uint8Array(0);
    const string_count = bitstream.read_integer(
      "string-count",
      this.count_bit_length
    );
    if (string_count === 0) {
      return;
    }
    for (let j = 0; j < string_count; j++) {
      for (let i = 0; i < k_language_count; i++) {
        this.m_offsets[i]!.push(
          bitstream.read_bool("exists")
            ? bitstream.read_integer("index", this.offset_bit_length)
            : -1
        );
      }
    }
    const buffer_size = bitstream.read_integer(
      "size",
      this.buffer_size_bit_length
    );
    this.m_buffer_is_compressed = bitstream.read_bool("compressed");
    this.m_buffer = this.m_buffer_is_compressed
      ? runtime_data_decompress(
          bitstream.read_raw_data(
            bitstream.read_integer(
              "compressed-buffer-size",
              this.buffer_size_bit_length
            ) * 8
          )
        )
      : bitstream.read_raw_data(buffer_size * 8);
  }
  encode(bitstream: c_bitstream_writer): void {
    const string_count = this.string_count;
    bitstream.write_integer(string_count, this.count_bit_length);
    if (string_count === 0) {
      return;
    }
    for (let j = 0; j < string_count; j++) {
      for (let i = 0; i < k_language_count; i++) {
        const offset = this.m_offsets[i]?.[j] ?? -1;
        bitstream.write_bool(offset >= 0);
        if (offset >= 0) {
          bitstream.write_integer(offset, this.offset_bit_length);
        }
      }
    }
    write_buffer_blob(bitstream, this.m_buffer, this.buffer_size_bit_length);
  }
}
