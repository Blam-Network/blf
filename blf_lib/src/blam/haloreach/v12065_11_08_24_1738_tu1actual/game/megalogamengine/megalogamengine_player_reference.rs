use serde::{Deserialize, Serialize};
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_explicit_player::c_explicit_player;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_explicit_team::c_explicit_team;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;
use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_explicit_object::c_explicit_object;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_player_reference {
    // type, 2 bits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player: Option<c_explicit_player>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object: Option<c_explicit_object>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_team: Option<c_explicit_team>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_variable_index: Option<u8>, // 2 bits
}

impl c_player_reference {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        match (&self.m_player, &self.m_object, &self.m_team, self.m_variable_index) {
            (Some(player), None, None, None) => {
                bitstream.write_integer(0u8, 2)?;
                player.encode(bitstream)?;
            }
            (Some(player), None, None, Some(variable_index)) => {
                bitstream.write_integer(1u8, 2)?;
                player.encode(bitstream)?;
                bitstream.write_integer(variable_index, 2)?;
            }
            (None, Some(object), None, Some(variable_index)) => {
                bitstream.write_integer(2u8, 2)?;
                object.encode(bitstream)?;
                bitstream.write_integer(variable_index, 2)?;
            }
            (None, None, Some(team), Some(variable_index)) => {
                bitstream.write_integer(3u8, 2)?;
                team.encode(bitstream)?;
                bitstream.write_integer(variable_index, 2)?;
            }
            _ => {
                return Err(format!("Invalid c_explicit_player: {self:?}").into())
            }
        };

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        let ref_type = bitstream.read_integer("type", 2)?;

        match ref_type {
            0 => {
                let mut player = c_explicit_player::default();
                player.decode(bitstream)?;
                self.m_player = Some(player);
            }
            1 => {
                let mut player = c_explicit_player::default();
                player.decode(bitstream)?;
                self.m_player = Some(player);
                self.m_variable_index = Some(bitstream.read_integer("m_variable_index", 2)?);
            }
            2 => {
                let mut object = c_explicit_object::default();
                object.decode(bitstream)?;
                self.m_object = Some(object);
                self.m_variable_index = Some(bitstream.read_integer("m_variable_index", 2)?);
            }
            3 => {
                let mut team = c_explicit_team::default();
                team.decode(bitstream)?;
                self.m_team = Some(team);
                self.m_variable_index = Some(bitstream.read_integer("m_variable_index", 2)?);
            }
            _ => {
                return Err(format!("Invalid c_explicit_player: type = {ref_type}").into())
            }
        }

        Ok(())
    }
}