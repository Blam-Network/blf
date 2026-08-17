import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../bitstream";
import { AutoMap } from "../../../../helpers/automap";
import { c_player_traits } from "./game_engine_player_traits";

/** `TraitSetTypes::Value` (0..1), 1 bit. */
export enum e_trait_set_type {
  player_traits = 0,
  player_traits_dynamic = 1,
}

/** Halo 4 MaxStrings=148 → 8-bit string indices; trailing hidden + traits-type. */
export class s_player_trait_option {
  @AutoMap(() => Number)
  m_name_string_index = 0;
  @AutoMap(() => Number)
  m_description_string_index = 0;
  @AutoMap(() => c_player_traits)
  m_player_traits = new c_player_traits();
  @AutoMap(() => Boolean)
  m_hidden = false;
  @AutoMap(() => Number)
  m_traits_type: e_trait_set_type = e_trait_set_type.player_traits;
  decode(bitstream: c_bitstream_reader): void {
    this.m_name_string_index = bitstream.read_integer("name-string-index", 8);
    this.m_description_string_index = bitstream.read_integer(
      "description-string-index",
      8
    );
    this.m_player_traits.decode(bitstream);
    this.m_hidden = bitstream.read_bool("hidden");
    this.m_traits_type = bitstream.read_enum(
      "traits-type",
      1,
      e_trait_set_type
    );
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_name_string_index, 8);
    bitstream.write_integer(this.m_description_string_index, 8);
    this.m_player_traits.encode(bitstream);
    bitstream.write_bool(this.m_hidden);
    bitstream.write_enum(this.m_traits_type, 1, e_trait_set_type);
  }
}
