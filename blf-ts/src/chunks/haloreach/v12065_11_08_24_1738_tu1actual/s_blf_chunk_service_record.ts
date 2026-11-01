import { c } from "@craftycodie/cstruct";
import { blf, CStructBLFChunk } from "../../../blf_chunk";
import { write_blffile } from "../../../blffile";

/** 32-bit unsigned Unix seconds (`time32_t`), exposed as a `Date`. Writes floor to whole seconds. */
const time32 = () =>
  c.u32().transform(
    (raw) => new Date(raw * 1000),
    (value: Date) => Math.floor(value.getTime() / 1000) >>> 0
  );

/** Per-difficulty campaign stats in `srid` (0x18 bytes). */
export const s_blf_chunk_service_record_campaign_difficulty_stats = c.struct({
  covenant_kills: c.u32(),
  vehicles_destroyed: c.u32(),
  missions_completed_without_dying_or_restarting: c.u32(),
  highest_skull_multiplier: c.u32(),
  missions_complete: c.u32(),
  unknown1: c.u32(),
});
export type s_blf_chunk_service_record_campaign_difficulty_stats = c.infer<
  typeof s_blf_chunk_service_record_campaign_difficulty_stats
>;

/** Per-difficulty firefight stats in `srid` (0x1C bytes). */
export const s_blf_chunk_service_record_firefight_difficulty_stats = c.struct({
  covenant_kills: c.u32(),
  vehicles_destroyed: c.u32(),
  highest_set_completed: c.u32(),
  most_consecutive_kills_without_dying: c.u32(),
  biggest_kill: c.u32(),
  times_beat_par: c.u32(),
  highest_official_score: c.u32(),
});
export type s_blf_chunk_service_record_firefight_difficulty_stats = c.infer<
  typeof s_blf_chunk_service_record_firefight_difficulty_stats
>;

/** Per-category matchmaking stats in `srid` (0x14 bytes). */
export const s_blf_chunk_service_record_matchmaking_category_stats = c.struct({
  games_won: c.u32(),
  kills: c.u32(),
  deaths: c.u32(),
  assists: c.u32(),
  percentage_of_matchmaking_games_played_in_category: c.u32(),
});
export type s_blf_chunk_service_record_matchmaking_category_stats = c.infer<
  typeof s_blf_chunk_service_record_matchmaking_category_stats
>;

/** Per-hopper arena stats in `srid` (0x68 bytes). */
export const s_blf_chunk_service_record_arena_hopper_stats = c.struct({
  hopper_name: c.string(32, "utf8"),
  games_played_today: c.u32(),
  current_best_set: c.u32(),
  yesterdays_best_set: c.u32(),
  days_rated: c.u32(),
  division_standing: c.u32(),
  division: c.u32(),
  unknown1: c.u32(),
  unknown2: c.u32(),
  unknown3: c.u32(),
  unknown4: c.u32(),
  unknown5: c.u32(),
  games_played: c.u32(),
  games_won: c.u32(),
  kills: c.u32(),
  assists: c.u32(),
  deaths: c.u32(),
  last_7_days_kill_and_assist_death_ratio: c.f32(),
  last_7_days_kill_death_ratio: c.f32(),
});
export type s_blf_chunk_service_record_arena_hopper_stats = c.infer<
  typeof s_blf_chunk_service_record_arena_hopper_stats
>;

/** Per-season arena stats in `srid` (0x348 bytes). */
export const s_blf_chunk_service_record_arena_season_stats = c.struct({
  season_number: c.u32(),
  hopper_stats_count: c.u32(),
  hopper_stats: s_blf_chunk_service_record_arena_hopper_stats.array(8),
});
export type s_blf_chunk_service_record_arena_season_stats = c.infer<
  typeof s_blf_chunk_service_record_arena_season_stats
>;

/** Commendation progress in `srid` (8 bytes). */
export const s_blf_chunk_service_record_commendation = c.struct({
  commendation: c.u32(),
  progress: c.u32(),
});
export type s_blf_chunk_service_record_commendation = c.infer<
  typeof s_blf_chunk_service_record_commendation
