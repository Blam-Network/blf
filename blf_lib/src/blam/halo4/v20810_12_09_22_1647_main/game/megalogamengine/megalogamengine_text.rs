use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::game::megalogamengine::megalogamengine_custom_timer_reference::c_custom_timer_reference;
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::game::megalogamengine::megalogamengine_custom_variable_reference::c_custom_variable_reference;
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::game::megalogamengine::megalogamengine_object_reference::c_object_reference;
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::game::megalogamengine::megalogamengine_player_reference::c_player_reference;
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::game::megalogamengine::megalogamengine_team_reference::c_team_reference;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;

#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive, Default, Serialize, Deserialize, crate::derive::c_enum)]
#[bits(3)]
pub enum e_replaceable_token_type {
    none = -1,
    #[default]
    player = 0,
    team = 1,
    object = 2,
    custom_variable = 3,
    custom_timer = 4,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_replaceable_token {
    pub m_type: e_replaceable_token_type,
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
        bitstream.write_enum(self.m_type)?;

        match (self.m_type, &self.m_player, &self.m_object, &self.m_team, &self.m_custom_timer, &self.m_custom_variable) {
            (e_replaceable_token_type::player, Some(player), None, None, None, None) => {
                player.encode(bitstream)?;
            }
            (e_replaceable_token_type::team, None, None, Some(team), None, None) => {
                team.encode(bitstream)?;
            }
            (e_replaceable_token_type::object, None, Some(object), None, None, None) => {
                object.encode(bitstream)?;
            }
            (e_replaceable_token_type::custom_variable, None, None, None, None, Some(custom_variable)) => {
                custom_variable.encode(bitstream)?;
            }
            (e_replaceable_token_type::custom_timer, None, None, None, Some(timer), None) => {
                timer.encode(bitstream)?;
            }
            (e_replaceable_token_type::none, _, _, _, _, _) => {}
            _ => {
                return Err(format!("Invalid c_replaceable_token: {self:?}").into())
            }
        };

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_type = bitstream.read_enum("token-type")?;

        match self.m_type {
            e_replaceable_token_type::player => {
                let mut player = c_player_reference::default();
                player.decode(bitstream)?;
                self.m_player = Some(player);
            }
            e_replaceable_token_type::team => {
                let mut team = c_team_reference::default();
                team.decode(bitstream)?;
                self.m_team = Some(team);
            }
            e_replaceable_token_type::object => {
                let mut object = c_object_reference::default();
                object.decode(bitstream)?;
                self.m_object = Some(object);
            }
            e_replaceable_token_type::custom_variable => {
                let mut custom_variable = c_custom_variable_reference::default();
                custom_variable.decode(bitstream)?;
                self.m_custom_variable = Some(custom_variable);
            }
            e_replaceable_token_type::custom_timer => {
                let mut custom_timer = c_custom_timer_reference::default();
                custom_timer.decode(bitstream)?;
                self.m_custom_timer = Some(custom_timer);
            }
            e_replaceable_token_type::none => {}
        }

        Ok(())
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_dynamic_string {
    /// Biased string index into the megalo string table (MaxStrings=148 → 8 bits).
    pub m_string_index: i16,
    /// Token count bit width is `ceil(log2(max_tokens+1))` for template N=`max_tokens`.
    #[serde(skip)]
    pub max_tokens: u8,
    pub m_tokens: Vec<c_replaceable_token>,
}

impl Default for c_dynamic_string {
    fn default() -> Self {
        Self {
            m_string_index: 0,
            max_tokens: 2,
            m_tokens: Vec::new(),
        }
    }
}

fn token_count_bit_width(max_tokens: u8) -> usize {
    // Encode token count in [0, maxTokens] — same as engine required_bits(max+1).
    let range = (max_tokens as usize) + 1;
    let mut bits = 1usize;
    while (1usize << bits) < range {
        bits += 1;
    }
    bits
}

impl c_dynamic_string {
    pub fn with_max_tokens(max_tokens: u8) -> Self {
        Self {
            max_tokens,
            ..Default::default()
        }
    }

    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_index::<148>(self.m_string_index as i32, 8)?;
        bitstream.write_integer(
            self.m_tokens.len() as u32,
            token_count_bit_width(self.max_tokens),
        )?;
        for token in &self.m_tokens {
            token.encode(bitstream)?;
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_string_index =
            bitstream.read_index::<148>("string-index", 8)? as i16;
        let token_count: u8 =
            bitstream.read_integer("token-count", token_count_bit_width(self.max_tokens))?;
        self.m_tokens.clear();
        for _ in 0..token_count {
            let mut token = c_replaceable_token::default();
            token.decode(bitstream)?;
            self.m_tokens.push(token);
        }

        Ok(())
    }
}
