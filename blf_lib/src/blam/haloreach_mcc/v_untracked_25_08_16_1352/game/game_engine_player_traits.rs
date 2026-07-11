use serde::{Deserialize, Serialize};
use num_derive::{FromPrimitive, ToPrimitive};
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;

/// Initial grenade loadout preset (`m_initial_grenade_count_setting`, 4 bits).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(4)]
pub enum e_grenade_count_setting {
    none = 0,
    #[default]
    map_default = 1,
    zero = 2,
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

/// Infinite ammo preset (`m_infinite_ammo_setting`, 2 bits).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(2)]
pub enum e_infinite_ammo_setting {
    #[default]
    unchanged = 0,
    disabled = 1,
    enabled = 2,
    bottomless_clip = 3,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(4)]
pub enum e_vehicle_usage_setting {
    #[default]
    unchanged = 0,
    none = 1,
    full = 2,
    passenger = 3,
    not_passenger = 4,
    driver = 5,
    gunner = 6,
    not_driver = 7,
    not_gunner = 8,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(2)]
pub enum e_waypoint_setting {
    #[default]
    unchanged = 0,
    off = 1,
    allies = 2,
    all = 3,
}

/// Double jump preset (`m_double_jump_setting`, 2 bits).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(2)]
pub enum e_double_jump_setting {
    #[default]
    unchanged = 0,
    off = 1,
    on = 2,
    on_lunge = 3,
}

/// Player aura color preset (`m_aura_setting`, 3 bits).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(3)]
pub enum e_aura_setting {
    #[default]
    unchanged = 0,
    off = 1,
    team_color = 2,
    black = 3,
    white = 4,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_player_trait_weapons {
    pub m_damage_modifier_percentage_setting: u8,
    pub m_melee_damage_modifier_percentage_setting: u8,
    pub m_initial_primary_weapon_absolute_index: i8,
    pub m_initial_secondary_weapon_absolute_index: i8,
    pub m_initial_grenade_count_setting: e_grenade_count_setting,
    pub m_infinite_ammo_setting: e_infinite_ammo_setting,
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
    pub m_vehicle_usage_setting: e_vehicle_usage_setting,
    pub m_double_jump_setting: e_double_jump_setting,
    pub m_jump_modifier: i16,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_player_trait_appearance {
    pub m_active_camo_setting: u8,
    pub m_waypoint_setting: e_waypoint_setting,
    pub m_gamertag_setting: e_waypoint_setting,
    pub m_aura_setting: e_aura_setting,
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
        bitstream.write_enum(self.m_weapon_traits.m_initial_grenade_count_setting)?;
        bitstream.write_enum(self.m_weapon_traits.m_infinite_ammo_setting)?;
        bitstream.write_integer(self.m_weapon_traits.m_recharging_grenades_setting, 2)?;
        bitstream.write_integer(self.m_weapon_traits.m_weapon_pickup_setting, 2)?;
        bitstream.write_integer(self.m_weapon_traits.m_equipment_usage_setting, 2)?;
        bitstream.write_integer(self.m_weapon_traits.m_equipment_drop_on_death_setting, 2)?;
        bitstream.write_integer(self.m_weapon_traits.m_infinite_equipment_setting, 2)?;
        bitstream.write_signed_integer(self.m_weapon_traits.m_initial_equipment_absolute_index, 8)?;
        bitstream.write_integer(self.m_movement_traits.m_speed_setting, 5)?;
        bitstream.write_integer(self.m_movement_traits.m_gravity_setting, 4)?;
        bitstream.write_enum(self.m_movement_traits.m_vehicle_usage_setting)?;
        bitstream.write_enum(self.m_movement_traits.m_double_jump_setting)?;
        if self.m_movement_traits.m_jump_modifier != -1 {
            bitstream.write_bool(true)?;
            bitstream.write_integer(self.m_movement_traits.m_jump_modifier as u32, 9)?;
        } else {
            bitstream.write_bool(false)?;
        }
        bitstream.write_integer(self.m_appearance_traits.m_active_camo_setting, 3)?;
        bitstream.write_enum(self.m_appearance_traits.m_waypoint_setting)?;
        bitstream.write_enum(self.m_appearance_traits.m_gamertag_setting)?;
        bitstream.write_enum(self.m_appearance_traits.m_aura_setting)?;
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
        self.m_weapon_traits.m_initial_grenade_count_setting = bitstream.read_enum("player-trait-initial-grenade-count")?;
        self.m_weapon_traits.m_infinite_ammo_setting = bitstream.read_enum("player-traits-infinite-ammo-setting")?;
        self.m_weapon_traits.m_recharging_grenades_setting = bitstream.read_integer("player-traits-recharging-grenades", 2)?;
        self.m_weapon_traits.m_weapon_pickup_setting = bitstream.read_integer("player-traits-weapon-pickup-allowed", 2)?;
        self.m_weapon_traits.m_equipment_usage_setting = bitstream.read_integer("player-traits-equipment-usage", 2)?;
        self.m_weapon_traits.m_equipment_drop_on_death_setting = bitstream.read_integer("player-traits-equipment-drop", 2)?;
        self.m_weapon_traits.m_infinite_equipment_setting = bitstream.read_integer("player-traits-infinite-equipment", 2)?;
        self.m_weapon_traits.m_initial_equipment_absolute_index = bitstream.read_signed_integer("player-trait-initial-equipment", 8)?;
        self.m_movement_traits.m_speed_setting = bitstream.read_integer("player-speed", 5)?;
        self.m_movement_traits.m_gravity_setting = bitstream.read_integer("player-gravity", 4)?;
        self.m_movement_traits.m_vehicle_usage_setting = bitstream.read_enum("player-traits-movement-vehicle-usage")?;
        self.m_movement_traits.m_double_jump_setting = bitstream.read_enum("player-traits-movement-double-jump")?;
        if bitstream.read_bool("player-traits-movement-jump-modifier-changed")? {
            self.m_movement_traits.m_jump_modifier = bitstream.read_integer("player-traits-movement-jump-modifier", 9)?;
        } else {
            self.m_movement_traits.m_jump_modifier = -1;
        }
        self.m_appearance_traits.m_active_camo_setting = bitstream.read_integer("player-traits-appearance-active-camo", 3)?;
        self.m_appearance_traits.m_waypoint_setting = bitstream.read_enum("player-traits-appearance-waypoint")?;
        self.m_appearance_traits.m_gamertag_setting = bitstream.read_enum("player-traits-appearance-gamertag")?;
        self.m_appearance_traits.m_aura_setting = bitstream.read_enum("player-traits-appearance-aura")?;
        self.m_appearance_traits.m_forced_change_color_setting = bitstream.read_integer("player-traits-appearance-forced-change-color", 4)?;
        self.m_sensor_traits.m_motion_tracker_setting = bitstream.read_integer("player-traits-sensors-motion-tracker", 3)?;
        self.m_sensor_traits.m_motion_tracker_range_setting = bitstream.read_integer("motion-tracker-range", 3)?;
        self.m_sensor_traits.m_directional_damage_setting = bitstream.read_integer("player-traits-sensors-directional-damage", 2)?;

        Ok(())
    }
}