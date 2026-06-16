use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use blf_lib::blam::haloreach_mcc::v_untracked_25_08_16_1352::game::megalogamengine::megalogamengine_explicit_player::c_explicit_player;
use blf_lib::blam::haloreach_mcc::v_untracked_25_08_16_1352::game::megalogamengine::megalogamengine_explicit_team::c_explicit_team;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;
use crate::blam::haloreach_mcc::v_untracked_25_08_16_1352::game::megalogamengine::megalogamengine_explicit_object::c_explicit_object;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(2)]
pub enum e_player_reference_type {
    #[default]
    global_player = 0,
    player_player = 1,
    object_player = 2,
    team_player = 3,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_player_reference {
    pub m_type: e_player_reference_type,
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
        bitstream.write_enum(self.m_type)?;

        match (
            self.m_type,
            self.m_player.as_ref(),
            self.m_object.as_ref(),
            self.m_team.as_ref(),
            self.m_variable_index,
        ) {
            (e_player_reference_type::global_player, Some(player), None, None, None) => {
                player.encode(bitstream)?;
            }
            (e_player_reference_type::player_player, Some(player), None, None, Some(variable_index)) => {
                player.encode(bitstream)?;
                bitstream.write_integer(variable_index, 2)?;
            }
            (e_player_reference_type::object_player, None, Some(object), None, Some(variable_index)) => {
                object.encode(bitstream)?;
                bitstream.write_integer(variable_index, 2)?;
            }
            (e_player_reference_type::team_player, None, None, Some(team), Some(variable_index)) => {
                team.encode(bitstream)?;
                bitstream.write_integer(variable_index, 2)?;
            }
            _ => {
                return Err(format!("Invalid c_player_reference: {self:?}").into())
            }
        };

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_type = bitstream.read_enum("type")?;

        match self.m_type {
            e_player_reference_type::global_player => {
                let mut player = c_explicit_player::default();
                player.decode(bitstream)?;
                self.m_player = Some(player);
            }
            e_player_reference_type::player_player => {
                let mut player = c_explicit_player::default();
                player.decode(bitstream)?;
                self.m_player = Some(player);
                self.m_variable_index = Some(bitstream.read_integer("variable-index", 2)?);
            }
            e_player_reference_type::object_player => {
                let mut object = c_explicit_object::default();
                object.decode(bitstream)?;
                self.m_object = Some(object);
                self.m_variable_index = Some(bitstream.read_integer("variable-index", 2)?);
            }
            e_player_reference_type::team_player => {
                let mut team = c_explicit_team::default();
                team.decode(bitstream)?;
                self.m_team = Some(team);
                self.m_variable_index = Some(bitstream.read_integer("variable-index", 2)?);
            }
        }

        Ok(())
    }
}
