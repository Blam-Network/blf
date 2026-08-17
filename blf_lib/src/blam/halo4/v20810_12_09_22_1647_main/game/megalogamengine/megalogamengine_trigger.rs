use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;
use crate::blam::halo4::v20810_12_09_22_1647_main::game::megalogamengine::megalogamengine_trigger::e_trigger_execution_mode::{game_object, object_with_label};

/// Halo 4 `e_trigger_execution_mode` (`c_enum<...,0,7>` → 0..6, 3 bits).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive, Default, Serialize, Deserialize)]
pub enum e_trigger_execution_mode {
    #[default]
    general = 0,
    player = 1,
    random_player = 2,
    team = 3,
    object = 4,
    object_with_label = 5,
    game_object = 6,
}

/// Halo 4 `e_trigger_type` (`c_enum<...,0,9>` → 0..8, 4 bits).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive, Default, Serialize, Deserialize)]
pub enum e_trigger_type {
    #[default]
    normal = 0,
    subroutine = 1,
    initialization = 2,
    local_initialization = 3,
    host_migration = 4,
    double_migration = 5,
    object_death = 6,
    local = 7,
    pregame = 8,
}

/// `MegaloGameObjectTypeEnum` (`c_enum<...,0,2>` → 0..1, 1 bit).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive, Default, Serialize, Deserialize)]
pub enum e_megalo_game_object_type {
    #[default]
    none = 0,
    candy_spawner = 1,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_trigger {
    pub m_execution_mode: e_trigger_execution_mode,
    pub m_trigger_type: e_trigger_type,
    pub m_object_filter_index: i8,
    pub m_game_object_type: e_megalo_game_object_type,
    pub m_game_object_filter_index: i8,
    pub m_first_condition: i16,
    pub m_condition_count: u16,
    pub m_first_action: i16,
    pub m_action_count: u16,
    pub m_frame_update_frequency: i16,
    pub m_frame_update_offset: i16,
}

impl c_trigger {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_enum_raw(self.m_execution_mode, 3)?;
        bitstream.write_enum_raw(self.m_trigger_type, 4)?;
        if self.m_execution_mode == object_with_label {
            bitstream.write_index::<16>(self.m_object_filter_index as i32, 4)?;
        } else if self.m_execution_mode == game_object {
            bitstream.write_enum_raw(self.m_game_object_type, 1)?;
            bitstream.write_index::<4>(self.m_game_object_filter_index as i32, 2)?;
        }
        bitstream.write_index::<576>(self.m_first_condition as i32, 10)?;
        bitstream.write_integer(self.m_condition_count, 10)?;
        bitstream.write_index::<1088>(self.m_first_action as i32, 11)?;
        bitstream.write_integer(self.m_action_count, 11)?;
        bitstream.write_index::<255>(self.m_frame_update_frequency as i32, 8)?;
        bitstream.write_index::<255>(self.m_frame_update_offset as i32, 8)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_execution_mode = bitstream.read_enum_raw("execution-mode", 3)?;
        self.m_trigger_type = bitstream.read_enum_raw("trigger-type", 4)?;
        self.m_object_filter_index = -1;
        self.m_game_object_filter_index = -1;
        if self.m_execution_mode == object_with_label {
            self.m_object_filter_index = bitstream.read_index::<16>("object-filter-index", 4)? as i8;
        } else if self.m_execution_mode == game_object {
            self.m_game_object_type = bitstream.read_enum_raw("game-object-type", 1)?;
            self.m_game_object_filter_index =
                bitstream.read_index::<4>("game-object-filter-index", 2)? as i8;
        }
        self.m_first_condition = bitstream.read_index::<576>("first-condition-index", 10)? as i16;
        self.m_condition_count = bitstream.read_integer("condition-count", 10)?;
        self.m_first_action = bitstream.read_index::<1088>("first-action-index", 11)? as i16;
        self.m_action_count = bitstream.read_integer("action-count", 11)?;
        self.m_frame_update_frequency =
            bitstream.read_index::<255>("frame-update-frequency", 8)? as i16;
        self.m_frame_update_offset =
            bitstream.read_index::<255>("frame-update-offset", 8)? as i16;

        Ok(())
    }
}
