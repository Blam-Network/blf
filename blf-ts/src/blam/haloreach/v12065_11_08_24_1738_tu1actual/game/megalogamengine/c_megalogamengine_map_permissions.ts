import { type c_bitstream_reader, c_bitstream_writer } from "../../../../../bitstream";

export class c_megalogamengine_map_permissions {
  m_except_map_ids: number[] = [];
  m_allow_by_default = false;

  decode(bitstream: c_bitstream_reader): void {
    const map_id_count = bitstream.read_integer("exception-count", 6);
    for (let i = 0; i < map_id_count; i++) {
      this.m_except_map_ids.push(bitstream.read_integer("map-id", 16));
    }
    this.m_allow_by_default = bitstream.read_bool("default-permission");
  }

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_except_map_ids.length, 6);
    for (const map_id of this.m_except_map_ids) {
      bitstream.write_integer(map_id, 16);
    }
    bitstream.write_bool(this.m_allow_by_default);
  }
}
