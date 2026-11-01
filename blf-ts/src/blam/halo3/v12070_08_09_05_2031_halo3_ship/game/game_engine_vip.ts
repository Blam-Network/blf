import { c } from "@craftycodie/cstruct";
import { c_player_traits } from "./game_engine_player_traits";

export const c_game_engine_vip_variant = c.struct({
  m_score_to_win_round: c.u16(),
  m_variant_flags: c.u16(),
  m_kill_points: c.i8(),
  m_takedown_points: c.i8(),
  m_kill_as_vip_points: c.i8(),
  m_vip_death_points: c.i8(),
  m_destination_arrival_points: c.i8(),
  m_suicide_points: c.i8(),
  m_betrayal_points: c.i8(),
  m_vip_suicide_points: c.i8(),
  m_vip_selection: c.u8(),
  m_zone_movement: c.u8(),
  m_zone_order: c.u8().padAfter(1),
  m_influence_radius: c.u16(),
  m_vip_team_traits: c_player_traits,
  m_vip_influence_traits: c_player_traits,
  m_vip_traits: c_player_traits.padAfter(2),
});
export type c_game_engine_vip_variant = c.infer<
  typeof c_game_engine_vip_variant
>;
