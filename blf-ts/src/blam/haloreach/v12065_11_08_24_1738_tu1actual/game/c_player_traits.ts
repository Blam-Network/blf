import { type c_bitstream_reader, c_bitstream_writer } from "../../../../bitstream";

export class c_player_trait_shield_vitality {
  m_damage_resistance_percentage_setting = 0;
  m_body_multiplier = 0;
  m_body_recharge_rate = 0;
  m_shield_multiplier = 0;
  m_shield_recharge_rate = 0;
  m_overshield_recharge_rate = 0;
  m_headshot_immunity_setting = 0;
  m_vampirism_percentage_setting = 0;
  m_assasination_immunity = 0;
  m_cannot_die_from_damage = 0;
}

export class c_player_trait_weapons {
  m_damage_modifier_percentage_setting = 0;
  m_melee_damage_modifier_percentage_setting = 0;
  m_initial_primary_weapon_absolute_index = 0;
  m_initial_secondary_weapon_absolute_index = 0;
  m_initial_grenade_count_setting = 0;
  m_infinite_ammo_setting = 0;
  m_recharging_grenades_setting = 0;
  m_weapon_pickup_setting = 0;
  m_equipment_usage_setting = 0;
  m_equipment_drop_on_death_setting = 0;
  m_infinite_equipment_setting = 0;
  m_initial_equipment_absolute_index = 0;
}

export class c_player_trait_movement {
  m_speed_setting = 0;
  m_gravity_setting = 0;
  m_vehicle_usage_setting = 0;
  m_double_jump_setting = 0;
  m_jump_modifier = -1;
}

export class c_player_trait_appearance {
  m_active_camo_setting = 0;
  m_waypoint_setting = 0;
  m_gamertag_setting = 0;
  m_aura_setting = 0;
  m_forced_change_color_setting = 0;
}

export class c_player_trait_sensors {
  m_motion_tracker_setting = 0;
  m_motion_tracker_range_setting = 0;
  m_directional_damage_setting = 0;
}

export class c_player_traits {
  m_shield_vitality_traits = new c_player_trait_shield_vitality();
  m_weapon_traits = new c_player_trait_weapons();
  m_movement_traits = new c_player_trait_movement();
  m_appearance_traits = new c_player_trait_appearance();
  m_sensor_traits = new c_player_trait_sensors();

