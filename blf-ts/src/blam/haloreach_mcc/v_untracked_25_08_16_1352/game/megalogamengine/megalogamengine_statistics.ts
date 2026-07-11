import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../../bitstream";
import { AutoMap } from "../../../../../helpers/automap";

/** Matches `e_megalo_game_statistic_format` in blf_lib `megalogamengine_statistics.rs`. */
export enum e_megalo_game_statistic_format {
  number = 0,
  number_with_sign = 1,
  percentage = 2,
  time = 3,
}

/** Matches `e_megalo_game_statistic_sort_order` in blf_lib `megalogamengine_statistics.rs`. */
export enum e_megalo_game_statistic_sort_order {
  none = -1,
  ascending = 0,
  descending = 1,
}

/** Matches `e_megalo_game_statistic_grouping` in blf_lib `megalogamengine_statistics.rs`. */
export enum e_megalo_game_statistic_grouping {
  player = 0,
  team = 1,
}

export class c_megalo_game_statistic {
  @AutoMap(() => Number)
  m_name_string_index = 0;
  @AutoMap(() => e_megalo_game_statistic_format)
  m_format: e_megalo_game_statistic_format =
    e_megalo_game_statistic_format.number;
  @AutoMap(() => e_megalo_game_statistic_sort_order)
  m_sort_order: e_megalo_game_statistic_sort_order =
    e_megalo_game_statistic_sort_order.none;
  @AutoMap(() => e_megalo_game_statistic_grouping)
  m_grouping: e_megalo_game_statistic_grouping =
    e_megalo_game_statistic_grouping.player;
  decode(bitstream: c_bitstream_reader): void {
    this.m_name_string_index = bitstream.read_integer("name-string-index", 7);
    this.m_format = bitstream.read_enum(
      "format",
      2,
      e_megalo_game_statistic_format
    );
    this.m_sort_order = bitstream.read_enum(
      "sort-order",
      2,
      e_megalo_game_statistic_sort_order
    );
    this.m_grouping = bitstream.read_enum(
      "grouping",
      1,
      e_megalo_game_statistic_grouping
    );
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_name_string_index, 7);
    bitstream.write_enum(this.m_format, 2, e_megalo_game_statistic_format);
    bitstream.write_enum(
      this.m_sort_order,
      2,
      e_megalo_game_statistic_sort_order
    );
    bitstream.write_enum(this.m_grouping, 1, e_megalo_game_statistic_grouping);
  }
}
