use serde::{Deserialize, Serialize};
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_explicit_player::c_explicit_player;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_explicit_team::c_explicit_team;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;
use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_explicit_object::c_explicit_object;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_object_type_reference {
    pub m_object_type_index: i16, // 11 bits
}

impl c_object_type_reference {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_index::<2048>(self.m_object_type_index, 11)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object_type_index = bitstream.read_index::<2048>("object-type-index", 11)? as i16;

        Ok(())
    }
}