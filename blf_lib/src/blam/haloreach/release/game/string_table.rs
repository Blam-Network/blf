use std::io::{Cursor, Read};
use binrw::{BinWrite, Endian};
use serde::{Deserialize, Serialize};
use blf_lib::io::bitstream::e_bitstream_byte_order;
use blf_lib_derivable::result::{BLFLibError, BLFLibResult};
use crate::assert_ok;
use crate::blam::common::memory::data_compress::{runtime_data_compress, runtime_data_decompress};
use crate::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use crate::types::c_string::StaticString;

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct c_single_language_string_table<const max_string_count: usize, const max_string_length: usize> {
    strings: Vec<StaticString<max_string_length>>,
}

impl<const max_string_count: usize, const max_string_length: usize>
c_single_language_string_table<max_string_count, max_string_length>
{
    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        let string_count = bitstream.read_integer(9)?;
        assert_ok!(string_count <= max_string_count);

        let mut offsets = vec![0; string_count];
        for i in 0..string_count {
            if bitstream.read_bool()? {
                offsets[i] = bitstream.read_integer(12)?;
            }
        }

        let buffer_size: usize = bitstream.read_integer(13)?;
        println!("buffer size: {}", buffer_size);

        let string_data = if bitstream.read_bool()? {
            let mut compressed_length: usize = bitstream.read_integer(13)?;
            let compressed_data = bitstream.read_raw_data(compressed_length * 8)?;
            let mut decompressed_data = Vec::with_capacity(buffer_size);
            runtime_data_decompress(&compressed_data, &mut decompressed_data, bitstream.get_byte_order())?;
            decompressed_data
        } else {
            bitstream.read_raw_data(buffer_size * 8)?
        };

        let mut string_stream = c_bitstream_reader::new(string_data.as_slice(), e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        string_stream.begin_reading();

        let mut strings = Vec::<String>::new();

        for &offset in &offsets {
            string_stream.seek(offset)?;
            let s = string_stream.read_string_utf8(max_string_length)?;
            strings.push(s);
        }

        string_stream.finish_reading();

        self.strings.clone_from(strings
            .into_iter()
            .map(|s| StaticString::<max_string_length>::from_string(s))
            .collect::<Result<Vec<_>, _>>()?
            .as_ref()
        );

        Ok(())
    }

    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        assert_ok!(self.strings.len() <= max_string_count);
        bitstream.write_integer(self.strings.len() as u32, 9)?;

        let mut offset: u16 = 0;
        for string in self.strings.iter() {
            bitstream.write_integer(offset, 12)?;
            offset += string.get_string()?.len() as u16 + 1;
        }

        let mut buffer = Vec::<u8>::with_capacity(max_string_count * max_string_length);
        let mut compressed_buffer = Vec::with_capacity(buffer.len());
        let mut string_writer = Cursor::new(&mut buffer);
        self.strings.write_options(&mut string_writer, Endian::Big, ())?;
        runtime_data_compress(&buffer, &mut compressed_buffer, bitstream.get_byte_order())?;

        Ok(())

    }
}
