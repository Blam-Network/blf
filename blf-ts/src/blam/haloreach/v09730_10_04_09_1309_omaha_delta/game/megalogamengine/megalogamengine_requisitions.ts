import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../../bitstream";
import { AutoMap } from "../../../../../helpers/automap";
import { c_object_type_reference } from "../../../v12065_11_08_24_1738_tu1actual/game/megalogamengine/megalogamengine_references";

export class s_requisition {
  @AutoMap(() => c_object_type_reference)
  m_object_type = new c_object_type_reference();
  @AutoMap(() => Number)
  m_unknown_1 = 0;
  @AutoMap(() => Boolean)
  m_unknown_2 = false;
  @AutoMap(() => Boolean)
  m_unknown_3 = false;
  @AutoMap(() => Number)
  m_unknown_4 = 0;

  decode(bitstream: c_bitstream_reader): void {
    this.m_object_type.decode(bitstream);
    this.m_unknown_1 = bitstream.read_index("unknown-1", 65535, 11);
    this.m_unknown_2 = bitstream.read_bool("unknown-2");
    if (this.m_unknown_2) {
      this.m_unknown_3 = bitstream.read_bool("unknown-3");
      if (this.m_unknown_3) {
        this.m_unknown_4 = bitstream.read_integer("unknown-4", 15);
      }
    }
  }

  encode(bitstream: c_bitstream_writer): void {
    this.m_object_type.encode(bitstream);
    bitstream.write_index(this.m_unknown_1, 65535, 11);
    bitstream.write_bool(this.m_unknown_2);
    if (this.m_unknown_2) {
      bitstream.write_bool(this.m_unknown_3);
      if (this.m_unknown_3) {
        bitstream.write_integer(this.m_unknown_4, 15);
      }
    }
  }
}

export class s_requisition_palette {
  @AutoMap(() => Number)
  m_baseline = 0;
  @AutoMap(() => [s_requisition])
  entries: s_requisition[] = [];

  decode(bitstream: c_bitstream_reader): void {
    this.m_baseline = bitstream.read_integer("baseline", 4);
    const entryCount = bitstream.read_integer("entries", 6);
    for (let i = 0; i < entryCount; i++) {
      const entry = new s_requisition();
      entry.decode(bitstream);
      this.entries.push(entry);
    }
  }

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_baseline, 4);
    bitstream.write_integer(this.entries.length, 6);
    for (const entry of this.entries) {
      entry.encode(bitstream);
    }
  }
}
