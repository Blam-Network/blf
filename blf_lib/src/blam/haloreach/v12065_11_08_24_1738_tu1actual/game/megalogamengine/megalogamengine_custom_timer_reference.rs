use serde::{Deserialize, Serialize};
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_explicit_player::c_explicit_player;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_explicit_team::c_explicit_team;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;
use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_explicit_object::c_explicit_object;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_custom_timer_reference {
    // type, 3 bits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player: Option<c_explicit_player>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object: Option<c_explicit_object>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_team: Option<c_explicit_team>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_variable_index: Option<u8>, // 2 or 3 bits
}

impl c_custom_timer_reference {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        match (&self.m_player, &self.m_object, &self.m_team, &self.m_variable_index) {
            (None, None, None, Some(variable_index)) => {
                bitstream.write_integer(0u8, 3)?;
                bitstream.write_integer(*variable_index, 3)?;
            }
            (Some(player), None, None, Some(variable_index)) => {
                bitstream.write_integer(1u8, 3)?;
                player.encode(bitstream)?;
                bitstream.write_integer(*variable_index, 2)?;
            }
            (None, Some(object), None, Some(variable_index)) => {
                bitstream.write_integer(2u8, 3)?;
                object.encode(bitstream)?;
                bitstream.write_integer(*variable_index, 1)?;
            }
            (None, None, Some(team), Some(variable_index)) => {
                bitstream.write_integer(3u8, 3)?;
                team.encode(bitstream)?;
                bitstream.write_integer(*variable_index, 2)?;
            }
            _ => {
                // return Err(format!("Invalid c_team_reference: {self:?}").into())
            }
        };

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        let ref_type = bitstream.read_integer("type", 3)?;

        match ref_type {
            0 => {
                self.m_variable_index = Some(bitstream.read_integer("global-variable-index", 3)?);
            }
            1 => {
                let mut player = c_explicit_player::default();
                player.decode(bitstream)?;
                self.m_player = Some(player);
                self.m_variable_index = Some(bitstream.read_integer("player-variable-index", 2)?);
            }
            2 => {
                let mut team = c_explicit_team::default();
                team.decode(bitstream)?;
                self.m_team = Some(team);
                self.m_variable_index = Some(bitstream.read_integer("team-variable-index", 2)?);
            }
            3 => {
                let mut object = c_explicit_object::default();
                object.decode(bitstream)?;
                self.m_object = Some(object);
                self.m_variable_index = Some(bitstream.read_integer("object-variable-index", 2)?);
            }
            _ => {
                // return Err(format!("Invalid c_team_reference: {self:?}").into())
            }
        }

        Ok(())
    }
}