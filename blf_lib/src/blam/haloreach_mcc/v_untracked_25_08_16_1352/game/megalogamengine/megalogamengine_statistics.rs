use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(2)]
pub enum e_megalo_game_statistic_format {
    #[default]
    number = 0,
    number_with_sign = 1,
    percentage = 2,
    time = 3,
}

#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(2)]
pub enum e_megalo_game_statistic_sort_order {
    #[default]
    none = -1,
    ascending = 0,
    descending = 1,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(1)]
pub enum e_megalo_game_statistic_grouping {
    #[default]
    player = 0,
    team = 1,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_megalo_game_statistic {
    pub m_name_string_index: u8, // 7 bits
    pub m_format: e_megalo_game_statistic_format,
    pub m_sort_order: e_megalo_game_statistic_sort_order,
    pub m_grouping: e_megalo_game_statistic_grouping,
}

impl c_megalo_game_statistic {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_name_string_index, 7)?;
        bitstream.write_enum(self.m_format)?;
        bitstream.write_enum(self.m_sort_order)?;
        bitstream.write_enum(self.m_grouping)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_name_string_index = bitstream.read_integer("name-string-index", 7)?;
        self.m_format = bitstream.read_enum("format")?;
        self.m_sort_order = bitstream.read_enum("sort-order")?;
        self.m_grouping = bitstream.read_enum("grouping")?;

        Ok(())
    }
}