>;

/** Reach TU1 service record (`srid` 7.1, 0xD3C bytes). */
@blf.chunk("srid", 7.1)
export class s_blf_chunk_service_record extends CStructBLFChunk(
  c.struct({
    player_name: c.wstring(16),
    player_info_available: c.bool(),
    unknown1: c.u8().array(4),
    armour_primary_color: c.u8(), // 0x25
    armour_secondary_color: c.u8(),
    armour_tertiary_color: c.u8(),
    player_model_choice: c.u8(), // 0x28
    // probably padding
    unknown2: c.u8().array(3),
    emblem_primary: c.u8(), // 0x2C
    emblem_background: c.u8(),
    emblem_secondary: c.bool(),
    emblem_primary_color: c.u8(),
    emblem_secondary_color: c.u8(),
    emblem_background_color: c.u8(), // 0x31
    unknown3: c.u8().array(14),
    service_tag: c.wstring(5), // 0x40
    unknown4: c.u8().array(2),
    career_overview_stats_available: c.bool(),
    credits_available: c.bool(),
    credits: c.i32(),
    campaign_record_available: c.bool(),
    campaign_completed_at: time32(),
    campaign_completion_difficulty: c.u32(),
    campaign_enemies_killed: c.u32(),
    campaign_vehicles_destroyed: c.u32(),
    campaign_seconds_played: c.u32(),
    campaign_difficulty_stats:
      s_blf_chunk_service_record_campaign_difficulty_stats.array(3),
    campaign_commendations_count: c.u32(), // 0xAF
    campaign_commendations: s_blf_chunk_service_record_commendation.array(16),
    firefight_record_available: c.bool(),
    firefight_covenant_kills: c.u32(),
    firefight_vehicles_destroyed: c.u32(),
    firefight_highest_set_completed: c.u32(),
    firefight_most_kills_in_game: c.u32(),
    firefight_waves_completed: c.u32(),
    firefight_generators_destroyed: c.u32(),
    firefight_enemy_players_killed: c.u32(),
    firefight_difficulty_stats:
      s_blf_chunk_service_record_firefight_difficulty_stats.array(3),
    firefight_commendations_count: c.u32(),
    firefight_commendations: s_blf_chunk_service_record_commendation.array(16),
    matchmaking_record_available: c.bool(), // 0x228
    matchmaking_games_won: c.u32(),
    matchmaking_kills: c.u32(),
    matchmaking_deaths: c.u32(),
    matchmaking_assists: c.u32(),
    matchmaking_category_stats:
      s_blf_chunk_service_record_matchmaking_category_stats.array(5),
    arena_season_stats_count: c.u32(),
    arena_season_stats: s_blf_chunk_service_record_arena_season_stats.array(3),
    matchmaking_commendations_count: c.u32(),
    matchmaking_commendations:
      s_blf_chunk_service_record_commendation.array(16),
    custom_games_record_available: c.bool(), // 0xCFD
    custom_games_multiplayer_played: c.u32(),
    custom_games_multiplayer_kills: c.u32(),
    custom_games_firefight_played: c.u32(),
    custom_games_firefight_killed: c.u32(),
    legacy_record_available: c.bool(),
    odst_first_played_time: time32(),
    halo3_first_played_time: time32(),
    halo2_first_played_time: time32(),
    halo2_highest_difficulty: c.u32(),
    halo2_unknown_1: c.u32(),
    halo2_unknown_2: c.u32(),
    halo3_highest_difficulty: c.u32(),
    halo3_games_played: c.u32(),
    halo3_multiplayer_kills: c.u32(),
    odst_highest_difficulty: c.u32(),
    odst_grunts_killed_in_firefight: c.u32(),
    // Not 100% sure this is here.
    unknown5: c.u8(),
  })
) {}

/**
 * Service record file: just the `srid` chunk, with no `_blf` or `_eof`
 * (matches blf_lib).
 */
export function build_service_record_file(
  srid: s_blf_chunk_service_record
): Uint8Array {
  return write_blffile("big", [srid]);
}
