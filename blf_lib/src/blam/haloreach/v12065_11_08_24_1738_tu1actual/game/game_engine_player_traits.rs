use serde::{Deserialize, Serialize};
use num_derive::{FromPrimitive, ToPrimitive};
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;

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

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(2)]
pub enum e_boolean_trait {
    #[default]
    unchanged = 0,
    off = 1,
    on = 2,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(2)]
pub enum e_equipment_usage_setting {
    #[default]
    unchanged = 0,
    off = 1,
    not_with_objectives = 2,
    on = 3,
}

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
    passenger = 2,
    driver = 3,
    gunner = 4,
    not_passenger = 5,
    not_driver = 6,
    not_gunner = 7,
    full = 8,
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

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(3)]
pub enum e_active_camo_setting {
    #[default]
    off = 0,
    on = 1,
    poor = 2,
    good = 3,
    excellent = 4,
    invisible = 5,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(2)]
pub enum e_double_jump_setting {
    #[default]
    unchanged = 0,
    off = 1,
    on = 2,
    triple = 3,
}

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

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(4)]
pub enum e_forced_change_color_setting {
    #[default]
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

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(3)]
pub enum e_motion_tracker_setting {
    #[default]
    unchanged = 0,
    off = 1,
    allies = 2,
    normal = 3,
    enhanced = 4,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(4)]
pub enum e_damage_resistance_percentage_setting {
    #[default]
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

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(4)]
pub enum e_damage_modifier_percentage_setting {
    #[default]
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

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(3)]
pub enum e_body_multiplier_setting {
    #[default]
    unchanged = 0,
    percent_0 = 1,
    percent_100 = 2,
    percent_150 = 3,
    percent_200 = 4,
    percent_300 = 5,
    percent_400 = 6,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(3)]
pub enum e_shield_multiplier_setting {
    #[default]
    unchanged = 0,
    percent_0 = 1,
    percent_100 = 2,
    percent_150 = 3,
    percent_200 = 4,
    percent_300 = 5,
    percent_400 = 6,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(4)]
pub enum e_recharge_rate_percentage_setting {
    #[default]
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

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(3)]
pub enum e_vampirism_percentage_setting {
    #[default]
    unchanged = 0,
    percent_0 = 1,
    percent_10 = 2,
    percent_25 = 3,
    percent_50 = 4,
    percent_100 = 5,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(5)]
