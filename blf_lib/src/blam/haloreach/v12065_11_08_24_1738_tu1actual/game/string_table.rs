use std::io::{Cursor, Seek, SeekFrom};
use binrw::{BinRead, BinWrite, NullString};
use serde::{Deserialize, Serialize};
use blf_lib_derivable::result::BLFLibResult;
use crate::assert_ok;
use crate::blam::common::memory::data_compress::{runtime_data_compress, runtime_data_decompress};
use crate::io::bitstream::{c_bitstream_reader, c_bitstream_writer};

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct c_single_language_string_table<const max_string_count: usize, const max_string_length: usize> {
    strings: Vec<String>,
}

impl<const max_string_count: usize, const max_string_length: usize>
c_single_language_string_table<max_string_count, max_string_length>
{
    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        let string_count: usize = bitstream.read_integer(9)?;

        let mut offsets = vec![0; string_count];
        for i in 0..string_count {
            if bitstream.read_bool()? {
                offsets[i] = bitstream.read_integer(12)?;
            }
        }

        let buffer_size: usize = bitstream.read_integer(13)?;

        let string_data = if bitstream.read_bool()? {
            let mut compressed_length: usize = bitstream.read_integer(13)?;
            let compressed_data = bitstream.read_raw_data(compressed_length * 8)?;
            let mut decompressed_data = Vec::with_capacity(buffer_size);

            runtime_data_decompress(&compressed_data, &mut decompressed_data, bitstream.get_byte_order())?;
            decompressed_data
        } else {
            bitstream.read_raw_data(buffer_size * 8)?
        };

        let mut string_reader = Cursor::new(string_data);
        for &offset in &offsets {
            string_reader.seek(SeekFrom::Start(offset))?;
            self.strings.push(NullString::read(&mut string_reader)?.to_string());

            string_reader.seek(SeekFrom::Start(offset))?;
        }

        Ok(())
    }

    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        assert_ok!(self.strings.len() <= max_string_count);
        bitstream.write_integer(self.strings.len() as u32, 9)?;

        let mut offset: u16 = 0;
        for string in self.strings.iter() {
            bitstream.write_bool(true)?;
            bitstream.write_integer(offset, 12)?;
            offset += string.len() as u16 + 1;
        }

        let mut buffer = Vec::<u8>::new();
        let mut compressed_buffer = Vec::with_capacity(buffer.len());
        let mut string_writer = Cursor::new(&mut buffer);
        self.strings
            .iter()
            .map(|string| NullString::from(string.to_string()))
            .collect::<Vec<_>>()
            .write_options(&mut string_writer, bitstream.get_byte_order().into(), ())?;
        runtime_data_compress(&buffer, &mut compressed_buffer, bitstream.get_byte_order())?;

        bitstream.write_integer(buffer.len() as u32, 13)?;
        bitstream.write_bool(true)?; // is compressed? always.
        bitstream.write_integer(compressed_buffer.len() as u32, 13)?;
        bitstream.write_raw_data(compressed_buffer.as_slice(), compressed_buffer.len() * 8)?;

        Ok(())
    }
}
