use serde::{Deserialize, Serialize};
use blf_lib::blam::haloreach::v09730_10_04_09_1309_omaha_delta::game::megalogamengine::megalogamengine_custom_variable_reference::c_custom_variable_reference;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_object_reference::c_object_reference;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_object_type_reference::c_object_type_reference;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_player_reference::c_player_reference;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;
use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_custom_timer_reference::c_custom_timer_reference;
use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_team_reference::c_team_reference;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_requisition {
    pub m_object_type: c_object_type_reference,
    pub m_unknown_1: i16, // 11 bit index
    pub m_unknown_2: bool,
    pub m_unknown_3: bool,
    pub m_unknown_4: u16,

}

impl s_requisition {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object_type.encode(bitstream)?;
        bitstream.write_index::<65535>(self.m_unknown_1, 11)?;
        bitstream.write_bool(self.m_unknown_2)?;
        if self.m_unknown_2 {
            bitstream.write_bool(self.m_unknown_3)?;
            if self.m_unknown_3 {
                bitstream.write_integer(self.m_unknown_4, 15)?;
            }
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object_type.decode(bitstream)?;
        self.m_unknown_1 = bitstream.read_index::<65535>("unknown-1", 11)? as i16;
        self.m_unknown_2 = bitstream.read_bool("unknown-2")?;
        if self.m_unknown_2 {
            self.m_unknown_3 = bitstream.read_bool("unknown-3")?;
            if self.m_unknown_3 {
                self.m_unknown_4 = bitstream.read_integer("unknown-4", 15)?;
            }
        }


        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_requisition_palette {
    pub m_baseline: u8, // 4 bits
    pub entries: Vec<s_requisition>,
}

impl s_requisition_palette {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_baseline, 4)?;
        bitstream.write_integer(self.entries.len() as u8, 6)?;
        for entry in &self.entries {
            entry.encode(bitstream)?;
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_baseline = bitstream.read_integer("baseline", 4)?;
        let entry_count = bitstream.read_integer("entries", 6)?;
        for i in 0..entry_count {
            let mut entry = s_requisition::default();
            entry.decode(bitstream)?;
            self.entries.push(entry);
        }

        Ok(())
    }
}