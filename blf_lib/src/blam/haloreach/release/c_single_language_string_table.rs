use std::io::{Cursor, Read};
use flate2::read::DeflateDecoder;
use serde::{Deserialize, Serialize};
use blf_lib::io::bitstream::e_bitstream_byte_order;
use blf_lib::types::array::StaticArray;
use blf_lib_derivable::result::{BLFLibError, BLFLibResult};
use crate::io::bitstream::c_bitstream_reader;
use crate::types::c_string::StaticString;

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct c_single_language_string_table<const max_string_count: usize, const max_string_length: usize> {
    pub strings: StaticArray<StaticString<max_string_length>, max_string_count>,
}

impl<const max_string_count: usize, const max_string_length: usize>
c_single_language_string_table<max_string_count, max_string_length>
{
    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        let string_count = bitstream.read_integer(9)? as usize;
        if string_count > max_string_count {
            return Err(BLFLibError::from(format!(
                "String count {} exceeds max {}",
                string_count, max_string_length
            )));
        }

        let mut offsets = vec![0; string_count];
        for i in 0..string_count {
            if bitstream.read_bool()? {
                offsets[i] = bitstream.read_integer(12)? as usize;
            }
        }

        let buffer_size = bitstream.read_integer(13)? as usize;

        let string_data = if bitstream.read_bool()? {
            let mut compressed_length = bitstream.read_integer(13)? as usize;

            // skip header
            compressed_length -= 6;
            bitstream.read_raw_data(6 * 8)?;

            let compressed_hopper_table_data: Vec<u8> = bitstream.read_raw_data(compressed_length * 8)?;
            let mut decompressed_string_table_data: Vec<u8> = Vec::new();
            let mut decoder = DeflateDecoder::new(Cursor::new(compressed_hopper_table_data));
            decoder.read_to_end(&mut decompressed_string_table_data)?;
            decompressed_string_table_data
        } else {
            bitstream.read_raw_data(buffer_size * 8)?
        };

        let mut string_stream = c_bitstream_reader::new(string_data.as_slice(), e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        string_stream.begin_reading();

        let mut strings = Vec::<String>::new();

        for &offset in &offsets {
            string_stream.seek(offset)?;
            let s = string_stream.read_string_utf8(max_string_length + 1)?;
            strings.push(s);
        }

        string_stream.finish_reading();

        self.strings = StaticArray::from_vec(
            strings
                .into_iter()
                .map(|s| StaticString::<max_string_length>::from_string(s))
                .collect::<Result<Vec<_>, _>>()?.as_ref()
        )?;

        Ok(())
    }
}
