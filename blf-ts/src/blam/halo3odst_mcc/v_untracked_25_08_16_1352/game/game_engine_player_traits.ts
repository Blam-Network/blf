import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../bitstream";
import { AutoMap } from "../../../../helpers/automap";

export class c_player_trait_weapons {
  @AutoMap(() => Number)
  m_initial_grenade_count_setting = 0;
  @AutoMap(() => Number)
  m_initial_primary_weapon_absolute_index = 0;
  @AutoMap(() => Number)
  m_initial_secondary_weapon_absolute_index = 0;
  @AutoMap(() => Number)
  m_damage_modifier_percentage_setting = 0;
  @AutoMap(() => Number)
  m_recharging_grenades_setting = 0;
  @AutoMap(() => Number)
  m_infinite_ammo_setting = 0;
  @AutoMap(() => Number)
  m_weapon_pickup_setting = 0;
}

export class c_player_trait_shield_vitality {
  @AutoMap(() => Number)
  m_damage_resistance_percentage_setting = 0;
  @AutoMap(() => Number)
  m_shield_recharge_rate_percentage_setting = 0;
  @AutoMap(() => Number)
  m_vampirism_percentage_setting = 0;
  @AutoMap(() => Number)
  m_headshot_immunity_setting = 0;
  @AutoMap(() => Number)
  m_shield_multiplier_setting = 0;
}

export class c_player_trait_movement {
  @AutoMap(() => Number)
  m_speed_setting = 0;
  @AutoMap(() => Number)
  m_gravity_setting = 0;
  @AutoMap(() => Number)
  m_vehicle_usage_setting = 0;
}

export class c_player_trait_appearance {
  @AutoMap(() => Number)
  m_active_camo_setting = 0;
  @AutoMap(() => Number)
  m_waypoint_setting = 0;
  @AutoMap(() => Number)
  m_aura_setting = 0;
  @AutoMap(() => Number)
  m_forced_change_color_setting = 0;
}

export class c_player_trait_sensors {
  @AutoMap(() => Number)
  m_motion_tracker_setting = 0;
  @AutoMap(() => Number)
  m_motion_tracker_range_setting = 0;
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

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(
      this.m_shield_vitality_traits.m_damage_resistance_percentage_setting,
      4
    );
    bitstream.write_integer(
      this.m_shield_vitality_traits.m_shield_recharge_rate_percentage_setting,
      4
    );
    bitstream.write_integer(
      this.m_shield_vitality_traits.m_vampirism_percentage_setting,
      3
    );
    bitstream.write_integer(
      this.m_shield_vitality_traits.m_headshot_immunity_setting,
      2
    );
    bitstream.write_integer(
      this.m_shield_vitality_traits.m_shield_multiplier_setting,
      3
    );
    bitstream.write_integer(
      this.m_weapon_traits.m_damage_modifier_percentage_setting,
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
      2
    );
    bitstream.write_integer(this.m_weapon_traits.m_infinite_ammo_setting, 2);
    bitstream.write_integer(
      this.m_weapon_traits.m_recharging_grenades_setting,
      2
    );
    bitstream.write_integer(this.m_weapon_traits.m_weapon_pickup_setting, 2);
    bitstream.write_integer(this.m_movement_traits.m_speed_setting, 4);
    bitstream.write_integer(this.m_movement_traits.m_gravity_setting, 3);
    bitstream.write_integer(this.m_movement_traits.m_vehicle_usage_setting, 2);
    bitstream.write_integer(this.m_appearance_traits.m_active_camo_setting, 3);
    bitstream.write_integer(this.m_appearance_traits.m_waypoint_setting, 2);
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
  }

  decode(bitstream: c_bitstream_reader): void {
    this.m_shield_vitality_traits.m_damage_resistance_percentage_setting =
      bitstream.read_integer("damage-resistance", 4);
    this.m_shield_vitality_traits.m_shield_recharge_rate_percentage_setting =
      bitstream.read_integer("shield-recharge-rate", 4);
    this.m_shield_vitality_traits.m_vampirism_percentage_setting =
      bitstream.read_integer("vampirism", 3);
    this.m_shield_vitality_traits.m_headshot_immunity_setting =
      bitstream.read_integer("headshot-immunity", 2);
    this.m_shield_vitality_traits.m_shield_multiplier_setting =
      bitstream.read_integer("shield-multiplier", 3);
    this.m_weapon_traits.m_damage_modifier_percentage_setting =
      bitstream.read_integer("damage-modifier", 4);
    this.m_weapon_traits.m_initial_primary_weapon_absolute_index =
      bitstream.read_signed_integer("initial-primary-weapon", 8);
    this.m_weapon_traits.m_initial_secondary_weapon_absolute_index =
      bitstream.read_signed_integer("initial-secondary-weapon", 8);
    this.m_weapon_traits.m_initial_grenade_count_setting =
      bitstream.read_integer("initial-grenade-count", 2);
    this.m_weapon_traits.m_infinite_ammo_setting = bitstream.read_integer(
      "infinite-ammo",
      2
    );
    this.m_weapon_traits.m_recharging_grenades_setting = bitstream.read_integer(
      "recharging-grenades",
      2
    );
    this.m_weapon_traits.m_weapon_pickup_setting = bitstream.read_integer(
      "weapon-pickup",
      2
    );
    this.m_movement_traits.m_speed_setting = bitstream.read_integer("speed", 4);
    this.m_movement_traits.m_gravity_setting = bitstream.read_integer(
      "gravity",
      3
    );
    this.m_movement_traits.m_vehicle_usage_setting = bitstream.read_integer(
      "vehicle-usage",
      2
    );
    this.m_appearance_traits.m_active_camo_setting = bitstream.read_integer(
      "active-camo",
      3
    );
    this.m_appearance_traits.m_waypoint_setting = bitstream.read_integer(
      "waypoint",
      2
    );
    this.m_appearance_traits.m_aura_setting = bitstream.read_integer("aura", 3);
    this.m_appearance_traits.m_forced_change_color_setting =
      bitstream.read_integer("forced-change-color", 4);
    this.m_sensor_traits.m_motion_tracker_setting = bitstream.read_integer(
      "motion-tracker",
      3
    );
    this.m_sensor_traits.m_motion_tracker_range_setting =
      bitstream.read_integer("motion-tracker-range", 3);
  }
}
