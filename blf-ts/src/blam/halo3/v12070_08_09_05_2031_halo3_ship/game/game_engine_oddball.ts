import { c } from "@craftycodie/cstruct";
import { c_player_traits } from "./game_engine_player_traits";

export const c_game_engine_oddball_variant = c.struct({
  m_variant_flags: c.u32(),
  m_score_to_win: c.i16(),
  m_carrying_points: c.i16(),
  m_kill_points: c.i8(),
  m_ball_kill_points: c.i8(),
  m_carrier_kill_points: c.i8(),
  m_ball_count: c.u8(),
  m_ball_spawn_delay: c.u16(),
  m_ball_inactive_respawn_delay: c.u16(),
  m_carrier_traits: c_player_traits.padAfter(2),
});
export type c_game_engine_oddball_variant = c.infer<
  typeof c_game_engine_oddball_variant
>;
