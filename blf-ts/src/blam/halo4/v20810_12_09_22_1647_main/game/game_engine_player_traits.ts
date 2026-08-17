import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../bitstream";
import { AutoMap } from "../../../../helpers/automap";

/** Quantized trait floats: bool presence + 16-bit real in [-200, 200], exact mid/endpoints. */
const k_trait_float_bits = 16;
const k_trait_float_min = -200;
const k_trait_float_max = 200;

export class c_player_trait_float {
  @AutoMap(() => Boolean)
  m_enabled = false;
  @AutoMap(() => Number)
  m_value = 0;
  clear(): void {
    this.m_enabled = false;
    this.m_value = 0;
  }
  decode(bitstream: c_bitstream_reader, name: string): void {
    this.m_enabled = bitstream.read_bool(name);
    if (this.m_enabled) {
      this.m_value = bitstream.read_quantized_real(
        k_trait_float_min,
        k_trait_float_max,
        k_trait_float_bits,
        true,
        true
      );
    } else {
      this.m_value = 0;
    }
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_bool(this.m_enabled);
    if (this.m_enabled) {
      bitstream.write_quantized_real(
        this.m_value,
        k_trait_float_min,
        k_trait_float_max,
        k_trait_float_bits,
        true,
        true
      );
    }
  }
}

/** `e_bool_player_trait_choices` (0..2), 2 bits. */
export enum e_bool_player_trait {
  unchanged = 0,
  off = 1,
  on = 2,
}

/** `e_player_trait_vitality_powerup_cancellation` (0..2), 2 bits. */
export enum e_player_trait_vitality_powerup_cancellation {
  unchanged = 0,
  unknown_1 = 1,
  unknown_2 = 2,
}

/** `e_player_trait_weapons_initial_grenade_count_choices` (0..18), 5 bits. H4: `N_frag` / `N_typeK`. */
export enum e_grenade_count_setting {
  unchanged = 0,
  map_default = 1,
  none = 2,
  frag_1 = 3,
  frag_2 = 4,
  plasma_1 = 5,
  plasma_2 = 6,
  type2_1 = 7,
  type2_2 = 8,
  type3_1 = 9,
  type3_2 = 10,
  type4_1 = 11,
  type4_2 = 12,
  type5_1 = 13,
  type5_2 = 14,
  type6_1 = 15,
  type6_2 = 16,
  type7_1 = 17,
  type7_2 = 18,
}

/** `e_player_trait_weapons_infinite_ammo_choices` (0..3), 2 bits. */
export enum e_infinite_ammo_setting {
  unchanged = 0,
  disabled = 1,
  enabled = 2,
  bottomless_clip = 3,
}

/** `e_player_trait_weapons_equipment_usage_choices` (0..3), 2 bits. */
export enum e_equipment_usage_setting {
  unchanged = 0,
  off = 1,
  not_with_objectives = 2,
  on = 3,
}

/** `e_player_trait_movement_vehicle_usage_choices` (0..8), 4 bits. */
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

/** `e_player_trait_movement_double_jump_choices` (0..3), 2 bits. */
export enum e_double_jump_setting {
  unchanged = 0,
  off = 1,
  on = 2,
  triple = 3,
}

/** `e_player_trait_appearance_active_camo_choices` (0..5), 3 bits. */
export enum e_active_camo_setting {
  off = 0,
  on = 1,
  poor = 2,
  good = 3,
  excellent = 4,
  invisible = 5,
}

/** `e_waypoint_setting` (0..3), 2 bits. */
export enum e_waypoint_setting {
  unchanged = 0,
  off = 1,
  allies = 2,
  all = 3,
}

/** `e_player_trait_appearance_aura_choices` (0..4), 3 bits. */
export enum e_aura_setting {
  unchanged = 0,
  off = 1,
  team_color = 2,
  black = 3,
  white = 4,
}

/** `e_player_trait_sensors_motion_tracker_choices` (0..4), 3 bits. */
export enum e_motion_tracker_setting {
  unchanged = 0,
  off = 1,
  allies = 2,
  normal = 3,
  enhanced = 4,
}

/** Back-compat aliases used by loadout / Reach-shaped call sites. */
export enum e_boolean_trait {
  unchanged = 0,
  off = 1,
  on = 2,
}

export class PlayerTraitChangeColor {
  @AutoMap(() => Boolean)
  m_override = false;
  @AutoMap(() => Number)
  m_red = 0;
  @AutoMap(() => Number)
  m_green = 0;
  @AutoMap(() => Number)
  m_blue = 0;
  clear(): void {
    this.m_override = false;
    this.m_red = 0;
    this.m_green = 0;
    this.m_blue = 0;
  }
  decode(bitstream: c_bitstream_reader): void {
    this.m_override = bitstream.read_bool("flags");
    this.m_red = bitstream.read_integer("red", 8);
    this.m_green = bitstream.read_integer("green", 8);
    this.m_blue = bitstream.read_integer("blue", 8);
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_bool(this.m_override);
    bitstream.write_integer(this.m_red, 8);
    bitstream.write_integer(this.m_green, 8);
    bitstream.write_integer(this.m_blue, 8);
  }
}

