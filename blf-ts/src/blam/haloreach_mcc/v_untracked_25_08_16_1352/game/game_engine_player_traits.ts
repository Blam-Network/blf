import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../bitstream";
import { AutoMap } from "../../../../helpers/automap";
export enum e_grenade_count_setting {
  unchanged = 0,
  map_default = 1,
  none = 2,
  frag_1 = 3,
  frag_2 = 4,
  frag_3 = 5,
  frag_4 = 6,
  plasma_1 = 7,
  plasma_2 = 8,
  plasma_3 = 9,
  plasma_4 = 10,
  each_1 = 11,
  each_2 = 12,
  each_3 = 13,
  each_4 = 14,
}
export enum e_boolean_trait {
  unchanged = 0,
  off = 1,
  on = 2,
}
export enum e_equipment_usage_setting {
  unchanged = 0,
  off = 1,
  not_with_objectives = 2,
  on = 3,
}
export enum e_infinite_ammo_setting {
  unchanged = 0,
  disabled = 1,
  enabled = 2,
  bottomless_clip = 3,
}

export enum e_vehicle_usage_setting {
  unchanged = 0,
  none = 1,
  passenger = 2,
  driver = 3,
  gunner = 4,
  not_passenger = 5,
  not_driver = 6,
  not_gunner = 7,
  full = 8,
}

export enum e_waypoint_setting {
  unchanged = 0,
  off = 1,
  allies = 2,
  all = 3,
}
export enum e_active_camo_setting {
  off = 0,
  on = 1,
  poor = 2,
  good = 3,
  excellent = 4,
  invisible = 5,
}
export enum e_double_jump_setting {
  unchanged = 0,
  off = 1,
  on = 2,
  triple = 3,
}
export enum e_aura_setting {
  unchanged = 0,
  off = 1,
  team_color = 2,
  black = 3,
  white = 4,
}
export enum e_forced_change_color_setting {
  unchanged = 0,
  off = 1,
  red = 2,
  blue = 3,
  green = 4,
  yellow = 5,
  purple = 6,
  orange = 7,
  brown = 8,
  pink = 9,
  white = 10,
  black = 11,
  zombie = 12,
  extra4 = 13,
}
export enum e_motion_tracker_setting {
  unchanged = 0,
  off = 1,
  allies = 2,
  normal = 3,
  enhanced = 4,
}

export enum e_damage_resistance_percentage_setting {
  unchanged = 0,
  percent_10 = 1,
  percent_50 = 2,
  percent_90 = 3,
  percent_100 = 4,
  percent_110 = 5,
  percent_150 = 6,
  percent_200 = 7,
  percent_300 = 8,
  percent_500 = 9,
  percent_1000 = 10,
  percent_2000 = 11,
  invulnerable = 12,
}

export enum e_damage_modifier_percentage_setting {
  unchanged = 0,
  percent_0 = 1,
  percent_25 = 2,
  percent_50 = 3,
  percent_75 = 4,
  percent_90 = 5,
  percent_100 = 6,
  percent_110 = 7,
  percent_125 = 8,
  percent_150 = 9,
  percent_200 = 10,
  percent_300 = 11,
  fatality = 12,
}

export enum e_body_multiplier_setting {
  unchanged = 0,
  percent_0 = 1,
  percent_100 = 2,
  percent_150 = 3,
  percent_200 = 4,
  percent_300 = 5,
  percent_400 = 6,
}

export enum e_shield_multiplier_setting {
  unchanged = 0,
  percent_0 = 1,
  percent_100 = 2,
  percent_150 = 3,
  percent_200 = 4,
  percent_300 = 5,
  percent_400 = 6,
}

export enum e_recharge_rate_percentage_setting {
  unchanged = 0,
  percent_negative_25 = 1,
  percent_negative_10 = 2,
  percent_negative_5 = 3,
  percent_0 = 4,
  percent_10 = 5,
  percent_25 = 6,
  percent_50 = 7,
  percent_75 = 8,
  percent_90 = 9,
  percent_100 = 10,
  percent_110 = 11,
  percent_125 = 12,
  percent_150 = 13,
  percent_200 = 14,
}

export enum e_vampirism_percentage_setting {
  unchanged = 0,
  percent_0 = 1,
  percent_10 = 2,
  percent_25 = 3,
  percent_50 = 4,
  percent_100 = 5,
}

