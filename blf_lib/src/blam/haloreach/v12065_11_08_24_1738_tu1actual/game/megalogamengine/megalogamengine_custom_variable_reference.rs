use serde::{Deserialize, Serialize};
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_explicit_player::c_explicit_player;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_explicit_team::c_explicit_team;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;
use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_explicit_object::c_explicit_object;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_custom_variable_reference {
    pub m_type: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_immediate_value: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player: Option<c_explicit_player>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object: Option<c_explicit_object>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_team: Option<c_explicit_team>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_variable_index: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_option_index: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_statistic_index: Option<u8>,
}

impl c_custom_variable_reference {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_type, 6)?;

        match (&self.m_type, self.m_immediate_value, &self.m_player, &self.m_object, &self.m_team, self.m_variable_index, self.m_option_index, self.m_statistic_index) {
            (0, Some(immediate_value), None, None, None, None, None, None) => {
                bitstream.write_signed_integer(immediate_value, 16)?;
            }
            (1, None, Some(player), None, None, Some(variable_index), None, None) => {
                player.encode(bitstream)?;
                bitstream.write_integer(variable_index, 3)?;
            }
            (2, None, None, Some(object), None, Some(variable_index), None, None) => {
                object.encode(bitstream)?;
                bitstream.write_integer(variable_index, 3)?;
            }
            (3, None, None, None, Some(team), Some(variable_index), None, None) => {
                team.encode(bitstream)?;
                bitstream.write_integer(variable_index, 3)?;
            }
            (4, None, None, None, None, Some(variable_index), None, None) => {
                bitstream.write_integer(variable_index, 4)?;
            }
            (5, None, None, None, None, None, Some(option_index), None ) => {
                bitstream.write_integer(option_index, 4)?;
            }
            (6, None, None, Some(object), None, None, None, None) => {
                object.encode(bitstream)?;
            }
            (7, None, None, None, Some(team), None, None, None) => {
                team.encode(bitstream)?;
            }
            (8 | 9 | 10, None, Some(player), None, None, None, None, None) => {
                player.encode(bitstream)?;
            }
            (11, None, Some(player), None, None, None, None, Some(statistic_index)) => {
                player.encode(bitstream)?;
                bitstream.write_integer(statistic_index, 2)?;
            }
            (12, None, None, None, Some(team), None, None, Some(statistic_index)) => {
                team.encode(bitstream)?;
                bitstream.write_integer(statistic_index, 2)?;
            }
            _ => {
                // return Err(format!("Invalid c_custom_variable_reference: {self:?}").into())
            }
        };

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_type = bitstream.read_integer("type", 6)?;

        match self.m_type {
            0 => {
                self.m_immediate_value = Some(bitstream.read_integer("immediate-value", 16)?);
            }
            1 => {
                let mut player = c_explicit_player::default();
                player.decode(bitstream)?;
                self.m_player = Some(player);
                self.m_variable_index = Some(bitstream.read_integer("variable-index", 3)?);
            }
            2 => {
                let mut object = c_explicit_object::default();
                object.decode(bitstream)?;
                self.m_object = Some(object);
                self.m_variable_index = Some(bitstream.read_integer("variable-index", 3)?);
            }
            3 => {
                let mut team = c_explicit_team::default();
                team.decode(bitstream)?;
                self.m_team = Some(team);
                self.m_variable_index = Some(bitstream.read_integer("variable-index", 3)?);
            }
            4 => {
                self.m_variable_index = Some(bitstream.read_integer("variable-index", 4)?);
            }
            5 => {
                self.m_option_index = Some(bitstream.read_integer("option-index", 4)?);
            }
            6 => {
                let mut object = c_explicit_object::default();
                object.decode(bitstream)?;
                self.m_object = Some(object);
            }
            7 => {
                let mut team = c_explicit_team::default();
                team.decode(bitstream)?;
                self.m_team = Some(team);
            }
            8 | 9 | 10 => {
                let mut player = c_explicit_player::default();
                player.decode(bitstream)?;
                self.m_player = Some(player);
            }
            11 => {
                let mut player = c_explicit_player::default();
                player.decode(bitstream)?;
                self.m_player = Some(player);
                self.m_statistic_index = Some(bitstream.read_integer("statistic-index", 2)?);
            }
            12 => {
                let mut team = c_explicit_team::default();
                team.decode(bitstream)?;
                self.m_team = Some(team);
                self.m_statistic_index = Some(bitstream.read_integer("statistic-index", 2)?);
            }
            _ => {
                // return Err(format!("Invalid c_custom_variable_reference: {self:?}").into())
            }
        }

        Ok(())
    }
}