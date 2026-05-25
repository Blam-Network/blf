export enum e_explicit_team_type {
  no_team = 0,
  team_0 = 1,
  team_1 = 2,
  team_2 = 3,
  team_3 = 4,
  team_4 = 5,
  team_5 = 6,
  team_6 = 7,
  team_7 = 8,
  neutral_team = 9,
  global_0 = 10,
  global_1 = 11,
  global_2 = 12,
  global_3 = 13,
  global_4 = 14,
  global_5 = 15,
  global_6 = 16,
  global_7 = 17,
  current = 18,
  hud_player_owner_team = 19,
  hud_target_player_owner_team = 20,
}

export enum e_explicit_player_type {
  no_player = 0,
  player_0 = 1,
  player_1 = 2,
  player_2 = 3,
  player_3 = 4,
  player_4 = 5,
  player_5 = 6,
  player_6 = 7,
  player_7 = 8,
  player_8 = 9,
  player_9 = 10,
  player_10 = 11,
  player_11 = 12,
  player_12 = 13,
  player_13 = 14,
  player_14 = 15,
  player_15 = 16,
  global_0 = 17,
  global_1 = 18,
  global_2 = 19,
  global_3 = 20,
  global_4 = 21,
  global_5 = 22,
  global_6 = 23,
  global_7 = 24,
  current = 25,
  hud = 26,
  hud_target = 27,
  killer = 28,
}

export enum e_explicit_object_type {
  no_object = 0,
  global_0 = 1,
  global_1 = 2,
  global_2 = 3,
  global_3 = 4,
  global_4 = 5,
  global_5 = 6,
  global_6 = 7,
  global_7 = 8,
  global_8 = 9,
  global_9 = 10,
  global_10 = 11,
  global_11 = 12,
  global_12 = 13,
  global_13 = 14,
  global_14 = 15,
  global_15 = 16,
  current = 17,
  hud_target = 18,
  killed = 19,
  killer = 20,
  unknown_21 = 21,
}

export enum e_action_team_or_player_target {
  team = 0,
  player = 1,
  all_players = 2,
}

export enum e_math_operation {
  add = 0,
  subtract = 1,
  multiply_by = 2,
  divide_by = 3,
  set_to = 4,
  modulo_by = 5,
  bitwise_and_with = 6,
  bitwise_or_with = 7,
  bitwise_xor_with = 8,
  bitwise_not_with = 9,
}

export enum e_grenade_type {
  frag_grenade = 0,
  plasma_grenade = 1,
}

export enum e_biped_give_weapon_mode {
  as_primary_weapon = 0,
  normally = 1,
  silently = 2,
}

export enum e_player_filter_type {
  no_one = 0,
  everyone = 1,
  allies = 2,
  enemies = 3,
  specific_player = 4,
  normal = 5,
}

export enum e_chud_navpoint_icon_type {
  speaker = 0,
  dead_teammate_marker = 1,
  lightning_bolt = 2,
  bullseye = 3,
  diamond = 4,
  bomb = 5,
  flag = 6,
  skull = 7,
  crown = 8,
  vip = 9,
  padlock = 10,
  territory_a = 11,
  territory_b = 12,
  territory_c = 13,
  territory_d = 14,
  territory_e = 15,
  territory_f = 16,
  territory_g = 17,
  territory_h = 18,
  territory_i = 19,
  supply = 20,
  supply_health = 21,
  supply_air_drop = 22,
  supply_ammo = 23,
  arrow = 24,
  defend = 25,
  ordnance = 26,
  inward = 27,
}

export enum e_navpoint_priority {
  low = 0,
  normal = 1,
  high = 2,
  blink = 3,
}

export enum e_megalogamengine_hud_meter_input_type {
  none = 0,
  number = 1,
  timer = 2,
}

export enum e_numeric_comparison {
  less_than = 0,
  greater_than = 1,
  equal_to = 2,
  less_than_or_equal_to = 3,
  greater_than_or_equal_to = 4,
  not_equal_to = 5,
}

/** Matches `e_create_object_flags` in blf_lib `megalogamengine_actions.rs`. */
export class e_create_object_flags {
  never_garbage_collect = false;
  suppress_effect = false;
  absolute_orientation = false;

  to_raw(): number {
    return (
      (this.never_garbage_collect ? 1 : 0) |
      (this.suppress_effect ? 1 << 1 : 0) |
      (this.absolute_orientation ? 1 << 2 : 0)
    );
  }

  static from_raw(raw: number): e_create_object_flags {
    const flags = new e_create_object_flags();
    flags.never_garbage_collect = (raw & 1) !== 0;
    flags.suppress_effect = (raw & (1 << 1)) !== 0;
    flags.absolute_orientation = (raw & (1 << 2)) !== 0;
    return flags;
  }
}
