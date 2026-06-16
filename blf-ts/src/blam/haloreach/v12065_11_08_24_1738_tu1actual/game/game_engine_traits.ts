import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../bitstream";
import { AutoMap } from "../../../../helpers/automap";
import { c_player_traits } from "./game_engine_player_traits";
export class s_player_trait_option {
  @AutoMap(() => Number)
  m_name_string_index = 0;
  @AutoMap(() => Number)
  m_description_string_index = 0;
  @AutoMap(() => c_player_traits)
  m_player_traits = new c_player_traits();
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
