use serde::{Deserialize, Serialize};
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_custom_timer_reference::c_custom_timer_reference;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_custom_variable_reference::c_custom_variable_reference;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_object_reference::c_object_reference;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_player_reference::c_player_reference;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_team_reference::c_team_reference;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_megalo_game_statistic {
    pub m_name_string_index: u8, // 7 bits
    pub m_format: u8, // 2 bits
    pub m_sort_order: u8, // 2 bits
    pub m_growuping: u8, // 1 bits
}

impl c_megalo_game_statistic {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_name_string_index, 7)?;
        bitstream.write_integer(self.m_format, 2)?;
        bitstream.write_integer(self.m_sort_order, 2)?;
        bitstream.write_integer(self.m_growuping, 1)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_name_string_index = bitstream.read_integer("name-string-index", 7)?;
        self.m_format = bitstream.read_integer("format", 2)?;
        self.m_sort_order = bitstream.read_integer("sort-order", 2)?;
        self.m_growuping = bitstream.read_integer("groupingt", 1)?;

        Ok(())
    }
}
