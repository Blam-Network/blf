//! Halo 4 player traits — float-based bitstream matching retail / blf-ts.
use serde::{Deserialize, Serialize};
use num_derive::{FromPrimitive, ToPrimitive};
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::memory::bitstream_reader::c_bitstream_reader_extensions;
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::memory::bitstream_writer::c_bitstream_writer_extensions;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;

/// Quantized trait floats: bool presence + 16-bit real in [-200, 200], exact mid/endpoints.
const k_trait_float_bits: usize = 16;
const k_trait_float_min: f32 = -200.0;
const k_trait_float_max: f32 = 200.0;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_player_trait_float {
    pub m_enabled: bool,
    pub m_value: f32,
}

impl c_player_trait_float {
    pub fn clear(&mut self) {
        self.m_enabled = false;
        self.m_value = 0.0;
    }

    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_bool(self.m_enabled)?;
        if self.m_enabled {
            bitstream.write_quantized_real(
                self.m_value,
                k_trait_float_min,
                k_trait_float_max,
                k_trait_float_bits,
                true,
                true,
            )?;
        }
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader, name: &str) -> BLFLibResult {
        self.m_enabled = bitstream.read_bool(name)?;
        if self.m_enabled {
            self.m_value = bitstream
                .read_quantized_real(
                    k_trait_float_min,
                    k_trait_float_max,
                    k_trait_float_bits,
                    true,
                    true,
                )?
                .0;
        } else {
            self.m_value = 0.0;
        }
        Ok(())
    }
}

/// `e_bool_player_trait_choices` (0..2), 2 bits. Alias kept as `e_boolean_trait`.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(2)]
pub enum e_boolean_trait {
    #[default]
    unchanged = 0,
    off = 1,
    on = 2,
}

/// `e_player_trait_vitality_powerup_cancellation` (0..2), 2 bits.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(2)]
pub enum e_player_trait_vitality_powerup_cancellation {
    #[default]
    unchanged = 0,
    unknown_1 = 1,
    unknown_2 = 2,
}

