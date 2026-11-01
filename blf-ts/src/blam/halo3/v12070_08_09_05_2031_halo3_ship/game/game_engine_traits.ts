import { c } from "@craftycodie/cstruct";
import { c_player_traits } from "./game_engine_player_traits";

export const c_game_engine_miscellaneous_options = c.struct({
  m_flags: c.u8(),
  m_round_time_limit_minutes: c.u8(),
  m_round_limit: c.u8(),
  m_early_victory_win_count: c.u8(),
});
export type c_game_engine_miscellaneous_options = c.infer<
  typeof c_game_engine_miscellaneous_options
>;

export const c_game_engine_respawn_options = c.struct({
  m_flags: c.u8(),
  m_lives_per_round: c.u8(),
  m_team_lives_per_round: c.u8(),
  m_respawn_time_seconds: c.u8(),
  m_suicide_penalty_seconds: c.u8(),
  m_betrayal_penalty_seconds: c.u8(),
  m_respawn_growth_seconds: c.u8(),
  m_respawn_player_traits_duration_seconds: c.u8(),
  m_respawn_player_traits: c_player_traits,
});
export type c_game_engine_respawn_options = c.infer<
  typeof c_game_engine_respawn_options
>;
