use serde::{Deserialize, Serialize};
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_custom_timer_reference::c_custom_timer_reference;
use blf_lib::blam::haloreach::v09730_10_04_09_1309_omaha_delta::game::megalogamengine::megalogamengine_custom_variable_reference::c_custom_variable_reference;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_object_reference::c_object_reference;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_player_reference::c_player_reference;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_team_reference::c_team_reference;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_replaceable_token {
    pub m_type: u8, // 3 bits
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

impl c_replaceable_token {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_type, 3)?;

        match (self.m_type, &self.m_player, &self.m_object, &self.m_team, &self.m_custom_timer, &self.m_custom_variable) {
            (1, Some(player), None, None, None, None) => {
                player.encode(bitstream)?;
            }
            (2, None, None, Some(team), None, None) => {
                team.encode(bitstream)?;
            }
            (3, None, Some(object), None, None, None) => {
                object.encode(bitstream)?;
            }
            (4, None, None, None, None, Some(custom_variable)) => {
                custom_variable.encode(bitstream)?; // seems ok
            }
            (5, None, None, None, Some(timer), None) => {
                timer.encode(bitstream)?;
            }
            _ => {
                return Err(format!("Invalid c_replaceable_token: {self:?}").into())
            }
        };

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_type = bitstream.read_integer("token-type", 3)?;

        match self.m_type {
            1 => {
                let mut player = c_player_reference::default();
                player.decode(bitstream)?;
                self.m_player = Some(player);
            }
            2 => {
                let mut team = c_team_reference::default();
                team.decode(bitstream)?;
                self.m_team = Some(team);
            }
            3 => {
                let mut object = c_object_reference::default();
                object.decode(bitstream)?;
                self.m_object = Some(object);
            }
            4 => {
                let mut custom_variable = c_custom_variable_reference::default();
                custom_variable.decode(bitstream)?;
                self.m_custom_variable = Some(custom_variable);
            }
            5 => {
                let mut custom_timer = c_custom_timer_reference::default();
                custom_timer.decode(bitstream)?;
                self.m_custom_timer = Some(custom_timer);
            }
            _ => { }
        }

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_dynamic_string {
    pub m_string_index: u8, // 7 bits
    // token count, 2 bits
    pub m_tokens: Vec<c_replaceable_token>,
}

impl c_dynamic_string {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_string_index, 7)?;
        bitstream.write_integer(self.m_tokens.len() as u8, 2)?;
        for token in &self.m_tokens {
            token.encode(bitstream)?;
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_string_index = bitstream.read_integer("string-index", 7)?;
        let token_count = bitstream.read_integer("token-count", 2)?;
        for i in 0..token_count {
            let mut token = c_replaceable_token::default();
            token.decode(bitstream)?;
            self.m_tokens.push(token);
        }

        Ok(())
    }
}