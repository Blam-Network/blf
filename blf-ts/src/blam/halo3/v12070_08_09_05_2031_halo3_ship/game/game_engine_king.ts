import { c } from "@craftycodie/cstruct";
import { c_player_traits } from "./game_engine_player_traits";

export const c_game_engine_king_variant = c.struct({
  m_variant_flags: c.u32(),
  m_score_to_win: c.u16(),
  m_moving_hill: c.u8(),
  m_moving_hill_order: c.u8(),
  m_uncontested_hill_bonus: c.i8(),
  m_kill_points: c.i8(),
  m_inside_hill_points: c.i8(),
  m_outside_hill_points: c.i8(),
  m_inside_hill_traits: c_player_traits.padAfter(6),
});
export type c_game_engine_king_variant = c.infer<
  typeof c_game_engine_king_variant
>;
