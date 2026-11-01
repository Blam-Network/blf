import { CFieldType, type c } from "@craftycodie/cstruct";
import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../../bitstream";
export class s_user_defined_option_value {
  m_value = 0;
  m_name_string_index?: number;
  m_description_string_index?: number;
  decode(bitstream: c_bitstream_reader, is_range: boolean): void {
    this.m_value = bitstream.read_signed_integer("value", 10);
    if (is_range) {
      this.m_name_string_index = -1;
      this.m_description_string_index = -1;
    } else {
      this.m_name_string_index = bitstream.read_integer("name-string-index", 7);
      this.m_description_string_index = bitstream.read_integer(
        "description-string-index",
        7
      );
    }
  }
  encode(bitstream: c_bitstream_writer, is_range: boolean): void {
    bitstream.write_signed_integer(this.m_value, 10);
    if (!is_range) {
      bitstream.write_integer(this.m_name_string_index!, 7);
      bitstream.write_integer(this.m_description_string_index!, 7);
    }
  }
}
export class s_user_defined_option {
  m_name_string_index = 0;
  m_description_string_index = 0;
  m_range_default_value?: s_user_defined_option_value;
  m_range_min_value?: s_user_defined_option_value;
  m_range_max_value?: s_user_defined_option_value;
  m_range_current_value?: number;
  m_default_value_index?: number;
  m_values?: s_user_defined_option_value[];
  m_current_value_index?: number;
  decode(bitstream: c_bitstream_reader): void {
    this.m_name_string_index = bitstream.read_integer("name-string-index", 7);
    this.m_description_string_index = bitstream.read_integer(
      "description-string-index",
      7
    );
    if (bitstream.read_bool("is-ranged-option")) {
      const range_default = new s_user_defined_option_value();
      const range_min = new s_user_defined_option_value();
      const range_max = new s_user_defined_option_value();
      range_default.decode(bitstream, true);
      range_min.decode(bitstream, true);
      range_max.decode(bitstream, true);
      this.m_range_default_value = range_default;
      this.m_range_min_value = range_min;
      this.m_range_max_value = range_max;
      this.m_range_current_value = bitstream.read_signed_integer(
        "user-defined-option-value",
        10
      );
    } else {
      this.m_default_value_index = bitstream.read_integer(
        "default-value-index",
        3
      );
      const value_count = bitstream.read_integer("value-count", 4);
      const values: s_user_defined_option_value[] = [];
      for (let i = 0; i < value_count; i++) {
        const value = new s_user_defined_option_value();
        value.decode(bitstream, false);
        values.push(value);
      }
      this.m_values = values;
      this.m_current_value_index = bitstream.read_integer(
        "current-value-index",
        3
      );
    }
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_name_string_index, 7);
    bitstream.write_integer(this.m_description_string_index, 7);
    if (this.m_range_default_value === undefined) {
      bitstream.write_bool(false);
      bitstream.write_integer(this.m_default_value_index!, 3);
      bitstream.write_integer(this.m_values!.length, 4);
      for (const value of this.m_values!) {
        value.encode(bitstream, false);
      }
      bitstream.write_integer(this.m_current_value_index!, 3);
    } else {
      bitstream.write_bool(true);
      this.m_range_default_value.encode(bitstream, true);
      this.m_range_min_value!.encode(bitstream, true);
      this.m_range_max_value!.encode(bitstream, true);
      bitstream.write_signed_integer(this.m_range_current_value!, 10);
    }
  }
}

const k_user_defined_option_capacity = 16;
const k_user_defined_option_value_capacity = 8;
const k_user_defined_option_size =
  12 + k_user_defined_option_value_capacity * 4;

export class CUserDefinedOptionsField extends CFieldType<
  s_user_defined_option[]
