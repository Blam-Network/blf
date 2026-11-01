import { c } from "@craftycodie/cstruct";
import { shifted_i16 } from "../../../blam/halo3/v12070_08_09_05_2031_halo3_ship/cseries/cseries_types";
import { k_maximum_players } from "../../../blam/halo3/v12070_08_09_05_2031_halo3_ship/game/game_options";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

// Each 16-bit statistic is read as `raw >> 1`, as in blf_lib.

/** 0x66 bytes. Also the per-team statistics in `mps3`. */
export const s_game_results_statistics = c.struct({
  games_played: shifted_i16(),
  games_completed: shifted_i16(),
  games_won: shifted_i16(),
  games_tied: shifted_i16(),
  rounds_completed: shifted_i16(),
  rounds_won: shifted_i16(),
  in_round_score: shifted_i16(),
  in_game_total_score: shifted_i16(),
  kills: shifted_i16(),
  assists: shifted_i16(),
  deaths: shifted_i16(),
  betrayals: shifted_i16(),
  suicides: shifted_i16(),
  most_kills_in_a_row: shifted_i16(),
  seconds_alive: shifted_i16(),
  ctf_flag_scores: shifted_i16(),
  ctf_flag_grabs: shifted_i16(),
  ctf_flag_carrier_kills: shifted_i16(),
  ctf_flag_returns: shifted_i16(),
  assault_bomb_arms: shifted_i16(),
  assault_bomb_grabs: shifted_i16(),
  assault_bomb_disarms: shifted_i16(),
  assault_bomb_detonations: shifted_i16(),
  oddball_time_with_ball: shifted_i16(),
  oddball_unused: shifted_i16(),
  oddball_kills_as_carrier: shifted_i16(),
  oddball_ball_carrier_kills: shifted_i16(),
  king_time_on_hill: shifted_i16(),
  king_total_control_time: shifted_i16(),
  king_unused0: shifted_i16(),
  king_unused1: shifted_i16(),
  unused0: shifted_i16(),
  unused1: shifted_i16(),
  unused2: shifted_i16(),
  vip_takedowns: shifted_i16(),
  vip_kills_as_vip: shifted_i16(),
  vip_guard_time: shifted_i16(),
  vip_time_as_vip: shifted_i16(),
  vip_lives_as_vip: shifted_i16(),
  juggernaut_kills: shifted_i16(),
  juggernaut_kills_as_juggernaut: shifted_i16(),
  juggernaut_total_control_time: shifted_i16(),
  total_wp: shifted_i16(),
  juggernaut_unused: shifted_i16(),
  territories_owned: shifted_i16(),
  territories_captures: shifted_i16(),
  territories_ousts: shifted_i16(),
  territories_time_in_territory: shifted_i16(),
  infection_zombie_kills: shifted_i16(),
  infection_infections: shifted_i16(),
  infection_time_as_human: shifted_i16(),
});
export type s_game_results_statistics = c.infer<
  typeof s_game_results_statistics
>;

