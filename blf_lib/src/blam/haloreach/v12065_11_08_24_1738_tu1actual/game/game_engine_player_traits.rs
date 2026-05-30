use serde::{Deserialize, Serialize};
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_player_trait_weapons {
    pub m_damage_modifier_percentage_setting: u8,
    pub m_melee_damage_modifier_percentage_setting: u8,
    pub m_initial_primary_weapon_absolute_index: i8,
    pub m_initial_secondary_weapon_absolute_index: i8,
    pub m_initial_grenade_count_setting: u16,
    pub m_infinite_ammo_setting: u8,
    pub m_recharging_grenades_setting: u8,
    pub m_weapon_pickup_setting: u8,
    pub m_equipment_usage_setting: u8,
    pub m_equipment_drop_on_death_setting: u8,
    pub m_infinite_equipment_setting: u8,
    pub m_initial_equipment_absolute_index: i8,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_player_trait_shield_vitality {
    pub m_damage_resistance_percentage_setting: u8,
    pub m_body_multiplier: u8,
    pub m_body_recharge_rate: u8,
    pub m_shield_multiplier: u8,
    pub m_shield_recharge_rate: u8,
    pub m_overshield_recharge_rate: u8,
    pub m_headshot_immunity_setting: u8,
    pub m_vampirism_percentage_setting: u8,
    pub m_assasination_immunity: u8,
    pub m_cannot_die_from_damage: u8,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_player_trait_movement {
    pub m_speed_setting: u8,
    pub m_gravity_setting: u8,
    pub m_vehicle_usage_setting: u8,
    pub m_double_jump_setting: u8,
    pub m_jump_modifier: i16,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_player_trait_appearance {
    pub m_active_camo_setting: u8,
    pub m_waypoint_setting: u8,
    pub m_gamertag_setting: u8,
    pub m_aura_setting: u8,
    pub m_forced_change_color_setting: u8,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_player_trait_sensors {
    pub m_motion_tracker_setting: u16,
    pub m_motion_tracker_range_setting: u16,
    pub m_directional_damage_setting: u16,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_player_traits {
    pub m_shield_vitality_traits: c_player_trait_shield_vitality,
    pub m_weapon_traits: c_player_trait_weapons,
    pub m_movement_traits: c_player_trait_movement,
    pub m_appearance_traits: c_player_trait_appearance,
    pub m_sensor_traits: c_player_trait_sensors,
}

impl c_player_traits {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_shield_vitality_traits.m_damage_resistance_percentage_setting, 4)?;
        bitstream.write_integer(self.m_shield_vitality_traits.m_body_multiplier, 3)?;
        bitstream.write_integer(self.m_shield_vitality_traits.m_body_recharge_rate, 4)?;
        bitstream.write_integer(self.m_shield_vitality_traits.m_shield_multiplier, 3)?;
        bitstream.write_integer(self.m_shield_vitality_traits.m_shield_recharge_rate, 4)?;
        bitstream.write_integer(self.m_shield_vitality_traits.m_overshield_recharge_rate, 4)?;
        bitstream.write_integer(self.m_shield_vitality_traits.m_headshot_immunity_setting, 2)?;
        bitstream.write_integer(self.m_shield_vitality_traits.m_vampirism_percentage_setting, 3)?;
        bitstream.write_integer(self.m_shield_vitality_traits.m_assasination_immunity, 2)?;
        bitstream.write_integer(self.m_shield_vitality_traits.m_cannot_die_from_damage, 2)?;
        bitstream.write_integer(self.m_weapon_traits.m_damage_modifier_percentage_setting, 4)?;
        bitstream.write_integer(self.m_weapon_traits.m_melee_damage_modifier_percentage_setting, 4)?;
        bitstream.write_signed_integer(self.m_weapon_traits.m_initial_primary_weapon_absolute_index, 8)?;
        bitstream.write_signed_integer(self.m_weapon_traits.m_initial_secondary_weapon_absolute_index, 8)?;
        bitstream.write_integer(self.m_weapon_traits.m_initial_grenade_count_setting, 4)?;
        bitstream.write_integer(self.m_weapon_traits.m_infinite_ammo_setting, 2)?;
        bitstream.write_integer(self.m_weapon_traits.m_recharging_grenades_setting, 2)?;
        bitstream.write_integer(self.m_weapon_traits.m_weapon_pickup_setting, 2)?;
        bitstream.write_integer(self.m_weapon_traits.m_equipment_usage_setting, 2)?;
        bitstream.write_integer(self.m_weapon_traits.m_equipment_drop_on_death_setting, 2)?;
        bitstream.write_integer(self.m_weapon_traits.m_infinite_equipment_setting, 2)?;
        bitstream.write_signed_integer(self.m_weapon_traits.m_initial_equipment_absolute_index, 8)?;
        bitstream.write_integer(self.m_movement_traits.m_speed_setting, 5)?;
        bitstream.write_integer(self.m_movement_traits.m_gravity_setting, 4)?;
        bitstream.write_integer(self.m_movement_traits.m_vehicle_usage_setting, 4)?;
        bitstream.write_integer(self.m_movement_traits.m_double_jump_setting, 2)?;
        if self.m_movement_traits.m_jump_modifier != -1 {
            bitstream.write_bool(true)?;
            bitstream.write_signed_integer(self.m_movement_traits.m_jump_modifier, 9)?;
        } else {
            bitstream.write_bool(false)?;
        }
        bitstream.write_integer(self.m_appearance_traits.m_active_camo_setting, 3)?;
        bitstream.write_integer(self.m_appearance_traits.m_waypoint_setting, 2)?;
        bitstream.write_integer(self.m_appearance_traits.m_gamertag_setting, 2)?;
        bitstream.write_integer(self.m_appearance_traits.m_aura_setting, 3)?;
        bitstream.write_integer(self.m_appearance_traits.m_forced_change_color_setting, 4)?;
        bitstream.write_integer(self.m_sensor_traits.m_motion_tracker_setting, 3)?;
        bitstream.write_integer(self.m_sensor_traits.m_motion_tracker_range_setting, 3)?;
        bitstream.write_integer(self.m_sensor_traits.m_directional_damage_setting, 2)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_shield_vitality_traits.m_damage_resistance_percentage_setting = bitstream.read_integer("damage-resistance", 4)?;
        self.m_shield_vitality_traits.m_body_multiplier = bitstream.read_integer("body-multiplier", 3)?;
        self.m_shield_vitality_traits.m_body_recharge_rate = bitstream.read_integer("body-recharge-rate", 4)?;
        self.m_shield_vitality_traits.m_shield_multiplier = bitstream.read_integer("shield-multiplier", 3)?;
        self.m_shield_vitality_traits.m_shield_recharge_rate = bitstream.read_integer("shield-recharge-rate", 4)?;
        self.m_shield_vitality_traits.m_overshield_recharge_rate = bitstream.read_integer("overshield-recharge-rate", 4)?;
        self.m_shield_vitality_traits.m_headshot_immunity_setting = bitstream.read_integer("headshot-immunity", 2)?;
        self.m_shield_vitality_traits.m_vampirism_percentage_setting = bitstream.read_integer("vampirism", 3)?;
        self.m_shield_vitality_traits.m_assasination_immunity = bitstream.read_integer("assasination-immunity", 2)?;
        self.m_shield_vitality_traits.m_cannot_die_from_damage = bitstream.read_integer("cannot-die-from-damage", 2)?;
        self.m_weapon_traits.m_damage_modifier_percentage_setting = bitstream.read_integer("damage-modifier", 4)?;
        self.m_weapon_traits.m_melee_damage_modifier_percentage_setting = bitstream.read_integer("melee-damage-modifier", 4)?;
        self.m_weapon_traits.m_initial_primary_weapon_absolute_index = bitstream.read_signed_integer("player-trait-initial-primary-weapon", 8)?;
        self.m_weapon_traits.m_initial_secondary_weapon_absolute_index = bitstream.read_signed_integer("player-trait-initial-secondary-weapon", 8)?;
        self.m_weapon_traits.m_initial_grenade_count_setting = bitstream.read_integer("player-trait-initial-grenade-count", 4)?;
        self.m_weapon_traits.m_infinite_ammo_setting = bitstream.read_integer("player-traits-infinite-ammo-setting", 2)?;
        self.m_weapon_traits.m_recharging_grenades_setting = bitstream.read_integer("player-traits-recharging-grenades", 2)?;
        self.m_weapon_traits.m_weapon_pickup_setting = bitstream.read_integer("player-traits-weapon-pickup-allowed", 2)?;
        self.m_weapon_traits.m_equipment_usage_setting = bitstream.read_integer("player-traits-equipment-usage", 2)?;
        self.m_weapon_traits.m_equipment_drop_on_death_setting = bitstream.read_integer("player-traits-equipment-drop", 2)?;
        self.m_weapon_traits.m_infinite_equipment_setting = bitstream.read_integer("player-traits-infinite-equipment", 2)?;
        self.m_weapon_traits.m_initial_equipment_absolute_index = bitstream.read_signed_integer("player-trait-initial-equipment", 8)?;
        self.m_movement_traits.m_speed_setting = bitstream.read_integer("player-speed", 5)?;
        self.m_movement_traits.m_gravity_setting = bitstream.read_integer("player-gravity", 4)?;
        self.m_movement_traits.m_vehicle_usage_setting = bitstream.read_integer("player-traits-movement-vehicle-usage", 4)?;
        self.m_movement_traits.m_double_jump_setting = bitstream.read_integer("player-traits-movement-double-jump", 2)?;
        if bitstream.read_bool("player-traits-movement-jump-modifier-changed")? {
            self.m_movement_traits.m_jump_modifier = bitstream.read_integer("player-traits-movement-jump-modifier", 9)?;
        } else {
            self.m_movement_traits.m_jump_modifier = -1;
        }
        self.m_appearance_traits.m_active_camo_setting = bitstream.read_integer("player-traits-appearance-active-camo", 3)?;
        self.m_appearance_traits.m_waypoint_setting = bitstream.read_integer("player-traits-appearance-waypoint", 2)?;
        self.m_appearance_traits.m_gamertag_setting = bitstream.read_integer("player-traits-appearance-gamertag", 2)?;
        self.m_appearance_traits.m_aura_setting = bitstream.read_integer("player-traits-appearance-aura", 3)?;
        self.m_appearance_traits.m_forced_change_color_setting = bitstream.read_integer("player-traits-appearance-forced-change-color", 4)?;
        self.m_sensor_traits.m_motion_tracker_setting = bitstream.read_integer("player-traits-sensors-motion-tracker", 3)?;
        self.m_sensor_traits.m_motion_tracker_range_setting = bitstream.read_integer("motion-tracker-range", 3)?;
        self.m_sensor_traits.m_directional_damage_setting = bitstream.read_integer("player-traits-sensors-directional-damage", 2)?;

        Ok(())
    }
}