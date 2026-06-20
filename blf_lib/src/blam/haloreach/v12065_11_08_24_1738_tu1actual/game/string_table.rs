use std::io::{Cursor, Seek, SeekFrom};
use binrw::{BinRead, BinWrite, NullString};
use serde::{Deserialize, Serialize};
use blf_lib::blam::halo3::v12070_08_09_05_2031_halo3_ship::cseries::language::k_language_count;
use blf_lib::types::array::StaticArray;
use blf_lib_derivable::result::BLFLibResult;
use crate::assert_ok;
use crate::blam::common::memory::data_compress::{runtime_data_compress, runtime_data_decompress};
use crate::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use crate::io::bitstream::e_bitstream_byte_order::_bitstream_byte_order_big_endian;

/// Reach `c_string_buffer::encode` only attempts compression at this size or above.
const k_string_buffer_compression_threshold: usize = 128;

fn write_string_buffer_blob(
    bitstream: &mut c_bitstream_writer,
    buffer: &[u8],
    buffer_size_bit_length: usize,
) -> BLFLibResult {
    bitstream.write_integer(buffer.len() as u32, buffer_size_bit_length)?;

    if buffer.len() >= k_string_buffer_compression_threshold {
        let mut compressed_buffer = Vec::with_capacity(buffer.len());
        runtime_data_compress(&buffer.to_vec(), &mut compressed_buffer, bitstream.get_byte_order())?;
        bitstream.write_bool(true)?;
        bitstream.write_integer(compressed_buffer.len() as u32, buffer_size_bit_length)?;
        bitstream.write_raw_data(compressed_buffer.as_slice(), compressed_buffer.len() * 8)?;
    } else {
        bitstream.write_bool(false)?;
        bitstream.write_raw_data(buffer, buffer.len() * 8)?;
    }

    Ok(())
}

fn read_null_terminated_utf8(data: &[u8], offset: u64) -> BLFLibResult<String> {
    let start = offset as usize;
    if start >= data.len() {
        return Err(format!("String offset {offset} out of bounds").into());
    }

    let end = data[start..]
        .iter()
        .position(|&byte| byte == 0)
        .map(|position| start + position)
        .ok_or("String missing null terminator")?;

    Ok(String::from_utf8(data[start..end].to_vec())?)
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct c_single_language_string_table<
    const max_string_count: usize,
    const max_string_length: usize,
    // these dont exist in this manner in reach but are configured some other way
    const offset_bit_length: usize,
    const buffer_size_bit_length: usize,
    const count_bit_length: usize,
> {
    strings: Vec<String>,
}

impl<
    const max_string_count: usize,
    const max_string_length: usize,
    const offset_bit_length: usize,
    const buffer_size_bit_length: usize,
    const count_bit_length: usize,
>
c_single_language_string_table<
    max_string_count,
    max_string_length,
    offset_bit_length,
    buffer_size_bit_length,
    count_bit_length,
>
{
    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        let string_count: usize = bitstream.read_integer("string-count", count_bit_length)?;

        if string_count == 0 {
            return Ok(());
        }

        let mut offsets = vec![0; string_count];
        for i in 0..string_count {
            if bitstream.read_bool("exists")? {
                offsets[i] = bitstream.read_integer("offset", offset_bit_length)?;
            }
        }

        let buffer_size: usize = bitstream.read_integer("size", buffer_size_bit_length)?;

        let string_data = if bitstream.read_bool("compressed")? {
            let mut compressed_length: usize = bitstream.read_integer("compressed-buffer-size", buffer_size_bit_length)?;
            let compressed_data = bitstream.read_raw_data(compressed_length * 8)?;
            let mut decompressed_data = Vec::with_capacity(buffer_size);

            runtime_data_decompress(&compressed_data, &mut decompressed_data, bitstream.get_byte_order())?;
            decompressed_data
        } else {
            bitstream.read_raw_data(buffer_size * 8)?
        };

        let mut string_reader = Cursor::new(string_data);
        for &offset in &offsets {
            self.strings.push(read_null_terminated_utf8(
                string_reader.get_ref(),
                offset as u64,
            )?);
        }

        Ok(())
    }

    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        assert_ok!(self.strings.len() <= max_string_count);
        bitstream.write_integer(self.strings.len() as u32, count_bit_length)?;

        if self.strings.len() == 0 {
            return Ok(());
        }

        let mut offset: u16 = 0;
        for string in self.strings.iter() {
            bitstream.write_bool(true)?;
            bitstream.write_integer(offset, offset_bit_length)?;
            offset += string.len() as u16 + 1;
        }

        let mut buffer = Vec::<u8>::new();
        let mut string_writer = Cursor::new(&mut buffer);
        self.strings
            .iter()
            .map(|string| NullString::from(string.to_string()))
            .collect::<Vec<_>>()
            .write_options(&mut string_writer, bitstream.get_byte_order().into(), ())?;
        write_string_buffer_blob(bitstream, &buffer, buffer_size_bit_length)?;

        Ok(())
    }
}