/** 0x74 bytes. */
export const s_game_results_player_statistics_medals = c.struct({
  extermination: shifted_i16(),
  perfection: shifted_i16(),
  multiple_kill_2: shifted_i16(),
  multiple_kill_3: shifted_i16(),
  multiple_kill_4: shifted_i16(),
  multiple_kill_5: shifted_i16(),
  multiple_kill_6: shifted_i16(),
  multiple_kill_7: shifted_i16(),
  multiple_kill_8: shifted_i16(),
  multiple_kill_9: shifted_i16(),
  multiple_kill_10: shifted_i16(),
  kills_in_a_row_5: shifted_i16(),
  kills_in_a_row_10: shifted_i16(),
  kills_in_a_row_15: shifted_i16(),
  kills_in_a_row_20: shifted_i16(),
  kills_in_a_row_25: shifted_i16(),
  kills_in_a_row_30: shifted_i16(),
  sniper_kill_5: shifted_i16(),
  sniper_kill_10: shifted_i16(),
  shotgun_kill_5: shifted_i16(),
  shotgun_kill_10: shifted_i16(),
  collision_kill_5: shifted_i16(),
  collision_kill_10: shifted_i16(),
  sword_kill_5: shifted_i16(),
  sword_kill_10: shifted_i16(),
  juggernaut_kill_5: shifted_i16(),
  juggernaut_kill_10: shifted_i16(),
  zombie_kill_5: shifted_i16(),
  zombie_kill_10: shifted_i16(),
  human_kill_5: shifted_i16(),
  human_kill_10: shifted_i16(),
  human_kill_15: shifted_i16(),
  koth_kill_5: shifted_i16(),
  shotgun_kill_sword: shifted_i16(),
  vehicle_impact_kill: shifted_i16(),
  vehicle_hijack: shifted_i16(),
  aircraft_hijack: shifted_i16(),
  deadplayer_kill: shifted_i16(),
  player_kill_spreeplayer: shifted_i16(),
  spartanlaser_kill: shifted_i16(),
  stickygrenade_kill: shifted_i16(),
  sniper_kill: shifted_i16(),
  bashbehind_kill: shifted_i16(),
  bash_kill: shifted_i16(),
  flame_kill: shifted_i16(),
  driver_assist_gunner: shifted_i16(),
  assault_bomb_planted: shifted_i16(),
  assault_player_kill_carrier: shifted_i16(),
  vip_player_kill_vip: shifted_i16(),
  juggernaut_player_kill_juggernaut: shifted_i16(),
  oddball_carrier_kill_player: shifted_i16(),
  ctf_flag_captured: shifted_i16(),
  ctf_flag_player_kill_carrier: shifted_i16(),
  ctf_flag_carrier_kill_player: shifted_i16(),
  infection_survive: shifted_i16(),
  nemesis: shifted_i16(),
  avenger: shifted_i16(),
  unused3: shifted_i16(),
});
export type s_game_results_player_statistics_medals = c.infer<
  typeof s_game_results_player_statistics_medals
>;

/** 0x62 bytes. */
export const s_game_results_player_statistics_achievements = c.struct({
  landfall: shifted_i16(),
  holdout: shifted_i16(),
  the_road: shifted_i16(),
  assault: shifted_i16(),
  cleansing: shifted_i16(),
  refuge: shifted_i16(),
  last_stand: shifted_i16(),
  the_key: shifted_i16(),
  _return: shifted_i16(),
  campaign_complete_normal: shifted_i16(),
  campaign_complete_heroic: shifted_i16(),
  campaign_complete_legendary: shifted_i16(),
  iron: shifted_i16(),
  black_eye: shifted_i16(),
  tough_luck: shifted_i16(),
  catch: shifted_i16(),
  fog: shifted_i16(),
  famine: shifted_i16(),
  thunderstorm: shifted_i16(),
  tilt: shifted_i16(),
  mythic: shifted_i16(),
  marathon_man: shifted_i16(),
  guerilla: shifted_i16(),
  demon: shifted_i16(),
  cavalier: shifted_i16(),
  askar: shifted_i16(),
  exterminator: shifted_i16(),
  ranger: shifted_i16(),
  vanguard: shifted_i16(),
  orpheus: shifted_i16(),
  reclaimer: shifted_i16(),
  graduate: shifted_i16(),
  unsc_spartan: shifted_i16(),
  spartan_officer: shifted_i16(),
  two_for_one: shifted_i16(),
  triple_kill: shifted_i16(),
  overkill: shifted_i16(),
  lee_r_wilson_memorial: shifted_i16(),
  killing_frenzy: shifted_i16(),
  steppin_razor: shifted_i16(),
  mongoose_mowdown: shifted_i16(),
  up_close_and_personal: shifted_i16(),
  mvp: shifted_i16(),
  maybe_next_time_buddy: shifted_i16(),
  too_close_to_the_sun: shifted_i16(),
  we_re_in_for_some_chop: shifted_i16(),
  fear_the_pink_mist: shifted_i16(),
  headshot_honcho: shifted_i16(),
  used_car_salesman: shifted_i16(),
});
export type s_game_results_player_statistics_achievements = c.infer<
  typeof s_game_results_player_statistics_achievements
>;

/** 0xC bytes. */
export const s_game_results_damage_statistics = c.struct({
  valid: c.i8(),
  kills: shifted_i16().padBefore(1),
  deaths: shifted_i16(),
  betrayals: shifted_i16(),
  suicides: shifted_i16(),
  headshots: shifted_i16(),
});
export type s_game_results_damage_statistics = c.infer<
  typeof s_game_results_damage_statistics
