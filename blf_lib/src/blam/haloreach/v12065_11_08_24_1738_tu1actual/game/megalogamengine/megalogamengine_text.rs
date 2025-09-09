use serde::{Deserialize, Serialize};
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_custom_timer_reference::c_custom_timer_reference;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_custom_variable_reference::c_custom_variable_reference;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_object_reference::c_object_reference;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_player_reference::c_player_reference;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_team_reference::c_team_reference;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_replaceable_token {
    // type, 3 bits
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
        match (&self.m_player, &self.m_object, &self.m_team, &self.m_custom_timer, &self.m_custom_variable) {
            (Some(player), None, None, None, None) => {
                bitstream.write_integer(0u8, 3)?;
                player.encode(bitstream)?;
            }
            (None, None, Some(team), None, None) => {
                bitstream.write_integer(1u8, 3)?;
                team.encode(bitstream)?;
            }
            (None, Some(object), None, None, None) => {
                bitstream.write_integer(2u8, 3)?;
                object.encode(bitstream)?;
            }
            (None, None, None, None, Some(custom_variable)) => {
                bitstream.write_integer(0u8, 3)?;
                custom_variable.encode(bitstream)?; // seems ok
            }
            (None, None, None, Some(timer), None) => {
                bitstream.write_integer(4u8, 3)?;
                timer.encode(bitstream)?;
            }
            _ => { }
        };

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        let ref_type = (bitstream.read_integer::<u8>("token-type", 3)? as i8) - 1;

        match ref_type {
            0 => {
                let mut player = c_player_reference::default();
                player.decode(bitstream)?;
                self.m_player = Some(player);
            }
            1 => {
                let mut team = c_team_reference::default();
                team.decode(bitstream)?;
                self.m_team = Some(team);
            }
            2 => {
                let mut object = c_object_reference::default();
                object.decode(bitstream)?;
                self.m_object = Some(object);
            }
            3 => {
                let mut custom_variable = c_custom_variable_reference::default();
                custom_variable.decode(bitstream)?;
                self.m_custom_variable = Some(custom_variable);
            }
            4 => {
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