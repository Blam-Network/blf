use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;
use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_trigger::e_trigger_execution_mode::for_each_object_with_label;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive, Default, Serialize, Deserialize)]
pub enum e_trigger_execution_mode {
    #[default]
    normal = 0,
    for_each_player = 1,
    for_each_player_randomly = 2,
    for_each_team = 3,
    for_each_object = 4,
    for_each_object_with_label = 5,
    unknown6 = 6,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive, Default, Serialize, Deserialize)]
pub enum e_trigger_type {
    #[default]
    normal = 0,
    subroutine = 1,
    on_init = 2,
    on_local_init = 3,
    on_host_migration = 4,
    on_object_death = 5,
    local = 6,
    pregame = 7,
    incident = 8,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_trigger {
    pub m_execution_mode: e_trigger_execution_mode, // 3 bits
    pub m_trigger_type: e_trigger_type, // 3 bits
    pub m_object_filter_index: i8, // 4 bits
    pub m_first_condition: u16, // 9 bits
    pub m_condition_count: u16, // 10 bits
    pub m_first_action: u16, // 10 bits
    pub m_action_count: u16, // 11 bits
}

impl c_trigger {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_enum(self.m_execution_mode, 3)?;
        bitstream.write_enum(self.m_trigger_type, 3)?;
        if self.m_execution_mode == for_each_object_with_label {
            bitstream.write_index::<16>(self.m_object_filter_index, 4)?;
        }
        bitstream.write_integer(self.m_first_condition, 9)?;
        bitstream.write_integer(self.m_condition_count, 10)?;
        bitstream.write_integer(self.m_first_action, 10)?;
        bitstream.write_integer(self.m_action_count, 11)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_execution_mode = bitstream.read_enum("execution-mode", 3)?;
        self.m_trigger_type = bitstream.read_enum("trigger-type", 3)?;
        if self.m_execution_mode == for_each_object_with_label {
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
