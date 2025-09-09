use serde::{Deserialize, Serialize};
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::game_engine_player_traits::c_player_traits;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::{SET_BIT, TEST_BIT};
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::game_engine_loadout_traits::c_game_engine_loadout_traits;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::game_engine_team::c_game_engine_team_options;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::game_engine_traits::{c_game_engine_miscellaneous_options, c_game_engine_respawn_options};
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::saved_games::saved_game_files::s_content_item_metadata;
use blf_lib_derivable::result::BLFLibResult;

pub const k_game_engine_type_count: usize = 11;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_game_engine_social_options {
    pub m_flags: u16,
    pub m_team_changing: u16,
    pub m_observers_enabled: bool,
}

impl c_game_engine_social_options {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_bool(self.m_observers_enabled)?; // TODO: what is this
        bitstream.write_integer(self.m_team_changing as u32, 2)?;
        bitstream.write_bool(TEST_BIT!(self.m_flags, 0))?;
        bitstream.write_bool(TEST_BIT!(self.m_flags, 1))?;
        bitstream.write_bool(TEST_BIT!(self.m_flags, 2))?;
        bitstream.write_bool(TEST_BIT!(self.m_flags, 3))?;
        bitstream.write_bool(TEST_BIT!(self.m_flags, 4))?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_observers_enabled = bitstream.read_unnamed_bool()?;
        self.m_team_changing = bitstream.read_unnamed_integer(2)?;
        SET_BIT!(self.m_flags, 0, bitstream.read_unnamed_bool()?);
        SET_BIT!(self.m_flags, 1, bitstream.read_unnamed_bool()?);
        SET_BIT!(self.m_flags, 2, bitstream.read_unnamed_bool()?);
        SET_BIT!(self.m_flags, 3, bitstream.read_unnamed_bool()?);
        SET_BIT!(self.m_flags, 4, bitstream.read_unnamed_bool()?);

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
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
    pub m_yellow_powerup_duration_seconds: u8,
}

impl c_game_engine_map_override_options {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_bool(TEST_BIT!(self.m_flags, 0))?;
        bitstream.write_bool(TEST_BIT!(self.m_flags, 1))?;
        bitstream.write_bool(TEST_BIT!(self.m_flags, 2))?;
        bitstream.write_bool(TEST_BIT!(self.m_flags, 3))?;
        bitstream.write_bool(TEST_BIT!(self.m_flags, 4))?;
        bitstream.write_bool(TEST_BIT!(self.m_flags, 5))?;
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
        self.m_flags = bitstream.read_integer("flags", 6)?;
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


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_game_engine_base_variant {
    pub m_metadata: s_content_item_metadata,
    pub m_built_in: bool,
    pub m_miscellaneous_options: c_game_engine_miscellaneous_options,
    pub m_respawn_options: c_game_engine_respawn_options,
    pub m_social_options: c_game_engine_social_options,
    pub m_map_override_options: c_game_engine_map_override_options,
    pub m_team_scoring_method: u16,
    pub m_team_options: c_game_engine_team_options,
    pub m_loadouts: c_game_engine_loadout_traits,

}

impl c_game_engine_base_variant {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_metadata.encode(bitstream)?;
        bitstream.write_bool(self.m_built_in)?;
        self.m_miscellaneous_options.encode(bitstream)?;
        self.m_respawn_options.encode(bitstream)?;
        self.m_social_options.encode(bitstream)?;
        self.m_map_override_options.encode(bitstream)?;
        bitstream.write_integer(self.m_team_scoring_method, 3)?;
        self.m_team_options.encode(bitstream)?;
        self.m_loadouts.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_metadata.decode(bitstream)?;
        self.m_built_in = bitstream.read_bool("variant-built-in")?;
        self.m_miscellaneous_options.decode(bitstream)?;
        self.m_respawn_options.decode(bitstream)?;
        self.m_social_options.decode(bitstream)?;
        self.m_map_override_options.decode(bitstream)?;
        self.m_team_scoring_method = bitstream.read_integer("team-scoring-method", 3)?;
        self.m_team_options.decode(bitstream)?;
        self.m_loadouts.decode(bitstream)?;


        Ok(())
    }
}
