import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../../bitstream";
import { AutoMap } from "../../../../../helpers/automap";

/** Halo 4 string-table index bits for MaxStrings=148. */
const k_string_index_bits = 8;

export class s_user_defined_option_value {
  @AutoMap(() => Number)
  m_value = 0;
  @AutoMap(() => Number)
  m_name_string_index?: number;
  @AutoMap(() => Number)
  m_description_string_index?: number;
  decode(bitstream: c_bitstream_reader, is_range: boolean): void {
    this.m_value = bitstream.read_signed_integer("value", 10);
    if (is_range) {
      this.m_name_string_index = -1;
      this.m_description_string_index = -1;
    } else {
      this.m_name_string_index = bitstream.read_integer(
        "name-string-index",
        k_string_index_bits
      );
      this.m_description_string_index = bitstream.read_integer(
        "description-string-index",
        k_string_index_bits
      );
    }
  }
  encode(bitstream: c_bitstream_writer, is_range: boolean): void {
    bitstream.write_signed_integer(this.m_value, 10);
    if (!is_range) {
      bitstream.write_integer(this.m_name_string_index!, k_string_index_bits);
      bitstream.write_integer(
        this.m_description_string_index!,
        k_string_index_bits
      );
    }
  }
}

/**
 * Halo 4 `s_user_defined_option` (IDA encode @ 0x82d7a518 / decode @ 0x82d7b3b8).
 * Current value / value-index is written by the custom variant after this encode.
 */
export class s_user_defined_option {
  @AutoMap(() => Number)
  m_name_string_index = 0;
  @AutoMap(() => Number)
  m_description_string_index = 0;
  @AutoMap(() => Boolean)
  m_is_ranged = false;
  @AutoMap(() => Number)
  m_default_value = 0;
  @AutoMap(() => Number)
  m_default_value_index = 0;
  /** Ranged: [min, max]. Enumerated: selectable values. */
  @AutoMap(() => [s_user_defined_option_value])
  m_values: s_user_defined_option_value[] = [];
  /** Filled by custom variant after option encode. */
  @AutoMap(() => Number)
  m_current_value?: number;
  @AutoMap(() => Number)
  m_current_value_index?: number;

  decode(bitstream: c_bitstream_reader): void {
    this.m_name_string_index = bitstream.read_integer(
      "name-string-index",
      k_string_index_bits
    );
    this.m_description_string_index = bitstream.read_integer(
      "description-string-index",
      k_string_index_bits
    );
    this.m_is_ranged = bitstream.read_bool("is-ranged-option");
    let value_count: number;
    if (this.m_is_ranged) {
      this.m_default_value = bitstream.read_signed_integer("default-value", 10);
      value_count = 2;
    } else {
      this.m_default_value_index = bitstream.read_integer(
        "default-value-index",
        4
      );
      value_count = bitstream.read_integer("value-count", 5);
    }
    this.m_values = [];
    for (let i = 0; i < value_count; i++) {
      const value = new s_user_defined_option_value();
      value.decode(bitstream, this.m_is_ranged);
      this.m_values.push(value);
    }
  }

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_name_string_index, k_string_index_bits);
    bitstream.write_integer(
      this.m_description_string_index,
      k_string_index_bits
    );
    bitstream.write_bool(this.m_is_ranged);
    if (this.m_is_ranged) {
      bitstream.write_signed_integer(this.m_default_value, 10);
    } else {
      bitstream.write_integer(this.m_default_value_index, 4);
      bitstream.write_integer(this.m_values.length, 5);
    }
    for (const value of this.m_values) {
      value.encode(bitstream, this.m_is_ranged);
    }
  }
}
