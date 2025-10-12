use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_explicit_player::c_explicit_player;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_explicit_team::c_explicit_team;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;
use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_explicit_object::c_explicit_object;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive)]
pub enum e_custom_timer_type {
    #[default]
    global = 0,
    player = 1,
    team = 2,
    object = 3,
    round = 4,
    sudden_death = 5,
    grace_period = 6,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_custom_timer_reference {
    pub m_type: e_custom_timer_type,
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
        bitstream.write_enum(self.m_type.clone(), 3)?;

        match (self.m_type.clone(), self.m_player.as_ref(), self.m_object.as_ref(), self.m_team.as_ref(), self.m_variable_index.as_ref()) {
            (e_custom_timer_type::global, None, None, None, Some(variable_index)) => {
                bitstream.write_integer(*variable_index, 3)?;
            }
            (e_custom_timer_type::player, Some(player), None, None, Some(variable_index)) => {
                player.encode(bitstream)?;
                bitstream.write_integer(*variable_index, 2)?;
            }
            (e_custom_timer_type::object, None, Some(object), None, Some(variable_index)) => {
                object.encode(bitstream)?;
                bitstream.write_integer(*variable_index, 2)?;
            }
            (e_custom_timer_type::team, None, None, Some(team), Some(variable_index)) => {
                team.encode(bitstream)?;
                bitstream.write_integer(*variable_index, 1)?;
            }
            (e_custom_timer_type::round, None, None, None, None) => {}
            (e_custom_timer_type::sudden_death, None, None, None, None) => {}
            (e_custom_timer_type::grace_period, None, None, None, None) => {}
            _ => {
                return Err(format!("Invalid c_custom_timer_reference: {self:?}").into())
            }
        };

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_type = bitstream.read_unnamed_enum(3)?;

        match self.m_type {
            e_custom_timer_type::global => {
                self.m_variable_index = Some(bitstream.read_integer("global-variable-index", 3)?);
            }
            e_custom_timer_type::player => {
                let mut player = c_explicit_player::default();
                player.decode(bitstream)?;
                self.m_player = Some(player);
                self.m_variable_index = Some(bitstream.read_integer("player-variable-index", 2)?);
            }
            e_custom_timer_type::team => {
                let mut team = c_explicit_team::default();
                team.decode(bitstream)?;
                self.m_team = Some(team);
                self.m_variable_index = Some(bitstream.read_integer("team-variable-index", 1)?);
            }
            e_custom_timer_type::object => {
                let mut object = c_explicit_object::default();
                object.decode(bitstream)?;
                self.m_object = Some(object);
                self.m_variable_index = Some(bitstream.read_integer("object-variable-index", 2)?);
            }
            e_custom_timer_type::round => {}
            e_custom_timer_type::sudden_death => {}
            e_custom_timer_type::grace_period => {}
            _ => {
                return Err(format!("Invalid c_custom_timer_reference: {self:?}").into())
            }
        }

        Ok(())
    }
}