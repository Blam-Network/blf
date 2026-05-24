import {
  type c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../../bitstream";

export class s_user_defined_option_value {
  m_value = 0;
  m_name_string_index?: number;
  m_description_string_index?: number;

  decode(bitstream: c_bitstream_reader, is_range: boolean): void {
    this.m_value = bitstream.read_signed_integer("value", 10);
    if (!is_range) {
      this.m_name_string_index = bitstream.read_integer("name-string-index", 7);
      this.m_description_string_index = bitstream.read_integer(
        "description-string-index",
        7,
      );
    } else {
      this.m_name_string_index = -1;
      this.m_description_string_index = -1;
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
      7,
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
        10,
      );
    } else {
      this.m_default_value_index = bitstream.read_integer(
        "default-value-index",
        3,
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
        3,
      );
    }
  }

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_name_string_index, 7);
    bitstream.write_integer(this.m_description_string_index, 7);

    if (this.m_range_default_value !== undefined) {
      bitstream.write_bool(true);
      this.m_range_default_value.encode(bitstream, true);
      this.m_range_min_value!.encode(bitstream, true);
      this.m_range_max_value!.encode(bitstream, true);
      bitstream.write_signed_integer(this.m_range_current_value!, 10);
    } else {
      bitstream.write_bool(false);
      bitstream.write_integer(this.m_default_value_index!, 3);
      bitstream.write_integer(this.m_values!.length, 4);
      for (const value of this.m_values!) {
        value.encode(bitstream, false);
      }
      bitstream.write_integer(this.m_current_value_index!, 3);
    }
  }
}
