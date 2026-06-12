use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive, Default, Serialize, Deserialize)]
pub enum e_explicit_object_type {
    #[default]
    no_object = 0,
    global_0 = 1,
    global_1 = 2,
    global_2 = 3,
    global_3 = 4,
    global_4 = 5,
    global_5 = 6,
    global_6 = 7,
    global_7 = 8,
    global_8 = 9,
    global_9 = 10,
    global_10 = 11,
    global_11 = 12,
    global_12 = 13,
    global_13 = 14,
    global_14 = 15,
    global_15 = 16,
    current = 17,
    hud_target = 18,
    killed = 19,
    killer = 20,
    unknown_21 = 21,
    temporary_0 = 22,
    temporary_1 = 23,
    temporary_2 = 24,
    temporary_3 = 25,
    temporary_4 = 26,
    temporary_5 = 27,
    temporary_6 = 28,
    temporary_7 = 29,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_explicit_object {
    pub m_explicit_object_type: e_explicit_object_type, // 5 bits
}

impl c_explicit_object {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_enum_raw(self.m_explicit_object_type, 5)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_explicit_object_type = bitstream.read_enum_raw("explicit-object-type", 5)?;

        Ok(())
    }
}
