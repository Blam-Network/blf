import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../bitstream";
import { AutoMap } from "../../../../helpers/automap";
import { e_active_camo_setting } from "../../v12065_11_08_24_1738_tu1actual/game/game_engine_player_traits";

/**
 * Omaha Delta/Beta player traits (matches blf_lib omaha_delta
 * `game_engine_player_traits.rs`).
 *
 * Shield block matches retail field order/widths but values are raw integers so
 * stock mglos with out-of-range option indices still decode. Weapons/movement
 * match Alpha (no weapon equipment_usage; sprint + equipment on movement;
 * Halo3-style quantized jump).
 */
export class c_player_trait_shield_vitality {
  @AutoMap(() => Number)
  m_damage_resistance_percentage_setting = 0;
  @AutoMap(() => Number)
  m_body_multiplier = 0;
  @AutoMap(() => Number)
  m_body_recharge_rate = 0;
  @AutoMap(() => Number)
  m_shield_multiplier = 0;
  @AutoMap(() => Number)
  m_shield_recharge_rate = 0;
  @AutoMap(() => Number)
  m_overshield_recharge_rate = 0;
  @AutoMap(() => Number)
  m_headshot_immunity_setting = 0;
  @AutoMap(() => Number)
  m_vampirism_percentage_setting = 0;
  @AutoMap(() => Number)
  m_assasination_immunity = 0;
  @AutoMap(() => Number)
  m_cannot_die_from_damage = 0;
  clear(): void {
    this.m_damage_resistance_percentage_setting = 0;
    this.m_body_multiplier = 0;
    this.m_body_recharge_rate = 0;
    this.m_shield_multiplier = 0;
    this.m_shield_recharge_rate = 0;
    this.m_overshield_recharge_rate = 0;
    this.m_headshot_immunity_setting = 0;
    this.m_vampirism_percentage_setting = 0;
    this.m_assasination_immunity = 0;
    this.m_cannot_die_from_damage = 0;
  }
}

export class c_player_trait_weapons {
  @AutoMap(() => Number)
  m_damage_modifier_percentage_setting = 0;
  @AutoMap(() => Number)
  m_melee_damage_modifier_percentage_setting = 0;
  @AutoMap(() => Number)
  m_initial_primary_weapon_absolute_index = -3;
  @AutoMap(() => Number)
  m_initial_secondary_weapon_absolute_index = -3;
  @AutoMap(() => Number)
  m_initial_grenade_count_setting = 0;
  @AutoMap(() => Number)
  m_infinite_ammo_setting = 0;
  @AutoMap(() => Number)
  m_recharging_grenades_setting = 0;
  @AutoMap(() => Number)
  m_weapon_pickup_setting = 0;
  @AutoMap(() => Number)
  m_equipment_drop_on_death_setting = 0;
  @AutoMap(() => Number)
  m_infinite_equipment_setting = 0;
  @AutoMap(() => Number)
  m_initial_equipment_absolute_index = -3;
  clear(): void {
    this.m_damage_modifier_percentage_setting = 0;
    this.m_melee_damage_modifier_percentage_setting = 0;
    this.m_initial_grenade_count_setting = 0;
    this.m_infinite_ammo_setting = 0;
    this.m_recharging_grenades_setting = 0;
    this.m_weapon_pickup_setting = 0;
    this.m_equipment_drop_on_death_setting = 0;
    this.m_infinite_equipment_setting = 0;
    this.m_initial_primary_weapon_absolute_index = -3;
    this.m_initial_secondary_weapon_absolute_index = -3;
    this.m_initial_equipment_absolute_index = -3;
  }
}

export class c_player_trait_movement {
  @AutoMap(() => Number)
  m_speed_setting = 0;
  @AutoMap(() => Number)
  m_gravity_setting = 0;
  @AutoMap(() => Number)
  m_vehicle_usage_setting = 0;
  @AutoMap(() => Number)
  m_double_jump_setting = 0;
  @AutoMap(() => Number)
  m_sprint_setting = 0;
  @AutoMap(() => Number)
  m_equipment_usage_setting = 0;
  @AutoMap(() => Number)
  m_jump_modifier = -1;
  clear(): void {
    this.m_speed_setting = 0;
    this.m_gravity_setting = 0;
    this.m_vehicle_usage_setting = 0;
    this.m_double_jump_setting = 0;
    this.m_sprint_setting = 0;
    this.m_equipment_usage_setting = 0;
    this.m_jump_modifier = -1;
  }
}

export class c_player_trait_appearance {
  @AutoMap(() => Number)
  m_active_camo_setting: e_active_camo_setting = e_active_camo_setting.off;
  @AutoMap(() => Number)
  m_waypoint_setting = 0;
  @AutoMap(() => Number)
  m_gamertag_setting = 0;
  @AutoMap(() => Number)
  m_aura_setting = 0;
  @AutoMap(() => Number)
  m_forced_change_color_setting = 0;
  clear(): void {
    this.m_active_camo_setting = e_active_camo_setting.off;
    this.m_waypoint_setting = 0;
    this.m_gamertag_setting = 0;
    this.m_aura_setting = 0;
    this.m_forced_change_color_setting = 0;
  }
}

