import { c } from "@craftycodie/cstruct";
import { blf, CStructBLFChunk } from "../../../blf_chunk";
import { optional_i16, optional_i32 } from "./optional_int";

/** Challenge category in `dcha` / `s_challenge_state`. */
export enum e_challenge_category {
  bounty = 0,
  weekly = 1,
  campaign = 2,
  firefight = 3,
  matchmaking = 4,
}

export const e_challenge_skull_flags_names = [
  "iron",
  "black_eye",
  "tough_luck",
  "catch",
  "fog",
  "famine",
  "thunderstorm",
  "tilt",
  "mythic",
  "assasin",
  "blind",
  "superman",
  "grunt_birthday_party",
  "iwhbyd",
] as const;

export type e_challenge_skull_flags = c.Bitfield<
  typeof e_challenge_skull_flags_names
>;

const skull_flags_bitfield = c.bitfield("u32", e_challenge_skull_flags_names);

/** Optional skull override: `-1` on the wire means no override. */
class OptionalSkullFlags extends c.AdvancedType<e_challenge_skull_flags | null> {
  readonly byteSize = 4;

  read(
    bytes: Uint8Array,
    offset: number,
    endian: c.Endian,
    label: string
  ): e_challenge_skull_flags | null {
    if (offset + 4 > bytes.length) {
      throw new c.CStructError(
        `Cannot read ${label}: need 4 bytes at offset ${offset}`
      );
    }
    const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
    const raw = view.getInt32(offset, endian === "little");
    if (raw === -1) {
      return null;
    }
    return skull_flags_bitfield.read(
      bytes,
      offset,
      endian,
      label
    ) as e_challenge_skull_flags;
  }

  write(
    bytes: Uint8Array,
    offset: number,
    value: e_challenge_skull_flags | null,
    endian: c.Endian,
    label: string
  ): void {
    if (offset + 4 > bytes.length) {
      throw new c.CStructError(
        `Cannot write ${label}: need 4 bytes at offset ${offset}`
      );
    }
    if (value === null) {
      const view = new DataView(
        bytes.buffer,
        bytes.byteOffset,
        bytes.byteLength
      );
      view.setInt32(offset, -1, endian === "little");
      return;
    }
    skull_flags_bitfield.write(bytes, offset, value, endian, label);
  }
}

/** Single challenge slot in a daily/weekly set (`s_challenge_state`, 28 bytes). */
@c.struct()
export class s_challenge_state {
  @c.field(c.enum("u8", e_challenge_category))
  category: e_challenge_category = e_challenge_category.bounty;

  @c.field("u8")
  challenge = 0;

  /** Cookie reward override; `null` / omitted writes `-1`. */
  @c.field(optional_i16())
  cookie_reward: number | null = null;

  @c.field(optional_i32())
  required_progress: number | null = null;

  @c.field(optional_i32())
  minimum_score: number | null = null;

  @c.field(optional_i32())
  maximum_level_completion_time: number | null = null;

  @c.field(new OptionalSkullFlags())
  skull_flags: e_challenge_skull_flags | null = null;

  @c.field(optional_i32())
  maximum_death_count: number | null = null;

  @c.field(optional_i32())
  toast_progress_count: number | null = null;
}

/** Reach TU1 daily/weekly challenge state (`dcha` 3.1). */
@blf.chunk("dcha", 3.1)
@c.struct()
export class s_blf_chunk_challenge_state extends CStructBLFChunk {
  @c.field("u32")
  active_challenge_set_1 = 0;

  @c.field("u32")
  active_challenge_set_2 = 0;

  @c.field(c.Time64())
  chalenge_set_1_timestamp = new Date(0);

  @c.field(c.Time64())
  chalenge_set_2_timestamp = new Date(0);

  @c.field("u8")
  chalenge_set_1_count = 0;

  @c.field("u8")
  chalenge_set_2_count = 0;

  @c.field(s_challenge_state, { count: 10 })
  chalenge_set_1 = Array.from({ length: 10 }, () => new s_challenge_state());

  @c.field(s_challenge_state, { count: 10 })
  chalenge_set_2 = Array.from({ length: 10 }, () => new s_challenge_state());
}