export enum e_player_speed_setting {
  unchanged = 0,
  percent_0 = 1,
  percent_25 = 2,
  percent_50 = 3,
  percent_75 = 4,
  percent_90 = 5,
  percent_100 = 6,
  percent_110 = 7,
  percent_120 = 8,
  percent_130 = 9,
  percent_140 = 10,
  percent_150 = 11,
  percent_160 = 12,
  percent_170 = 13,
  percent_180 = 14,
  percent_190 = 15,
  percent_200 = 16,
  percent_300 = 17,
}

export enum e_player_gravity_setting {
  unchanged = 0,
  percent_50 = 1,
  percent_75 = 2,
  percent_100 = 3,
  percent_110 = 4,
  percent_120 = 5,
  percent_130 = 6,
  percent_140 = 7,
  percent_150 = 8,
  percent_160 = 9,
  percent_170 = 10,
  percent_180 = 11,
  percent_190 = 12,
  percent_200 = 13,
}

export enum e_motion_tracker_range_setting {
  unchanged = 0,
  meters_10 = 1,
  meters_15 = 2,
  meters_25 = 3,
  meters_50 = 4,
  meters_75 = 5,
  meters_100 = 6,
  meters_150 = 7,
}

export class c_player_trait_shield_vitality {
  @AutoMap(() => e_damage_resistance_percentage_setting)
  m_damage_resistance_percentage_setting: e_damage_resistance_percentage_setting =
    e_damage_resistance_percentage_setting.unchanged;
  @AutoMap(() => e_body_multiplier_setting)
  m_body_multiplier: e_body_multiplier_setting =
    e_body_multiplier_setting.unchanged;
  @AutoMap(() => e_recharge_rate_percentage_setting)
  m_body_recharge_rate: e_recharge_rate_percentage_setting =
    e_recharge_rate_percentage_setting.unchanged;
  @AutoMap(() => e_shield_multiplier_setting)
  m_shield_multiplier: e_shield_multiplier_setting =
    e_shield_multiplier_setting.unchanged;
  @AutoMap(() => e_recharge_rate_percentage_setting)
  m_shield_recharge_rate: e_recharge_rate_percentage_setting =
    e_recharge_rate_percentage_setting.unchanged;
  @AutoMap(() => e_recharge_rate_percentage_setting)
  m_overshield_recharge_rate: e_recharge_rate_percentage_setting =
    e_recharge_rate_percentage_setting.unchanged;
  @AutoMap(() => e_boolean_trait)
  m_headshot_immunity_setting: e_boolean_trait = e_boolean_trait.unchanged;
  @AutoMap(() => e_vampirism_percentage_setting)
  m_vampirism_percentage_setting: e_vampirism_percentage_setting =
    e_vampirism_percentage_setting.unchanged;
  @AutoMap(() => e_boolean_trait)
  m_assasination_immunity: e_boolean_trait = e_boolean_trait.unchanged;
  @AutoMap(() => e_boolean_trait)
  m_cannot_die_from_damage: e_boolean_trait = e_boolean_trait.unchanged;
  clear(): void {
    this.m_damage_resistance_percentage_setting =
      e_damage_resistance_percentage_setting.unchanged;
    this.m_body_multiplier = e_body_multiplier_setting.unchanged;
    this.m_body_recharge_rate = e_recharge_rate_percentage_setting.unchanged;
    this.m_shield_multiplier = e_shield_multiplier_setting.unchanged;
    this.m_shield_recharge_rate = e_recharge_rate_percentage_setting.unchanged;
    this.m_overshield_recharge_rate =
      e_recharge_rate_percentage_setting.unchanged;
    this.m_headshot_immunity_setting = e_boolean_trait.unchanged;
    this.m_vampirism_percentage_setting =
      e_vampirism_percentage_setting.unchanged;
    this.m_assasination_immunity = e_boolean_trait.unchanged;
    this.m_cannot_die_from_damage = e_boolean_trait.unchanged;
  }
}
export class c_player_trait_weapons {
  @AutoMap(() => e_damage_modifier_percentage_setting)
  m_damage_modifier_percentage_setting: e_damage_modifier_percentage_setting =
    e_damage_modifier_percentage_setting.unchanged;
  @AutoMap(() => e_damage_modifier_percentage_setting)
  m_melee_damage_modifier_percentage_setting: e_damage_modifier_percentage_setting =
    e_damage_modifier_percentage_setting.unchanged;
  @AutoMap(() => Number)
  m_initial_primary_weapon_absolute_index = -3;
  @AutoMap(() => Number)
  m_initial_secondary_weapon_absolute_index = -3;
  @AutoMap(() => e_grenade_count_setting)
  m_initial_grenade_count_setting: e_grenade_count_setting =
    e_grenade_count_setting.unchanged;
  @AutoMap(() => e_infinite_ammo_setting)
  m_infinite_ammo_setting: e_infinite_ammo_setting =
    e_infinite_ammo_setting.unchanged;
  @AutoMap(() => e_boolean_trait)
  m_recharging_grenades_setting: e_boolean_trait = e_boolean_trait.unchanged;
  @AutoMap(() => e_boolean_trait)
  m_weapon_pickup_setting: e_boolean_trait = e_boolean_trait.unchanged;
  @AutoMap(() => e_equipment_usage_setting)
  m_equipment_usage_setting: e_equipment_usage_setting =
    e_equipment_usage_setting.unchanged;
  @AutoMap(() => e_boolean_trait)
  m_equipment_drop_on_death_setting: e_boolean_trait =
    e_boolean_trait.unchanged;
  @AutoMap(() => e_boolean_trait)
  m_infinite_equipment_setting: e_boolean_trait = e_boolean_trait.unchanged;
  @AutoMap(() => Number)
  m_initial_equipment_absolute_index = -3;
  clear(): void {
    this.m_damage_modifier_percentage_setting =
      e_damage_modifier_percentage_setting.unchanged;
    this.m_melee_damage_modifier_percentage_setting =
      e_damage_modifier_percentage_setting.unchanged;
    this.m_initial_grenade_count_setting = e_grenade_count_setting.unchanged;
    this.m_infinite_ammo_setting = e_infinite_ammo_setting.unchanged;
    this.m_recharging_grenades_setting = e_boolean_trait.unchanged;
    this.m_weapon_pickup_setting = e_boolean_trait.unchanged;
    this.m_equipment_usage_setting = e_equipment_usage_setting.unchanged;
    this.m_equipment_drop_on_death_setting = e_boolean_trait.unchanged;
    this.m_infinite_equipment_setting = e_boolean_trait.unchanged;
    this.m_initial_primary_weapon_absolute_index = -3;
    this.m_initial_secondary_weapon_absolute_index = -3;
    this.m_initial_equipment_absolute_index = -3;
  }
}
export class c_player_trait_movement {
  @AutoMap(() => e_player_speed_setting)
  m_speed_setting: e_player_speed_setting = e_player_speed_setting.unchanged;
  @AutoMap(() => e_player_gravity_setting)
  m_gravity_setting: e_player_gravity_setting =
    e_player_gravity_setting.unchanged;
  @AutoMap(() => e_vehicle_usage_setting)
  m_vehicle_usage_setting: e_vehicle_usage_setting =
    e_vehicle_usage_setting.unchanged;
  @AutoMap(() => e_double_jump_setting)
  m_double_jump_setting: e_double_jump_setting =
    e_double_jump_setting.unchanged;
  @AutoMap(() => Number)
  m_jump_modifier = -1;
  clear(): void {
    this.m_speed_setting = e_player_speed_setting.unchanged;
    this.m_gravity_setting = e_player_gravity_setting.unchanged;
    this.m_vehicle_usage_setting = e_vehicle_usage_setting.unchanged;
    this.m_double_jump_setting = e_double_jump_setting.unchanged;
    this.m_jump_modifier = -1;
  }
}
export class c_player_trait_appearance {
  @AutoMap(() => e_active_camo_setting)
  m_active_camo_setting: e_active_camo_setting = e_active_camo_setting.off;
  @AutoMap(() => e_waypoint_setting)
  m_waypoint_setting: e_waypoint_setting = e_waypoint_setting.unchanged;
  @AutoMap(() => e_waypoint_setting)
  m_gamertag_setting: e_waypoint_setting = e_waypoint_setting.unchanged;
  @AutoMap(() => e_aura_setting)
  m_aura_setting: e_aura_setting = e_aura_setting.unchanged;
  @AutoMap(() => e_forced_change_color_setting)
  m_forced_change_color_setting: e_forced_change_color_setting =
    e_forced_change_color_setting.unchanged;
  clear(): void {
    this.m_active_camo_setting = e_active_camo_setting.off;
    this.m_waypoint_setting = e_waypoint_setting.unchanged;
    this.m_gamertag_setting = e_waypoint_setting.unchanged;
    this.m_aura_setting = e_aura_setting.unchanged;
    this.m_forced_change_color_setting =
      e_forced_change_color_setting.unchanged;
  }
}
export class c_player_trait_sensors {
  @AutoMap(() => e_motion_tracker_setting)
  m_motion_tracker_setting: e_motion_tracker_setting =
    e_motion_tracker_setting.unchanged;
  @AutoMap(() => e_motion_tracker_range_setting)
  m_motion_tracker_range_setting: e_motion_tracker_range_setting =
    e_motion_tracker_range_setting.unchanged;
  @AutoMap(() => e_boolean_trait)
  m_directional_damage_setting: e_boolean_trait = e_boolean_trait.unchanged;
  clear(): void {
    this.m_motion_tracker_setting = e_motion_tracker_setting.unchanged;
    this.m_motion_tracker_range_setting =
      e_motion_tracker_range_setting.unchanged;
    this.m_directional_damage_setting = e_boolean_trait.unchanged;
  }
}
export class c_player_traits {
  @AutoMap(() => c_player_trait_shield_vitality)
  m_shield_vitality_traits = new c_player_trait_shield_vitality();
  @AutoMap(() => c_player_trait_weapons)
  m_weapon_traits = new c_player_trait_weapons();
  @AutoMap(() => c_player_trait_movement)
  m_movement_traits = new c_player_trait_movement();
  @AutoMap(() => c_player_trait_appearance)
  m_appearance_traits = new c_player_trait_appearance();
  @AutoMap(() => c_player_trait_sensors)
  m_sensor_traits = new c_player_trait_sensors();
  clear(): void {
    this.m_shield_vitality_traits.clear();
    this.m_weapon_traits.clear();
    this.m_movement_traits.clear();
    this.m_appearance_traits.clear();
    this.m_sensor_traits.clear();
  }
  decode(bitstream: c_bitstream_reader): void {
    this.m_shield_vitality_traits.m_damage_resistance_percentage_setting =
      bitstream.read_enum(
        "damage-resistance",
        4,
        e_damage_resistance_percentage_setting
      );
    this.m_shield_vitality_traits.m_body_multiplier = bitstream.read_enum(
      "body-multiplier",
      3,
      e_body_multiplier_setting
    );
    this.m_shield_vitality_traits.m_body_recharge_rate = bitstream.read_enum(
      "body-recharge-rate",
      4,
      e_recharge_rate_percentage_setting
    );
    this.m_shield_vitality_traits.m_shield_multiplier = bitstream.read_enum(
      "shield-multiplier",
      3,
      e_shield_multiplier_setting
    );
    this.m_shield_vitality_traits.m_shield_recharge_rate = bitstream.read_enum(
      "shield-recharge-rate",
      4,
      e_recharge_rate_percentage_setting
    );
    this.m_shield_vitality_traits.m_overshield_recharge_rate =
      bitstream.read_enum(
        "overshield-recharge-rate",
        4,
        e_recharge_rate_percentage_setting
      );
    this.m_shield_vitality_traits.m_headshot_immunity_setting =
      bitstream.read_enum("headshot-immunity", 2, e_boolean_trait);
    this.m_shield_vitality_traits.m_vampirism_percentage_setting =
      bitstream.read_enum("vampirism", 3, e_vampirism_percentage_setting);
    this.m_shield_vitality_traits.m_assasination_immunity = bitstream.read_enum(
      "assasination-immunity",
      2,
      e_boolean_trait
    );
    this.m_shield_vitality_traits.m_cannot_die_from_damage =
      bitstream.read_enum("cannot-die-from-damage", 2, e_boolean_trait);
    this.m_weapon_traits.m_damage_modifier_percentage_setting =
      bitstream.read_enum(
        "damage-modifier",
        4,
        e_damage_modifier_percentage_setting
      );
    this.m_weapon_traits.m_melee_damage_modifier_percentage_setting =
      bitstream.read_enum(
        "melee-damage-modifier",
        4,
        e_damage_modifier_percentage_setting
      );
    this.m_weapon_traits.m_initial_primary_weapon_absolute_index =
      bitstream.read_signed_integer("player-trait-initial-primary-weapon", 8);
    this.m_weapon_traits.m_initial_secondary_weapon_absolute_index =
      bitstream.read_signed_integer("player-trait-initial-secondary-weapon", 8);
    this.m_weapon_traits.m_initial_grenade_count_setting = bitstream.read_enum(
      "player-trait-initial-grenade-count",
      4,
      e_grenade_count_setting
    );
    this.m_weapon_traits.m_infinite_ammo_setting = bitstream.read_enum(
      "player-traits-infinite-ammo-setting",
      2,
      e_infinite_ammo_setting
    );
    this.m_weapon_traits.m_recharging_grenades_setting = bitstream.read_enum(
      "player-traits-recharging-grenades",
      2,
      e_boolean_trait
    );
    this.m_weapon_traits.m_weapon_pickup_setting = bitstream.read_enum(
      "player-traits-weapon-pickup-allowed",
      2,
      e_boolean_trait
    );
    this.m_weapon_traits.m_equipment_usage_setting = bitstream.read_enum(
      "player-traits-equipment-usage",
      2,
      e_equipment_usage_setting
    );
    this.m_weapon_traits.m_equipment_drop_on_death_setting =
      bitstream.read_enum("player-traits-equipment-drop", 2, e_boolean_trait);
    this.m_weapon_traits.m_infinite_equipment_setting = bitstream.read_enum(
      "player-traits-infinite-equipment",
      2,
      e_boolean_trait
    );
    this.m_weapon_traits.m_initial_equipment_absolute_index =
      bitstream.read_signed_integer("player-trait-initial-equipment", 8);
    this.m_movement_traits.m_speed_setting = bitstream.read_enum(
      "player-speed",
      5,
      e_player_speed_setting
    );
    this.m_movement_traits.m_gravity_setting = bitstream.read_enum(
      "player-gravity",
      4,
      e_player_gravity_setting
    );
    this.m_movement_traits.m_vehicle_usage_setting = bitstream.read_enum(
      "player-traits-movement-vehicle-usage",
      4,
      e_vehicle_usage_setting
    );
    this.m_movement_traits.m_double_jump_setting = bitstream.read_enum(
      "player-traits-movement-double-jump",
      2,
      e_double_jump_setting
    );
    if (bitstream.read_bool("player-traits-movement-jump-modifier-changed")) {
      this.m_movement_traits.m_jump_modifier = bitstream.read_integer(
        "player-traits-movement-jump-modifier",
        9
      );
    } else {
      this.m_movement_traits.m_jump_modifier = -1;
    }
    this.m_appearance_traits.m_active_camo_setting = bitstream.read_enum(
      "player-traits-appearance-active-camo",
      3,
      e_active_camo_setting
    );
    this.m_appearance_traits.m_waypoint_setting = bitstream.read_enum(
      "player-traits-appearance-waypoint",
      2,
      e_waypoint_setting
    );
    this.m_appearance_traits.m_gamertag_setting = bitstream.read_enum(
      "player-traits-appearance-gamertag",
      2,
      e_waypoint_setting
    );
    this.m_appearance_traits.m_aura_setting = bitstream.read_enum(
      "player-traits-appearance-aura",
      3,
      e_aura_setting
    );
    this.m_appearance_traits.m_forced_change_color_setting =
      bitstream.read_enum(
        "player-traits-appearance-forced-change-color",
        4,
        e_forced_change_color_setting
      );
    this.m_sensor_traits.m_motion_tracker_setting = bitstream.read_enum(
      "player-traits-sensors-motion-tracker",
      3,
      e_motion_tracker_setting
    );
    this.m_sensor_traits.m_motion_tracker_range_setting = bitstream.read_enum(
      "motion-tracker-range",
      3,
      e_motion_tracker_range_setting
    );
    this.m_sensor_traits.m_directional_damage_setting = bitstream.read_enum(
      "player-traits-sensors-directional-damage",
      2,
      e_boolean_trait
    );
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_enum(
      this.m_shield_vitality_traits.m_damage_resistance_percentage_setting,
      4,
      e_damage_resistance_percentage_setting
    );
    bitstream.write_enum(
      this.m_shield_vitality_traits.m_body_multiplier,
      3,
      e_body_multiplier_setting
    );
    bitstream.write_enum(
      this.m_shield_vitality_traits.m_body_recharge_rate,
      4,
      e_recharge_rate_percentage_setting
    );
    bitstream.write_enum(
      this.m_shield_vitality_traits.m_shield_multiplier,
      3,
      e_shield_multiplier_setting
    );
    bitstream.write_enum(
      this.m_shield_vitality_traits.m_shield_recharge_rate,
      4,
      e_recharge_rate_percentage_setting
    );
    bitstream.write_enum(
      this.m_shield_vitality_traits.m_overshield_recharge_rate,
      4,
      e_recharge_rate_percentage_setting
    );
    bitstream.write_enum(
      this.m_shield_vitality_traits.m_headshot_immunity_setting,
      2,
      e_boolean_trait
    );
    bitstream.write_enum(
      this.m_shield_vitality_traits.m_vampirism_percentage_setting,
      3,
      e_vampirism_percentage_setting
    );
    bitstream.write_enum(
      this.m_shield_vitality_traits.m_assasination_immunity,
      2,
      e_boolean_trait
    );
    bitstream.write_enum(
      this.m_shield_vitality_traits.m_cannot_die_from_damage,
      2,
      e_boolean_trait
    );
    bitstream.write_enum(
      this.m_weapon_traits.m_damage_modifier_percentage_setting,
      4,
      e_damage_modifier_percentage_setting
    );
    bitstream.write_enum(
      this.m_weapon_traits.m_melee_damage_modifier_percentage_setting,
      4,
      e_damage_modifier_percentage_setting
    );
    bitstream.write_signed_integer(
      this.m_weapon_traits.m_initial_primary_weapon_absolute_index,
      8
    );
    bitstream.write_signed_integer(
      this.m_weapon_traits.m_initial_secondary_weapon_absolute_index,
      8
    );
    bitstream.write_enum(
      this.m_weapon_traits.m_initial_grenade_count_setting,
      4,
      e_grenade_count_setting
    );
    bitstream.write_enum(
      this.m_weapon_traits.m_infinite_ammo_setting,
      2,
      e_infinite_ammo_setting
    );
    bitstream.write_enum(
      this.m_weapon_traits.m_recharging_grenades_setting,
      2,
      e_boolean_trait
    );
    bitstream.write_enum(
      this.m_weapon_traits.m_weapon_pickup_setting,
      2,
      e_boolean_trait
    );
    bitstream.write_enum(
      this.m_weapon_traits.m_equipment_usage_setting,
      2,
      e_equipment_usage_setting
    );
    bitstream.write_enum(
      this.m_weapon_traits.m_equipment_drop_on_death_setting,
      2,
      e_boolean_trait
    );
    bitstream.write_enum(
      this.m_weapon_traits.m_infinite_equipment_setting,
      2,
      e_boolean_trait
    );
    bitstream.write_signed_integer(
      this.m_weapon_traits.m_initial_equipment_absolute_index,
      8
    );
    bitstream.write_enum(
      this.m_movement_traits.m_speed_setting,
      5,
      e_player_speed_setting
    );
    bitstream.write_enum(
      this.m_movement_traits.m_gravity_setting,
      4,
      e_player_gravity_setting
    );
    bitstream.write_enum(
      this.m_movement_traits.m_vehicle_usage_setting,
      4,
      e_vehicle_usage_setting
    );
    bitstream.write_enum(
      this.m_movement_traits.m_double_jump_setting,
      2,
      e_double_jump_setting
    );
    if (this.m_movement_traits.m_jump_modifier === -1) {
      bitstream.write_bool(false);
    } else {
      bitstream.write_bool(true);
      bitstream.write_integer(this.m_movement_traits.m_jump_modifier, 9);
    }
    bitstream.write_enum(
      this.m_appearance_traits.m_active_camo_setting,
      3,
      e_active_camo_setting
    );
    bitstream.write_enum(
      this.m_appearance_traits.m_waypoint_setting,
      2,
      e_waypoint_setting
    );
    bitstream.write_enum(
      this.m_appearance_traits.m_gamertag_setting,
      2,
      e_waypoint_setting
    );
    bitstream.write_enum(
      this.m_appearance_traits.m_aura_setting,
      3,
      e_aura_setting
    );
    bitstream.write_enum(
      this.m_appearance_traits.m_forced_change_color_setting,
      4,
      e_forced_change_color_setting
    );
    bitstream.write_enum(
      this.m_sensor_traits.m_motion_tracker_setting,
      3,
      e_motion_tracker_setting
    );
    bitstream.write_enum(
      this.m_sensor_traits.m_motion_tracker_range_setting,
      3,
      e_motion_tracker_range_setting
    );
    bitstream.write_enum(
      this.m_sensor_traits.m_directional_damage_setting,
      2,
      e_boolean_trait
    );
  }
}
