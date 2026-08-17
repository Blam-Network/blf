use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;

/// Halo 4 `e_explicit_object_type` — ManagedMegalo count 32 (0..31), 5 bits.
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
    global_16 = 17,
    global_17 = 18,
    temporary_0 = 19,
    temporary_1 = 20,
    temporary_2 = 21,
    temporary_3 = 22,
    temporary_4 = 23,
    temporary_5 = 24,
    temporary_6 = 25,
    temporary_7 = 26,
    current_object = 27,
    target_object = 28,
    object_death_dead_object = 29,
    object_death_killing_object = 30,
    current_spawner = 31,
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
