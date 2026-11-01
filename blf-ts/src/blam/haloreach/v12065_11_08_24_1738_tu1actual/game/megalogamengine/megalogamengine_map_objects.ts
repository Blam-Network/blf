import { c } from "@craftycodie/cstruct";
import type {
  BitfieldOf,
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../../bitstream";
import { bitfieldFromRaw, bitfieldToRaw } from "../../../../../bitstream";
import { c_object_type_reference } from "./megalogamengine_references";

const k_object_filter_valid_parameters = [
  "object_type",
  "team",
  "user_data",
] as const;

export type e_object_filter_valid_parameters = BitfieldOf<
  typeof k_object_filter_valid_parameters
>;

export enum e_object_team_filter {
  none = -1,
  defenders = 0,
  attackers = 1,
  third_party = 2,
  fourth_party = 3,
  fifth_party = 4,
  sixth_party = 5,
  seventh_party = 6,
  eighth_party = 7,
  neutral = 8,
  each = 9,
}

@c.class()
export class c_object_filter {
  @c.field(c.i32())
  m_label_string_index = -1;
  @c.field(c_object_type_reference)
  m_object_type = Object.assign(new c_object_type_reference(), {
    m_object_type_index: -1,
  });
  @c.field(c.i16())
  m_user_data = 0;
  @c.field(c.i8())
  m_team: e_object_team_filter = e_object_team_filter.none;
  @c.field(
    c.u8().transform(
      (raw: number) => bitfieldFromRaw(raw, k_object_filter_valid_parameters),
      (flags: e_object_filter_valid_parameters) =>
        bitfieldToRaw(flags, k_object_filter_valid_parameters)
    )
  )
  m_valid_parameters: e_object_filter_valid_parameters = {
    object_type: false,
    team: false,
    user_data: false,
  };
  @c.field(c.i16(), { padAfter: 2 })
  m_min = 0;
  decode(bitstream: c_bitstream_reader): void {
    this.m_label_string_index = bitstream.read_index(
      "label-string-index",
      112,
      7
    );
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
      this.m_team = bitstream.read_enum("team", 4, e_object_team_filter);
    }
    if (this.m_valid_parameters.user_data) {
      this.m_user_data = bitstream.read_signed_integer("user-data", 16);
    }
    this.m_min = bitstream.read_integer("min", 7);
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_index(this.m_label_string_index, 112, 7);
    bitstream.write_bitfield(
      this.m_valid_parameters,
      3,
      k_object_filter_valid_parameters
    );
    if (this.m_valid_parameters.object_type) {
      this.m_object_type!.encode(bitstream);
    }
    if (this.m_valid_parameters.team) {
      bitstream.write_enum(this.m_team!, 4, e_object_team_filter);
    }
    if (this.m_valid_parameters.user_data) {
      bitstream.write_signed_integer(this.m_user_data!, 16);
    }
    bitstream.write_integer(this.m_min, 7);
  }
}
