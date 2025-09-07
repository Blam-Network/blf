use serde::{Deserialize, Serialize};
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::OPTION_TO_RESULT;
use blf_lib_derivable::result::BLFLibResult;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_user_defined_option_value {
    pub m_value: i16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_name_string_index: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_description_string_index: Option<i8>,
}

impl s_user_defined_option_value {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer, is_range: bool) -> BLFLibResult {
        bitstream.write_signed_integer(self.m_value, 10)?;

        if !is_range {
            let name_index = OPTION_TO_RESULT!(
                self.m_name_string_index,
                "Tried to write a ranged s_user_defined_option with no name!"
            )?;
            let description_index = OPTION_TO_RESULT!(
                self.m_name_string_index,
                "Tried to write a ranged s_user_defined_option with no description!"
            )?;
            bitstream.write_integer(name_index as u8, 7)?;
            bitstream.write_integer(description_index as u8, 7)?;
        }


        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader, is_range: bool) -> BLFLibResult {
        self.m_value = bitstream.read_signed_integer("value", 10)?;

        if !is_range {
            self.m_name_string_index = Some(bitstream.read_integer("name-string-index", 7)?);
            self.m_description_string_index = Some(bitstream.read_integer("description-string-index", 7)?);
        } else {
            self.m_name_string_index = Some(-1);
            self.m_description_string_index = Some(-1);
        }

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_user_defined_option {
    pub m_name_string_index: u8,
    pub m_description_string_index: u8,

    // ranged
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_range_default_value: Option<s_user_defined_option_value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_range_min_value: Option<s_user_defined_option_value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_range_max_value: Option<s_user_defined_option_value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_range_current_value: Option<i16>,

    // non-ranged
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_default_value_index: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_values: Option<Vec<s_user_defined_option_value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_current_value_index: Option<u8>,

}

impl s_user_defined_option {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_name_string_index, 7)?;
        bitstream.write_integer(self.m_description_string_index, 7)?;

        if let (
            Some(range_default_value),
            Some(range_min_value),
            Some(range_max_value),
            Some(range_current_value)
        ) = (
            &self.m_range_default_value,
            &self.m_range_min_value,
            &self.m_range_max_value,
            &self.m_range_current_value
        ) {
            range_default_value.encode(bitstream, true)?;
            range_min_value.encode(bitstream, true)?;
            range_max_value.encode(bitstream, true)?;
            bitstream.write_signed_integer(*range_current_value, 10)?;
        }
        else if let (
            Some(default_value_index),
            Some(values),
            Some(current_value_index)
        ) = (
            &self.m_default_value_index,
            &self.m_values,
            &self.m_current_value_index,
        ) {
            bitstream.write_integer(*default_value_index, 3)?;
            bitstream.write_integer(values.len() as u32, 4)?;
            for i in 0..values.len() {
                values[i].encode(bitstream, false)?;
            }
            bitstream.write_signed_integer(*current_value_index, 3)?;
        }
        else {
            return Err(format!("Can't encode invalid s_user_defined_option: {self:?}").into())
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_name_string_index = bitstream.read_integer("name-string-index", 7)?;
        self.m_description_string_index = bitstream.read_integer("description-string-index", 7)?;

        if bitstream.read_bool("is-ranged-option")? {
            let mut range_default_value = s_user_defined_option_value::default();
            let mut range_min_value = s_user_defined_option_value::default();
            let mut range_max_value = s_user_defined_option_value::default();

            range_default_value.decode(bitstream, true)?;
            range_min_value.decode(bitstream, true)?;
            range_max_value.decode(bitstream, true)?;

            self.m_range_default_value = Some(range_default_value);
            self.m_range_min_value = Some(range_min_value);
            self.m_range_max_value = Some(range_max_value);
            self.m_range_current_value = Some(bitstream.read_signed_integer("user-defined-option-value", 10)?);
        }
        else {
            self.m_default_value_index = Some(bitstream.read_integer("default-value-index", 3)?);
            let value_count = bitstream.read_integer("value-count", 4)?;
            let mut values = Vec::<s_user_defined_option_value>::new();
            for i in 0..value_count {
                let mut value = s_user_defined_option_value::default();
                value.decode(bitstream, false)?;
                values.push(value);
            }
            self.m_values = Some(values);
            self.m_current_value_index = Some(bitstream.read_integer("current-value-index", 3)?);
        }


        Ok(())
    }
}