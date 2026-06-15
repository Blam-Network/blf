import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../../bitstream";
import { AutoMap } from "../../../../../helpers/automap";
import { c_object_type_reference } from "./megalogamengine_references";

export enum e_object_team_filter {
  none = -1,
  team_1 = 0,
  team_2 = 1,
  team_3 = 2,
  team_4 = 3,
  team_5 = 4,
  team_6 = 5,
  team_7 = 6,
  team_8 = 7,
  neutral = 8,
}

export class c_object_filter {
  @AutoMap(() => Number)
  m_label_string_index = 0;
  @AutoMap(() => Number)
  m_valid_parameters = 0;
  @AutoMap(() => c_object_type_reference)
  m_object_type?: c_object_type_reference;
  @AutoMap(() => e_object_team_filter)
  m_team?: e_object_team_filter;
  @AutoMap(() => Number)
  m_user_data?: number;
  @AutoMap(() => Number)
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
      this.m_team = bitstream.read_enum("team", 4, e_object_team_filter);
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
      bitstream.write_enum(this.m_team!, 4, e_object_team_filter);
    }
    if ((this.m_valid_parameters & 4) !== 0) {
      bitstream.write_signed_integer(this.m_user_data!, 16);
    }
    bitstream.write_integer(this.m_min, 7);
  }
}
