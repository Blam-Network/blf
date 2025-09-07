use serde::{Deserialize, Serialize};
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_megalogamengine_map_permissions {
    pub m_except_map_ids: Vec<u16>,
    pub m_allow_by_default: bool,
}

impl c_megalogamengine_map_permissions {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_except_map_ids.len() as u32, 6)?;
        for map_id in &self.m_except_map_ids {
            bitstream.write_integer(*map_id as u32, 16)?;
        }
        bitstream.write_bool(self.m_allow_by_default)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        let map_id_count = bitstream.read_integer("exception-count", 6)?;
        for i in 0..map_id_count {
            self.m_except_map_ids.push(bitstream.read_integer("map-id", i)?);
        }
        self.m_allow_by_default = bitstream.read_bool("default-permission")?;

        Ok(())
    }
}