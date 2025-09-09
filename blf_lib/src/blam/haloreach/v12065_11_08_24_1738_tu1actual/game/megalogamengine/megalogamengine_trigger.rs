use serde::{Deserialize, Serialize};
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_custom_timer_reference::c_custom_timer_reference;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_custom_variable_reference::c_custom_variable_reference;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_object_reference::c_object_reference;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_player_reference::c_player_reference;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_team_reference::c_team_reference;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_trigger {
    pub m_execution_mode: u8, // 3 bits
    pub m_trigger_type: u8, // 3 bits
    pub m_object_filter_index: i8, // 4 bits
    pub m_first_condition: u16, // 9 bits
    pub m_condition_count: u16, // 10 bits
    pub m_first_action: u16, // 10 bits
    pub m_action_count: u16, // 11 bits
}

impl c_trigger {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_execution_mode, 3)?;
        bitstream.write_integer(self.m_trigger_type, 3)?;
        if self.m_execution_mode == 5 {
            bitstream.write_index::<16>(self.m_object_filter_index, 4)?;
        }
        bitstream.write_integer(self.m_first_condition, 9)?;
        bitstream.write_integer(self.m_condition_count, 10)?;
        bitstream.write_integer(self.m_first_action, 10)?;
        bitstream.write_integer(self.m_action_count, 11)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_execution_mode = bitstream.read_integer("execution-mode", 3)?;
        self.m_trigger_type = bitstream.read_integer("trigger-type", 3)?;
        if self.m_execution_mode == 5 {
            self.m_object_filter_index = bitstream.read_index::<16>("object-filter-index", 4)? as i8;
        } else {
            self.m_object_filter_index = -1;
        }
        self.m_first_condition = bitstream.read_integer("first-condition", 9)?;
        self.m_condition_count = bitstream.read_integer("condition-count", 10)?;
        self.m_first_action = bitstream.read_integer("first-action", 10)?;
        self.m_action_count = bitstream.read_integer("action-count", 11)?;

        Ok(())
    }
}