export class c_player_trait_sensors {
  @AutoMap(() => Number)
  m_motion_tracker_setting = 0;
  @AutoMap(() => Number)
  m_motion_tracker_range_setting = 0;
  @AutoMap(() => Number)
  m_directional_damage_setting = 0;
  clear(): void {
    this.m_motion_tracker_setting = 0;
    this.m_motion_tracker_range_setting = 0;
    this.m_directional_damage_setting = 0;
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
      bitstream.read_integer("damage-resistance", 4);
    this.m_shield_vitality_traits.m_body_multiplier = bitstream.read_integer(
      "body-multiplier",
      3
    );
    this.m_shield_vitality_traits.m_body_recharge_rate = bitstream.read_integer(
      "body-recharge-rate",
      4
    );
    this.m_shield_vitality_traits.m_shield_multiplier = bitstream.read_integer(
      "shield-multiplier",
      3
    );
    this.m_shield_vitality_traits.m_shield_recharge_rate =
      bitstream.read_integer("shield-recharge-rate", 4);
    this.m_shield_vitality_traits.m_overshield_recharge_rate =
      bitstream.read_integer("overshield-recharge-rate", 4);
    this.m_shield_vitality_traits.m_headshot_immunity_setting =
      bitstream.read_integer("headshot-immunity", 2);
    this.m_shield_vitality_traits.m_vampirism_percentage_setting =
      bitstream.read_integer("vampirism", 3);
    this.m_shield_vitality_traits.m_assasination_immunity =
      bitstream.read_integer("assasination-immunity", 2);
    this.m_shield_vitality_traits.m_cannot_die_from_damage =
      bitstream.read_integer("cannot-die-from-damage", 2);
    this.m_weapon_traits.m_damage_modifier_percentage_setting =
      bitstream.read_integer("damage-modifier", 4);
    this.m_weapon_traits.m_melee_damage_modifier_percentage_setting =
      bitstream.read_integer("melee-damage-modifier", 4);
    this.m_weapon_traits.m_initial_primary_weapon_absolute_index =
      bitstream.read_signed_integer("player-trait-initial-primary-weapon", 8);
    this.m_weapon_traits.m_initial_secondary_weapon_absolute_index =
      bitstream.read_signed_integer("player-trait-initial-secondary-weapon", 8);
    this.m_weapon_traits.m_initial_grenade_count_setting =
      bitstream.read_integer("player-trait-initial-grenade-count", 4);
    this.m_weapon_traits.m_infinite_ammo_setting = bitstream.read_integer(
      "player-traits-infinite-ammo-setting",
      2
    );
    this.m_weapon_traits.m_recharging_grenades_setting = bitstream.read_integer(
      "player-traits-recharging-grenades",
      2
    );
    this.m_weapon_traits.m_weapon_pickup_setting = bitstream.read_integer(
      "player-traits-weapon-pickup-allowed",
      2
    );
    this.m_weapon_traits.m_equipment_drop_on_death_setting =
      bitstream.read_integer("player-traits-equipment-drop", 2);
    this.m_weapon_traits.m_infinite_equipment_setting = bitstream.read_integer(
      "player-traits-infinite-equipment",
      2
    );
    this.m_weapon_traits.m_initial_equipment_absolute_index =
      bitstream.read_signed_integer("player-trait-initial-equipment", 8);
    this.m_movement_traits.m_speed_setting = bitstream.read_integer(
      "player-speed",
      5
    );
    this.m_movement_traits.m_gravity_setting = bitstream.read_integer(
      "player-gravity",
      4
    );
    this.m_movement_traits.m_vehicle_usage_setting = bitstream.read_integer(
      "player-traits-movement-vehicle-usage",
      4
    );
    this.m_movement_traits.m_double_jump_setting = bitstream.read_integer(
      "player-traits-movement-double-jump",
      2
    );
    this.m_movement_traits.m_sprint_setting = bitstream.read_integer(
      "player-traits-movement-sprint",
      2
    );
    this.m_movement_traits.m_equipment_usage_setting = bitstream.read_integer(
      "player-traits-movement-equipment-usage",
      2
    );
    if (bitstream.read_bool("player-traits-movement-jump-modifier-changed")) {
      this.m_movement_traits.m_jump_modifier = bitstream.read_quantized_real(
        0,
        4,
        4,
        false,
        false
      );
    } else {
      this.m_movement_traits.m_jump_modifier = -1;
    }
    this.m_appearance_traits.m_active_camo_setting = bitstream.read_integer(
      "player-traits-appearance-active-camo",
      3
    ) as e_active_camo_setting;
    this.m_appearance_traits.m_waypoint_setting = bitstream.read_integer(
      "player-traits-appearance-waypoint",
      2
    );
    this.m_appearance_traits.m_gamertag_setting = bitstream.read_integer(
      "player-traits-appearance-gamertag",
      2
    );
    this.m_appearance_traits.m_aura_setting = bitstream.read_integer(
      "player-traits-appearance-aura",
      3
    );
    this.m_appearance_traits.m_forced_change_color_setting =
      bitstream.read_integer("player-traits-appearance-forced-change-color", 4);
    this.m_sensor_traits.m_motion_tracker_setting = bitstream.read_integer(
      "player-traits-sensors-motion-tracker",
      3
    );
    this.m_sensor_traits.m_motion_tracker_range_setting =
      bitstream.read_integer("motion-tracker-range", 3);
    this.m_sensor_traits.m_directional_damage_setting = bitstream.read_integer(
      "player-traits-sensors-directional-damage",
      2
    );
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(
      this.m_shield_vitality_traits.m_damage_resistance_percentage_setting,
      4
    );
    bitstream.write_integer(this.m_shield_vitality_traits.m_body_multiplier, 3);
    bitstream.write_integer(
      this.m_shield_vitality_traits.m_body_recharge_rate,
      4
    );
    bitstream.write_integer(
      this.m_shield_vitality_traits.m_shield_multiplier,
      3
    );
    bitstream.write_integer(
      this.m_shield_vitality_traits.m_shield_recharge_rate,
      4
    );
    bitstream.write_integer(
      this.m_shield_vitality_traits.m_overshield_recharge_rate,
      4
    );
    bitstream.write_integer(
      this.m_shield_vitality_traits.m_headshot_immunity_setting,
      2
    );
    bitstream.write_integer(
      this.m_shield_vitality_traits.m_vampirism_percentage_setting,
      3
    );
    bitstream.write_integer(
      this.m_shield_vitality_traits.m_assasination_immunity,
      2
    );
    bitstream.write_integer(
      this.m_shield_vitality_traits.m_cannot_die_from_damage,
      2
    );
    bitstream.write_integer(
      this.m_weapon_traits.m_damage_modifier_percentage_setting,
      4
    );
    bitstream.write_integer(
      this.m_weapon_traits.m_melee_damage_modifier_percentage_setting,
      4
    );
    bitstream.write_signed_integer(
      this.m_weapon_traits.m_initial_primary_weapon_absolute_index,
      8
    );
    bitstream.write_signed_integer(
      this.m_weapon_traits.m_initial_secondary_weapon_absolute_index,
      8
    );
    bitstream.write_integer(
      this.m_weapon_traits.m_initial_grenade_count_setting,
      4
    );
    bitstream.write_integer(this.m_weapon_traits.m_infinite_ammo_setting, 2);
    bitstream.write_integer(
      this.m_weapon_traits.m_recharging_grenades_setting,
      2
    );
    bitstream.write_integer(this.m_weapon_traits.m_weapon_pickup_setting, 2);
    bitstream.write_integer(
      this.m_weapon_traits.m_equipment_drop_on_death_setting,
      2
    );
    bitstream.write_integer(
      this.m_weapon_traits.m_infinite_equipment_setting,
      2
    );
    bitstream.write_signed_integer(
      this.m_weapon_traits.m_initial_equipment_absolute_index,
      8
    );
    bitstream.write_integer(this.m_movement_traits.m_speed_setting, 5);
    bitstream.write_integer(this.m_movement_traits.m_gravity_setting, 4);
    bitstream.write_integer(this.m_movement_traits.m_vehicle_usage_setting, 4);
    bitstream.write_integer(this.m_movement_traits.m_double_jump_setting, 2);
    bitstream.write_integer(this.m_movement_traits.m_sprint_setting, 2);
    bitstream.write_integer(
      this.m_movement_traits.m_equipment_usage_setting,
      2
    );
    if (this.m_movement_traits.m_jump_modifier === -1) {
      bitstream.write_bool(false);
    } else {
      bitstream.write_bool(true);
      bitstream.write_quantized_real(
        this.m_movement_traits.m_jump_modifier,
        0,
        4,
        4,
        false,
        false
      );
    }
    bitstream.write_integer(this.m_appearance_traits.m_active_camo_setting, 3);
    bitstream.write_integer(this.m_appearance_traits.m_waypoint_setting, 2);
    bitstream.write_integer(this.m_appearance_traits.m_gamertag_setting, 2);
    bitstream.write_integer(this.m_appearance_traits.m_aura_setting, 3);
    bitstream.write_integer(
      this.m_appearance_traits.m_forced_change_color_setting,
      4
    );
    bitstream.write_integer(this.m_sensor_traits.m_motion_tracker_setting, 3);
    bitstream.write_integer(
      this.m_sensor_traits.m_motion_tracker_range_setting,
      3
    );
    bitstream.write_integer(
      this.m_sensor_traits.m_directional_damage_setting,
      2
    );
  }
}
