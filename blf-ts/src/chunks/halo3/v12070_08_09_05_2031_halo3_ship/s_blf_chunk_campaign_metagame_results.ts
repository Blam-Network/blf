import { c } from "@craftycodie/cstruct";
import { k_maximum_campaign_players } from "../../../blam/halo3/v12070_08_09_05_2031_halo3_ship/game/game_options";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

export const k_campaign_metagame_damage_stack_count = 256;

export enum e_campaign_metagame_bucket_type {
  brute = 0,
  grunt = 1,
  jackal = 2,
  marine = 3,
  bugger = 4,
  hunter = 5,
  flood_infection = 6,
  flood_carrier = 7,
  flood_combat = 8,
  flood_pure = 9,
  sentinel = 10,
  elite = 11,
  turret = 12,
  mongoose = 13,
  warthog = 14,
  scorpion = 15,
  hornet = 16,
  pelican = 17,
  shade = 18,
  watchtower = 19,
  ghost = 20,
  chopper = 21,
  prowler = 22,
  wraith = 23,
  banshee = 24,
  phantom = 25,
  scarab = 26,
  guntower = 27,
}

export enum e_campaign_metagame_bucket_class {
  infantry = 0,
  leader = 1,
  hero = 2,
  specialist = 3,
  light_vehicle = 4,
  heavy_vehicle = 5,
  giant_vehicle = 6,
  standard_vehicle = 7,
}

/** Kills of one enemy type, by class. */
export const s_metagame_state_class_kill_counts = c.struct({
  infantry: c.i32(),
  leader: c.i32(),
  hero: c.i32(),
  specialist: c.i32(),
  light_vehicle: c.i32(),
  heavy_vehicle: c.i32(),
  giant_vehicle: c.i32(),
  standard_vehicle: c.i32(),
});
export type s_metagame_state_class_kill_counts = c.infer<
  typeof s_metagame_state_class_kill_counts
>;

/** Kills by enemy type (keyed by `e_campaign_metagame_bucket_type` name). */
export const s_metagame_state_type_kill_counts = c.struct({
  brute: s_metagame_state_class_kill_counts,
  grunt: s_metagame_state_class_kill_counts,
  jackal: s_metagame_state_class_kill_counts,
  marine: s_metagame_state_class_kill_counts,
  bugger: s_metagame_state_class_kill_counts,
  hunter: s_metagame_state_class_kill_counts,
  flood_infection: s_metagame_state_class_kill_counts,
  flood_carrier: s_metagame_state_class_kill_counts,
  flood_combat: s_metagame_state_class_kill_counts,
  flood_pure: s_metagame_state_class_kill_counts,
  sentinel: s_metagame_state_class_kill_counts,
  elite: s_metagame_state_class_kill_counts,
  turret: s_metagame_state_class_kill_counts,
  mongoose: s_metagame_state_class_kill_counts,
  warthog: s_metagame_state_class_kill_counts,
  scorpion: s_metagame_state_class_kill_counts,
  hornet: s_metagame_state_class_kill_counts,
  pelican: s_metagame_state_class_kill_counts,
  shade: s_metagame_state_class_kill_counts,
  watchtower: s_metagame_state_class_kill_counts,
  ghost: s_metagame_state_class_kill_counts,
  chopper: s_metagame_state_class_kill_counts,
  prowler: s_metagame_state_class_kill_counts,
  wraith: s_metagame_state_class_kill_counts,
  banshee: s_metagame_state_class_kill_counts,
  phantom: s_metagame_state_class_kill_counts,
  scarab: s_metagame_state_class_kill_counts,
  guntower: s_metagame_state_class_kill_counts,
});
export type s_metagame_state_type_kill_counts = c.infer<
  typeof s_metagame_state_type_kill_counts
>;

