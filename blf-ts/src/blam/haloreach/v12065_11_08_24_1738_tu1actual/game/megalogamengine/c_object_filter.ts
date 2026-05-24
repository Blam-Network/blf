import { type c_bitstream_reader, c_bitstream_writer } from "../../../../../bitstream";
import { c_object_type_reference } from "./megalogamengine_references";

export class c_object_filter {
  m_label_string_index = 0;
  m_valid_parameters = 0;
  m_object_type?: c_object_type_reference;
  m_team?: number;
  m_user_data?: number;
  m_min = 0;

  decode(bitstream: c_bitstream_reader): void {
    this.m_label_string_index = bitstream.read_integer("label-string-index", 7);
    this.m_valid_parameters = bitstream.read_integer("valid-parameters", 3);
    if ((this.m_valid_parameters & 1) !== 0) {
      const object_type = new c_object_type_reference();
      object_type.decode(bitstream);
      this.m_object_type = object_type;
    }
    if ((this.m_valid_parameters & 2) !== 0) {
      this.m_team = bitstream.read_integer("team", 4);
    }
    if ((this.m_valid_parameters & 4) !== 0) {
      this.m_user_data = bitstream.read_signed_integer("user-data", 16);
    }
    this.m_min = bitstream.read_integer("min", 7);
  }

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_label_string_index, 7);
    bitstream.write_integer(this.m_valid_parameters, 3);
    if ((this.m_valid_parameters & 1) !== 0) {
      this.m_object_type!.encode(bitstream);
    }
    if ((this.m_valid_parameters & 2) !== 0) {
      bitstream.write_integer(this.m_team!, 4);
    }
    if ((this.m_valid_parameters & 4) !== 0) {
      bitstream.write_signed_integer(this.m_user_data!, 16);
    }
    bitstream.write_integer(this.m_min, 7);
  }
}