export class PlayerTraitModelVariant {
  @AutoMap(() => Boolean)
  m_override = false;
  @AutoMap(() => Number)
  m_model = 0;
  clear(): void {
    this.m_override = false;
    this.m_model = 0;
  }
  decode(bitstream: c_bitstream_reader): void {
    this.m_override = bitstream.read_bool("flags");
    this.m_model = bitstream.read_integer("model", 8);
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_bool(this.m_override);
    bitstream.write_integer(this.m_model, 8);
  }
}

export class c_player_trait_shield_vitality {
  @AutoMap(() => c_player_trait_float)
  m_damage_resistance = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_shield_multiplier = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_body_multiplier = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_shield_stun_duration = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_shield_recharge_rate = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_body_recharge_rate = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_overshield_recharge_rate = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_vampirism_percent = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_explosive_damage_resistance = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_wheelman_vehicle_stun_time = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_wheelman_vehicle_recharge_time = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_wheelman_vehicle_emp_disabled_time = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_fall_damage_multiplier = new c_player_trait_float();
  @AutoMap(() => Number)
  m_headshot_immunity: e_bool_player_trait = e_bool_player_trait.unchanged;
  @AutoMap(() => Number)
  m_assassination_immunity: e_bool_player_trait = e_bool_player_trait.unchanged;
  @AutoMap(() => Number)
  m_deathless: e_bool_player_trait = e_bool_player_trait.unchanged;
  @AutoMap(() => Number)
  m_fast_track_armor: e_bool_player_trait = e_bool_player_trait.unchanged;
  @AutoMap(() => Number)
  m_powerup_cancellation: e_player_trait_vitality_powerup_cancellation =
    e_player_trait_vitality_powerup_cancellation.unchanged;
  clear(): void {
    this.m_damage_resistance.clear();
    this.m_shield_multiplier.clear();
    this.m_body_multiplier.clear();
    this.m_shield_stun_duration.clear();
    this.m_shield_recharge_rate.clear();
    this.m_body_recharge_rate.clear();
    this.m_overshield_recharge_rate.clear();
    this.m_vampirism_percent.clear();
    this.m_explosive_damage_resistance.clear();
    this.m_wheelman_vehicle_stun_time.clear();
    this.m_wheelman_vehicle_recharge_time.clear();
    this.m_wheelman_vehicle_emp_disabled_time.clear();
    this.m_fall_damage_multiplier.clear();
    this.m_headshot_immunity = e_bool_player_trait.unchanged;
    this.m_assassination_immunity = e_bool_player_trait.unchanged;
    this.m_deathless = e_bool_player_trait.unchanged;
    this.m_fast_track_armor = e_bool_player_trait.unchanged;
    this.m_powerup_cancellation =
      e_player_trait_vitality_powerup_cancellation.unchanged;
  }
}