#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct c_string_table<
    const max_string_count: usize,
    const max_string_length: usize,
    // these dont exist in this manner in reach but are configured some other way
    const offset_bit_length: usize,
    const buffer_size_bit_length: usize,
    const count_bit_length: usize,
> {
    strings: StaticArray<Vec<Option<String>>, k_language_count>,
}

impl<
    const max_string_count: usize,
    const max_string_length: usize,
    const offset_bit_length: usize,
    const buffer_size_bit_length: usize,
    const count_bit_length: usize,
>
c_string_table<
    max_string_count,
    max_string_length,
    offset_bit_length,
    buffer_size_bit_length,
    count_bit_length,
>
{
    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        let string_count: usize = bitstream.read_integer("string-count", count_bit_length)?;

        if string_count == 0 {
            return Ok(());
        }

        let mut offsets = vec![vec![0i16; string_count]; k_language_count];
        for j in 0..string_count {
            for i in 0..k_language_count {
                if bitstream.read_bool("exists")? {
                    offsets[i][j] = bitstream.read_integer("index", offset_bit_length)?;
                }
                else {
                    offsets[i][j] = -1;
                }
            }
        }

        let buffer_size: usize = bitstream.read_integer("size", buffer_size_bit_length)?;

        let string_data = if bitstream.read_bool("compressed")? {
            let mut compressed_length: usize = bitstream.read_integer("compressed-buffer-size", buffer_size_bit_length)?;
            let compressed_data = bitstream.read_raw_data(compressed_length * 8)?;
            let mut decompressed_data = Vec::with_capacity(buffer_size);

            runtime_data_decompress(&compressed_data, &mut decompressed_data, bitstream.get_byte_order())?;
            decompressed_data
        } else {
            bitstream.read_raw_data(buffer_size * 8)?
        };

        for (language_index, language_offsets) in offsets.iter().enumerate() {
            for &offset in language_offsets {
                if offset == -1 {
                    self.strings[language_index].push(None);
                    continue;
                }
                assert_ok!(offset >= 0);

                self.strings[language_index].push(Some(read_null_terminated_utf8(
                    &string_data,
                    offset as u64,
                )?));
            }
        }

        Ok(())
    }

    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        assert_ok!(self.strings[0].len() <= max_string_count);
        bitstream.write_integer(self.strings[0].len() as u32, count_bit_length)?;

        if self.strings[0].len() == 0 {
            return Ok(());
        }

        let mut string_writer = c_bitstream_writer::new(
            k_language_count * max_string_count * max_string_length,
            _bitstream_byte_order_big_endian
        );
        let mut offset = 0u16;
        string_writer.begin_writing();
        for i in 0..self.strings[0].len() {
            // If the string is the same in every language, we consider it non-localized
            // and only write it once.
            // This is how Bungie handles duplicates.
            let mut deduplicate = true;
            let string = self.strings[0][i].clone();

            if string.is_none() {
                deduplicate = false;
            }

            for language_index in 1..k_language_count {
                if string != self.strings[language_index][i] {
                    deduplicate = false;
                    break;
                }
            }

            for language_index in 0..k_language_count {
                let string = &self.strings[language_index][i];
                match string {
                    None => {
                        bitstream.write_bool(false)?;
                        continue;
                    }
                    Some(string) => {
                        bitstream.write_bool(true)?;
                        bitstream.write_integer(offset, offset_bit_length)?;

                        if !deduplicate {
                            string_writer.write_string_utf8(string, max_string_length as u32)?;
                            offset += string.len() as u16 + 1;
                        }
                    }
                }
            }

            if deduplicate {
                let string = string.unwrap();
                string_writer.write_string_utf8(&string, max_string_length as u32)?;
                offset += string.len() as u16 + 1;
            }
        }

        string_writer.finish_writing();
        let buffer = string_writer.get_data()?;
        write_string_buffer_blob(bitstream, &buffer, buffer_size_bit_length)?;

        Ok(())
    }
}
