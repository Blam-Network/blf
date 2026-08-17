use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::game::megalogamengine::megalogamengine_explicit_player::c_explicit_player;
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::game::megalogamengine::megalogamengine_explicit_team::c_explicit_team;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;
use crate::blam::halo4::v20810_12_09_22_1647_main::game::megalogamengine::megalogamengine_explicit_object::c_explicit_object;

/// Halo 4 `e_custom_variable_type` — IDA count 73 (0..72), 7 bits.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(7)]
pub enum e_custom_variable_type {
    #[default]
    constant = 0,
    player_number = 1,
    object_number = 2,
    team_number = 3,
    global_number = 4,
    temporary_number = 5,
    option = 6,
    unknown_7 = 7,
    unknown_8 = 8,
    unknown_9 = 9,
    unknown_10 = 10,
    unknown_11 = 11,
    unknown_12 = 12,
    unknown_13 = 13,
    player_stat = 14,
    team_stat = 15,
    unknown_16 = 16,
    unknown_17 = 17,
    unknown_18 = 18,
    unknown_19 = 19,
    unknown_20 = 20,
    unknown_21 = 21,
    unknown_22 = 22,
    unknown_23 = 23,
    unknown_24 = 24,
    unknown_25 = 25,
    unknown_26 = 26,
    unknown_27 = 27,
    unknown_28 = 28,
    unknown_29 = 29,
    unknown_30 = 30,
    unknown_31 = 31,
    unknown_32 = 32,
    unknown_33 = 33,
    unknown_34 = 34,
    unknown_35 = 35,
    unknown_36 = 36,
    unknown_37 = 37,
    unknown_38 = 38,
    unknown_39 = 39,
    unknown_40 = 40,
    unknown_41 = 41,
    unknown_42 = 42,
    unknown_43 = 43,
    unknown_44 = 44,
    unknown_45 = 45,
    unknown_46 = 46,
    unknown_47 = 47,
    unknown_48 = 48,
    unknown_49 = 49,
    unknown_50 = 50,
    unknown_51 = 51,
    unknown_52 = 52,
    unknown_53 = 53,
    unknown_54 = 54,
    unknown_55 = 55,
    unknown_56 = 56,
    unknown_57 = 57,
    unknown_58 = 58,
    unknown_59 = 59,
    unknown_60 = 60,
    unknown_61 = 61,
    unknown_62 = 62,
    unknown_63 = 63,
    unknown_64 = 64,
    unknown_65 = 65,
    unknown_66 = 66,
    unknown_67 = 67,
    unknown_68 = 68,
    unknown_69 = 69,
    unknown_70 = 70,
    unknown_71 = 71,
    unknown_72 = 72,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_custom_variable_reference {
    pub m_type: e_custom_variable_type,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_immediate_value: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player: Option<c_explicit_player>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object: Option<c_explicit_object>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_team: Option<c_explicit_team>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_variable_index: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_option_index: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_statistic_index: Option<u8>,
}

impl c_custom_variable_reference {
    pub fn is_writeable(&self) -> bool {
        matches!(
            self.m_type,
            e_custom_variable_type::player_number
                | e_custom_variable_type::object_number
                | e_custom_variable_type::team_number
                | e_custom_variable_type::global_number
                | e_custom_variable_type::temporary_number
                | e_custom_variable_type::unknown_10
                | e_custom_variable_type::unknown_11
                | e_custom_variable_type::unknown_12
                | e_custom_variable_type::player_stat
                | e_custom_variable_type::team_stat
                | e_custom_variable_type::unknown_19
                | e_custom_variable_type::unknown_20
                | e_custom_variable_type::unknown_21
                | e_custom_variable_type::unknown_22
                | e_custom_variable_type::unknown_23
                | e_custom_variable_type::unknown_24
        )
    }

    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_enum(self.m_type)?;

