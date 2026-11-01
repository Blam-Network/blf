import { c } from "@craftycodie/cstruct";
import { c_player_traits } from "./game_engine_player_traits";

export const c_game_engine_sandbox_variant = c.struct({
  m_variant_flags: c.u8(),
  m_edit_mode: c.u8(),
  m_respawn_time: c.u16(),
  m_player_traits: c_player_traits,
});
export type c_game_engine_sandbox_variant = c.infer<
  typeof c_game_engine_sandbox_variant
>;