export class c_player_trait_weapons {
  @AutoMap(() => c_player_trait_float)
  m_damage_multiplier = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_melee_damage_multiplier = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_grenade_recharge_frag = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_grenade_recharge_plasma = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_grenade_recharge_spike = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_hero_equipment_energy_use_rate = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_hero_equipment_energy_recharge_delay = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_hero_equipment_energy_recharge_rate = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_hero_equipment_initial_energy = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_equipment_energy_use_rate = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_equipment_energy_recharge_delay = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_equipment_energy_recharge_rate = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_equipment_energy_initial_energy = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_switch_speed_modifier = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_reload_speed_modifier = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_ordnance_points_modifier = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_explosive_aoe_radius_modifier = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_gunner_armor_modifier = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_stability_armor_modifier = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_drop_recon_warning_seconds = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_drop_recon_distance_modifier = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_assassination_speed_modifier = new c_player_trait_float();
  @AutoMap(() => Number)
  m_weapon_pickup_allowed: e_bool_player_trait = e_bool_player_trait.unchanged;
  @AutoMap(() => Number)
  m_initial_grenade_count_setting: e_grenade_count_setting =
    e_grenade_count_setting.unchanged;
  @AutoMap(() => Number)
  m_infinite_ammo_setting: e_infinite_ammo_setting =
    e_infinite_ammo_setting.unchanged;
  @AutoMap(() => Number)
  m_equipment_usage_setting: e_equipment_usage_setting =
    e_equipment_usage_setting.unchanged;
  @AutoMap(() => Number)
  m_equipment_usage_except_auto_turret: e_equipment_usage_setting =
    e_equipment_usage_setting.unchanged;
  @AutoMap(() => Number)
  m_equipment_drop: e_bool_player_trait = e_bool_player_trait.unchanged;
  @AutoMap(() => Number)
  m_infinite_equipment: e_bool_player_trait = e_bool_player_trait.unchanged;
  @AutoMap(() => Number)
  m_ammopack: e_bool_player_trait = e_bool_player_trait.unchanged;
  @AutoMap(() => Number)
  m_grenadier: e_bool_player_trait = e_bool_player_trait.unchanged;
  @AutoMap(() => Number)
  m_explode_on_death_armormod: e_bool_player_trait =
    e_bool_player_trait.unchanged;
  @AutoMap(() => Number)
  m_ordnance_markers_visible: e_bool_player_trait =
    e_bool_player_trait.unchanged;
  @AutoMap(() => Number)
  m_ordnance_reroll_available: e_bool_player_trait =
    e_bool_player_trait.unchanged;
  @AutoMap(() => Number)
  m_resourceful: e_bool_player_trait = e_bool_player_trait.unchanged;
  @AutoMap(() => Number)
  m_well_equipped: e_bool_player_trait = e_bool_player_trait.unchanged;
  @AutoMap(() => Number)
  m_ordnance_disabled: e_bool_player_trait = e_bool_player_trait.unchanged;
  @AutoMap(() => Number)
  m_initial_primary_weapon_absolute_index = -3;
  @AutoMap(() => Number)
  m_initial_secondary_weapon_absolute_index = -3;
  @AutoMap(() => Number)
  m_initial_equipment_absolute_index = -3;
  @AutoMap(() => Number)
  m_initial_tactical_package_absolute_index = -3;
  @AutoMap(() => Number)
  m_initial_support_upgrade_absolute_index = -3;
  clear(): void {
    this.m_damage_multiplier.clear();
    this.m_melee_damage_multiplier.clear();
    this.m_grenade_recharge_frag.clear();
    this.m_grenade_recharge_plasma.clear();
    this.m_grenade_recharge_spike.clear();
    this.m_hero_equipment_energy_use_rate.clear();
    this.m_hero_equipment_energy_recharge_delay.clear();
    this.m_hero_equipment_energy_recharge_rate.clear();
    this.m_hero_equipment_initial_energy.clear();
    this.m_equipment_energy_use_rate.clear();
    this.m_equipment_energy_recharge_delay.clear();
    this.m_equipment_energy_recharge_rate.clear();
    this.m_equipment_energy_initial_energy.clear();
    this.m_switch_speed_modifier.clear();
    this.m_reload_speed_modifier.clear();
    this.m_ordnance_points_modifier.clear();
    this.m_explosive_aoe_radius_modifier.clear();
    this.m_gunner_armor_modifier.clear();
    this.m_stability_armor_modifier.clear();
    this.m_drop_recon_warning_seconds.clear();
    this.m_drop_recon_distance_modifier.clear();
    this.m_assassination_speed_modifier.clear();
    this.m_weapon_pickup_allowed = e_bool_player_trait.unchanged;
    this.m_initial_grenade_count_setting = e_grenade_count_setting.unchanged;
    this.m_infinite_ammo_setting = e_infinite_ammo_setting.unchanged;
    this.m_equipment_usage_setting = e_equipment_usage_setting.unchanged;
    this.m_equipment_usage_except_auto_turret =
      e_equipment_usage_setting.unchanged;
    this.m_equipment_drop = e_bool_player_trait.unchanged;
    this.m_infinite_equipment = e_bool_player_trait.unchanged;
    this.m_ammopack = e_bool_player_trait.unchanged;
    this.m_grenadier = e_bool_player_trait.unchanged;
    this.m_explode_on_death_armormod = e_bool_player_trait.unchanged;
    this.m_ordnance_markers_visible = e_bool_player_trait.unchanged;
    this.m_ordnance_reroll_available = e_bool_player_trait.unchanged;
    this.m_resourceful = e_bool_player_trait.unchanged;
    this.m_well_equipped = e_bool_player_trait.unchanged;
    this.m_ordnance_disabled = e_bool_player_trait.unchanged;
    this.m_initial_primary_weapon_absolute_index = -3;
    this.m_initial_secondary_weapon_absolute_index = -3;
    this.m_initial_equipment_absolute_index = -3;
    this.m_initial_tactical_package_absolute_index = -3;
    this.m_initial_support_upgrade_absolute_index = -3;
  }
}