pub enum e_player_speed_setting {
    #[default]
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

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(4)]
pub enum e_player_gravity_setting {
    #[default]
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

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(3)]
pub enum e_motion_tracker_range_setting {
    #[default]
    unchanged = 0,
    meters_10 = 1,
    meters_15 = 2,
    meters_25 = 3,
    meters_50 = 4,
    meters_75 = 5,
    meters_100 = 6,
    meters_150 = 7,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_player_trait_weapons {
    pub m_damage_modifier_percentage_setting: e_damage_modifier_percentage_setting,
    pub m_melee_damage_modifier_percentage_setting: e_damage_modifier_percentage_setting,
    pub m_initial_primary_weapon_absolute_index: i8,
    pub m_initial_secondary_weapon_absolute_index: i8,
    pub m_initial_grenade_count_setting: e_grenade_count_setting,
    pub m_infinite_ammo_setting: e_infinite_ammo_setting,
    pub m_recharging_grenades_setting: e_boolean_trait,
    pub m_weapon_pickup_setting: e_boolean_trait,
    pub m_equipment_usage_setting: e_equipment_usage_setting,
    pub m_equipment_drop_on_death_setting: e_boolean_trait,
    pub m_infinite_equipment_setting: e_boolean_trait,
    pub m_initial_equipment_absolute_index: i8,
}

impl c_player_trait_weapons {
    pub fn clear(&mut self) {
        self.m_damage_modifier_percentage_setting = e_damage_modifier_percentage_setting::unchanged;
        self.m_melee_damage_modifier_percentage_setting = e_damage_modifier_percentage_setting::unchanged;
        self.m_initial_grenade_count_setting = e_grenade_count_setting::none;
        self.m_infinite_ammo_setting = e_infinite_ammo_setting::unchanged;
        self.m_recharging_grenades_setting = e_boolean_trait::unchanged;
        self.m_weapon_pickup_setting = e_boolean_trait::unchanged;
        self.m_equipment_usage_setting = e_equipment_usage_setting::unchanged;
        self.m_equipment_drop_on_death_setting = e_boolean_trait::unchanged;
        self.m_infinite_equipment_setting = e_boolean_trait::unchanged;
        self.m_initial_primary_weapon_absolute_index = -3;
        self.m_initial_secondary_weapon_absolute_index = -3;
        self.m_initial_equipment_absolute_index = -3;
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_player_trait_shield_vitality {
    pub m_damage_resistance_percentage_setting: e_damage_resistance_percentage_setting,
    pub m_body_multiplier: e_body_multiplier_setting,
    pub m_body_recharge_rate: e_recharge_rate_percentage_setting,
    pub m_shield_multiplier: e_shield_multiplier_setting,
    pub m_shield_recharge_rate: e_recharge_rate_percentage_setting,
    pub m_overshield_recharge_rate: e_recharge_rate_percentage_setting,
    pub m_headshot_immunity_setting: e_boolean_trait,
    pub m_vampirism_percentage_setting: e_vampirism_percentage_setting,
    pub m_assasination_immunity: e_boolean_trait,
    pub m_cannot_die_from_damage: e_boolean_trait,
}

impl c_player_trait_shield_vitality {
    pub fn clear(&mut self) {
        self.m_damage_resistance_percentage_setting = e_damage_resistance_percentage_setting::unchanged;
        self.m_body_multiplier = e_body_multiplier_setting::unchanged;
        self.m_body_recharge_rate = e_recharge_rate_percentage_setting::unchanged;
        self.m_shield_multiplier = e_shield_multiplier_setting::unchanged;
        self.m_shield_recharge_rate = e_recharge_rate_percentage_setting::unchanged;
        self.m_overshield_recharge_rate = e_recharge_rate_percentage_setting::unchanged;
        self.m_headshot_immunity_setting = e_boolean_trait::unchanged;
        self.m_vampirism_percentage_setting = e_vampirism_percentage_setting::unchanged;
        self.m_assasination_immunity = e_boolean_trait::unchanged;
        self.m_cannot_die_from_damage = e_boolean_trait::unchanged;
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_player_trait_movement {
    pub m_speed_setting: e_player_speed_setting,
    pub m_gravity_setting: e_player_gravity_setting,
    pub m_vehicle_usage_setting: e_vehicle_usage_setting,
    pub m_double_jump_setting: e_double_jump_setting,
    pub m_jump_modifier: i16,
}

impl c_player_trait_movement {
    pub fn clear(&mut self) {
        self.m_speed_setting = e_player_speed_setting::unchanged;
        self.m_gravity_setting = e_player_gravity_setting::unchanged;
        self.m_vehicle_usage_setting = e_vehicle_usage_setting::unchanged;
        self.m_double_jump_setting = e_double_jump_setting::unchanged;
        self.m_jump_modifier = -1;
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_player_trait_appearance {
    pub m_active_camo_setting: e_active_camo_setting,
    pub m_waypoint_setting: e_waypoint_setting,
    pub m_gamertag_setting: e_waypoint_setting,
    pub m_aura_setting: e_aura_setting,
    pub m_forced_change_color_setting: e_forced_change_color_setting,
}

impl c_player_trait_appearance {
    pub fn clear(&mut self) {
        self.m_active_camo_setting = e_active_camo_setting::off;
        self.m_waypoint_setting = e_waypoint_setting::unchanged;
        self.m_gamertag_setting = e_waypoint_setting::unchanged;
        self.m_aura_setting = e_aura_setting::unchanged;
        self.m_forced_change_color_setting = e_forced_change_color_setting::unchanged;
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_player_trait_sensors {
    pub m_motion_tracker_setting: e_motion_tracker_setting,
    pub m_motion_tracker_range_setting: e_motion_tracker_range_setting,
    pub m_directional_damage_setting: e_boolean_trait,
}

impl c_player_trait_sensors {
    pub fn clear(&mut self) {
        self.m_motion_tracker_setting = e_motion_tracker_setting::unchanged;
        self.m_motion_tracker_range_setting = e_motion_tracker_range_setting::unchanged;
        self.m_directional_damage_setting = e_boolean_trait::unchanged;
    }
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
    pub fn clear(&mut self) {
        self.m_shield_vitality_traits.clear();
        self.m_weapon_traits.clear();
        self.m_movement_traits.clear();
        self.m_appearance_traits.clear();
        self.m_sensor_traits.clear();
    }

    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_enum(self.m_shield_vitality_traits.m_damage_resistance_percentage_setting)?;
        bitstream.write_enum(self.m_shield_vitality_traits.m_body_multiplier)?;
        bitstream.write_enum(self.m_shield_vitality_traits.m_body_recharge_rate)?;
        bitstream.write_enum(self.m_shield_vitality_traits.m_shield_multiplier)?;
        bitstream.write_enum(self.m_shield_vitality_traits.m_shield_recharge_rate)?;
        bitstream.write_enum(self.m_shield_vitality_traits.m_overshield_recharge_rate)?;
        bitstream.write_enum(self.m_shield_vitality_traits.m_headshot_immunity_setting)?;
        bitstream.write_enum(self.m_shield_vitality_traits.m_vampirism_percentage_setting)?;
        bitstream.write_enum(self.m_shield_vitality_traits.m_assasination_immunity)?;
        bitstream.write_enum(self.m_shield_vitality_traits.m_cannot_die_from_damage)?;
        bitstream.write_enum(self.m_weapon_traits.m_damage_modifier_percentage_setting)?;
        bitstream.write_enum(self.m_weapon_traits.m_melee_damage_modifier_percentage_setting)?;
        bitstream.write_signed_integer(self.m_weapon_traits.m_initial_primary_weapon_absolute_index, 8)?;
        bitstream.write_signed_integer(self.m_weapon_traits.m_initial_secondary_weapon_absolute_index, 8)?;
        bitstream.write_enum(self.m_weapon_traits.m_initial_grenade_count_setting)?;
        bitstream.write_enum(self.m_weapon_traits.m_infinite_ammo_setting)?;
        bitstream.write_enum(self.m_weapon_traits.m_recharging_grenades_setting)?;
        bitstream.write_enum(self.m_weapon_traits.m_weapon_pickup_setting)?;
        bitstream.write_enum(self.m_weapon_traits.m_equipment_usage_setting)?;
        bitstream.write_enum(self.m_weapon_traits.m_equipment_drop_on_death_setting)?;
        bitstream.write_enum(self.m_weapon_traits.m_infinite_equipment_setting)?;
        bitstream.write_signed_integer(self.m_weapon_traits.m_initial_equipment_absolute_index, 8)?;
        bitstream.write_enum(self.m_movement_traits.m_speed_setting)?;
        bitstream.write_enum(self.m_movement_traits.m_gravity_setting)?;
        bitstream.write_enum(self.m_movement_traits.m_vehicle_usage_setting)?;
        bitstream.write_enum(self.m_movement_traits.m_double_jump_setting)?;
        if self.m_movement_traits.m_jump_modifier != -1 {
            bitstream.write_bool(true)?;
            bitstream.write_integer(self.m_movement_traits.m_jump_modifier as u32, 9)?;
        } else {
            bitstream.write_bool(false)?;
        }
        bitstream.write_enum(self.m_appearance_traits.m_active_camo_setting)?;
        bitstream.write_enum(self.m_appearance_traits.m_waypoint_setting)?;
        bitstream.write_enum(self.m_appearance_traits.m_gamertag_setting)?;
        bitstream.write_enum(self.m_appearance_traits.m_aura_setting)?;
        bitstream.write_enum(self.m_appearance_traits.m_forced_change_color_setting)?;
        bitstream.write_enum(self.m_sensor_traits.m_motion_tracker_setting)?;
        bitstream.write_enum(self.m_sensor_traits.m_motion_tracker_range_setting)?;
        bitstream.write_enum(self.m_sensor_traits.m_directional_damage_setting)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_shield_vitality_traits.m_damage_resistance_percentage_setting = bitstream.read_enum("damage-resistance")?;
        self.m_shield_vitality_traits.m_body_multiplier = bitstream.read_enum("body-multiplier")?;
        self.m_shield_vitality_traits.m_body_recharge_rate = bitstream.read_enum("body-recharge-rate")?;
        self.m_shield_vitality_traits.m_shield_multiplier = bitstream.read_enum("shield-multiplier")?;
        self.m_shield_vitality_traits.m_shield_recharge_rate = bitstream.read_enum("shield-recharge-rate")?;
        self.m_shield_vitality_traits.m_overshield_recharge_rate = bitstream.read_enum("overshield-recharge-rate")?;
        self.m_shield_vitality_traits.m_headshot_immunity_setting = bitstream.read_enum("headshot-immunity")?;
        self.m_shield_vitality_traits.m_vampirism_percentage_setting = bitstream.read_enum("vampirism")?;
        self.m_shield_vitality_traits.m_assasination_immunity = bitstream.read_enum("assasination-immunity")?;
        self.m_shield_vitality_traits.m_cannot_die_from_damage = bitstream.read_enum("cannot-die-from-damage")?;
        self.m_weapon_traits.m_damage_modifier_percentage_setting = bitstream.read_enum("damage-modifier")?;
        self.m_weapon_traits.m_melee_damage_modifier_percentage_setting = bitstream.read_enum("melee-damage-modifier")?;
        self.m_weapon_traits.m_initial_primary_weapon_absolute_index = bitstream.read_signed_integer("player-trait-initial-primary-weapon", 8)?;
        self.m_weapon_traits.m_initial_secondary_weapon_absolute_index = bitstream.read_signed_integer("player-trait-initial-secondary-weapon", 8)?;
        self.m_weapon_traits.m_initial_grenade_count_setting = bitstream.read_enum("player-trait-initial-grenade-count")?;
        self.m_weapon_traits.m_infinite_ammo_setting = bitstream.read_enum("player-traits-infinite-ammo-setting")?;
        self.m_weapon_traits.m_recharging_grenades_setting = bitstream.read_enum("player-traits-recharging-grenades")?;
        self.m_weapon_traits.m_weapon_pickup_setting = bitstream.read_enum("player-traits-weapon-pickup-allowed")?;
        self.m_weapon_traits.m_equipment_usage_setting = bitstream.read_enum("player-traits-equipment-usage")?;
        self.m_weapon_traits.m_equipment_drop_on_death_setting = bitstream.read_enum("player-traits-equipment-drop")?;
        self.m_weapon_traits.m_infinite_equipment_setting = bitstream.read_enum("player-traits-infinite-equipment")?;
        self.m_weapon_traits.m_initial_equipment_absolute_index = bitstream.read_signed_integer("player-trait-initial-equipment", 8)?;
        self.m_movement_traits.m_speed_setting = bitstream.read_enum("player-speed")?;
        self.m_movement_traits.m_gravity_setting = bitstream.read_enum("player-gravity")?;
        self.m_movement_traits.m_vehicle_usage_setting = bitstream.read_enum("player-traits-movement-vehicle-usage")?;
        self.m_movement_traits.m_double_jump_setting = bitstream.read_enum("player-traits-movement-double-jump")?;
        if bitstream.read_bool("player-traits-movement-jump-modifier-changed")? {
            self.m_movement_traits.m_jump_modifier = bitstream.read_integer("player-traits-movement-jump-modifier", 9)?;
        } else {
            self.m_movement_traits.m_jump_modifier = -1;
        }
        self.m_appearance_traits.m_active_camo_setting = bitstream.read_enum("player-traits-appearance-active-camo")?;
        self.m_appearance_traits.m_waypoint_setting = bitstream.read_enum("player-traits-appearance-waypoint")?;
        self.m_appearance_traits.m_gamertag_setting = bitstream.read_enum("player-traits-appearance-gamertag")?;
        self.m_appearance_traits.m_aura_setting = bitstream.read_enum("player-traits-appearance-aura")?;
        self.m_appearance_traits.m_forced_change_color_setting = bitstream.read_enum("player-traits-appearance-forced-change-color")?;
        self.m_sensor_traits.m_motion_tracker_setting = bitstream.read_enum("player-traits-sensors-motion-tracker")?;
        self.m_sensor_traits.m_motion_tracker_range_setting = bitstream.read_enum("motion-tracker-range")?;
        self.m_sensor_traits.m_directional_damage_setting = bitstream.read_enum("player-traits-sensors-directional-damage")?;

        Ok(())
    }
}