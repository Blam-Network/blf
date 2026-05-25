import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../../bitstream";

export class c_megalo_game_statistic {
  m_name_string_index = 0;
  m_format = 0;
  m_sort_order = 0;
  m_growuping = 0;

  decode(bitstream: c_bitstream_reader): void {
    this.m_name_string_index = bitstream.read_integer("name-string-index", 7);
    this.m_format = bitstream.read_integer("format", 2);
    this.m_sort_order = bitstream.read_integer("sort-order", 2);
    this.m_growuping = bitstream.read_integer("groupingt", 1);
  }

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_name_string_index, 7);
    bitstream.write_integer(this.m_format, 2);
    bitstream.write_integer(this.m_sort_order, 2);
    bitstream.write_integer(this.m_growuping, 1);
  }
}