export class c_player_trait_movement {
  @AutoMap(() => c_player_trait_float)
  m_speed = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_gravity = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_jump_multiplier = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_turn_speed_multiplier = new c_player_trait_float();
  @AutoMap(() => Number)
  m_vehicle_usage_setting: e_vehicle_usage_setting =
    e_vehicle_usage_setting.unchanged;
  @AutoMap(() => Number)
  m_double_jump_setting: e_double_jump_setting =
    e_double_jump_setting.unchanged;
  @AutoMap(() => Number)
  m_sprint_usage: e_bool_player_trait = e_bool_player_trait.unchanged;
  @AutoMap(() => Number)
  m_automatic_momentum_usage: e_bool_player_trait =
    e_bool_player_trait.unchanged;
  @AutoMap(() => Number)
  m_vaulting_enabled: e_bool_player_trait = e_bool_player_trait.unchanged;
  @AutoMap(() => Number)
  m_stealthy: e_bool_player_trait = e_bool_player_trait.unchanged;
  clear(): void {
    this.m_speed.clear();
    this.m_gravity.clear();
    this.m_jump_multiplier.clear();
    this.m_turn_speed_multiplier.clear();
    this.m_vehicle_usage_setting = e_vehicle_usage_setting.unchanged;
    this.m_double_jump_setting = e_double_jump_setting.unchanged;
    this.m_sprint_usage = e_bool_player_trait.unchanged;
    this.m_automatic_momentum_usage = e_bool_player_trait.unchanged;
    this.m_vaulting_enabled = e_bool_player_trait.unchanged;
    this.m_stealthy = e_bool_player_trait.unchanged;
  }
}

export class c_player_trait_appearance {
  @AutoMap(() => c_player_trait_float)
  m_player_scale = new c_player_trait_float();
  @AutoMap(() => Number)
  m_active_camo_setting: e_active_camo_setting = e_active_camo_setting.off;
  @AutoMap(() => Number)
  m_waypoint_setting: e_waypoint_setting = e_waypoint_setting.unchanged;
  @AutoMap(() => Number)
  m_gamertag_setting: e_waypoint_setting = e_waypoint_setting.unchanged;
  @AutoMap(() => Number)
  m_aura_setting: e_aura_setting = e_aura_setting.unchanged;
  @AutoMap(() => PlayerTraitChangeColor)
  m_primary_color = new PlayerTraitChangeColor();
  @AutoMap(() => PlayerTraitChangeColor)
  m_secondary_color = new PlayerTraitChangeColor();
  @AutoMap(() => PlayerTraitModelVariant)
  m_model_variant = new PlayerTraitModelVariant();
  @AutoMap(() => Number)
  m_death_effect = -1;
  @AutoMap(() => Number)
  m_looping_effect = -1;
  @AutoMap(() => Number)
  m_shield_hud: e_bool_player_trait = e_bool_player_trait.unchanged;
  clear(): void {
    this.m_player_scale.clear();
    this.m_active_camo_setting = e_active_camo_setting.off;
    this.m_waypoint_setting = e_waypoint_setting.unchanged;
    this.m_gamertag_setting = e_waypoint_setting.unchanged;
    this.m_aura_setting = e_aura_setting.unchanged;
    this.m_primary_color.clear();
    this.m_secondary_color.clear();
    this.m_model_variant.clear();
    this.m_death_effect = -1;
    this.m_looping_effect = -1;
    this.m_shield_hud = e_bool_player_trait.unchanged;
  }
}

