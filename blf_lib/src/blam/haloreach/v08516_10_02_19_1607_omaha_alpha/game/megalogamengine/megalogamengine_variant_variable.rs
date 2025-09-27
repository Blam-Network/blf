use serde::{Deserialize, Serialize};
use blf_lib::blam::haloreach::v08516_10_02_19_1607_omaha_alpha::game::megalogamengine::megalogamengine_custom_variable_reference::c_custom_variable_reference;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_object_reference::c_object_reference;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_player_reference::c_player_reference;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;
use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_custom_timer_reference::c_custom_timer_reference;
use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_team_reference::c_team_reference;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_variant_variable {
    pub m_type: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player: Option<c_player_reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object: Option<c_object_reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_team: Option<c_team_reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_custom_timer: Option<c_custom_timer_reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_custom_variable: Option<c_custom_variable_reference>,
}

impl s_variant_variable {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_type, 3)?;

        match (self.m_type, self.m_player.as_ref(), self.m_object.as_ref(), self.m_team.as_ref(), self.m_custom_timer.as_ref(), self.m_custom_variable.as_ref()) {
            (0, None, None, None, None, Some(custom_variable)) => {
                custom_variable.encode(bitstream)?;
            }
            (1, Some(player), None, None, None, None) => {
                player.encode(bitstream)?;
            }
            (2, None, Some(object), None, None, None) => {
                object.encode(bitstream)?;
            }
            (3, None, None, Some(team), None, None) => {
                team.encode(bitstream)?;
            }
            (4, None, None, None, Some(timer), None) => {
                timer.encode(bitstream)?;
            }
            _ => {
                //return Err(format!("Invalid s_variant_variable: {self:?}").into())
            }
        };

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_type = bitstream.read_integer("type", 3)?;

        match self.m_type {
            0 => {
                let mut custom_variable = c_custom_variable_reference::default();
                custom_variable.decode(bitstream)?;
                self.m_custom_variable = Some(custom_variable);
            }
            1 => {
                let mut player = c_player_reference::default();
                player.decode(bitstream)?;
                self.m_player = Some(player);
            }
            2 => {
                let mut object = c_object_reference::default();
                object.decode(bitstream)?;
                self.m_object = Some(object);
            }
            3 => {
                let mut team = c_team_reference::default();
                team.decode(bitstream)?;
                self.m_team = Some(team);
            }
            4 => {
                let mut custom_timer = c_custom_timer_reference::default();
                custom_timer.decode(bitstream)?;
                self.m_custom_timer = Some(custom_timer);
            }
            _ => {
                // return Err(format!("Invalid s_variant_variable: {self:?}").into())
            }
        }

        Ok(())
    }
}