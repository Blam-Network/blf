import { c } from "@craftycodie/cstruct";
import { c_player_traits } from "./game_engine_player_traits";

export const c_game_engine_ctf_variant = c.struct({
  m_variant_flags: c.u8(),
  m_home_flag_waypoint: c.u8(),
  m_game_type: c.u8(),
  m_respawn: c.u8(),
  m_touch_return_timeout: c.i16(),
  m_sudden_death_time: c.i16(),
  m_score_to_win: c.u16(),
  m_flag_reset_time: c.u16(),
  m_carrier_traits: c_player_traits,
});
export type c_game_engine_ctf_variant = c.infer<
  typeof c_game_engine_ctf_variant
>;
