use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use blf_lib::blam::haloreach::v08516_10_02_19_1607_omaha_alpha::game::megalogamengine::megalogamengine_trigger::e_trigger_execution_mode::object_with_label;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive, Default, Serialize, Deserialize)]
pub enum e_trigger_execution_mode {
    #[default]
    general = 0,
    player = 1,
    random_player = 2,
    team = 3,
    object_with_label = 4,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive, Default, Serialize, Deserialize)]
pub enum e_trigger_type {
    #[default]
    normal = 0,
    subroutine = 1,
    initialization = 2,
    local_initialization = 3,
    host_migration = 4,
    object_death = 5,
    local = 6,
    pregame = 7,
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
        bitstream.write_enum_raw(self.m_execution_mode, 3)?;
        bitstream.write_enum_raw(self.m_trigger_type, 3)?;
        if self.m_execution_mode == object_with_label {
            bitstream.write_index::<16>(self.m_object_filter_index, 4)?;
        }
        bitstream.write_integer(self.m_first_condition, 9)?;
        bitstream.write_integer(self.m_condition_count, 10)?;
        bitstream.write_integer(self.m_first_action, 10)?;
        bitstream.write_integer(self.m_action_count, 11)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_execution_mode = bitstream.read_enum_raw("execution-mode", 3)?;
        self.m_trigger_type = bitstream.read_enum_raw("trigger-type", 3)?;
        if self.m_execution_mode == object_with_label {
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
