use binrw::{BinRead, BinWrite};
#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use blf_lib::blam::halo3::v12070_08_09_05_2031_halo3_ship::game::game_engine_player_traits::c_player_traits;
use blf_lib::blam::halo3::v12070_08_09_05_2031_halo3_ship::game::game_engine_traits::{c_game_engine_miscellaneous_options, c_game_engine_respawn_options};
use crate::blam::halo3::v12070_08_09_05_2031_halo3_ship::saved_games::saved_game_files::s_content_item_metadata;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::{SET_BIT, TEST_BIT};
use blf_lib_derivable::result::BLFLibResult;

pub const k_game_engine_type_count: usize = 11;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct c_game_engine_social_options {
    pub m_flags: u16,
    pub m_team_changing: u16,
}

impl c_game_engine_social_options {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_bool(false)?; // TODO: what is this
        bitstream.write_integer(self.m_team_changing as u32, 2)?;
        bitstream.write_bool(TEST_BIT!(self.m_flags, 0))?;
        bitstream.write_bool(TEST_BIT!(self.m_flags, 1))?;
        bitstream.write_bool(TEST_BIT!(self.m_flags, 2))?;
        bitstream.write_bool(TEST_BIT!(self.m_flags, 3))?;
        bitstream.write_bool(TEST_BIT!(self.m_flags, 4))?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        bitstream.seek_relative(1)?;
        self.m_team_changing = bitstream.read_unnamed_integer(2)?;
        SET_BIT!(self.m_flags, 0, bitstream.read_unnamed_bool()?);
        SET_BIT!(self.m_flags, 1, bitstream.read_unnamed_bool()?);
        SET_BIT!(self.m_flags, 2, bitstream.read_unnamed_bool()?);
        SET_BIT!(self.m_flags, 3, bitstream.read_unnamed_bool()?);
        SET_BIT!(self.m_flags, 4, bitstream.read_unnamed_bool()?);

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
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
        SET_BIT!(self.m_flags, 0, bitstream.read_unnamed_bool()?);
        SET_BIT!(self.m_flags, 1, bitstream.read_unnamed_bool()?);
        self.m_base_player_traits.decode(bitstream)?;
        self.m_weapon_set_absolute_index = bitstream.read_unnamed_signed_integer(8)?;
        self.m_vehicle_set_absolute_index = bitstream.read_unnamed_signed_integer(8)?;
        self.m_red_powerup_traits.decode(bitstream)?;
        self.m_blue_powerup_traits.decode(bitstream)?;
        self.m_yellow_powerup_traits.decode(bitstream)?;
        self.m_red_powerup_duration_seconds = bitstream.read_unnamed_integer(7)?;
        self.m_blue_powerup_duration_seconds = bitstream.read_unnamed_integer(7)?;
        self.m_yellow_powerup_duration_seconds = bitstream.read_unnamed_integer(7)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct c_game_engine_base_variant {
    #[serde(skip_serializing,skip_deserializing)]
    pub m_checksum: u32,
    #[brw(pad_before = 4)]
    pub m_metadata: s_content_item_metadata,
    pub m_miscellaneous_options: c_game_engine_miscellaneous_options,
    pub m_respawn_options: c_game_engine_respawn_options,
    pub m_social_options: c_game_engine_social_options,
    pub m_map_override_options: c_game_engine_map_override_options,
    #[brw(pad_before = 2)]
    pub m_flags: u16,
    #[brw(pad_after = 2)]
    pub m_team_scoring_method: u16,
}

impl c_game_engine_base_variant {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult<()> {
        self.m_metadata.encode(bitstream)?;
        bitstream.write_integer(self.m_flags as u32, 1)?;
        self.m_miscellaneous_options.encode(bitstream)?;
        self.m_respawn_options.encode(bitstream)?;
        self.m_social_options.encode(bitstream)?;
        self.m_map_override_options.encode(bitstream)?;
        bitstream.write_integer(self.m_team_scoring_method as u32, 3)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult<()> {
        self.m_metadata.decode(bitstream)?;
        self.m_flags = bitstream.read_unnamed_integer(1)?;
        self.m_miscellaneous_options.decode(bitstream)?;
        self.m_respawn_options.decode(bitstream)?;
        self.m_social_options.decode(bitstream)?;
        self.m_map_override_options.decode(bitstream)?;
        self.m_team_scoring_method = bitstream.read_unnamed_integer(3)?;

        Ok(())
    }
}