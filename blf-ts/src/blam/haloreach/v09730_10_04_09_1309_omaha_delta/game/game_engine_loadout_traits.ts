import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../bitstream";
import { AutoMap } from "../../../../helpers/automap";
import { c_object_type_reference } from "../../v12065_11_08_24_1738_tu1actual/game/megalogamengine/megalogamengine_references";

export class s_loadout_unknown_struct {
  @AutoMap(() => [c_object_type_reference])
  m_object_types: c_object_type_reference[] = Array.from(
    { length: 4 },
    () => new c_object_type_reference()
  );
  @AutoMap(() => [Number])
  m_slot_numbers: number[] = [0, 0, 0, 0];

  decode(bitstream: c_bitstream_reader): void {
    for (let i = 0; i < 4; i++) {
      this.m_object_types[i].decode(bitstream);
    }
    for (let i = 0; i < 4; i++) {
      this.m_slot_numbers[i] = bitstream.read_integer("number-slot", 8);
    }
  }

  encode(bitstream: c_bitstream_writer): void {
    for (const objectType of this.m_object_types) {
      objectType.encode(bitstream);
    }
    for (const slotNumber of this.m_slot_numbers) {
      bitstream.write_integer(slotNumber, 8);
    }
  }
}

export class s_loadout_palette_unknown_struct {
  @AutoMap(() => [Number])
  m_loadouts: number[] = [];

  decode(bitstream: c_bitstream_reader): void {
    const entryCount = bitstream.read_integer("entries-count", 3);
    for (let i = 0; i < entryCount; i++) {
      this.m_loadouts.push(
        bitstream.read_integer("loadout-reference-index", 8)
      );
    }
  }

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_loadouts.length, 3);
    for (const loadoutReferenceIndex of this.m_loadouts) {
      bitstream.write_integer(loadoutReferenceIndex, 8);
    }
  }
}
