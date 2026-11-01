import { c } from "@craftycodie/cstruct";
import { c_player_traits } from "./game_engine_player_traits";

export const c_game_engine_infection_variant = c.struct({
  m_variant_flags: c.u8(),
  m_safe_havens: c.u8(),
  m_next_zombie: c.u8(),
  m_initial_zombie_count: c.u8(),
  m_safe_haven_movement_time: c.u16(),
  m_zombie_kill_points: c.i8(),
  m_infection_points: c.i8(),
  m_safe_haven_arrival_points: c.i8(),
  m_suicide_points: c.i8(),
  m_betrayal_points: c.i8(),
  m_last_man_bonus_points: c.i8(),
  m_zombie_traits: c_player_traits,
  m_first_zombie_traits: c_player_traits,
  m_safe_haven_defender_traits: c_player_traits,
  m_last_human_traits: c_player_traits.padAfter(4),
});
export type c_game_engine_infection_variant = c.infer<
  typeof c_game_engine_infection_variant
>;
