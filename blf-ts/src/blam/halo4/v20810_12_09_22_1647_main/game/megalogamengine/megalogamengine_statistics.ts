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

/** Matches Halo 4 `e_megalo_game_statistic_sort` (`c_enum<...,char,-1,3>`). */
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
  @AutoMap(() => Boolean)
  m_hide_from_display = false;
  @AutoMap(() => Boolean)
  m_show_in_mini_scoreboard = false;
  decode(bitstream: c_bitstream_reader): void {
    // Halo 4 uses 8-bit name-string-index (Reach uses 7).
    this.m_name_string_index = bitstream.read_integer("name-string-index", 8);
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
    this.m_hide_from_display = bitstream.read_bool("hide-from-display");
    this.m_show_in_mini_scoreboard = bitstream.read_bool(
      "show-in-mini-scoreboard"
    );
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_name_string_index, 8);
    bitstream.write_enum(this.m_format, 2, e_megalo_game_statistic_format);
    bitstream.write_enum(
      this.m_sort_order,
      2,
      e_megalo_game_statistic_sort_order
    );
    bitstream.write_enum(this.m_grouping, 1, e_megalo_game_statistic_grouping);
    bitstream.write_bool(this.m_hide_from_display);
    bitstream.write_bool(this.m_show_in_mini_scoreboard);
  }
}
