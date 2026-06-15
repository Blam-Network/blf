import { AutoMap } from "../../../../helpers/automap";
export enum e_game_engine_timer_rate {
  zero = 0,
  minus_10x = 1,
  minus_25x = 2,
  minus_50x = 3,
  minus_75x = 4,
  minus_100x = 5,
  minus_125x = 6,
  minus_150x = 7,
  minus_175x = 8,
  minus_200x = 9,
  minus_300x = 10,
  minus_400x = 11,
  minus_500x = 12,
  minus_1000x = 13,
  _10x = 14,
  _25x = 15,
  _50x = 16,
  _75x = 17,
  _100x = 18,
  _125x = 19,
  _150x = 20,
  _175x = 21,
  _200x = 22,
  _300x = 23,
  _400x = 24,
  _500x = 25,
  _1000x = 26,
}
export enum e_weapon_pickup_priority {
  normal = 0,
  high = 1,
  automatic = 2,
}
/** Matches `e_survival_variant_flags` in blf_lib `game_engine_survival.rs`. */
export class e_survival_variant_flags {
  @AutoMap(() => Boolean)
  hazards_enabled = false;
  @AutoMap(() => Boolean)
  all_generators_must_survive = false;
  @AutoMap(() => Boolean)
  random_generator_spawns = false;
  @AutoMap(() => Boolean)
  weapon_drops_enabled = false;
  @AutoMap(() => Boolean)
  ammo_crates_enabled = false;
  to_raw(): number {
    return (
      (this.hazards_enabled ? 1 : 0) |
      (this.all_generators_must_survive ? 1 << 1 : 0) |
      (this.random_generator_spawns ? 1 << 2 : 0) |
      (this.weapon_drops_enabled ? 1 << 3 : 0) |
      (this.ammo_crates_enabled ? 1 << 4 : 0)
    );
  }
  static from_raw(raw: number): e_survival_variant_flags {
    const flags = new e_survival_variant_flags();
    flags.hazards_enabled = (raw & 1) !== 0;
    flags.all_generators_must_survive = (raw & (1 << 1)) !== 0;
    flags.random_generator_spawns = (raw & (1 << 2)) !== 0;
    flags.weapon_drops_enabled = (raw & (1 << 3)) !== 0;
    flags.ammo_crates_enabled = (raw & (1 << 4)) !== 0;
    return flags;
  }
}
/** Matches `e_sandbox_variant_flags` in blf_lib `game_engine_sandbox.rs`. */
export class e_sandbox_variant_flags {
  @AutoMap(() => Boolean)
  open_channel_voice = false;
  @AutoMap(() => Boolean)
  requires_all_objects = false;
  to_raw(): number {
    return (
      (this.open_channel_voice ? 1 : 0) |
      (this.requires_all_objects ? 1 << 1 : 0)
    );
  }
  static from_raw(raw: number): e_sandbox_variant_flags {
    const flags = new e_sandbox_variant_flags();
    flags.open_channel_voice = (raw & 1) !== 0;
    flags.requires_all_objects = (raw & (1 << 1)) !== 0;
    return flags;
  }
}
/** Matches `e_sandbox_edit_mode_settings` in blf_lib `game_engine_sandbox.rs`. */
export enum e_sandbox_edit_mode_settings {
  all_players = 0,
  only_leader = 1,
}