/** 0x3F0 bytes. */
export const s_metagame_state = c.struct({
  player_identifier: c.u64(),
  transient_subtotal: c.i32(),
  subtotal: c.i32(),
  medal_points: c.i32(),
  scripted_points: c.i32(),
  kill_counts: s_metagame_state_type_kill_counts,
  grenade_stick_kills: c.i32(),
  headshot_kills: c.i32(),
  melee_kills: c.i32(),
  assassination_kills: c.i32(),
  splatter_kills: c.i32(),
  multi_kills: c.i32(),
  needler_supercombine_kills: c.i32(),
  emp_kills: c.i32(),
  ai_betrayal_count: c.i32(),
  multikill_display_timer: c.i32(),
  negative_toast_amount: c.i32(),
  negative_toast_timer: c.i32(),
  transient_scripted_points_amount: c.i32(),
  transient_scripted_points_timer: c.i32(),
  infantry_kills: c.i16(),
  leader_kills: c.i16(),
  hero_kills: c.i16(),
  specialist_kills: c.i16(),
  light_vehicle_kills: c.i16(),
  heavy_vehicle_kills: c.i16(),
  giant_vehicle_kills: c.i16(),
  standard_vehicle_kills: c.i16(),
  kill_total_count: c.i32(),
  style_total_count: c.i32(),
  finalized: c.bool().padAfter(3),
  player_final_score: c.i32(),
});
export type s_metagame_state = c.infer<typeof s_metagame_state>;

export const s_metagame_state_secondary = c.struct({
  player_identifier: c.u64(),
  death_count: c.i32(),
  betrayal_count: c.i32(),
  elapsed_tick_count: c.i32(),
  secondary_subtotal: c.i32(),
});
export type s_metagame_state_secondary = c.infer<
  typeof s_metagame_state_secondary
>;

/** 0x8 bytes. */
export const s_campaign_metagame_bucket = c.struct({
  /** Bit 0: only counts with riders. */
  flags: c.u8(),
  campaign_metagame_bucket_type: c.enum(
    e_campaign_metagame_bucket_type,
    c.i8()
  ),
  campaign_metagame_bucket_class: c.enum(
    e_campaign_metagame_bucket_class,
    c.i8()
  ),
  point_count: c.i16().padBefore(1).padAfter(2),
});
export type s_campaign_metagame_bucket = c.infer<
  typeof s_campaign_metagame_bucket
>;

/** 0x14 bytes. */
export const s_campaign_metagame_damage_event = c.struct({
  metagame_bucket: s_campaign_metagame_bucket,
  game_time: c.i32(),
  damage_owner_player_index: c.u32(),
  damage_receiver_object_index: c.u32(),
});
export type s_campaign_metagame_damage_event = c.infer<
  typeof s_campaign_metagame_damage_event
>;

/** 0x68 bytes. */
export const s_campaign_metagame_runtime_globals_secondary = c.struct({
  total_elapsed_tick_count: c.i32(),
  initialized: c.bool().padAfter(3),
  stats: s_metagame_state_secondary.array(k_maximum_campaign_players),
});
export type s_campaign_metagame_runtime_globals_secondary = c.infer<
  typeof s_campaign_metagame_runtime_globals_secondary
>;

/** 0x2438 bytes. */
export const s_campaign_metagame_runtime_globals = c.struct({
  saved_secondary_globals: s_campaign_metagame_runtime_globals_secondary,
  stats: s_metagame_state.array(k_maximum_campaign_players),
  damage_stack: s_campaign_metagame_damage_event.array(
    k_campaign_metagame_damage_stack_count
  ),
  damage_stack_count: c.i32(),
  flags: c.i32(),
  time_bonus: c.f32(),
  final_total_score: c.f32(),
});
export type s_campaign_metagame_runtime_globals = c.infer<
  typeof s_campaign_metagame_runtime_globals
>;

/** Halo 3 campaign metagame (scoring) results (`cmrp` 1.1, 0x243C bytes). */
@blf.chunk("cmrp", 1.1)
export class s_blf_chunk_campaign_metagame_results extends CStructBLFChunk(
  c.struct({
    valid: c.bool().padAfter(3),
    results: s_campaign_metagame_runtime_globals,
  })
) {}
