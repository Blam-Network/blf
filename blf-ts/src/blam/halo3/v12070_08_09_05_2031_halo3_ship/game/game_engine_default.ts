import { c } from "@craftycodie/cstruct";
import { s_content_item_metadata } from "../saved_games/saved_game_files";
import { c_player_traits } from "./game_engine_player_traits";
import {
  c_game_engine_miscellaneous_options,
  c_game_engine_respawn_options,
} from "./game_engine_traits";

export const k_game_engine_type_count = 11;

export const c_game_engine_social_options = c.struct({
  m_flags: c.u16(),
  m_team_changing: c.u16(),
});
export type c_game_engine_social_options = c.infer<
  typeof c_game_engine_social_options
>;

export const c_game_engine_map_override_options = c.struct({
  m_flags: c.u32(),
  m_base_player_traits: c_player_traits,
  m_weapon_set_absolute_index: c.i16(),
  m_vehicle_set_absolute_index: c.i16(),
  m_red_powerup_traits: c_player_traits,
  m_blue_powerup_traits: c_player_traits,
  m_yellow_powerup_traits: c_player_traits,
  m_red_powerup_duration_seconds: c.u8(),
  m_blue_powerup_duration_seconds: c.u8(),
  m_yellow_powerup_duration_seconds: c.u8().padAfter(1),
});
export type c_game_engine_map_override_options = c.infer<
  typeof c_game_engine_map_override_options
>;

/** Unpacked (in-memory) layout, 0x1B2 bytes. */
export const c_game_engine_base_variant = c.struct({
  m_checksum: c.u32(),
  m_metadata: s_content_item_metadata.padBefore(4),
  m_miscellaneous_options: c_game_engine_miscellaneous_options,
  m_respawn_options: c_game_engine_respawn_options,
  m_social_options: c_game_engine_social_options,
  m_map_override_options: c_game_engine_map_override_options,
  m_flags: c.u16().padBefore(2),
  m_team_scoring_method: c.u16().padAfter(2),
});
export type c_game_engine_base_variant = c.infer<
  typeof c_game_engine_base_variant
>;