  decode(bitstream: c_bitstream_reader): void {
    const sv = this.m_shield_vitality_traits;
    sv.m_damage_resistance_percentage_setting = bitstream.read_integer(
      "damage-resistance",
      4,
    );
    sv.m_body_multiplier = bitstream.read_integer("body-multiplier", 3);
    sv.m_body_recharge_rate = bitstream.read_integer("body-recharge-rate", 4);
    sv.m_shield_multiplier = bitstream.read_integer("shield-multiplier", 3);
    sv.m_shield_recharge_rate = bitstream.read_integer(
      "shield-recharge-rate",
      4,
    );
    sv.m_overshield_recharge_rate = bitstream.read_integer(
      "overshield-recharge-rate",
      4,
    );
    sv.m_headshot_immunity_setting = bitstream.read_integer(
      "headshot-immunity",
      2,
    );
    sv.m_vampirism_percentage_setting = bitstream.read_integer("vampirism", 3);
    sv.m_assasination_immunity = bitstream.read_integer(
      "assasination-immunity",
      2,
    );
    sv.m_cannot_die_from_damage = bitstream.read_integer(
      "cannot-die-from-damage",
      2,
    );

    const w = this.m_weapon_traits;
    w.m_damage_modifier_percentage_setting = bitstream.read_integer(
      "damage-modifier",
      4,
    );
    w.m_melee_damage_modifier_percentage_setting = bitstream.read_integer(
      "melee-damage-modifier",
      4,
    );
    w.m_initial_primary_weapon_absolute_index = bitstream.read_signed_integer(
      "player-trait-initial-primary-weapon",
      8,
    );
    w.m_initial_secondary_weapon_absolute_index = bitstream.read_signed_integer(
      "player-trait-initial-secondary-weapon",
      8,
    );
    w.m_initial_grenade_count_setting = bitstream.read_integer(
      "player-trait-initial-grenade-count",
      4,
    );
    w.m_infinite_ammo_setting = bitstream.read_integer(
      "player-traits-infinite-ammo-setting",
      2,
    );
    w.m_recharging_grenades_setting = bitstream.read_integer(
      "player-traits-recharging-grenades",
      2,
    );
    w.m_weapon_pickup_setting = bitstream.read_integer(
      "player-traits-weapon-pickup-allowed",
      2,
    );
    w.m_equipment_usage_setting = bitstream.read_integer(
      "player-traits-equipment-usage",
      2,
    );
    w.m_equipment_drop_on_death_setting = bitstream.read_integer(
      "player-traits-equipment-drop",
      2,
    );
    w.m_infinite_equipment_setting = bitstream.read_integer(
      "player-traits-infinite-equipment",
      2,
    );
    w.m_initial_equipment_absolute_index = bitstream.read_signed_integer(
      "player-trait-initial-equipment",
      8,
    );

    const m = this.m_movement_traits;
    m.m_speed_setting = bitstream.read_integer("player-speed", 5);
    m.m_gravity_setting = bitstream.read_integer("player-gravity", 4);
    m.m_vehicle_usage_setting = bitstream.read_integer(
      "player-traits-movement-vehicle-usage",
      4,
    );
    m.m_double_jump_setting = bitstream.read_integer(
      "player-traits-movement-double-jump",
      2,
    );
    if (bitstream.read_bool("player-traits-movement-jump-modifier-changed")) {
      m.m_jump_modifier = bitstream.read_integer(
        "player-traits-movement-jump-modifier",
        9,
      );
    } else {
      m.m_jump_modifier = -1;
    }

    const a = this.m_appearance_traits;
    a.m_active_camo_setting = bitstream.read_integer(
      "player-traits-appearance-active-camo",
      3,
    );
    a.m_waypoint_setting = bitstream.read_integer(
      "player-traits-appearance-waypoint",
      2,
    );
    a.m_gamertag_setting = bitstream.read_integer(
      "player-traits-appearance-gamertag",
      2,
    );
    a.m_aura_setting = bitstream.read_integer(
      "player-traits-appearance-aura",
      3,
    );
    a.m_forced_change_color_setting = bitstream.read_integer(
      "player-traits-appearance-forced-change-color",
      4,
    );

    const s = this.m_sensor_traits;
    s.m_motion_tracker_setting = bitstream.read_integer(
      "player-traits-sensors-motion-tracker",
      3,
    );
    s.m_motion_tracker_range_setting = bitstream.read_integer(
      "motion-tracker-range",
      3,
    );
    s.m_directional_damage_setting = bitstream.read_integer(
      "player-traits-sensors-directional-damage",
      2,
    );
  }

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_shield_vitality_traits.m_damage_resistance_percentage_setting, 4);
            bitstream.write_integer(this.m_shield_vitality_traits.m_body_multiplier, 3);
            bitstream.write_integer(this.m_shield_vitality_traits.m_body_recharge_rate, 4);
            bitstream.write_integer(this.m_shield_vitality_traits.m_shield_multiplier, 3);
            bitstream.write_integer(this.m_shield_vitality_traits.m_shield_recharge_rate, 4);
            bitstream.write_integer(this.m_shield_vitality_traits.m_overshield_recharge_rate, 4);
            bitstream.write_integer(this.m_shield_vitality_traits.m_headshot_immunity_setting, 2);
            bitstream.write_integer(this.m_shield_vitality_traits.m_vampirism_percentage_setting, 3);
            bitstream.write_integer(this.m_shield_vitality_traits.m_assasination_immunity, 2);
            bitstream.write_integer(this.m_shield_vitality_traits.m_cannot_die_from_damage, 2);
            bitstream.write_integer(this.m_weapon_traits.m_damage_modifier_percentage_setting, 4);
            bitstream.write_integer(this.m_weapon_traits.m_melee_damage_modifier_percentage_setting, 4);
            bitstream.write_signed_integer(this.m_weapon_traits.m_initial_primary_weapon_absolute_index, 8);
            bitstream.write_signed_integer(this.m_weapon_traits.m_initial_secondary_weapon_absolute_index, 8);
            bitstream.write_integer(this.m_weapon_traits.m_initial_grenade_count_setting, 4);
            bitstream.write_integer(this.m_weapon_traits.m_infinite_ammo_setting, 2);
            bitstream.write_integer(this.m_weapon_traits.m_recharging_grenades_setting, 2);
            bitstream.write_integer(this.m_weapon_traits.m_weapon_pickup_setting, 2);
            bitstream.write_integer(this.m_weapon_traits.m_equipment_usage_setting, 2);
            bitstream.write_integer(this.m_weapon_traits.m_equipment_drop_on_death_setting, 2);
            bitstream.write_integer(this.m_weapon_traits.m_infinite_equipment_setting, 2);
            bitstream.write_signed_integer(this.m_weapon_traits.m_initial_equipment_absolute_index, 8);
            bitstream.write_integer(this.m_movement_traits.m_speed_setting, 5);
            bitstream.write_integer(this.m_movement_traits.m_gravity_setting, 4);
            bitstream.write_integer(this.m_movement_traits.m_vehicle_usage_setting, 4);
            bitstream.write_integer(this.m_movement_traits.m_double_jump_setting, 2);
    if (this.m_movement_traits.m_jump_modifier !== -1) {
      bitstream.write_bool(true);
      bitstream.write_signed_integer(this.m_movement_traits.m_jump_modifier, 9);
    } else {
      bitstream.write_bool(false);
    }
            bitstream.write_integer(this.m_appearance_traits.m_active_camo_setting, 3);
            bitstream.write_integer(this.m_appearance_traits.m_waypoint_setting, 2);
            bitstream.write_integer(this.m_appearance_traits.m_gamertag_setting, 2);
            bitstream.write_integer(this.m_appearance_traits.m_aura_setting, 3);
            bitstream.write_integer(this.m_appearance_traits.m_forced_change_color_setting, 4);
            bitstream.write_integer(this.m_sensor_traits.m_motion_tracker_setting, 3);
            bitstream.write_integer(this.m_sensor_traits.m_motion_tracker_range_setting, 3);
            bitstream.write_integer(this.m_sensor_traits.m_directional_damage_setting, 2);
  }
}
