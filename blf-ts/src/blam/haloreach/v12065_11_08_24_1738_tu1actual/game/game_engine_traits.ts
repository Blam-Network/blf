import { c } from "@craftycodie/cstruct";
import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../bitstream";
import { c_player_traits } from "./game_engine_player_traits";
@c.class()
export class s_player_trait_option {
  @c.field(c_player_traits)
  m_player_traits = new c_player_traits();
  @c.field(c.u8())
  m_name_string_index = 0;
  @c.field(c.u8(), { padAfter: 2 })
  m_description_string_index = 0;
  decode(bitstream: c_bitstream_reader): void {
    this.m_name_string_index = bitstream.read_integer("name-string-index", 7);
    this.m_description_string_index = bitstream.read_integer(
      "description-string-index",
      7
    );
    this.m_player_traits.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_name_string_index, 7);
    bitstream.write_integer(this.m_description_string_index, 7);
    this.m_player_traits.encode(bitstream);
  }
}