export class c_player_trait_sensors {
  @AutoMap(() => c_player_trait_float)
  m_motion_tracker_range = new c_player_trait_float();
  @AutoMap(() => c_player_trait_float)
  m_nemesis_duration = new c_player_trait_float();
  @AutoMap(() => Number)
  m_motion_tracker_setting: e_motion_tracker_setting =
    e_motion_tracker_setting.unchanged;
  @AutoMap(() => Number)
  m_motion_tracker_while_zoomed: e_bool_player_trait =
    e_bool_player_trait.unchanged;
  @AutoMap(() => Number)
  m_directional_damage_indicator: e_bool_player_trait =
    e_bool_player_trait.unchanged;
  @AutoMap(() => Number)
  m_vision_mode: e_bool_player_trait = e_bool_player_trait.unchanged;
  @AutoMap(() => Number)
  m_battle_awareness: e_bool_player_trait = e_bool_player_trait.unchanged;
  @AutoMap(() => Number)
  m_threat_view: e_bool_player_trait = e_bool_player_trait.unchanged;
  @AutoMap(() => Number)
  m_aural_enhancement: e_bool_player_trait = e_bool_player_trait.unchanged;
  @AutoMap(() => Number)
  m_nemesis: e_bool_player_trait = e_bool_player_trait.unchanged;
  clear(): void {
    this.m_motion_tracker_range.clear();
    this.m_nemesis_duration.clear();
    this.m_motion_tracker_setting = e_motion_tracker_setting.unchanged;
    this.m_motion_tracker_while_zoomed = e_bool_player_trait.unchanged;
    this.m_directional_damage_indicator = e_bool_player_trait.unchanged;
    this.m_vision_mode = e_bool_player_trait.unchanged;
    this.m_battle_awareness = e_bool_player_trait.unchanged;
    this.m_threat_view = e_bool_player_trait.unchanged;
    this.m_aural_enhancement = e_bool_player_trait.unchanged;
    this.m_nemesis = e_bool_player_trait.unchanged;
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
    const v = this.m_shield_vitality_traits;
    v.m_damage_resistance.decode(
      bitstream,
      "has-player-trait-damage-resistance"
    );
    v.m_shield_multiplier.decode(
      bitstream,
      "has-player-trait-shield-multiplier"
    );
    v.m_body_multiplier.decode(bitstream, "has-player-trait-body-multiplier");
    v.m_shield_stun_duration.decode(
      bitstream,
      "has-player-trait-shield-stun-duration"
    );
    v.m_shield_recharge_rate.decode(
      bitstream,
      "has-player-trait-shield-recharge-rate"
    );
    v.m_body_recharge_rate.decode(
      bitstream,
      "has-player-trait-body-recharge-rate"
    );
    v.m_overshield_recharge_rate.decode(
      bitstream,
      "has-player-trait-overshield-recharge-rate"
    );
    v.m_vampirism_percent.decode(
      bitstream,
      "has-player-trait-vampirism-percent"
    );
    v.m_explosive_damage_resistance.decode(
      bitstream,
      "has-player-trait-explosive-damage-resistance"
    );
    v.m_wheelman_vehicle_stun_time.decode(
      bitstream,
      "has-player-trait-wheelman-armor-vehicle-stun-time-modifier"
    );
    v.m_wheelman_vehicle_recharge_time.decode(
      bitstream,
      "has-player-trait-wheelman-armor-vehicle-recharge-time-modifier"
    );
    v.m_wheelman_vehicle_emp_disabled_time.decode(
      bitstream,
      "has-player-trait-wheelman-armor-vehicle-emp-disabled-time-modifier"
    );
    v.m_fall_damage_multiplier.decode(
      bitstream,
      "has-player-trait-fall-damage-multiplier"
    );
    v.m_headshot_immunity = bitstream.read_enum(
      "player-trait-headshot-immunity",
      2,
      e_bool_player_trait
    );
    v.m_assassination_immunity = bitstream.read_enum(
      "player-trait-assassination-immunity",
      2,
      e_bool_player_trait
    );
    v.m_deathless = bitstream.read_enum(
      "player-trait-deathless",
      2,
      e_bool_player_trait
    );
    v.m_fast_track_armor = bitstream.read_enum(
      "player-trait-fast-track-armor",
      2,
      e_bool_player_trait
    );
    v.m_powerup_cancellation = bitstream.read_enum(
      "player-trait-powerup-cancellation",
      2,
      e_player_trait_vitality_powerup_cancellation
    );

    const w = this.m_weapon_traits;
    w.m_damage_multiplier.decode(
      bitstream,
      "has-player-trait-damage-multiplier"
    );
    w.m_melee_damage_multiplier.decode(
      bitstream,
      "has-player-trait-melee-damage-multiplier"
    );
    w.m_grenade_recharge_frag.decode(
      bitstream,
      "has-player-trait-grenade-recharge-seconds-frag"
    );
    w.m_grenade_recharge_plasma.decode(
      bitstream,
      "has-player-trait-grenade-recharge-seconds-plasma"
    );
    w.m_grenade_recharge_spike.decode(
      bitstream,
      "has-player-trait-grenade-recharge-seconds-spike"
    );
    w.m_hero_equipment_energy_use_rate.decode(
      bitstream,
      "has-player-trait-hero-equipment-energy-use-rate-modifier"
    );
    w.m_hero_equipment_energy_recharge_delay.decode(
      bitstream,
      "has-player-trait-hero-equipment-energy-recharge-delay-modifier"
    );
    w.m_hero_equipment_energy_recharge_rate.decode(
      bitstream,
      "has-player-trait-hero-equipment-energy-recharge-rate-modifier"
    );
    w.m_hero_equipment_initial_energy.decode(
      bitstream,
      "has-player-trait-hero-equipment-initial-energy-modifier"
    );
    w.m_equipment_energy_use_rate.decode(
      bitstream,
      "has-player-trait-equipment-energy-use-rate-modifier"
    );
    w.m_equipment_energy_recharge_delay.decode(
      bitstream,
      "has-player-trait-equipment-energy-recharge-delay-modifier"
    );
    w.m_equipment_energy_recharge_rate.decode(
      bitstream,
      "has-player-trait-equipment-energy-use-recharge-rate-modifier"
    );
    w.m_equipment_energy_initial_energy.decode(
      bitstream,
      "has-player-trait-equipment-energy-initial-energy-modifier"
    );
    w.m_switch_speed_modifier.decode(
      bitstream,
      "has-player-trait-switch-speed-modifier"
    );
    w.m_reload_speed_modifier.decode(
      bitstream,
      "has-player-trait-reload-speed-modifier"
    );
    w.m_ordnance_points_modifier.decode(
      bitstream,
      "has-player-trait-ordnance-points-modifier"
    );
    w.m_explosive_aoe_radius_modifier.decode(
      bitstream,
      "has-player-trait-explosive-area-of-effect-radius-modifier"
    );
    w.m_gunner_armor_modifier.decode(
      bitstream,
      "has-player-trait-gunner-armor-modifier"
    );
    w.m_stability_armor_modifier.decode(
      bitstream,
      "has-player-trait-stability-armor-modifier"
    );
    w.m_drop_recon_warning_seconds.decode(
      bitstream,
      "has-player-trait-drop-recon-warning-seconds"
    );
    w.m_drop_recon_distance_modifier.decode(
      bitstream,
      "has-player-trait-drop-recon-distance-modifier"
    );
    w.m_assassination_speed_modifier.decode(
      bitstream,
      "has-player-trait-assassination-speed-modifier"
    );
    w.m_weapon_pickup_allowed = bitstream.read_enum(
      "player-trait-weapon-pickup-allowed",
      2,
      e_bool_player_trait
    );
    w.m_initial_grenade_count_setting = bitstream.read_enum(
      "player-trait-initial-grenade-count",
      5,
      e_grenade_count_setting
    );
    w.m_infinite_ammo_setting = bitstream.read_enum(
      "player-trait-infinite-ammo",
      2,
      e_infinite_ammo_setting
    );
    w.m_equipment_usage_setting = bitstream.read_enum(
      "player-trait-equipment-usage",
      2,
      e_equipment_usage_setting
    );
    w.m_equipment_usage_except_auto_turret = bitstream.read_enum(
      "player-trait-equipment-usage-excepting-auto-turret",
      2,
      e_equipment_usage_setting
    );
    w.m_equipment_drop = bitstream.read_enum(
      "player-trait-equipment-drop",
      2,
      e_bool_player_trait
    );
    w.m_infinite_equipment = bitstream.read_enum(
      "player-trait-infinite-equipment",
      2,
      e_bool_player_trait
    );
    w.m_ammopack = bitstream.read_enum(
      "player-trait-weapons-ammopack",
      2,
      e_bool_player_trait
    );
    w.m_grenadier = bitstream.read_enum(
      "player-trait-weapons-grenadier",
      2,
      e_bool_player_trait
    );
    w.m_explode_on_death_armormod = bitstream.read_enum(
      "player-trait-weapons-explode-on-death-armormod",
      2,
      e_bool_player_trait
    );
    w.m_ordnance_markers_visible = bitstream.read_enum(
      "player-trait-ordnance-markers-visible",
      2,
      e_bool_player_trait
    );
    w.m_ordnance_reroll_available = bitstream.read_enum(
      "player-trait-weapons-ordnance-reroll-available",
      2,
      e_bool_player_trait
    );
    w.m_resourceful = bitstream.read_enum(
      "player-trait-weapons-resourceful",
      2,
      e_bool_player_trait
    );
    w.m_well_equipped = bitstream.read_enum(
      "player-trait-weapons-well-equipped",
      2,
      e_bool_player_trait
    );
    w.m_ordnance_disabled = bitstream.read_enum(
      "player-trait-ordnance-disabled",
      2,
      e_bool_player_trait
    );
    w.m_initial_primary_weapon_absolute_index = bitstream.read_signed_integer(
      "player-trait-initial-primary-weapon",
      8
    );
    w.m_initial_secondary_weapon_absolute_index = bitstream.read_signed_integer(
      "player-trait-initial-secondary-weapon",
      8
    );
    w.m_initial_equipment_absolute_index = bitstream.read_signed_integer(
      "player-trait-initial-equipment",
      8
    );
    w.m_initial_tactical_package_absolute_index = bitstream.read_signed_integer(
      "player-trait-initial-tactical-package",
      8
    );
    w.m_initial_support_upgrade_absolute_index = bitstream.read_signed_integer(
      "player-trait-initial-support-upgrade",
      8
    );

    const m = this.m_movement_traits;
    m.m_speed.decode(bitstream, "has-player-trait-speed");
    m.m_gravity.decode(bitstream, "has-player-trait-gravity-multiplier");
    m.m_jump_multiplier.decode(bitstream, "has-player-trait-jump-multiplier");
    m.m_turn_speed_multiplier.decode(
      bitstream,
      "has-player-trait-turn-speed-multiplier"
    );
    m.m_vehicle_usage_setting = bitstream.read_enum(
      "player-trait-vehicle-usage",
      4,
      e_vehicle_usage_setting
    );
    m.m_double_jump_setting = bitstream.read_enum(
      "player-trait-double-jump",
      2,
      e_double_jump_setting
    );
    m.m_sprint_usage = bitstream.read_enum(
      "player-trait-sprint-usage",
      2,
      e_bool_player_trait
    );
    m.m_automatic_momentum_usage = bitstream.read_enum(
      "player-trait-automatic-momentum-usage",
      2,
      e_bool_player_trait
    );
    m.m_vaulting_enabled = bitstream.read_enum(
      "player-trait-vaulting-enabled",
      2,
      e_bool_player_trait
    );
    m.m_stealthy = bitstream.read_enum(
      "player-trait-stealthy",
      2,
      e_bool_player_trait
    );

    const a = this.m_appearance_traits;
    a.m_player_scale.decode(bitstream, "has-player-trait-player-scale");
    a.m_active_camo_setting = bitstream.read_enum(
      "player-trait-active-camo",
      3,
      e_active_camo_setting
    );
    a.m_waypoint_setting = bitstream.read_enum(
      "player-trait-waypoint",
      2,
      e_waypoint_setting
    );
    a.m_gamertag_setting = bitstream.read_enum(
      "player-trait-gamertag-visible",
      2,
      e_waypoint_setting
    );
    a.m_aura_setting = bitstream.read_enum(
      "player-trait-aura",
      3,
      e_aura_setting
    );
    a.m_primary_color.decode(bitstream);
    a.m_secondary_color.decode(bitstream);
    a.m_model_variant.decode(bitstream);
    a.m_death_effect = bitstream.read_signed_integer(
      "player-trait-death-effect",
      32
    );
    a.m_looping_effect = bitstream.read_signed_integer(
      "player-trait-looping-effect",
      32
    );
    a.m_shield_hud = bitstream.read_enum(
      "player-trait-shield-hud",
      2,
      e_bool_player_trait
    );

    const s = this.m_sensor_traits;
    s.m_motion_tracker_range.decode(bitstream, "has-motion-tracker-range");
    s.m_nemesis_duration.decode(bitstream, "has-nemesis-duration");
    s.m_motion_tracker_setting = bitstream.read_enum(
      "player-trait-motion-tracker",
      3,
      e_motion_tracker_setting
    );
    s.m_motion_tracker_while_zoomed = bitstream.read_enum(
      "player-trait-motion-tracker-while-zoomed",
      2,
      e_bool_player_trait
    );
    s.m_directional_damage_indicator = bitstream.read_enum(
      "player-trait-directional-damage-indicator",
      2,
      e_bool_player_trait
    );
    s.m_vision_mode = bitstream.read_enum(
      "player-trait-vision-mode",
      2,
      e_bool_player_trait
    );
    s.m_battle_awareness = bitstream.read_enum(
      "player-trait-battle-awareness",
      2,
      e_bool_player_trait
    );
    s.m_threat_view = bitstream.read_enum(
      "player-trait-threat-view",
      2,
      e_bool_player_trait
    );
    s.m_aural_enhancement = bitstream.read_enum(
      "player-trait-aural-enhancement",
      2,
      e_bool_player_trait
    );
    s.m_nemesis = bitstream.read_enum(
      "player-trait-nemesis",
      2,
      e_bool_player_trait
    );
  }
  encode(bitstream: c_bitstream_writer): void {
    const v = this.m_shield_vitality_traits;
    v.m_damage_resistance.encode(bitstream);
    v.m_shield_multiplier.encode(bitstream);
    v.m_body_multiplier.encode(bitstream);
    v.m_shield_stun_duration.encode(bitstream);
    v.m_shield_recharge_rate.encode(bitstream);
    v.m_body_recharge_rate.encode(bitstream);
    v.m_overshield_recharge_rate.encode(bitstream);
    v.m_vampirism_percent.encode(bitstream);
    v.m_explosive_damage_resistance.encode(bitstream);
    v.m_wheelman_vehicle_stun_time.encode(bitstream);
    v.m_wheelman_vehicle_recharge_time.encode(bitstream);
    v.m_wheelman_vehicle_emp_disabled_time.encode(bitstream);
    v.m_fall_damage_multiplier.encode(bitstream);
    bitstream.write_enum(v.m_headshot_immunity, 2, e_bool_player_trait);
    bitstream.write_enum(v.m_assassination_immunity, 2, e_bool_player_trait);
    bitstream.write_enum(v.m_deathless, 2, e_bool_player_trait);
    bitstream.write_enum(v.m_fast_track_armor, 2, e_bool_player_trait);
    bitstream.write_enum(
      v.m_powerup_cancellation,
      2,
      e_player_trait_vitality_powerup_cancellation
    );

    const w = this.m_weapon_traits;
    w.m_damage_multiplier.encode(bitstream);
    w.m_melee_damage_multiplier.encode(bitstream);
    w.m_grenade_recharge_frag.encode(bitstream);
    w.m_grenade_recharge_plasma.encode(bitstream);
    w.m_grenade_recharge_spike.encode(bitstream);
    w.m_hero_equipment_energy_use_rate.encode(bitstream);
    w.m_hero_equipment_energy_recharge_delay.encode(bitstream);
    w.m_hero_equipment_energy_recharge_rate.encode(bitstream);
    w.m_hero_equipment_initial_energy.encode(bitstream);
    w.m_equipment_energy_use_rate.encode(bitstream);
    w.m_equipment_energy_recharge_delay.encode(bitstream);
    w.m_equipment_energy_recharge_rate.encode(bitstream);
    w.m_equipment_energy_initial_energy.encode(bitstream);
    w.m_switch_speed_modifier.encode(bitstream);
    w.m_reload_speed_modifier.encode(bitstream);
    w.m_ordnance_points_modifier.encode(bitstream);
    w.m_explosive_aoe_radius_modifier.encode(bitstream);
    w.m_gunner_armor_modifier.encode(bitstream);
    w.m_stability_armor_modifier.encode(bitstream);
    w.m_drop_recon_warning_seconds.encode(bitstream);
    w.m_drop_recon_distance_modifier.encode(bitstream);
    w.m_assassination_speed_modifier.encode(bitstream);
    bitstream.write_enum(w.m_weapon_pickup_allowed, 2, e_bool_player_trait);
    bitstream.write_enum(
      w.m_initial_grenade_count_setting,
      5,
      e_grenade_count_setting
    );
    bitstream.write_enum(w.m_infinite_ammo_setting, 2, e_infinite_ammo_setting);
    bitstream.write_enum(
      w.m_equipment_usage_setting,
      2,
      e_equipment_usage_setting
    );
    bitstream.write_enum(
      w.m_equipment_usage_except_auto_turret,
      2,
      e_equipment_usage_setting
    );
    bitstream.write_enum(w.m_equipment_drop, 2, e_bool_player_trait);
    bitstream.write_enum(w.m_infinite_equipment, 2, e_bool_player_trait);
    bitstream.write_enum(w.m_ammopack, 2, e_bool_player_trait);
    bitstream.write_enum(w.m_grenadier, 2, e_bool_player_trait);
    bitstream.write_enum(w.m_explode_on_death_armormod, 2, e_bool_player_trait);
    bitstream.write_enum(w.m_ordnance_markers_visible, 2, e_bool_player_trait);
    bitstream.write_enum(w.m_ordnance_reroll_available, 2, e_bool_player_trait);
    bitstream.write_enum(w.m_resourceful, 2, e_bool_player_trait);
    bitstream.write_enum(w.m_well_equipped, 2, e_bool_player_trait);
    bitstream.write_enum(w.m_ordnance_disabled, 2, e_bool_player_trait);
    bitstream.write_signed_integer(
      w.m_initial_primary_weapon_absolute_index,
      8
    );
    bitstream.write_signed_integer(
      w.m_initial_secondary_weapon_absolute_index,
      8
    );
    bitstream.write_signed_integer(w.m_initial_equipment_absolute_index, 8);
    bitstream.write_signed_integer(
      w.m_initial_tactical_package_absolute_index,
      8
    );
    bitstream.write_signed_integer(
      w.m_initial_support_upgrade_absolute_index,
      8
    );

    const m = this.m_movement_traits;
    m.m_speed.encode(bitstream);
    m.m_gravity.encode(bitstream);
    m.m_jump_multiplier.encode(bitstream);
    m.m_turn_speed_multiplier.encode(bitstream);
    bitstream.write_enum(m.m_vehicle_usage_setting, 4, e_vehicle_usage_setting);
    bitstream.write_enum(m.m_double_jump_setting, 2, e_double_jump_setting);
    bitstream.write_enum(m.m_sprint_usage, 2, e_bool_player_trait);
    bitstream.write_enum(m.m_automatic_momentum_usage, 2, e_bool_player_trait);
    bitstream.write_enum(m.m_vaulting_enabled, 2, e_bool_player_trait);
    bitstream.write_enum(m.m_stealthy, 2, e_bool_player_trait);

    const a = this.m_appearance_traits;
    a.m_player_scale.encode(bitstream);
    bitstream.write_enum(a.m_active_camo_setting, 3, e_active_camo_setting);
    bitstream.write_enum(a.m_waypoint_setting, 2, e_waypoint_setting);
    bitstream.write_enum(a.m_gamertag_setting, 2, e_waypoint_setting);
    bitstream.write_enum(a.m_aura_setting, 3, e_aura_setting);
    a.m_primary_color.encode(bitstream);
    a.m_secondary_color.encode(bitstream);
    a.m_model_variant.encode(bitstream);
    bitstream.write_signed_integer(a.m_death_effect, 32);
    bitstream.write_signed_integer(a.m_looping_effect, 32);
    bitstream.write_enum(a.m_shield_hud, 2, e_bool_player_trait);

    const s = this.m_sensor_traits;
    s.m_motion_tracker_range.encode(bitstream);
    s.m_nemesis_duration.encode(bitstream);
    bitstream.write_enum(
      s.m_motion_tracker_setting,
      3,
      e_motion_tracker_setting
    );
    bitstream.write_enum(
      s.m_motion_tracker_while_zoomed,
      2,
      e_bool_player_trait
    );
    bitstream.write_enum(
      s.m_directional_damage_indicator,
      2,
      e_bool_player_trait
    );
    bitstream.write_enum(s.m_vision_mode, 2, e_bool_player_trait);
    bitstream.write_enum(s.m_battle_awareness, 2, e_bool_player_trait);
    bitstream.write_enum(s.m_threat_view, 2, e_bool_player_trait);
    bitstream.write_enum(s.m_aural_enhancement, 2, e_bool_player_trait);
    bitstream.write_enum(s.m_nemesis, 2, e_bool_player_trait);
  }
}