> {
  getSize(): number {
    return (
      4 +
      k_user_defined_option_capacity * k_user_defined_option_size +
      k_user_defined_option_capacity * 2
    );
  }

  read(
    bytes: Uint8Array,
    offset: number,
    endian: c.Endian
  ): s_user_defined_option[] {
    const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
    const little = endian === "little";
    const count = Math.min(
      k_user_defined_option_capacity,
      Math.max(0, view.getInt32(offset, little))
    );
    const current_offset =
      offset + 4 + k_user_defined_option_capacity * k_user_defined_option_size;
    const options: s_user_defined_option[] = [];
    for (let i = 0; i < count; i++) {
      const at = offset + 4 + i * k_user_defined_option_size;
      const option = new s_user_defined_option();
      option.m_name_string_index = view.getUint8(at);
      option.m_description_string_index = view.getUint8(at + 1);
      const is_range = view.getUint8(at + 2) !== 0;
      const default_value = view.getInt16(at + 4, little);
      const value_count = Math.min(
        k_user_defined_option_value_capacity,
        Math.max(0, view.getInt32(at + 8, little))
      );
      const values: s_user_defined_option_value[] = [];
      for (let j = 0; j < value_count; j++) {
        const value_at = at + 12 + j * 4;
        const value = new s_user_defined_option_value();
        value.m_value = view.getInt16(value_at, little);
        if (!is_range) {
          value.m_name_string_index = view.getUint8(value_at + 2);
          value.m_description_string_index = view.getUint8(value_at + 3);
        }
        values.push(value);
      }
      const current = view.getInt16(current_offset + i * 2, little);
      if (is_range) {
        const range_default = new s_user_defined_option_value();
        range_default.m_value = default_value;
        option.m_range_default_value = range_default;
        option.m_range_min_value =
          values[0] ?? new s_user_defined_option_value();
        option.m_range_max_value =
          values[1] ?? new s_user_defined_option_value();
        option.m_range_current_value = current;
      } else {
        option.m_default_value_index = default_value;
        option.m_values = values;
        option.m_current_value_index = current;
      }
      options.push(option);
    }
    return options;
  }

  write(
    bytes: Uint8Array,
    offset: number,
    value: s_user_defined_option[],
    endian: c.Endian
  ): void {
    const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
    const little = endian === "little";
    const count = Math.min(k_user_defined_option_capacity, value.length);
    view.setInt32(offset, count, little);
    const current_offset =
      offset + 4 + k_user_defined_option_capacity * k_user_defined_option_size;
    for (let i = 0; i < count; i++) {
      const option = value[i]!;
      const at = offset + 4 + i * k_user_defined_option_size;
      const is_range = option.m_range_default_value !== undefined;
      view.setUint8(at, option.m_name_string_index);
      view.setUint8(at + 1, option.m_description_string_index);
      view.setUint8(at + 2, is_range ? 1 : 0);
      const values = is_range
        ? [option.m_range_min_value, option.m_range_max_value]
        : (option.m_values ?? []);
      view.setInt16(
        at + 4,
        is_range
          ? option.m_range_default_value!.m_value
          : (option.m_default_value_index ?? 0),
        little
      );
      const value_count = Math.min(
        k_user_defined_option_value_capacity,
        values.length
      );
      view.setInt32(at + 8, value_count, little);
      for (let j = 0; j < value_count; j++) {
        const entry = values[j];
        const value_at = at + 12 + j * 4;
        view.setInt16(value_at, entry?.m_value ?? 0, little);
        view.setUint8(
          value_at + 2,
          is_range ? 0xff : (entry?.m_name_string_index ?? 0)
        );
        view.setUint8(
          value_at + 3,
          is_range ? 0xff : (entry?.m_description_string_index ?? 0)
        );
      }
      view.setInt16(
        current_offset + i * 2,
        is_range
          ? (option.m_range_current_value ?? 0)
          : (option.m_current_value_index ?? 0),
        little
      );
    }
  }
}

export const user_defined_options_field = () => new CUserDefinedOptionsField();
