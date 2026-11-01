import { c } from "@craftycodie/cstruct";
import { c_map_variant } from "../saved_games/scenario_map_variant";
import { c_game_variant } from "./game_engine_variant";
import { s_player_configuration } from "./players";

export const k_maximum_players = 16;
export const k_maximum_machines = 16;
export const k_maximum_teams = 16;
export const k_maximum_campaign_players = 4;

export const s_campaign_armaments_weapon = c.struct({
  damage_reporting_type: c.u16(),
  rounds_inventory: c.u16(),
  rounds_loaded: c.u16(),
  battery: c.u16(),
});
export type s_campaign_armaments_weapon = c.infer<
  typeof s_campaign_armaments_weapon
>;

/** 0x1E bytes. */
export const s_campaign_armaments_player = c.struct({
  valid: c.bool().padAfter(1),
  primary_weapon: s_campaign_armaments_weapon,
  backpack_weapon: s_campaign_armaments_weapon,
  secondary_weapon: s_campaign_armaments_weapon,
  frag_grenade_count: c.u8(),
  plasma_grenade_count: c.u8(),
  spike_grenade_count: c.u8(),
  fire_grenade_count: c.u8(),
});
export type s_campaign_armaments_player = c.infer<
  typeof s_campaign_armaments_player
>;

/** 0x78 bytes. */
export const s_campaign_armaments = c.struct({
  player_armaments: s_campaign_armaments_player.array(
    k_maximum_campaign_players
  ),
});
export type s_campaign_armaments = c.infer<typeof s_campaign_armaments>;

export const s_machine_identifier = c.struct({
  identifier: c.u8().array(6),
});
export type s_machine_identifier = c.infer<typeof s_machine_identifier>;

/** 0x6C bytes. */
export const game_machine_options = c.struct({
  machine_valid_mask: c.u32(),
  machine_identifiers: s_machine_identifier.array(k_maximum_machines),
  local_machine_exists: c.bool(),
  local_machine_identifier: s_machine_identifier.padBefore(1),
});
export type game_machine_options = c.infer<typeof game_machine_options>;

/** 0x128 bytes. */
export const game_player_options = c.struct({
  valid: c.bool(),
  left_game: c.bool(),
  /** Sic (blf_lib's spelling of `user_index`). */
  user_inex: c.i16(),
  controller_index: c.i32(),
  machine_identifier: s_machine_identifier,
  player_identifier: c.u64(),
  configuration: s_player_configuration.padBefore(2),
});
export type game_player_options = c.infer<typeof game_player_options>;

/** 0x5C bytes. */
export const s_game_matchmaking_options = c.struct({
  hopper_identifier: c.i16(),
  xlast_index: c.i8(),
  hopper_ranked: c.bool(),
  team_game: c.bool(),
  hopper_name: c.wstring(32).padBefore(1),
  draw_probability: c.i32().padBefore(2),
  beta: c.f32(),
  tau: c.f32(),
  experience_base_increment: c.i32(),
  experience_penalty_decrement: c.i32(),
});
export type s_game_matchmaking_options = c.infer<
  typeof s_game_matchmaking_options
>;

/** 0xF810 bytes. */
export const game_options = c.struct({
  game_mode: c.i32(),
  game_simulation: c.u8(),
  game_network_type: c.u8(),
  game_tick_rate: c.u16(),
  game_instance: c.u64(),
  random_seed: c.u32(),
  language: c.i32(),
  determinism_version: c.i32(),
  campaign_id: c.i32(),
  map_id: c.i32(),
  scenario_path: c.string(260, "utf8"),
  initial_zone_set_index: c.i16(),
  load_level_only: c.bool(),
  dump_machine_index: c.u8(),
  dump_object_log: c.bool(),
  dump_random_seeds: c.bool(),
  playtest_mode: c.bool(),
  game_playback: c.i16().padBefore(1),
  record_saved_film: c.bool(),
  playback_start_tick: c.i32().padBefore(1),
  playback_length_in_ticks: c.i32(),
  campaign_difficulty: c.i16(),
  campaign_insertion_point: c.i16(),
  campaign_metagame_scoring: c.i16(),
  campaign_metagame_enabled: c.bool(),
  campaign_allow_persistent_storage: c.bool(),
  campaign_active_primary_skulls: c.i32(),
  campaign_active_secondary_skulls: c.i32(),
  campaign_armaments: s_campaign_armaments,
  matchmade_game: c.bool(),
  game_matchmaking_options: s_game_matchmaking_options.padBefore(7),
  multiplayer_variant: c_game_variant(),
  map_variant: c_map_variant.padBefore(4),
  machines: game_machine_options,
  players: game_player_options.array(k_maximum_players).padBefore(4),
});
export type game_options = c.infer<typeof game_options>;
