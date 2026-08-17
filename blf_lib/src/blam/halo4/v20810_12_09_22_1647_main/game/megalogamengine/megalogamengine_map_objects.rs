use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use blf_lib::bitfield;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::{BLFLibError, BLFLibResult};
use crate::blam::halo4::v20810_12_09_22_1647_main::game::megalogamengine::megalogamengine_object_type_reference::c_object_type_reference;

#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(4)]
pub enum e_object_team_filter {
    none = -1,
    defenders = 0,
    attackers = 1,
    third_party = 2,
    fourth_party = 3,
    fifth_party = 4,
    sixth_party = 5,
    seventh_party = 6,
    eighth_party = 7,
    neutral = 8,
    each = 9,
}

bitfield! {
    #[derive(Serialize, Deserialize)]
    pub struct e_filter_parameters: u8 {
        object_type,
        team,
        user_data,
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_object_filter {
    /// Halo 4: biased string index (MaxStrings=148 → 8 bits). Reach used 7-bit plain.
    pub m_label_string_index: i16,
    pub m_valid_parameters: e_filter_parameters, // 3 bits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_type: Option<c_object_type_reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_team: Option<e_object_team_filter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_user_data: Option<i16>, // 16 bits
    pub m_min: u8,
}

impl c_object_filter {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_index::<148>(self.m_label_string_index as i32, 8)?;
        bitstream.write_integer(self.m_valid_parameters.to_raw(), 3)?;
        if self.m_valid_parameters.object_type {
            self.m_object_type.as_ref()
                .ok_or_else(|| BLFLibError::from("m_object_type does not exist."))?
                .encode(bitstream)?;
        }
        if self.m_valid_parameters.team {
            bitstream.write_enum(
                *self.m_team.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_team does not exist."))?,
            )?;
        }
        if self.m_valid_parameters.user_data {
            bitstream.write_signed_integer(
                *self.m_user_data.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_user_data does not exist."))?,
                16
            )?;
        }
        bitstream.write_integer(self.m_min, 7)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_label_string_index =
            bitstream.read_index::<148>("label-string-index", 8)? as i16;
        self.m_valid_parameters =
            e_filter_parameters::from_raw(bitstream.read_integer("valid-parameters", 3)?);
        if self.m_valid_parameters.object_type {
            let mut object_type = c_object_type_reference::default();
            object_type.decode(bitstream)?;
            self.m_object_type = Some(object_type);
        }
        if self.m_valid_parameters.team {
            self.m_team = Some(bitstream.read_enum("team")?);
        }
        if self.m_valid_parameters.user_data {
            self.m_user_data = Some(
                bitstream.read_signed_integer("user-data", 16)?
            )
        }
        self.m_min = bitstream.read_integer("min", 7)?;
        Ok(())
    }
}
