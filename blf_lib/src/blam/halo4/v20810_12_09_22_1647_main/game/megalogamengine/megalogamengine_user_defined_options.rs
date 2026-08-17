use serde::{Deserialize, Serialize};
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;

/// Halo 4 string-table index bits for MaxStrings=148.
const k_string_index_bits: usize = 8;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_user_defined_option_value {
    pub m_value: i16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_name_string_index: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_description_string_index: Option<i16>,
}

impl s_user_defined_option_value {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer, is_range: bool) -> BLFLibResult {
        bitstream.write_signed_integer(self.m_value, 10)?;
        if !is_range {
            bitstream.write_integer(self.m_name_string_index.unwrap_or(0) as u32, k_string_index_bits)?;
            bitstream.write_integer(
                self.m_description_string_index.unwrap_or(0) as u32,
                k_string_index_bits,
            )?;
        }
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader, is_range: bool) -> BLFLibResult {
        self.m_value = bitstream.read_signed_integer("value", 10)?;
        if is_range {
            self.m_name_string_index = Some(-1);
            self.m_description_string_index = Some(-1);
        } else {
            self.m_name_string_index =
                Some(bitstream.read_integer("name-string-index", k_string_index_bits)?);
            self.m_description_string_index =
                Some(bitstream.read_integer("description-string-index", k_string_index_bits)?);
        }
        Ok(())
    }
}

/// Halo 4 `s_user_defined_option`.
/// Current value / value-index is written by the custom variant after this encode.
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_user_defined_option {
    pub m_name_string_index: u16,
    pub m_description_string_index: u16,
    pub m_is_ranged: bool,
    pub m_default_value: i16,
    pub m_default_value_index: u8,
    pub m_values: Vec<s_user_defined_option_value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_current_value: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_current_value_index: Option<u8>,
}

impl s_user_defined_option {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_name_string_index, k_string_index_bits)?;
        bitstream.write_integer(self.m_description_string_index, k_string_index_bits)?;
        bitstream.write_bool(self.m_is_ranged)?;
        if self.m_is_ranged {
            bitstream.write_signed_integer(self.m_default_value, 10)?;
        } else {
            bitstream.write_integer(self.m_default_value_index, 4)?;
            bitstream.write_integer(self.m_values.len() as u32, 5)?;
        }
        for value in &self.m_values {
            value.encode(bitstream, self.m_is_ranged)?;
        }
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_name_string_index = bitstream.read_integer("name-string-index", k_string_index_bits)?;
        self.m_description_string_index =
            bitstream.read_integer("description-string-index", k_string_index_bits)?;
        self.m_is_ranged = bitstream.read_bool("is-ranged-option")?;
        let value_count = if self.m_is_ranged {
            self.m_default_value = bitstream.read_signed_integer("default-value", 10)?;
            2usize
        } else {
            self.m_default_value_index = bitstream.read_integer("default-value-index", 4)?;
            bitstream.read_integer::<u16>("value-count", 5)? as usize
        };
        self.m_values.clear();
        for _ in 0..value_count {
            let mut value = s_user_defined_option_value::default();
            value.decode(bitstream, self.m_is_ranged)?;
            self.m_values.push(value);
        }
        Ok(())
    }
}
