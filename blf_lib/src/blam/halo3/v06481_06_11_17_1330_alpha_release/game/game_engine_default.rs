use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib::blam::halo3::v10015_07_05_14_2217_delta::game::game_engine_player_traits::c_player_traits;
use blf_lib::blam::halo3::v10015_07_05_14_2217_delta::game::game_engine_traits::{c_game_engine_miscellaneous_options, c_game_engine_respawn_options};
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::{SET_BIT, TEST_BIT};
use blf_lib::types::c_string::StaticWcharString;
use blf_lib_derivable::result::BLFLibResult;

pub const k_game_engine_type_count: usize = 11;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
pub struct c_game_engine_social_options {
    pub m_flags: u32,
}

impl c_game_engine_social_options {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_bool(TEST_BIT!(self.m_flags, 0))?; // social-options-observers
        bitstream.write_bool(TEST_BIT!(self.m_flags, 1))?; // social-options-team-changing
        bitstream.write_bool(TEST_BIT!(self.m_flags, 2))?; // social-options-team-changing-balancing-only
        bitstream.write_bool(TEST_BIT!(self.m_flags, 3))?; // social-options-friendly-fire
        bitstream.write_bool(TEST_BIT!(self.m_flags, 4))?; // social-options-betrayal-booting
        bitstream.write_bool(TEST_BIT!(self.m_flags, 5))?; // social-options-enemy-voice
        bitstream.write_bool(TEST_BIT!(self.m_flags, 6))?; // social-options-open-channel-voice
        bitstream.write_bool(TEST_BIT!(self.m_flags, 7))?; // social-options-dead-player-voice

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        SET_BIT!(self.m_flags, 0, bitstream.read_bool("social-options-observers")?);
        SET_BIT!(self.m_flags, 1, bitstream.read_bool("social-options-team-changing")?);
        SET_BIT!(self.m_flags, 2, bitstream.read_bool("social-options-team-changing-balancing-only")?);
        SET_BIT!(self.m_flags, 3, bitstream.read_bool("social-options-friendly-fire")?);
        SET_BIT!(self.m_flags, 4, bitstream.read_bool("social-options-betrayal-booting")?);
        SET_BIT!(self.m_flags, 5, bitstream.read_bool("social-options-enemy-voice")?);
        SET_BIT!(self.m_flags, 6, bitstream.read_bool("social-options-open-channel-voice")?);
        SET_BIT!(self.m_flags, 7, bitstream.read_bool("social-options-dead-player-voice")?);

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
pub struct c_game_engine_map_override_options {
    pub m_flags: u32,
    pub m_base_player_traits: c_player_traits,
    pub m_weapon_set_absolute_index: i16,
    pub m_vehicle_set_absolute_index: i16,
    pub m_red_powerup_traits: c_player_traits,
    pub m_blue_powerup_traits: c_player_traits,
    pub m_yellow_powerup_traits: c_player_traits,
    pub m_red_powerup_duration_seconds: u8,
    pub m_blue_powerup_duration_seconds: u8,
    #[brw(pad_after = 1)]
    pub m_yellow_powerup_duration_seconds: u8,
}

impl c_game_engine_map_override_options {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_bool(TEST_BIT!(self.m_flags, 0))?;
        bitstream.write_bool(TEST_BIT!(self.m_flags, 1))?;
        self.m_base_player_traits.encode(bitstream)?;
        bitstream.write_signed_integer(self.m_weapon_set_absolute_index as i32, 8)?;
        bitstream.write_signed_integer(self.m_vehicle_set_absolute_index as i32, 8)?;
        self.m_red_powerup_traits.encode(bitstream)?;
        self.m_blue_powerup_traits.encode(bitstream)?;
        self.m_yellow_powerup_traits.encode(bitstream)?;
        bitstream.write_integer(self.m_red_powerup_duration_seconds as u32, 7)?;
        bitstream.write_integer(self.m_blue_powerup_duration_seconds as u32, 7)?;
        bitstream.write_integer(self.m_yellow_powerup_duration_seconds as u32, 7)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        SET_BIT!(self.m_flags, 0, bitstream.read_bool("map-override-grenades-on-map")?);
        SET_BIT!(self.m_flags, 1, bitstream.read_bool("map-override-indestructible-vehicles")?);
        self.m_base_player_traits.decode(bitstream)?;
        self.m_weapon_set_absolute_index = bitstream.read_signed_integer("map-override-weapon-set", 8)?;
        self.m_vehicle_set_absolute_index = bitstream.read_signed_integer("map-override-vehicle-set", 8)?;
        self.m_red_powerup_traits.decode(bitstream)?;
        self.m_blue_powerup_traits.decode(bitstream)?;
        self.m_yellow_powerup_traits.decode(bitstream)?;
        self.m_red_powerup_duration_seconds = bitstream.read_integer("map-override-red-powerup-duration", 7)?;
        self.m_blue_powerup_duration_seconds = bitstream.read_integer("map-override-blue-powerup-duration", 7)?;
        self.m_yellow_powerup_duration_seconds = bitstream.read_integer("map-override-yellow-powerup-duration", 7)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
pub struct c_game_engine_base_variant {
    #[serde(skip_serializing,skip_deserializing)]
    pub m_checksum: u32,
    pub m_name: StaticWcharString<32>,
    pub m_description: StaticWcharString<32>,
    pub m_miscellaneous_options: c_game_engine_miscellaneous_options,
    pub m_respawn_options: c_game_engine_respawn_options,
    pub m_social_options: c_game_engine_social_options,
    pub m_map_override_options: c_game_engine_map_override_options,
}

impl c_game_engine_base_variant {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult<()> {
        bitstream.write_string_wchar(&self.m_name.get_string(), 32)?;
        bitstream.write_string_wchar(&self.m_description.get_string(), 32)?;
        self.m_miscellaneous_options.encode(bitstream)?;
        self.m_respawn_options.encode(bitstream)?;
        self.m_social_options.encode(bitstream)?;
        self.m_map_override_options.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult<()> {
        self.m_name = StaticWcharString::from_string(bitstream.read_string_wchar(32)?)?;
        self.m_description = StaticWcharString::from_string(bitstream.read_string_wchar(32)?)?;
        self.m_miscellaneous_options.decode(bitstream)?;
        self.m_respawn_options.decode(bitstream)?;
        self.m_social_options.decode(bitstream)?;
        self.m_map_override_options.decode(bitstream)?;

        Ok(())
    }
}