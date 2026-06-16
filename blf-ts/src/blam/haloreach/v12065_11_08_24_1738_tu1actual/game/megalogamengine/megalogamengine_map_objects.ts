import type {
  BitfieldOf,
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../../bitstream";
import { AutoMap } from "../../../../../helpers/automap";
import { c_object_type_reference } from "./megalogamengine_references";

const k_object_filter_valid_parameters = [
  "object_type",
  "team",
  "user_data",
] as const;

export type e_object_filter_valid_parameters = BitfieldOf<
  typeof k_object_filter_valid_parameters
>;

export class c_object_filter {
  @AutoMap(() => Number)
  m_label_string_index = 0;
  @AutoMap(() => Object)
  m_valid_parameters: e_object_filter_valid_parameters = {
    object_type: false,
    team: false,
    user_data: false,
  };
  @AutoMap(() => c_object_type_reference)
  m_object_type?: c_object_type_reference;
  @AutoMap(() => Number)
  m_team?: number;
  @AutoMap(() => Number)
  m_user_data?: number;
  @AutoMap(() => Number)
  m_min = 0;
  decode(bitstream: c_bitstream_reader): void {
    this.m_label_string_index = bitstream.read_integer("label-string-index", 7);
    this.m_valid_parameters = bitstream.read_bitfield(
      "valid-parameters",
      3,
      k_object_filter_valid_parameters
    );
    if (this.m_valid_parameters.object_type) {
      const object_type = new c_object_type_reference();
      object_type.decode(bitstream);
      this.m_object_type = object_type;
    }
    if (this.m_valid_parameters.team) {
      this.m_team = bitstream.read_integer("team", 4);
    }
    if (this.m_valid_parameters.user_data) {
      this.m_user_data = bitstream.read_signed_integer("user-data", 16);
    }
    this.m_min = bitstream.read_integer("min", 7);
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_label_string_index, 7);
    bitstream.write_bitfield(
      this.m_valid_parameters,
      3,
      k_object_filter_valid_parameters
    );
    if (this.m_valid_parameters.object_type) {
      this.m_object_type!.encode(bitstream);
    }
    if (this.m_valid_parameters.team) {
      bitstream.write_integer(this.m_team!, 4);
    }
    if (this.m_valid_parameters.user_data) {
      bitstream.write_signed_integer(this.m_user_data!, 16);
    }
    bitstream.write_integer(this.m_min, 7);
  }
}
