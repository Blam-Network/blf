import { c } from "@craftycodie/cstruct";
import { c_player_traits } from "./game_engine_player_traits";

export const c_game_engine_assault_variant = c.struct({
  m_variant_flags: c.u16(),
  m_respawn: c.u16(),
  m_game_type: c.u16(),
  m_enemy_bomb_waypoint: c.u16(),
  m_score_to_win: c.u16(),
  m_sudden_death_time: c.i16(),
  m_bomb_reset_time: c.u16(),
  m_bomb_arming_time: c.u16(),
  m_bomb_disarming_time: c.u16(),
  m_bomb_fuse_time: c.u16(),
  m_carrier_traits: c_player_traits,
  m_arming_traits: c_player_traits.padAfter(4),
});
export type c_game_engine_assault_variant = c.infer<
  typeof c_game_engine_assault_variant
>;