>;

/** Damage statistics per damage source (0x2DC bytes). */
export const s_game_results_player_damage_source_statistics = c.struct({
  unknown: s_game_results_damage_statistics,
  falling_damage: s_game_results_damage_statistics,
  collision_damage: s_game_results_damage_statistics,
  melee: s_game_results_damage_statistics,
  explosion: s_game_results_damage_statistics,
  magnum_pistol: s_game_results_damage_statistics,
  plasma_pistol: s_game_results_damage_statistics,
  needler: s_game_results_damage_statistics,
  excavator: s_game_results_damage_statistics,
  smg: s_game_results_damage_statistics,
  plasma_rifle: s_game_results_damage_statistics,
  battle_rifle: s_game_results_damage_statistics,
  carbine: s_game_results_damage_statistics,
  shotgun: s_game_results_damage_statistics,
  sniper_rifle: s_game_results_damage_statistics,
  beam_rifle: s_game_results_damage_statistics,
  assault_rifle: s_game_results_damage_statistics,
  spike_rifle: s_game_results_damage_statistics,
  flak_cannon: s_game_results_damage_statistics,
  missile_launcher: s_game_results_damage_statistics,
  rocket_launcher: s_game_results_damage_statistics,
  spartan_laser: s_game_results_damage_statistics,
  brute_shot: s_game_results_damage_statistics,
  flame_thrower: s_game_results_damage_statistics,
  sentinal_gun: s_game_results_damage_statistics,
  energy_sword: s_game_results_damage_statistics,
  gravity_hammer: s_game_results_damage_statistics,
  frag_grenade: s_game_results_damage_statistics,
  plasma_grenade: s_game_results_damage_statistics,
  claymore_grenade: s_game_results_damage_statistics,
  firebomb_grenade: s_game_results_damage_statistics,
  flag: s_game_results_damage_statistics,
  bomb: s_game_results_damage_statistics,
  bomb_explosion: s_game_results_damage_statistics,
  ball: s_game_results_damage_statistics,
  human_turret: s_game_results_damage_statistics,
  plasma_cannon: s_game_results_damage_statistics,
  plasma_mortar: s_game_results_damage_statistics,
  plasma_turret: s_game_results_damage_statistics,
  banshee: s_game_results_damage_statistics,
  ghost: s_game_results_damage_statistics,
  mongoose: s_game_results_damage_statistics,
  scorpion: s_game_results_damage_statistics,
  scorpion_gunner: s_game_results_damage_statistics,
  spectre_driver: s_game_results_damage_statistics,
  spectre_gunner: s_game_results_damage_statistics,
  warthog_driver: s_game_results_damage_statistics,
  warthog_gunner: s_game_results_damage_statistics,
  warthog_gunner_gauss: s_game_results_damage_statistics,
  wraith: s_game_results_damage_statistics,
  wraith_anti_infantry: s_game_results_damage_statistics,
  tank: s_game_results_damage_statistics,
  chopper: s_game_results_damage_statistics,
  hornet: s_game_results_damage_statistics,
  mantis: s_game_results_damage_statistics,
  mauler: s_game_results_damage_statistics,
  sentinel_beam: s_game_results_damage_statistics,
  sentinel_rpg: s_game_results_damage_statistics,
  teleporter: s_game_results_damage_statistics,
  sandtrap_mine: s_game_results_damage_statistics,
  elephant_turret: s_game_results_damage_statistics,
});
export type s_game_results_player_damage_source_statistics = c.infer<
  typeof s_game_results_player_damage_source_statistics
>;

/** 0x418 bytes. */
export const s_game_results_player_statistics = c.struct({
  statistics: s_game_results_statistics,
  medals: s_game_results_player_statistics_medals,
  achievements: s_game_results_player_statistics_achievements,
  damage_statistics: s_game_results_player_damage_source_statistics,
});
export type s_game_results_player_statistics = c.infer<
  typeof s_game_results_player_statistics
>;

/** Halo 3 per-player statistics (`mps1` 2.1, 0x4184 bytes), nested in `_par`. */
@blf.chunk("mps1", 2.1)
export class s_blf_chunk_multiplayer_player_statistics extends CStructBLFChunk(
  c.struct({
    players: s_game_results_player_statistics
      .array(k_maximum_players)
      .padBefore(4),
  })
) {}
