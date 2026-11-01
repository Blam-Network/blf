import { c } from "@craftycodie/cstruct";
import { c_player_traits } from "./game_engine_player_traits";

export const c_game_engine_territories_variant = c.struct({
  m_variant_flags: c.u16(),
  m_respawn_on_capture: c.u16(),
  m_capture_time: c.u16(),
  m_sudden_death_time: c.i16(),
  m_defender_traits: c_player_traits,
  m_attacker_traits: c_player_traits,
});
export type c_game_engine_territories_variant = c.infer<
  typeof c_game_engine_territories_variant
>;
