import { c } from "@craftycodie/cstruct";

export enum e_damage_resistance_percentage_setting {
  _damage_resistance_percentage_setting_unchanged = 0,
  _damage_resistance_percentage_setting_10_percent = 1,
  _damage_resistance_percentage_setting_50_percent = 2,
  _damage_resistance_percentage_setting_90_percent = 3,
  _damage_resistance_percentage_setting_100_percent = 4,
  _damage_resistance_percentage_setting_110_percent = 5,
  _damage_resistance_percentage_setting_150_percent = 6,
  _damage_resistance_percentage_setting_200_percent = 7,
  _damage_resistance_percentage_setting_300_percent = 8,
  _damage_resistance_percentage_setting_500_percent = 9,
  _damage_resistance_percentage_setting_1000_percent = 10,
  _damage_resistance_percentage_setting_2000_percent = 11,
  _damage_resistance_percentage_setting_invulnerable = 12,
}

export enum e_shield_recharge_rate_percentage_setting {
  _shield_recharge_rate_percentage_setting_unchanged = 0,
  _shield_recharge_rate_percentage_setting_negative_25_percent = 1,
  _shield_recharge_rate_percentage_setting_negative_10_percent = 2,
  _shield_recharge_rate_percentage_setting_negative_5_percent = 3,
  _shield_recharge_rate_percentage_setting_0_percent = 4,
  _shield_recharge_rate_percentage_setting_50_percent = 5,
  _shield_recharge_rate_percentage_setting_90_percent = 6,
  _shield_recharge_rate_percentage_setting_100_percent = 7,
  _shield_recharge_rate_percentage_setting_110_percent = 8,
  _shield_recharge_rate_percentage_setting_200_percent = 9,
}

/** Stored as a byte; `0xFF` is `_vampirism_percentage_setting_default` (-1). */
export enum e_vampirism_percentage_setting {
  _vampirism_percentage_setting_default = -1,
  _vampirism_percentage_setting_unchanged = 0,
  _vampirism_percentage_setting_0_percent = 1,
  _vampirism_percentage_setting_10_percent = 2,
  _vampirism_percentage_setting_25_percent = 3,
  _vampirism_percentage_setting_50_percent = 4,
  _vampirism_percentage_setting_100_percent = 5,
}

export enum e_shield_multiplier_setting {
  _shield_multiplier_setting_unchanged = 0,
  _shield_multiplier_setting_0x = 1,
  _shield_multiplier_setting_1x = 2,
  _shield_multiplier_setting_2x = 3,
  _shield_multiplier_setting_3x = 4,
  _shield_multiplier_setting_4x = 5,
}

export enum e_damage_modifier_percentage_setting {
  _damage_modifier_percentage_setting_unchanged = 0,
  _damage_modifier_percentage_setting_0_percent = 1,
  _damage_modifier_percentage_setting_25_percent = 2,
  _damage_modifier_percentage_setting_50_percent = 3,
  _damage_modifier_percentage_setting_75_percent = 4,
  _damage_modifier_percentage_setting_90_percent = 5,
  _damage_modifier_percentage_setting_100_percent = 6,
  _damage_modifier_percentage_setting_110_percent = 7,
  _damage_modifier_percentage_setting_125_percent = 8,
  _damage_modifier_percentage_setting_150_percent = 9,
  _damage_modifier_percentage_setting_200_percent = 10,
  _damage_modifier_percentage_setting_300_percent = 11,
  _damage_modifier_percentage_setting_fatality = 12,
}

export enum e_headshot_immunity_setting {
  _headshot_immunity_setting_unchanged = 0,
  _headshot_immunity_setting_immune_to_headshots = 1,
  _headshot_immunity_setting_not_immune_to_headshots = 2,
}

export const c_player_trait_shield_vitality = c.struct({
  m_damage_resistance_percentage_setting: c.enum(
    e_damage_resistance_percentage_setting,
    c.u8()
  ),
  m_shield_recharge_rate_percentage_setting: c.enum(
    e_shield_recharge_rate_percentage_setting,
    c.u8()
  ),
  m_vampirism_percentage_setting: c.enum(
    e_vampirism_percentage_setting,
    c.i8()
  ),
  m_headshot_immunity_setting: c.enum(e_headshot_immunity_setting, c.u8()),
  m_shield_multiplier_setting: c
    .enum(e_shield_multiplier_setting, c.u8())
    .padAfter(3),
});
export type c_player_trait_shield_vitality = c.infer<
  typeof c_player_trait_shield_vitality
>;

export const c_player_trait_weapons = c.struct({
  m_initial_grenade_count_setting: c.u16(),
  m_initial_primary_weapon_absolute_index: c.i8(),
  m_initial_secondary_weapon_absolute_index: c.i8(),
  m_damage_modifier_percentage_setting: c.u8(),
  m_recharging_grenades_setting: c.u8(),
  m_infinite_ammo_setting: c.u8(),
  m_weapon_pickup_setting: c.u8(),
});
export type c_player_trait_weapons = c.infer<typeof c_player_trait_weapons>;

export const c_player_trait_movement = c.struct({
  m_speed_setting: c.u8(),
  m_gravity_setting: c.u8(),
  m_vehicle_usage_setting: c.u8().padAfter(1),
});
export type c_player_trait_movement = c.infer<typeof c_player_trait_movement>;

export const c_player_trait_appearance = c.struct({
  m_active_camo_setting: c.u8(),
  m_waypoint_setting: c.u8(),
  m_aura_setting: c.u8(),
  m_forced_change_color_setting: c.u8(),
});
export type c_player_trait_appearance = c.infer<
  typeof c_player_trait_appearance
>;

export const c_player_trait_sensors = c.struct({
  m_motion_tracker_setting: c.u16(),
  m_motion_tracker_range_setting: c.u16(),
});
export type c_player_trait_sensors = c.infer<typeof c_player_trait_sensors>;

/** 0x1C bytes. */
export const c_player_traits = c.struct({
  m_shield_vitality_traits: c_player_trait_shield_vitality,
  m_weapon_traits: c_player_trait_weapons,
  m_movement_traits: c_player_trait_movement,
  m_appearance_traits: c_player_trait_appearance,
  m_sensor_traits: c_player_trait_sensors,
});
export type c_player_traits = c.infer<typeof c_player_traits>;