        match self.m_type {
            e_custom_variable_type::constant => {
                bitstream.write_signed_integer(
                    self.m_immediate_value.ok_or("m_immediate_value does not exist.")?,
                    16,
                )?;
            }
            e_custom_variable_type::player_number => {
                self.m_player
                    .as_ref()
                    .ok_or("m_player does not exist.")?
                    .encode(bitstream)?;
                bitstream.write_integer(
                    self.m_variable_index.ok_or("m_variable_index does not exist.")?,
                    4,
                )?;
            }
            e_custom_variable_type::object_number => {
                self.m_object
                    .as_ref()
                    .ok_or("m_object does not exist.")?
                    .encode(bitstream)?;
                bitstream.write_integer(
                    self.m_variable_index.ok_or("m_variable_index does not exist.")?,
                    4,
                )?;
            }
            e_custom_variable_type::team_number => {
                self.m_team
                    .as_ref()
                    .ok_or("m_team does not exist.")?
                    .encode(bitstream)?;
                bitstream.write_integer(
                    self.m_variable_index.ok_or("m_variable_index does not exist.")?,
                    4,
                )?;
            }
            e_custom_variable_type::global_number => {
                bitstream.write_integer(
                    self.m_variable_index.ok_or("m_variable_index does not exist.")?,
                    5,
                )?;
            }
            e_custom_variable_type::temporary_number => {
                bitstream.write_integer(
                    self.m_variable_index.ok_or("m_variable_index does not exist.")?,
                    4,
                )?;
            }
            e_custom_variable_type::option => {
                bitstream.write_integer(
                    self.m_option_index.ok_or("m_option_index does not exist.")?,
                    4,
                )?;
            }
            e_custom_variable_type::unknown_7
            | e_custom_variable_type::unknown_8
            | e_custom_variable_type::unknown_9
            | e_custom_variable_type::unknown_31
            | e_custom_variable_type::unknown_32 => {
                self.m_object
                    .as_ref()
                    .ok_or("m_object does not exist.")?
                    .encode(bitstream)?;
            }
            e_custom_variable_type::unknown_10
            | e_custom_variable_type::unknown_22
            | e_custom_variable_type::unknown_28
            | e_custom_variable_type::unknown_29
            | e_custom_variable_type::unknown_30 => {
                self.m_team
                    .as_ref()
                    .ok_or("m_team does not exist.")?
                    .encode(bitstream)?;
            }
            e_custom_variable_type::unknown_11
            | e_custom_variable_type::unknown_12
            | e_custom_variable_type::unknown_13
            | e_custom_variable_type::unknown_23
            | e_custom_variable_type::unknown_24
            | e_custom_variable_type::unknown_25
            | e_custom_variable_type::unknown_26
            | e_custom_variable_type::unknown_27 => {
                self.m_player
                    .as_ref()
                    .ok_or("m_player does not exist.")?
                    .encode(bitstream)?;
            }
            e_custom_variable_type::player_stat => {
                self.m_player
                    .as_ref()
                    .ok_or("m_player does not exist.")?
                    .encode(bitstream)?;
                bitstream.write_integer(
                    self.m_statistic_index.ok_or("m_statistic_index does not exist.")?,
                    2,
                )?;
            }
            e_custom_variable_type::team_stat => {
                self.m_team
                    .as_ref()
                    .ok_or("m_team does not exist.")?
                    .encode(bitstream)?;
                bitstream.write_integer(
                    self.m_statistic_index.ok_or("m_statistic_index does not exist.")?,
                    2,
                )?;
            }
            _ => {}
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_type = bitstream.read_enum("type")?;

        match self.m_type {
            e_custom_variable_type::constant => {
                self.m_immediate_value =
                    Some(bitstream.read_signed_integer("immediate-value", 16)?);
            }
            e_custom_variable_type::player_number => {
                let mut player = c_explicit_player::default();
                player.decode(bitstream)?;
                self.m_player = Some(player);
                self.m_variable_index = Some(bitstream.read_integer("variable-index", 4)?);
            }
            e_custom_variable_type::object_number => {
                let mut object = c_explicit_object::default();
                object.decode(bitstream)?;
                self.m_object = Some(object);
                self.m_variable_index = Some(bitstream.read_integer("variable-index", 4)?);
            }
            e_custom_variable_type::team_number => {
                let mut team = c_explicit_team::default();
                team.decode(bitstream)?;
                self.m_team = Some(team);
                self.m_variable_index = Some(bitstream.read_integer("variable-index", 4)?);
            }
            e_custom_variable_type::global_number => {
                self.m_variable_index = Some(bitstream.read_integer("variable-index", 5)?);
            }
            e_custom_variable_type::temporary_number => {
                self.m_variable_index = Some(bitstream.read_integer("variable-index", 4)?);
            }
            e_custom_variable_type::option => {
                self.m_option_index = Some(bitstream.read_integer("option-index", 4)?);
            }
            e_custom_variable_type::unknown_7
            | e_custom_variable_type::unknown_8
            | e_custom_variable_type::unknown_9
            | e_custom_variable_type::unknown_31
            | e_custom_variable_type::unknown_32 => {
                let mut object = c_explicit_object::default();
                object.decode(bitstream)?;
                self.m_object = Some(object);
            }
            e_custom_variable_type::unknown_10
            | e_custom_variable_type::unknown_22
            | e_custom_variable_type::unknown_28
            | e_custom_variable_type::unknown_29
            | e_custom_variable_type::unknown_30 => {
                let mut team = c_explicit_team::default();
                team.decode(bitstream)?;
                self.m_team = Some(team);
            }
            e_custom_variable_type::unknown_11
            | e_custom_variable_type::unknown_12
            | e_custom_variable_type::unknown_13
            | e_custom_variable_type::unknown_23
            | e_custom_variable_type::unknown_24
            | e_custom_variable_type::unknown_25
            | e_custom_variable_type::unknown_26
            | e_custom_variable_type::unknown_27 => {
                let mut player = c_explicit_player::default();
                player.decode(bitstream)?;
                self.m_player = Some(player);
            }
            e_custom_variable_type::player_stat => {
                let mut player = c_explicit_player::default();
                player.decode(bitstream)?;
                self.m_player = Some(player);
                self.m_statistic_index = Some(bitstream.read_integer("statistic-index", 2)?);
            }
            e_custom_variable_type::team_stat => {
                let mut team = c_explicit_team::default();
                team.decode(bitstream)?;
                self.m_team = Some(team);
                self.m_statistic_index = Some(bitstream.read_integer("statistic-index", 2)?);
            }
            _ => {}
        }

        Ok(())
    }
}
