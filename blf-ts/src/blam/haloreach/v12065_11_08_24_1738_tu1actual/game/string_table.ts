import { type c, CFieldType } from "@craftycodie/cstruct";
import {
  type c_bitstream_reader,
  c_bitstream_writer,
  e_bitstream_byte_order,
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
    if (compressed.length < buffer.length) {
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
  strings: (string | null)[][] = Array.from(
    { length: k_language_count },
    () => []
  );
  m_buffer_is_compressed = true;
  constructor(
    readonly max_string_count: number,
    readonly max_string_length: number,
    readonly offset_bit_length: number,
    readonly buffer_size_bit_length: number,
    readonly count_bit_length: number
  ) {}
  decode(bitstream: c_bitstream_reader): void {
    this.strings = Array.from({ length: k_language_count }, () => []);
    const string_count = bitstream.read_integer(
      "string-count",
      this.count_bit_length
    );
    if (string_count === 0) {
      return;
    }
    const offsets: number[][] = Array.from({ length: k_language_count }, () =>
      Array.from({ length: string_count }, () => -1)
    );
    for (let j = 0; j < string_count; j++) {
      for (let i = 0; i < k_language_count; i++) {
        if (bitstream.read_bool("exists")) {
          offsets[i]![j] = bitstream.read_integer(
            "index",
            this.offset_bit_length
          );
        }
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
    for (let i = 0; i < k_language_count; i++) {
      for (let j = 0; j < string_count; j++) {
        const offset = offsets[i]![j]!;
        if (offset < 0) {
          this.strings[i]!.push(null);
        } else {
          this.strings[i]!.push(
            read_null_terminated_string(string_data, offset)
          );
        }
      }
    }
  }
  encode(bitstream: c_bitstream_writer): void {
    const string_count = this.strings[0]?.length ?? 0;
    bitstream.write_integer(string_count, this.count_bit_length);
    if (string_count === 0) {
      return;
    }
    const string_writer = c_bitstream_writer.new(
      k_language_count * this.max_string_count * this.max_string_length,
      e_bitstream_byte_order._bitstream_byte_order_big_endian
    );
    string_writer.begin_writing();
    let offset = 0;
    for (let j = 0; j < string_count; j++) {
      let deduplicate = true;
      const reference = this.strings[0]?.[j] ?? null;
      if (reference === null) {
        deduplicate = false;
      } else {
        for (let i = 1; i < k_language_count; i++) {
          if (this.strings[i]?.[j] !== reference) {
            deduplicate = false;
            break;
          }
        }
      }
      for (let i = 0; i < k_language_count; i++) {
        const string = this.strings[i]?.[j] ?? null;
        if (string === null) {
          bitstream.write_bool(false);
          continue;
        }
        bitstream.write_bool(true);
        bitstream.write_integer(offset, this.offset_bit_length);
        if (!deduplicate) {
          write_null_terminated_string(
            string_writer,
            string,
            this.max_string_length
          );
          offset += string_byte_length(string) + 1;
        }
      }
      if (deduplicate && reference !== null) {
        write_null_terminated_string(
          string_writer,
          reference,
          this.max_string_length
        );
        offset += string_byte_length(reference) + 1;
      }
    }
    string_writer.finish_writing();
    const buffer = string_writer.get_data();
    write_buffer_blob(bitstream, buffer, this.buffer_size_bit_length);
  }
}

export class CStringTableField extends CFieldType<c_string_table> {
  constructor(
    private readonly create: () => c_string_table,
    private readonly string_capacity: number,
    private readonly buffer_capacity: number
  ) {
    super();
  }

  getSize(): number {
    return (
      this.string_capacity * k_language_count * 2 + this.buffer_capacity + 8
    );
  }

  read(bytes: Uint8Array, offset: number, endian: c.Endian): c_string_table {
    const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
    const little = endian === "little";
    const offsets_size = this.string_capacity * k_language_count * 2;
    const buffer_offset = offset + offsets_size;
    const buffer = bytes.subarray(
      buffer_offset,
      buffer_offset + this.buffer_capacity
    );
    const string_count = Math.min(
      this.string_capacity,
      Math.max(
        0,
        view.getInt32(buffer_offset + this.buffer_capacity + 4, little)
      )
    );
    const table = this.create();
    table.strings = Array.from({ length: k_language_count }, () => []);
    for (let language = 0; language < k_language_count; language++) {
      for (let index = 0; index < string_count; index++) {
        const string_offset = view.getInt16(
          offset + (index * k_language_count + language) * 2,
          little
        );
        table.strings[language]!.push(
          string_offset < 0
            ? null
            : read_null_terminated_string(buffer, string_offset)
        );
      }
    }
    return table;
  }

  write(
    bytes: Uint8Array,
    offset: number,
    value: c_string_table,
    endian: c.Endian
  ): void {
    const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
    const little = endian === "little";
    const offsets_size = this.string_capacity * k_language_count * 2;
    const buffer_offset = offset + offsets_size;
    const string_count = Math.min(
      this.string_capacity,
      value.strings[0]?.length ?? 0
    );
    const encoder = new TextEncoder();
    let used = 0;
    for (let index = 0; index < this.string_capacity; index++) {
      const reference = value.strings[0]?.[index] ?? null;
      const shared =
        index < string_count &&
        reference !== null &&
        value.strings.every((language) => language[index] === reference);
      let shared_offset = -1;
      for (let language = 0; language < k_language_count; language++) {
        const string =
          index < string_count
            ? (value.strings[language]?.[index] ?? null)
            : null;
        let string_offset = -1;
        if (string !== null) {
          if (shared && shared_offset >= 0) {
            string_offset = shared_offset;
          } else {
            const encoded = encoder.encode(string);
            string_offset = used;
            bytes.set(
              encoded.subarray(0, this.buffer_capacity - used - 1),
              buffer_offset + used
            );
            used = Math.min(this.buffer_capacity, used + encoded.length + 1);
            shared_offset = string_offset;
          }
        }
        view.setInt16(
          offset + (index * k_language_count + language) * 2,
          string_offset,
          little
        );
      }
    }
    view.setUint32(buffer_offset + this.buffer_capacity, used, little);
    view.setInt32(
      buffer_offset + this.buffer_capacity + 4,
      string_count,
      little
    );
  }
}

export const string_table_field = (
  create: () => c_string_table,
  string_capacity: number,
  buffer_capacity: number
) => new CStringTableField(create, string_capacity, buffer_capacity);
