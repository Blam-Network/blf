use binrw::{BinRead, BinWrite};
#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct c_player_trait_weapons {
    pub m_initial_grenade_count_setting: u16,
    pub m_initial_primary_weapon_absolute_index: i8,
    pub m_initial_secondary_weapon_absolute_index: i8,
    pub m_damage_modifier_percentage_setting: u8,
    pub m_recharging_grenades_setting: u8,
    pub m_infinite_ammo_setting: u8,
    pub m_weapon_pickup_setting: u8,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct c_player_trait_shield_vitality {
    pub m_damage_resistance_percentage_setting: u8,
    pub m_shield_recharge_rate_percentage_setting: u8,
    pub m_vampirism_percentage_setting: u8,
    pub m_headshot_immunity_setting: u8,
    #[brw(pad_after = 3)]
    pub m_shield_multiplier_setting: u8,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct c_player_trait_movement {
    pub m_speed_setting: u8,
    pub m_gravity_setting: u8,
    #[brw(pad_after = 1)]
    pub m_vehicle_usage_setting: u8,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct c_player_trait_appearance {
    pub m_active_camo_setting: u8,
    pub m_waypoint_setting: u8,
    pub m_aura_setting: u8,
    pub m_forced_change_color_setting: u8,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct c_player_trait_sensors {
    pub m_motion_tracker_setting: u16,
    pub m_motion_tracker_range_setting: u16,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct c_player_traits {
    pub m_shield_vitality_traits: c_player_trait_shield_vitality,
    pub m_weapon_traits: c_player_trait_weapons,
    pub m_movement_traits: c_player_trait_movement,
    pub m_appearance_traits: c_player_trait_appearance,
    pub m_sensor_traits: c_player_trait_sensors,
}

impl c_player_traits {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_shield_vitality_traits.m_damage_resistance_percentage_setting as u32, 4)?;
        bitstream.write_integer(self.m_shield_vitality_traits.m_shield_recharge_rate_percentage_setting as u32, 4)?;
        bitstream.write_integer(self.m_shield_vitality_traits.m_vampirism_percentage_setting as u32, 3)?;
        bitstream.write_integer(self.m_shield_vitality_traits.m_headshot_immunity_setting as u32, 2)?;
        bitstream.write_integer(self.m_shield_vitality_traits.m_shield_multiplier_setting as u32, 3)?;
        bitstream.write_integer(self.m_weapon_traits.m_damage_modifier_percentage_setting as u32, 4)?;
        bitstream.write_signed_integer(self.m_weapon_traits.m_initial_primary_weapon_absolute_index as i32, 8)?;
        bitstream.write_signed_integer(self.m_weapon_traits.m_initial_secondary_weapon_absolute_index as i32, 8)?;
        bitstream.write_integer(self.m_weapon_traits.m_initial_grenade_count_setting as u32, 2)?;
        bitstream.write_integer(self.m_weapon_traits.m_infinite_ammo_setting as u32, 2)?;
        bitstream.write_integer(self.m_weapon_traits.m_recharging_grenades_setting as u32, 2)?;
        bitstream.write_integer(self.m_weapon_traits.m_weapon_pickup_setting as u32, 2)?;
        bitstream.write_integer(self.m_movement_traits.m_speed_setting as u32, 4)?;
        bitstream.write_integer(self.m_movement_traits.m_gravity_setting as u32, 3)?;
        bitstream.write_integer(self.m_movement_traits.m_vehicle_usage_setting as u32, 2)?;
        bitstream.write_integer(self.m_appearance_traits.m_active_camo_setting as u32, 3)?;
        bitstream.write_integer(self.m_appearance_traits.m_waypoint_setting as u32, 2)?;
        bitstream.write_integer(self.m_appearance_traits.m_aura_setting as u32, 3)?;
        bitstream.write_integer(self.m_appearance_traits.m_forced_change_color_setting as u32, 4)?;
        bitstream.write_integer(self.m_sensor_traits.m_motion_tracker_setting as u32, 3)?;
        bitstream.write_integer(self.m_sensor_traits.m_motion_tracker_range_setting as u32, 3)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_shield_vitality_traits.m_damage_resistance_percentage_setting = bitstream.read_integer(4)? as u8;
        self.m_shield_vitality_traits.m_shield_recharge_rate_percentage_setting = bitstream.read_integer(4)? as u8;
        self.m_shield_vitality_traits.m_vampirism_percentage_setting = bitstream.read_integer(3)? as u8;
        self.m_shield_vitality_traits.m_headshot_immunity_setting = bitstream.read_integer(2)? as u8;
        self.m_shield_vitality_traits.m_shield_multiplier_setting = bitstream.read_integer(3)? as u8;
        self.m_weapon_traits.m_damage_modifier_percentage_setting = bitstream.read_integer(4)? as u8;
        self.m_weapon_traits.m_initial_primary_weapon_absolute_index = bitstream.read_signed_integer(8)? as i8;
        self.m_weapon_traits.m_initial_secondary_weapon_absolute_index = bitstream.read_signed_integer(8)? as i8;
        self.m_weapon_traits.m_initial_grenade_count_setting = bitstream.read_integer(2)? as u16;
        self.m_weapon_traits.m_infinite_ammo_setting = bitstream.read_integer(2)? as u8;
        self.m_weapon_traits.m_recharging_grenades_setting = bitstream.read_integer(2)? as u8;
        self.m_weapon_traits.m_weapon_pickup_setting = bitstream.read_integer(2)? as u8;
        self.m_movement_traits.m_speed_setting = bitstream.read_integer(4)? as u8;
        self.m_movement_traits.m_gravity_setting = bitstream.read_integer(3)? as u8;
        self.m_movement_traits.m_vehicle_usage_setting = bitstream.read_integer(2)? as u8;
        self.m_appearance_traits.m_active_camo_setting = bitstream.read_integer(3)? as u8;
        self.m_appearance_traits.m_waypoint_setting = bitstream.read_integer(2)? as u8;
        self.m_appearance_traits.m_aura_setting = bitstream.read_integer(3)? as u8;
        self.m_appearance_traits.m_forced_change_color_setting = bitstream.read_integer(4)? as u8;
        self.m_sensor_traits.m_motion_tracker_setting = bitstream.read_integer(3)? as u16;
        self.m_sensor_traits.m_motion_tracker_range_setting = bitstream.read_integer(3)? as u16;

        Ok(())
    }
}