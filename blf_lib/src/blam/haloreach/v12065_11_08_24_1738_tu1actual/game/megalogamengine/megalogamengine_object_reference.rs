use serde::{Deserialize, Serialize};
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_explicit_player::c_explicit_player;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_explicit_team::c_explicit_team;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;
use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_explicit_object::c_explicit_object;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_object_reference {
    pub m_type: u8, // 3 bits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player: Option<c_explicit_player>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object: Option<c_explicit_object>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_team: Option<c_explicit_team>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_variable_index: Option<u8>, // 2 or 3 bits
}

impl c_object_reference {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_type, 3)?;

        match (self.m_type, &self.m_player, &self.m_object, &self.m_team, self.m_variable_index) {
            (0, None, Some(object), None, None) => {
                object.encode(bitstream)?;
            }
            (1, Some(player), None, None, Some(variable_index)) => {
                player.encode(bitstream)?;
                bitstream.write_integer(variable_index, 2)?;
            }
            (2, None, Some(object), None, Some(variable_index)) => {
                object.encode(bitstream)?;
                bitstream.write_integer(variable_index, 2)?;
            }
            (3, None, None, Some(team), Some(variable_index)) => {
                team.encode(bitstream)?;
                bitstream.write_integer(variable_index, 3)?;
            }
            (4, Some(player), None, None, None) => {
                player.encode(bitstream)?;
            }
            (5, Some(player), None, None, Some(variable_index)) => {
                player.encode(bitstream)?;
                bitstream.write_integer(variable_index, 2)?;
            }
            (6, None, Some(object), None, Some(variable_index)) => {
                object.encode(bitstream)?;
                bitstream.write_integer(variable_index, 2)?;
            }
            (7, None, None, Some(team), Some(variable_index)) => {
                team.encode(bitstream)?;
                bitstream.write_integer(variable_index, 2)?;
            }
            _ => {
                return Err(format!("Invalid c_object_reference: {self:?}").into())
            }
        };

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_type = bitstream.read_integer("type", 3)?;

        match self.m_type {
            0 => {
                let mut object = c_explicit_object::default();
                object.decode(bitstream)?;
                self.m_object = Some(object);
            }
            1 => {
                let mut player = c_explicit_player::default();
                player.decode(bitstream)?;
                self.m_player = Some(player);
                self.m_variable_index = Some(bitstream.read_integer("variable-index", 2)?);
            }
            2 => {
                let mut object = c_explicit_object::default();
                object.decode(bitstream)?;
                self.m_object = Some(object);
                self.m_variable_index = Some(bitstream.read_integer("variable-index", 2)?);
            }
            3 => {
                let mut team = c_explicit_object::default();
                team.decode(bitstream)?;
                self.m_object = Some(team);
                self.m_variable_index = Some(bitstream.read_integer("variable-index", 3)?);
            }
            4 => {
                let mut player = c_explicit_player::default();
                player.decode(bitstream)?;
                self.m_player = Some(player);
            }
            5 => {
                let mut player = c_explicit_player::default();
                player.decode(bitstream)?;
                self.m_player = Some(player);
                self.m_variable_index = Some(bitstream.read_integer("variable-index", 2)?);
            }
            6 => {
                let mut object = c_explicit_object::default();
                object.decode(bitstream)?;
                self.m_object = Some(object);
                self.m_variable_index = Some(bitstream.read_integer("variable-index", 2)?);
            }
            7 => {
                let mut team = c_explicit_object::default();
                team.decode(bitstream)?;
                self.m_object = Some(team);
                self.m_variable_index = Some(bitstream.read_integer("variable-index", 2)?);
            }
            _ => {
                return Err(format!("Invalid c_object_reference: {self:?}").into())
            }
        }

        Ok(())
    }
}