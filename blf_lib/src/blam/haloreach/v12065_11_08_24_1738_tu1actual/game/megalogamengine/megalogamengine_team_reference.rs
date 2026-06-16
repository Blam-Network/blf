use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_explicit_player::c_explicit_player;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_explicit_team::c_explicit_team;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;
use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_explicit_object::c_explicit_object;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(3)]
pub enum e_team_reference_type {
    #[default]
    global_team = 0,
    player_team = 1,
    object_team = 2,
    team_team = 3,
    player_owner_team = 4,
    object_owner_team = 5,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_team_reference {
    pub m_type: e_team_reference_type,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player: Option<c_explicit_player>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object: Option<c_explicit_object>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_team: Option<c_explicit_team>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_variable_index: Option<u8>,
}

impl c_team_reference {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_enum(self.m_type)?;

        match (
            self.m_type,
            self.m_player.as_ref(),
            self.m_object.as_ref(),
            self.m_team.as_ref(),
            self.m_variable_index,
        ) {
            (e_team_reference_type::global_team, None, None, Some(team), None) => {
                team.encode(bitstream)?;
            }
            (e_team_reference_type::player_team, Some(player), None, None, Some(variable_index)) => {
                player.encode(bitstream)?;
                bitstream.write_integer(variable_index, 2)?;
            }
            (e_team_reference_type::object_team, None, Some(object), None, Some(variable_index)) => {
                object.encode(bitstream)?;
                bitstream.write_integer(variable_index, 1)?;
            }
            (e_team_reference_type::team_team, None, None, Some(team), Some(variable_index)) => {
                team.encode(bitstream)?;
                bitstream.write_integer(variable_index, 2)?;
            }
            (e_team_reference_type::player_owner_team, Some(player), None, None, None) => {
                player.encode(bitstream)?;
            }
            (e_team_reference_type::object_owner_team, None, Some(object), None, None) => {
                object.encode(bitstream)?;
            }
            _ => {
                return Err(format!("Invalid c_team_reference: {self:?}").into())
            }
        };

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_type = bitstream.read_enum("type")?;

        match self.m_type {
            e_team_reference_type::global_team => {
                let mut team = c_explicit_team::default();
                team.decode(bitstream)?;
                self.m_team = Some(team);
            }
            e_team_reference_type::player_team => {
                let mut player = c_explicit_player::default();
                player.decode(bitstream)?;
                self.m_player = Some(player);
                self.m_variable_index = Some(bitstream.read_integer("variable-index", 2)?);
            }
            e_team_reference_type::object_team => {
                let mut object = c_explicit_object::default();
                object.decode(bitstream)?;
                self.m_object = Some(object);
                self.m_variable_index = Some(bitstream.read_integer("variable-index", 1)?);
            }
            e_team_reference_type::team_team => {
                let mut team = c_explicit_team::default();
                team.decode(bitstream)?;
                self.m_team = Some(team);
                self.m_variable_index = Some(bitstream.read_integer("variable-index", 2)?);
            }
            e_team_reference_type::player_owner_team => {
                let mut player = c_explicit_player::default();
                player.decode(bitstream)?;
                self.m_player = Some(player);
            }
            e_team_reference_type::object_owner_team => {
                let mut object = c_explicit_object::default();
                object.decode(bitstream)?;
                self.m_object = Some(object);
            }
        }

        Ok(())
    }
}
