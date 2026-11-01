import { c } from "@craftycodie/cstruct";
import { c_player_traits } from "./game_engine_player_traits";

export const c_game_engine_juggernaut_variant = c.struct({
  m_score_to_win_round: c.u16().padAfter(2),
  m_initial_juggernaut: c.u8(),
  m_next_juggernaut: c.u8(),
  m_variant_flags: c.u8(),
  m_zone_movement: c.u8(),
  m_zone_order: c.u8(),
  m_kill_points: c.i8(),
  m_juggernaut_kill_points: c.i8(),
  m_kill_as_juggernaut_points: c.i8(),
  m_destination_arrival_points: c.i8(),
  m_suicide_points: c.i8(),
  m_betrayal_points: c.i8(),
  m_juggernaut_delay: c.u8(),
  m_juggernaut_traits: c_player_traits.padAfter(4),
});
export type c_game_engine_juggernaut_variant = c.infer<
  typeof c_game_engine_juggernaut_variant
>;
