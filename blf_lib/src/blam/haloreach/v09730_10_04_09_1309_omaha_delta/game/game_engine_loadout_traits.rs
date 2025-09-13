use serde::{Deserialize, Serialize};
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;
use crate::types::array::StaticArray;
use serde_hex::SerHex;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_object_type_reference::c_object_type_reference;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_loadout_unknown_struct {
    pub m_object_types: StaticArray<c_object_type_reference, 4>,
    pub m_slot_numbers: StaticArray<u8, 4>,
}

impl s_loadout_unknown_struct {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        for object_type in &self.m_object_types {
            object_type.encode(bitstream)?;
        }

        for slot_number in &self.m_slot_numbers {
            bitstream.write_integer(*slot_number, 8)?;
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        for object_type in &mut self.m_object_types {
            object_type.decode(bitstream)?;
        }

        for i in 0..4 {
            self.m_slot_numbers[i] = bitstream.read_integer("number-slot", 8)?;
        }

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_loadout_palette_unknown_struct {
    pub m_loadouts: Vec<u8>,
}

impl s_loadout_palette_unknown_struct {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_loadouts.len() as u8, 3)?;
        for loadout_reference_index in &self.m_loadouts {
            bitstream.write_integer(*loadout_reference_index, 8)?;
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        let entry_count = bitstream.read_integer("entries-count", 3)?;
        for i in 0..entry_count {
            self.m_loadouts.push(bitstream.read_integer("loadout-reference-index", 8)?);
        }

        Ok(())
    }
}