/// `e_player_trait_weapons_initial_grenade_count_choices` (0..18), 5 bits.
/// H4 engine names: `1_frag`/`2_frag`, `1_plasma`/`2_plasma`, `1_type2`..`2_type7`.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(5)]
pub enum e_grenade_count_setting {
    #[default]
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

/// `e_player_trait_weapons_infinite_ammo_choices` (0..3), 2 bits.
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

/// `e_player_trait_weapons_equipment_usage_choices` (0..3), 2 bits.
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

/// `e_player_trait_movement_vehicle_usage_choices` (0..8), 4 bits.
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

/// `e_player_trait_movement_double_jump_choices` (0..3), 2 bits.
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

/// `e_player_trait_appearance_active_camo_choices` (0..5), 3 bits.
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

/// `e_waypoint_setting` (0..3), 2 bits.
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

/// `e_player_trait_appearance_aura_choices` (0..4), 3 bits.
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

/// `e_player_trait_sensors_motion_tracker_choices` (0..4), 3 bits.
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

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct PlayerTraitChangeColor {
    pub m_override: bool,
    pub m_red: u8,
    pub m_green: u8,
    pub m_blue: u8,
}

impl PlayerTraitChangeColor {
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_bool(self.m_override)?;
        bitstream.write_integer(self.m_red, 8)?;
        bitstream.write_integer(self.m_green, 8)?;
        bitstream.write_integer(self.m_blue, 8)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_override = bitstream.read_bool("flags")?;
        self.m_red = bitstream.read_integer("red", 8)?;
        self.m_green = bitstream.read_integer("green", 8)?;
        self.m_blue = bitstream.read_integer("blue", 8)?;
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct PlayerTraitModelVariant {
    pub m_override: bool,
    pub m_model: u8,
}

impl PlayerTraitModelVariant {
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_bool(self.m_override)?;
        bitstream.write_integer(self.m_model, 8)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_override = bitstream.read_bool("flags")?;
        self.m_model = bitstream.read_integer("model", 8)?;
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_player_trait_shield_vitality {
    pub m_damage_resistance: c_player_trait_float,
    pub m_shield_multiplier: c_player_trait_float,
    pub m_body_multiplier: c_player_trait_float,
    pub m_shield_stun_duration: c_player_trait_float,
    pub m_shield_recharge_rate: c_player_trait_float,
    pub m_body_recharge_rate: c_player_trait_float,
    pub m_overshield_recharge_rate: c_player_trait_float,
    pub m_vampirism_percent: c_player_trait_float,
    pub m_explosive_damage_resistance: c_player_trait_float,
    pub m_wheelman_vehicle_stun_time: c_player_trait_float,
    pub m_wheelman_vehicle_recharge_time: c_player_trait_float,
    pub m_wheelman_vehicle_emp_disabled_time: c_player_trait_float,
    pub m_fall_damage_multiplier: c_player_trait_float,
    pub m_headshot_immunity: e_boolean_trait,
    pub m_assassination_immunity: e_boolean_trait,
    pub m_deathless: e_boolean_trait,
    pub m_fast_track_armor: e_boolean_trait,
    pub m_powerup_cancellation: e_player_trait_vitality_powerup_cancellation,
}

impl c_player_trait_shield_vitality {
    pub fn clear(&mut self) {
        self.m_damage_resistance.clear();
        self.m_shield_multiplier.clear();
        self.m_body_multiplier.clear();
        self.m_shield_stun_duration.clear();
        self.m_shield_recharge_rate.clear();
        self.m_body_recharge_rate.clear();
        self.m_overshield_recharge_rate.clear();
        self.m_vampirism_percent.clear();
        self.m_explosive_damage_resistance.clear();
        self.m_wheelman_vehicle_stun_time.clear();
        self.m_wheelman_vehicle_recharge_time.clear();
        self.m_wheelman_vehicle_emp_disabled_time.clear();
        self.m_fall_damage_multiplier.clear();
        self.m_headshot_immunity = e_boolean_trait::unchanged;
        self.m_assassination_immunity = e_boolean_trait::unchanged;
        self.m_deathless = e_boolean_trait::unchanged;
        self.m_fast_track_armor = e_boolean_trait::unchanged;
        self.m_powerup_cancellation = e_player_trait_vitality_powerup_cancellation::unchanged;
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_player_trait_weapons {
    pub m_damage_multiplier: c_player_trait_float,
    pub m_melee_damage_multiplier: c_player_trait_float,
    pub m_grenade_recharge_frag: c_player_trait_float,
    pub m_grenade_recharge_plasma: c_player_trait_float,
    pub m_grenade_recharge_spike: c_player_trait_float,
    pub m_hero_equipment_energy_use_rate: c_player_trait_float,
    pub m_hero_equipment_energy_recharge_delay: c_player_trait_float,
    pub m_hero_equipment_energy_recharge_rate: c_player_trait_float,
    pub m_hero_equipment_initial_energy: c_player_trait_float,
    pub m_equipment_energy_use_rate: c_player_trait_float,
    pub m_equipment_energy_recharge_delay: c_player_trait_float,
    pub m_equipment_energy_recharge_rate: c_player_trait_float,
    pub m_equipment_energy_initial_energy: c_player_trait_float,
    pub m_switch_speed_modifier: c_player_trait_float,
    pub m_reload_speed_modifier: c_player_trait_float,
    pub m_ordnance_points_modifier: c_player_trait_float,
    pub m_explosive_aoe_radius_modifier: c_player_trait_float,
    pub m_gunner_armor_modifier: c_player_trait_float,
    pub m_stability_armor_modifier: c_player_trait_float,
    pub m_drop_recon_warning_seconds: c_player_trait_float,
    pub m_drop_recon_distance_modifier: c_player_trait_float,
    pub m_assassination_speed_modifier: c_player_trait_float,
    pub m_weapon_pickup_allowed: e_boolean_trait,
    pub m_initial_grenade_count_setting: e_grenade_count_setting,
    pub m_infinite_ammo_setting: e_infinite_ammo_setting,
    pub m_equipment_usage_setting: e_equipment_usage_setting,
    pub m_equipment_usage_except_auto_turret: e_equipment_usage_setting,
    pub m_equipment_drop: e_boolean_trait,
    pub m_infinite_equipment: e_boolean_trait,
    pub m_ammopack: e_boolean_trait,
    pub m_grenadier: e_boolean_trait,
    pub m_explode_on_death_armormod: e_boolean_trait,
    pub m_ordnance_markers_visible: e_boolean_trait,
    pub m_ordnance_reroll_available: e_boolean_trait,
    pub m_resourceful: e_boolean_trait,
    pub m_well_equipped: e_boolean_trait,
    pub m_ordnance_disabled: e_boolean_trait,
    pub m_initial_primary_weapon_absolute_index: i8,
    pub m_initial_secondary_weapon_absolute_index: i8,
    pub m_initial_equipment_absolute_index: i8,
    pub m_initial_tactical_package_absolute_index: i8,
    pub m_initial_support_upgrade_absolute_index: i8,
}

impl c_player_trait_weapons {
    pub fn clear(&mut self) {
        self.m_damage_multiplier.clear();
        self.m_melee_damage_multiplier.clear();
        self.m_grenade_recharge_frag.clear();
        self.m_grenade_recharge_plasma.clear();
        self.m_grenade_recharge_spike.clear();
        self.m_hero_equipment_energy_use_rate.clear();
        self.m_hero_equipment_energy_recharge_delay.clear();
        self.m_hero_equipment_energy_recharge_rate.clear();
        self.m_hero_equipment_initial_energy.clear();
        self.m_equipment_energy_use_rate.clear();
        self.m_equipment_energy_recharge_delay.clear();
        self.m_equipment_energy_recharge_rate.clear();
        self.m_equipment_energy_initial_energy.clear();
        self.m_switch_speed_modifier.clear();
        self.m_reload_speed_modifier.clear();
        self.m_ordnance_points_modifier.clear();
        self.m_explosive_aoe_radius_modifier.clear();
        self.m_gunner_armor_modifier.clear();
        self.m_stability_armor_modifier.clear();
        self.m_drop_recon_warning_seconds.clear();
        self.m_drop_recon_distance_modifier.clear();
        self.m_assassination_speed_modifier.clear();
        self.m_weapon_pickup_allowed = e_boolean_trait::unchanged;
        self.m_initial_grenade_count_setting = e_grenade_count_setting::unchanged;
        self.m_infinite_ammo_setting = e_infinite_ammo_setting::unchanged;
        self.m_equipment_usage_setting = e_equipment_usage_setting::unchanged;
        self.m_equipment_usage_except_auto_turret = e_equipment_usage_setting::unchanged;
        self.m_equipment_drop = e_boolean_trait::unchanged;
        self.m_infinite_equipment = e_boolean_trait::unchanged;
        self.m_ammopack = e_boolean_trait::unchanged;
        self.m_grenadier = e_boolean_trait::unchanged;
        self.m_explode_on_death_armormod = e_boolean_trait::unchanged;
        self.m_ordnance_markers_visible = e_boolean_trait::unchanged;
        self.m_ordnance_reroll_available = e_boolean_trait::unchanged;
        self.m_resourceful = e_boolean_trait::unchanged;
        self.m_well_equipped = e_boolean_trait::unchanged;
        self.m_ordnance_disabled = e_boolean_trait::unchanged;
        self.m_initial_primary_weapon_absolute_index = -3;
        self.m_initial_secondary_weapon_absolute_index = -3;
        self.m_initial_equipment_absolute_index = -3;
        self.m_initial_tactical_package_absolute_index = -3;
        self.m_initial_support_upgrade_absolute_index = -3;
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_player_trait_movement {
    pub m_speed: c_player_trait_float,
    pub m_gravity: c_player_trait_float,
    pub m_jump_multiplier: c_player_trait_float,
    pub m_turn_speed_multiplier: c_player_trait_float,
    pub m_vehicle_usage_setting: e_vehicle_usage_setting,
    pub m_double_jump_setting: e_double_jump_setting,
    pub m_sprint_usage: e_boolean_trait,
    pub m_automatic_momentum_usage: e_boolean_trait,
    pub m_vaulting_enabled: e_boolean_trait,
    pub m_stealthy: e_boolean_trait,
}

impl c_player_trait_movement {
    pub fn clear(&mut self) {
        self.m_speed.clear();
        self.m_gravity.clear();
        self.m_jump_multiplier.clear();
        self.m_turn_speed_multiplier.clear();
        self.m_vehicle_usage_setting = e_vehicle_usage_setting::unchanged;
        self.m_double_jump_setting = e_double_jump_setting::unchanged;
        self.m_sprint_usage = e_boolean_trait::unchanged;
        self.m_automatic_momentum_usage = e_boolean_trait::unchanged;
        self.m_vaulting_enabled = e_boolean_trait::unchanged;
        self.m_stealthy = e_boolean_trait::unchanged;
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_player_trait_appearance {
    pub m_player_scale: c_player_trait_float,
    pub m_active_camo_setting: e_active_camo_setting,
    pub m_waypoint_setting: e_waypoint_setting,
    pub m_gamertag_setting: e_waypoint_setting,
    pub m_aura_setting: e_aura_setting,
    pub m_primary_color: PlayerTraitChangeColor,
    pub m_secondary_color: PlayerTraitChangeColor,
    pub m_model_variant: PlayerTraitModelVariant,
    pub m_death_effect: i32,
    pub m_looping_effect: i32,
    pub m_shield_hud: e_boolean_trait,
}

impl c_player_trait_appearance {
    pub fn clear(&mut self) {
        self.m_player_scale.clear();
        self.m_active_camo_setting = e_active_camo_setting::off;
        self.m_waypoint_setting = e_waypoint_setting::unchanged;
        self.m_gamertag_setting = e_waypoint_setting::unchanged;
        self.m_aura_setting = e_aura_setting::unchanged;
        self.m_primary_color.clear();
        self.m_secondary_color.clear();
        self.m_model_variant.clear();
        self.m_death_effect = -1;
        self.m_looping_effect = -1;
        self.m_shield_hud = e_boolean_trait::unchanged;
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_player_trait_sensors {
    pub m_motion_tracker_range: c_player_trait_float,
    pub m_nemesis_duration: c_player_trait_float,
    pub m_motion_tracker_setting: e_motion_tracker_setting,
    pub m_motion_tracker_while_zoomed: e_boolean_trait,
    pub m_directional_damage_indicator: e_boolean_trait,
    pub m_vision_mode: e_boolean_trait,
    pub m_battle_awareness: e_boolean_trait,
    pub m_threat_view: e_boolean_trait,
    pub m_aural_enhancement: e_boolean_trait,
    pub m_nemesis: e_boolean_trait,
}

impl c_player_trait_sensors {
    pub fn clear(&mut self) {
        self.m_motion_tracker_range.clear();
        self.m_nemesis_duration.clear();
        self.m_motion_tracker_setting = e_motion_tracker_setting::unchanged;
        self.m_motion_tracker_while_zoomed = e_boolean_trait::unchanged;
        self.m_directional_damage_indicator = e_boolean_trait::unchanged;
        self.m_vision_mode = e_boolean_trait::unchanged;
        self.m_battle_awareness = e_boolean_trait::unchanged;
        self.m_threat_view = e_boolean_trait::unchanged;
        self.m_aural_enhancement = e_boolean_trait::unchanged;
        self.m_nemesis = e_boolean_trait::unchanged;
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
        let v = &self.m_shield_vitality_traits;
        v.m_damage_resistance.encode(bitstream)?;
        v.m_shield_multiplier.encode(bitstream)?;
        v.m_body_multiplier.encode(bitstream)?;
        v.m_shield_stun_duration.encode(bitstream)?;
        v.m_shield_recharge_rate.encode(bitstream)?;
        v.m_body_recharge_rate.encode(bitstream)?;
        v.m_overshield_recharge_rate.encode(bitstream)?;
        v.m_vampirism_percent.encode(bitstream)?;
        v.m_explosive_damage_resistance.encode(bitstream)?;
        v.m_wheelman_vehicle_stun_time.encode(bitstream)?;
        v.m_wheelman_vehicle_recharge_time.encode(bitstream)?;
        v.m_wheelman_vehicle_emp_disabled_time.encode(bitstream)?;
        v.m_fall_damage_multiplier.encode(bitstream)?;
        bitstream.write_enum(v.m_headshot_immunity)?;
        bitstream.write_enum(v.m_assassination_immunity)?;
        bitstream.write_enum(v.m_deathless)?;
        bitstream.write_enum(v.m_fast_track_armor)?;
        bitstream.write_enum(v.m_powerup_cancellation)?;

        let w = &self.m_weapon_traits;
        w.m_damage_multiplier.encode(bitstream)?;
        w.m_melee_damage_multiplier.encode(bitstream)?;
        w.m_grenade_recharge_frag.encode(bitstream)?;
        w.m_grenade_recharge_plasma.encode(bitstream)?;
        w.m_grenade_recharge_spike.encode(bitstream)?;
        w.m_hero_equipment_energy_use_rate.encode(bitstream)?;
        w.m_hero_equipment_energy_recharge_delay.encode(bitstream)?;
        w.m_hero_equipment_energy_recharge_rate.encode(bitstream)?;
        w.m_hero_equipment_initial_energy.encode(bitstream)?;
        w.m_equipment_energy_use_rate.encode(bitstream)?;
        w.m_equipment_energy_recharge_delay.encode(bitstream)?;
        w.m_equipment_energy_recharge_rate.encode(bitstream)?;
        w.m_equipment_energy_initial_energy.encode(bitstream)?;
        w.m_switch_speed_modifier.encode(bitstream)?;
        w.m_reload_speed_modifier.encode(bitstream)?;
        w.m_ordnance_points_modifier.encode(bitstream)?;
        w.m_explosive_aoe_radius_modifier.encode(bitstream)?;
        w.m_gunner_armor_modifier.encode(bitstream)?;
        w.m_stability_armor_modifier.encode(bitstream)?;
        w.m_drop_recon_warning_seconds.encode(bitstream)?;
        w.m_drop_recon_distance_modifier.encode(bitstream)?;
        w.m_assassination_speed_modifier.encode(bitstream)?;
        bitstream.write_enum(w.m_weapon_pickup_allowed)?;
        bitstream.write_enum(w.m_initial_grenade_count_setting)?;
        bitstream.write_enum(w.m_infinite_ammo_setting)?;
        bitstream.write_enum(w.m_equipment_usage_setting)?;
        bitstream.write_enum(w.m_equipment_usage_except_auto_turret)?;
        bitstream.write_enum(w.m_equipment_drop)?;
        bitstream.write_enum(w.m_infinite_equipment)?;
        bitstream.write_enum(w.m_ammopack)?;
        bitstream.write_enum(w.m_grenadier)?;
        bitstream.write_enum(w.m_explode_on_death_armormod)?;
        bitstream.write_enum(w.m_ordnance_markers_visible)?;
        bitstream.write_enum(w.m_ordnance_reroll_available)?;
        bitstream.write_enum(w.m_resourceful)?;
        bitstream.write_enum(w.m_well_equipped)?;
        bitstream.write_enum(w.m_ordnance_disabled)?;
        bitstream.write_signed_integer(w.m_initial_primary_weapon_absolute_index, 8)?;
        bitstream.write_signed_integer(w.m_initial_secondary_weapon_absolute_index, 8)?;
        bitstream.write_signed_integer(w.m_initial_equipment_absolute_index, 8)?;
        bitstream.write_signed_integer(w.m_initial_tactical_package_absolute_index, 8)?;
        bitstream.write_signed_integer(w.m_initial_support_upgrade_absolute_index, 8)?;

        let m = &self.m_movement_traits;
        m.m_speed.encode(bitstream)?;
        m.m_gravity.encode(bitstream)?;
        m.m_jump_multiplier.encode(bitstream)?;
        m.m_turn_speed_multiplier.encode(bitstream)?;
        bitstream.write_enum(m.m_vehicle_usage_setting)?;
        bitstream.write_enum(m.m_double_jump_setting)?;
        bitstream.write_enum(m.m_sprint_usage)?;
        bitstream.write_enum(m.m_automatic_momentum_usage)?;
        bitstream.write_enum(m.m_vaulting_enabled)?;
        bitstream.write_enum(m.m_stealthy)?;

        let a = &self.m_appearance_traits;
        a.m_player_scale.encode(bitstream)?;
        bitstream.write_enum(a.m_active_camo_setting)?;
        bitstream.write_enum(a.m_waypoint_setting)?;
        bitstream.write_enum(a.m_gamertag_setting)?;
        bitstream.write_enum(a.m_aura_setting)?;
        a.m_primary_color.encode(bitstream)?;
        a.m_secondary_color.encode(bitstream)?;
        a.m_model_variant.encode(bitstream)?;
        bitstream.write_signed_integer(a.m_death_effect, 32)?;
        bitstream.write_signed_integer(a.m_looping_effect, 32)?;
        bitstream.write_enum(a.m_shield_hud)?;

        let s = &self.m_sensor_traits;
        s.m_motion_tracker_range.encode(bitstream)?;
        s.m_nemesis_duration.encode(bitstream)?;
        bitstream.write_enum(s.m_motion_tracker_setting)?;
        bitstream.write_enum(s.m_motion_tracker_while_zoomed)?;
        bitstream.write_enum(s.m_directional_damage_indicator)?;
        bitstream.write_enum(s.m_vision_mode)?;
        bitstream.write_enum(s.m_battle_awareness)?;
        bitstream.write_enum(s.m_threat_view)?;
        bitstream.write_enum(s.m_aural_enhancement)?;
        bitstream.write_enum(s.m_nemesis)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        let v = &mut self.m_shield_vitality_traits;
        v.m_damage_resistance
            .decode(bitstream, "has-player-trait-damage-resistance")?;
        v.m_shield_multiplier
            .decode(bitstream, "has-player-trait-shield-multiplier")?;
        v.m_body_multiplier
            .decode(bitstream, "has-player-trait-body-multiplier")?;
        v.m_shield_stun_duration
            .decode(bitstream, "has-player-trait-shield-stun-duration")?;
        v.m_shield_recharge_rate
            .decode(bitstream, "has-player-trait-shield-recharge-rate")?;
        v.m_body_recharge_rate
            .decode(bitstream, "has-player-trait-body-recharge-rate")?;
        v.m_overshield_recharge_rate
            .decode(bitstream, "has-player-trait-overshield-recharge-rate")?;
        v.m_vampirism_percent
            .decode(bitstream, "has-player-trait-vampirism-percent")?;
        v.m_explosive_damage_resistance
            .decode(bitstream, "has-player-trait-explosive-damage-resistance")?;
        v.m_wheelman_vehicle_stun_time.decode(
            bitstream,
            "has-player-trait-wheelman-armor-vehicle-stun-time-modifier",
        )?;
        v.m_wheelman_vehicle_recharge_time.decode(
            bitstream,
            "has-player-trait-wheelman-armor-vehicle-recharge-time-modifier",
        )?;
        v.m_wheelman_vehicle_emp_disabled_time.decode(
            bitstream,
            "has-player-trait-wheelman-armor-vehicle-emp-disabled-time-modifier",
        )?;
        v.m_fall_damage_multiplier
            .decode(bitstream, "has-player-trait-fall-damage-multiplier")?;
        v.m_headshot_immunity = bitstream.read_enum("player-trait-headshot-immunity")?;
        v.m_assassination_immunity =
            bitstream.read_enum("player-trait-assassination-immunity")?;
        v.m_deathless = bitstream.read_enum("player-trait-deathless")?;
        v.m_fast_track_armor = bitstream.read_enum("player-trait-fast-track-armor")?;
        v.m_powerup_cancellation = bitstream.read_enum("player-trait-powerup-cancellation")?;

        let w = &mut self.m_weapon_traits;
        w.m_damage_multiplier
            .decode(bitstream, "has-player-trait-damage-multiplier")?;
        w.m_melee_damage_multiplier
            .decode(bitstream, "has-player-trait-melee-damage-multiplier")?;
        w.m_grenade_recharge_frag
            .decode(bitstream, "has-player-trait-grenade-recharge-seconds-frag")?;
        w.m_grenade_recharge_plasma.decode(
            bitstream,
            "has-player-trait-grenade-recharge-seconds-plasma",
        )?;
        w.m_grenade_recharge_spike.decode(
            bitstream,
            "has-player-trait-grenade-recharge-seconds-spike",
        )?;
        w.m_hero_equipment_energy_use_rate.decode(
            bitstream,
            "has-player-trait-hero-equipment-energy-use-rate-modifier",
        )?;
        w.m_hero_equipment_energy_recharge_delay.decode(
            bitstream,
            "has-player-trait-hero-equipment-energy-recharge-delay-modifier",
        )?;
        w.m_hero_equipment_energy_recharge_rate.decode(
            bitstream,
            "has-player-trait-hero-equipment-energy-recharge-rate-modifier",
        )?;
        w.m_hero_equipment_initial_energy.decode(
            bitstream,
            "has-player-trait-hero-equipment-initial-energy-modifier",
        )?;
        w.m_equipment_energy_use_rate.decode(
            bitstream,
            "has-player-trait-equipment-energy-use-rate-modifier",
        )?;
        w.m_equipment_energy_recharge_delay.decode(
            bitstream,
            "has-player-trait-equipment-energy-recharge-delay-modifier",
        )?;
        w.m_equipment_energy_recharge_rate.decode(
            bitstream,
            "has-player-trait-equipment-energy-use-recharge-rate-modifier",
        )?;
        w.m_equipment_energy_initial_energy.decode(
            bitstream,
            "has-player-trait-equipment-energy-initial-energy-modifier",
        )?;
        w.m_switch_speed_modifier
            .decode(bitstream, "has-player-trait-switch-speed-modifier")?;
        w.m_reload_speed_modifier
            .decode(bitstream, "has-player-trait-reload-speed-modifier")?;
        w.m_ordnance_points_modifier
            .decode(bitstream, "has-player-trait-ordnance-points-modifier")?;
        w.m_explosive_aoe_radius_modifier.decode(
            bitstream,
            "has-player-trait-explosive-area-of-effect-radius-modifier",
        )?;
        w.m_gunner_armor_modifier
            .decode(bitstream, "has-player-trait-gunner-armor-modifier")?;
        w.m_stability_armor_modifier
            .decode(bitstream, "has-player-trait-stability-armor-modifier")?;
        w.m_drop_recon_warning_seconds
            .decode(bitstream, "has-player-trait-drop-recon-warning-seconds")?;
        w.m_drop_recon_distance_modifier
            .decode(bitstream, "has-player-trait-drop-recon-distance-modifier")?;
        w.m_assassination_speed_modifier
            .decode(bitstream, "has-player-trait-assassination-speed-modifier")?;
        w.m_weapon_pickup_allowed =
            bitstream.read_enum("player-trait-weapon-pickup-allowed")?;
        w.m_initial_grenade_count_setting =
            bitstream.read_enum("player-trait-initial-grenade-count")?;
        w.m_infinite_ammo_setting = bitstream.read_enum("player-trait-infinite-ammo")?;
        w.m_equipment_usage_setting = bitstream.read_enum("player-trait-equipment-usage")?;
        w.m_equipment_usage_except_auto_turret =
            bitstream.read_enum("player-trait-equipment-usage-excepting-auto-turret")?;
        w.m_equipment_drop = bitstream.read_enum("player-trait-equipment-drop")?;
        w.m_infinite_equipment = bitstream.read_enum("player-trait-infinite-equipment")?;
        w.m_ammopack = bitstream.read_enum("player-trait-weapons-ammopack")?;
        w.m_grenadier = bitstream.read_enum("player-trait-weapons-grenadier")?;
        w.m_explode_on_death_armormod =
            bitstream.read_enum("player-trait-weapons-explode-on-death-armormod")?;
        w.m_ordnance_markers_visible =
            bitstream.read_enum("player-trait-ordnance-markers-visible")?;
        w.m_ordnance_reroll_available =
            bitstream.read_enum("player-trait-weapons-ordnance-reroll-available")?;
        w.m_resourceful = bitstream.read_enum("player-trait-weapons-resourceful")?;
        w.m_well_equipped = bitstream.read_enum("player-trait-weapons-well-equipped")?;
        w.m_ordnance_disabled = bitstream.read_enum("player-trait-ordnance-disabled")?;
        w.m_initial_primary_weapon_absolute_index =
            bitstream.read_signed_integer("player-trait-initial-primary-weapon", 8)?;
        w.m_initial_secondary_weapon_absolute_index =
            bitstream.read_signed_integer("player-trait-initial-secondary-weapon", 8)?;
        w.m_initial_equipment_absolute_index =
            bitstream.read_signed_integer("player-trait-initial-equipment", 8)?;
        w.m_initial_tactical_package_absolute_index =
            bitstream.read_signed_integer("player-trait-initial-tactical-package", 8)?;
        w.m_initial_support_upgrade_absolute_index =
            bitstream.read_signed_integer("player-trait-initial-support-upgrade", 8)?;

        let m = &mut self.m_movement_traits;
        m.m_speed.decode(bitstream, "has-player-trait-speed")?;
        m.m_gravity
            .decode(bitstream, "has-player-trait-gravity-multiplier")?;
        m.m_jump_multiplier
            .decode(bitstream, "has-player-trait-jump-multiplier")?;
        m.m_turn_speed_multiplier
            .decode(bitstream, "has-player-trait-turn-speed-multiplier")?;
        m.m_vehicle_usage_setting =
            bitstream.read_enum("player-traits-movement-vehicle-usage")?;
        m.m_double_jump_setting = bitstream.read_enum("player-trait-double-jump")?;
        m.m_sprint_usage = bitstream.read_enum("player-trait-sprint-usage")?;
        m.m_automatic_momentum_usage =
            bitstream.read_enum("player-trait-automatic-momentum-usage")?;
        m.m_vaulting_enabled = bitstream.read_enum("player-trait-vaulting-enabled")?;
        m.m_stealthy = bitstream.read_enum("player-trait-stealthy")?;

        let a = &mut self.m_appearance_traits;
        a.m_player_scale
            .decode(bitstream, "has-player-trait-player-scale")?;
        a.m_active_camo_setting = bitstream.read_enum("player-trait-active-camo")?;
        a.m_waypoint_setting = bitstream.read_enum("player-trait-waypoint")?;
        a.m_gamertag_setting = bitstream.read_enum("player-trait-gamertag-visible")?;
        a.m_aura_setting = bitstream.read_enum("player-trait-aura")?;
        a.m_primary_color.decode(bitstream)?;
        a.m_secondary_color.decode(bitstream)?;
        a.m_model_variant.decode(bitstream)?;
        a.m_death_effect = bitstream.read_signed_integer("player-trait-death-effect", 32)?;
        a.m_looping_effect =
            bitstream.read_signed_integer("player-trait-looping-effect", 32)?;
        a.m_shield_hud = bitstream.read_enum("player-trait-shield-hud")?;

        let s = &mut self.m_sensor_traits;
        s.m_motion_tracker_range
            .decode(bitstream, "has-motion-tracker-range")?;
        s.m_nemesis_duration
            .decode(bitstream, "has-nemesis-duration")?;
        s.m_motion_tracker_setting = bitstream.read_enum("player-trait-motion-tracker")?;
        s.m_motion_tracker_while_zoomed =
            bitstream.read_enum("player-trait-motion-tracker-while-zoomed")?;
        s.m_directional_damage_indicator =
            bitstream.read_enum("player-trait-directional-damage-indicator")?;
        s.m_vision_mode = bitstream.read_enum("player-trait-vision-mode")?;
        s.m_battle_awareness = bitstream.read_enum("player-trait-battle-awareness")?;
        s.m_threat_view = bitstream.read_enum("player-trait-threat-view")?;
        s.m_aural_enhancement = bitstream.read_enum("player-trait-aural-enhancement")?;
        s.m_nemesis = bitstream.read_enum("player-trait-nemesis")?;

        Ok(())
    }
}
