import { c } from "@craftycodie/cstruct";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

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

const skull_flags_bitfield = c.Bitfield(e_challenge_skull_flags_names, c.u32());

export type e_challenge_skull_flags = c.infer<typeof skull_flags_bitfield>;

/**
 * Single challenge slot in a daily/weekly set (`s_challenge_state`, 28 bytes).
 * Overrides are `null` when unset, stored as `-1` (Blam's `NONE`).
 */
export const s_challenge_state = c.struct({
  category: c.enum(e_challenge_category, c.u8()),
  challenge: c.u8(),
  cookie_reward: c.i16().nullable(-1).default(null),
  required_progress: c.i32().nullable(-1).default(null),
  minimum_score: c.i32().nullable(-1).default(null),
  maximum_level_completion_time: c.i32().nullable(-1).default(null),
  skull_flags: skull_flags_bitfield.nullable(0xff_ff_ff_ff).default(null),
  maximum_death_count: c.i32().nullable(-1).default(null),
  toast_progress_count: c.i32().nullable(-1).default(null),
});
export type s_challenge_state = c.infer<typeof s_challenge_state>;

const challenge_set = s_challenge_state.array(10);

/** Reach TU1 daily/weekly challenge state (`dcha` 3.1). */
@blf.chunk("dcha", 3.1)
export class s_blf_chunk_challenge_state extends CStructBLFChunk(
  c.struct({
    active_challenge_set_1: c.u32(),
    active_challenge_set_2: c.u32(),
    chalenge_set_1_timestamp: c.Time64(),
    chalenge_set_2_timestamp: c.Time64(),
    chalenge_set_1_count: c.u8(),
    chalenge_set_2_count: c.u8(),
    chalenge_set_1: challenge_set,
    chalenge_set_2: challenge_set,
  })
) {}
