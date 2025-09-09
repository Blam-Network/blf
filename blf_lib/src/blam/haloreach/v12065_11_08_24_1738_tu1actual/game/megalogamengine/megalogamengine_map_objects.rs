use serde::{Deserialize, Serialize};
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::{BLFLibError, BLFLibResult};
use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_object_type_reference::c_object_type_reference;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_object_filter {
    pub m_label_string_index: u8, // 7 bits
    pub m_valid_parameters: u8, // 3 bits, flags
    pub m_object_type: Option<c_object_type_reference>,
    pub m_team: Option<u8>, // 4 bits
    pub m_user_data: Option<i16>, // 16 bits
    pub m_min: u8,
}

impl c_object_filter {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_label_string_index, 7)?;
        bitstream.write_integer(self.m_valid_parameters, 3)?;
        if (self.m_valid_parameters & 1) != 0 {
            self.m_object_type.as_ref()
                .ok_or_else(|| BLFLibError::from("m_object_type does not exist."))?
                .encode(bitstream)?;
        }
        if (self.m_valid_parameters & 2) != 0 {
            bitstream.write_integer(
                *self.m_team.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_team does not exist."))?,
                4
            )?;
        }
        if (self.m_valid_parameters & 4) != 0 {
            bitstream.write_signed_integer(
                *self.m_user_data.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_team does not exist."))?,
                16
            )?;
        }
        bitstream.write_integer(self.m_min, 7)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_label_string_index = bitstream.read_integer("label-string-index", 7)?;
        self.m_valid_parameters = bitstream.read_integer("valid-parameters", 3)?;
        if (self.m_valid_parameters & 1) != 0 {
            let mut object_type = c_object_type_reference::default();
            object_type.decode(bitstream)?;
            self.m_object_type = Some(object_type);
        }
        if (self.m_valid_parameters & 2) != 0 {
            self.m_team = Some(
                bitstream.read_integer("team", 4)?
            )
        }
        if (self.m_valid_parameters & 4) != 0 {
            self.m_user_data = Some(
                bitstream.read_signed_integer("user-data", 16)?
            )
        }
        self.m_min = bitstream.read_integer("min", 7)?;
        Ok(())
    }
}
