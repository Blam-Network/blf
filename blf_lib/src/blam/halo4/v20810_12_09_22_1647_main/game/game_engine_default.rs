use serde::{Deserialize, Serialize};
use num_derive::{FromPrimitive, ToPrimitive};
use blf_lib::bitfield;
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::game::game_engine_player_traits::c_player_traits;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::game::game_engine_loadout_traits::c_game_engine_loadout_traits;
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::game::game_engine_team::c_game_engine_team_options;
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::game::game_engine_traits::{c_game_engine_miscellaneous_options, c_game_engine_respawn_options};
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::game::game_engine_h4_options::{
    c_game_engine_prototype_options, c_game_engine_requisition_options, GameEngineOrdnanceOptions,
};
pub use blf_lib::blam::halo4::v20810_12_09_22_1647_main::game::game_engine_h4_options::MapLoadoutInfo;
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::saved_games::saved_game_files::c_content_item_metadata;
use blf_lib_derivable::result::BLFLibResult;

pub const k_game_engine_type_count: usize = 11;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(2)]
pub enum e_team_changing_type {
    #[default]
    disabled = 0,
    enabled = 1,
    balancing_only = 2,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(3)]
pub enum e_team_scoring_method {
    #[default]
    sum = 0,
    minimum = 1,
    maximum = 2,
}

bitfield! {
    #[derive(Serialize, Deserialize)]
    pub struct e_map_override_option_flags: u8 {
        grenades_on_map,
        shortcuts_on_map,
        equipment_on_map,
        powerups_on_map,
        turrets_on_map,
        indestructible_vehicles,
    }
}

