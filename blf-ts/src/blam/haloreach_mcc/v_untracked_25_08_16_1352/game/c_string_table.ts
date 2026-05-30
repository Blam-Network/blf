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

/** One wire byte per code unit (U+00–U+FF), matching blf_lib extended ASCII / NullString slots. */
function read_null_terminated_string(data: Uint8Array, offset: number): string {
  let end = offset;
  while (end < data.length && data[end] !== 0) {
    end++;
  }
  let out = "";
  for (let i = offset; i < end; i++) {
    out += String.fromCharCode(data[i]!);
  }
  return out;
}

function write_null_terminated_string(
  writer: c_bitstream_writer,
  value: string,
  max_length: number
): void {
  writer.write_string_extended_ascii(value, max_length);
}

/** Wire byte length (one byte per decoded code unit). */
function string_byte_length(value: string): number {
  for (let i = 0; i < value.length; i++) {
    if (value.charCodeAt(i) > 0xff) {
      throw new Error(
        `String table entry U+${value.charCodeAt(i).toString(16)} cannot be encoded as Latin-1`
      );
    }
  }
  return value.length;
}

function write_buffer_blob(
  bitstream: c_bitstream_writer,
  buffer: Uint8Array,
  buffer_size_bit_length: number,
  is_compressed: boolean
): void {
  bitstream.write_integer(buffer.length, buffer_size_bit_length);
  if (is_compressed) {
    const compressed = runtime_data_compress(buffer, true);
    bitstream.write_bool(true);
    bitstream.write_integer(compressed.length, buffer_size_bit_length);
    bitstream.write_raw_data(compressed, compressed.length * 8);
  } else {
    bitstream.write_bool(false);
    bitstream.write_raw_data(buffer, buffer.length * 8);
  }
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
    write_buffer_blob(
      bitstream,
      buffer,
      this.buffer_size_bit_length,
      this.m_buffer_is_compressed
    );
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
    write_buffer_blob(
      bitstream,
      buffer,
      this.buffer_size_bit_length,
      this.m_buffer_is_compressed
    );
  }
}
