import { c } from "@craftycodie/cstruct";
import { c_player_traits } from "./game_engine_player_traits";

export const c_game_engine_slayer_variant = c.struct({
  m_score_to_win: c.i16(),
  m_kill_points: c.i16(),
  m_assist_points: c.i8(),
  m_death_points: c.i8(),
  m_suicide_points: c.i8(),
  m_betrayal_points: c.i8(),
  m_leader_killed_points: c.i8(),
  m_elimination_points: c.i8(),
  m_assassination_points: c.i8(),
  m_headshot_points: c.i8(),
  m_melee_points: c.i8(),
  m_sticky_points: c.i8(),
  m_splatter_points: c.i8(),
  m_killing_spree_points: c.i8(),
  m_leader_traits: c_player_traits,
});
export type c_game_engine_slayer_variant = c.infer<
  typeof c_game_engine_slayer_variant
>;