bitfield! {
    #[derive(Serialize, Deserialize)]
    pub struct e_game_engine_social_options_flags: u8 {
        friendly_fire_enabled,
        betrayal_booting_enabled,
        enemy_voice_enabled,
        open_channel_voice_enabled,
        dead_player_voice_enabled,
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_game_engine_social_options {
    pub m_flags: e_game_engine_social_options_flags,
    pub m_team_changing: e_team_changing_type,
    pub m_observers_enabled: bool,
}

impl c_game_engine_social_options {
    pub fn initialize(&mut self) {
        *self = Self::default();
        self.m_flags.friendly_fire_enabled = true;
        self.m_flags.betrayal_booting_enabled = true;
        self.m_flags.enemy_voice_enabled = true;
        self.m_team_changing = e_team_changing_type::enabled;
    }

    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_bool(self.m_observers_enabled)?; // TODO: what is this
        bitstream.write_enum(self.m_team_changing)?;
        bitstream.write_bool(self.m_flags.friendly_fire_enabled)?;
        bitstream.write_bool(self.m_flags.betrayal_booting_enabled)?;
        bitstream.write_bool(self.m_flags.enemy_voice_enabled)?;
        bitstream.write_bool(self.m_flags.open_channel_voice_enabled)?;
        bitstream.write_bool(self.m_flags.dead_player_voice_enabled)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_observers_enabled = bitstream.read_unnamed_bool()?;
        self.m_team_changing = bitstream.read_enum("team-changing")?;
        self.m_flags.friendly_fire_enabled = bitstream.read_unnamed_bool()?;
        self.m_flags.betrayal_booting_enabled = bitstream.read_unnamed_bool()?;
        self.m_flags.enemy_voice_enabled = bitstream.read_unnamed_bool()?;
        self.m_flags.open_channel_voice_enabled = bitstream.read_unnamed_bool()?;
        self.m_flags.dead_player_voice_enabled = bitstream.read_unnamed_bool()?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct PowerupEffectData {
    pub m_traits: c_player_traits,
    pub m_duration_seconds: u8,
}

impl PowerupEffectData {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_traits.encode(bitstream)?;
        bitstream.write_integer(self.m_duration_seconds as u32, 7)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_traits.decode(bitstream)?;
        self.m_duration_seconds = bitstream.read_integer("effect-duration", 7)?;
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct PowerupData {
    pub m_effects: [PowerupEffectData; 2],
}

impl PowerupData {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        for effect in &self.m_effects {
            effect.encode(bitstream)?;
        }
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        for effect in &mut self.m_effects {
            effect.decode(bitstream)?;
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_game_engine_map_override_options {
    pub m_flags: e_map_override_option_flags,
    pub m_base_player_traits: c_player_traits,
    pub m_weapon_set_absolute_index: i16,
    pub m_vehicle_set_absolute_index: i16,
    pub m_equipment_set_absolute_index: i16,
    pub m_powerups: [PowerupData; 4],
}

impl c_game_engine_map_override_options {
    pub fn initialize(&mut self) {
        *self = Self::default();
        self.m_flags.grenades_on_map = true;
        self.m_flags.shortcuts_on_map = true;
        self.m_flags.equipment_on_map = true;
        self.m_flags.powerups_on_map = true;
        self.m_flags.turrets_on_map = true;
        self.m_base_player_traits.clear();
        self.m_weapon_set_absolute_index = -2;
        self.m_vehicle_set_absolute_index = -2;
        self.m_equipment_set_absolute_index = -2;
    }

    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_flags.to_raw(), 6)?;
        self.m_base_player_traits.encode(bitstream)?;
        bitstream.write_signed_integer(self.m_weapon_set_absolute_index as i32, 8)?;
        bitstream.write_signed_integer(self.m_vehicle_set_absolute_index as i32, 8)?;
        bitstream.write_signed_integer(self.m_equipment_set_absolute_index as i32, 8)?;
        for powerup in &self.m_powerups {
            powerup.encode(bitstream)?;
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_flags = e_map_override_option_flags::from_raw(bitstream.read_integer("flags", 6)?);
        self.m_base_player_traits.decode(bitstream)?;
        self.m_weapon_set_absolute_index =
            bitstream.read_signed_integer("map-override-weapon-set", 8)?;
        self.m_vehicle_set_absolute_index =
            bitstream.read_signed_integer("map-override-vehicle-set", 8)?;
        self.m_equipment_set_absolute_index =
            bitstream.read_signed_integer("map-override-equipment-set", 8)?;
        for powerup in &mut self.m_powerups {
            powerup.decode(bitstream)?;
        }

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_game_engine_base_variant {
    pub m_metadata: c_content_item_metadata,
    pub m_built_in: bool,
    pub m_user_created: bool,
    pub m_miscellaneous_options: c_game_engine_miscellaneous_options,
    pub m_prototype_options: c_game_engine_prototype_options,
    pub m_respawn_options: c_game_engine_respawn_options,
    pub m_social_options: c_game_engine_social_options,
    pub m_map_override_options: c_game_engine_map_override_options,
    pub m_requisition_options: c_game_engine_requisition_options,
    pub m_infinity_mission_id: u32,
    pub m_team_options: c_game_engine_team_options,
    pub m_loadouts: c_game_engine_loadout_traits,
    pub m_ordnance_options: GameEngineOrdnanceOptions,
}

impl c_game_engine_base_variant {
    pub fn initialize(&mut self) {
        self.m_metadata.content_item_metadata_set_defaults();
        self.m_miscellaneous_options.initialize();
        self.m_respawn_options.initialize();
        self.m_social_options.initialize();
        self.m_map_override_options.initialize();
        self.m_team_options.initialize();
        self.m_loadouts.initialize();
        self.m_built_in = false;
        self.m_user_created = false;
        self.m_infinity_mission_id = 0;
    }

    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_metadata.encode(bitstream)?;
        bitstream.write_bool(self.m_built_in)?;
        bitstream.write_bool(self.m_user_created)?;
        self.m_miscellaneous_options.encode(bitstream)?;
        self.m_prototype_options.encode(bitstream)?;
        self.m_respawn_options.encode(bitstream)?;
        self.m_social_options.encode(bitstream)?;
        self.m_map_override_options.encode(bitstream)?;
        self.m_requisition_options.encode(bitstream)?;
        bitstream.write_integer(self.m_infinity_mission_id, 32)?;
        self.m_team_options.encode(bitstream)?;
        self.m_loadouts.encode(bitstream)?;
        self.m_ordnance_options.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_metadata.decode(bitstream)?;
        self.m_built_in = bitstream.read_bool("variant-built-in")?;
        self.m_user_created = bitstream.read_bool("variant-user-created")?;
        self.m_miscellaneous_options.decode(bitstream)?;
        self.m_prototype_options.decode(bitstream)?;
        self.m_respawn_options.decode(bitstream)?;
        self.m_social_options.decode(bitstream)?;
        self.m_map_override_options.decode(bitstream)?;
        self.m_requisition_options.decode(bitstream)?;
        self.m_infinity_mission_id = bitstream.read_integer("infinity-mission-id", 32)?;
        self.m_team_options.decode(bitstream)?;
        self.m_loadouts.decode(bitstream)?;
        self.m_ordnance_options.decode(bitstream)?;

        Ok(())
    }
}
