use std::io::{Cursor, Seek, SeekFrom};
use binrw::{BinRead, BinWrite, NullString};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Error as _;
use serde::ser::Error as _;
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
        if compressed_buffer.len() <= buffer.len() {
            bitstream.write_bool(true)?;
            bitstream.write_integer(compressed_buffer.len() as u32, buffer_size_bit_length)?;
            bitstream.write_raw_data(compressed_buffer.as_slice(), compressed_buffer.len() * 8)?;
            return Ok(());
        }
    }

    bitstream.write_bool(false)?;
    bitstream.write_raw_data(buffer, buffer.len() * 8)?;
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


/// Reach's localized string table, held as the game holds it: each language's byte offset into
/// one shared buffer of NUL-terminated UTF-8 strings. Files keep whatever packing they were
/// written with, so tables round-trip byte for byte. `strings()` / `set_strings()` read and
/// repack them; JSON uses the strings.
#[derive(Default, Clone, Debug, PartialEq)]
pub struct c_string_table<
    const max_string_count: usize,
    const max_string_length: usize,
    // these dont exist in this manner in reach but are configured some other way
    const offset_bit_length: usize,
    const buffer_size_bit_length: usize,
    const count_bit_length: usize,
> {
    /// Per language, each string's byte offset into `m_buffer`, or -1 when it has none.
    pub m_offsets: StaticArray<Vec<i32>, k_language_count>,
    pub m_buffer: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct c_string_table_json {
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
    pub fn string_count(&self) -> usize {
        self.m_offsets[0].len()
    }

    pub fn get_string(&self, language: usize, index: usize) -> BLFLibResult<Option<String>> {
        match self.m_offsets[language].get(index) {
            Some(&offset) if offset >= 0 => Ok(Some(read_null_terminated_utf8(&self.m_buffer, offset as u64)?)),
            _ => Ok(None),
        }
    }

    /// Sets one string, reusing an identical string already in the buffer. Strings this replaces
    /// stay in the buffer; `set_strings` repacks in MegaloEdit's layout.
    pub fn set_string(&mut self, language: usize, index: usize, value: Option<&str>) -> BLFLibResult {
        assert_ok!(index < max_string_count);
        for language_index in 0..k_language_count {
            if self.m_offsets[language_index].len() <= index {
                self.m_offsets[language_index].resize(index + 1, -1);
            }
        }
        let offset = match value {
            None => -1,
            Some(value) => self.store(value)?,
        };
        self.m_offsets[language][index] = offset;
        Ok(())
    }

    pub fn strings(&self) -> BLFLibResult<StaticArray<Vec<Option<String>>, k_language_count>> {
        let mut strings = StaticArray::<Vec<Option<String>>, k_language_count>::default();
        for language in 0..k_language_count {
            for index in 0..self.m_offsets[language].len() {
                strings[language].push(self.get_string(language, index)?);
            }
        }
        Ok(strings)
    }

    /// Repacks the buffer in MegaloEdit's layout: strings in index then language order, a string
    /// that's the same in every language stored once, any other string stored once per language.
    pub fn set_strings(&mut self, strings: &StaticArray<Vec<Option<String>>, k_language_count>) -> BLFLibResult {
        let count = strings[0].len();
        assert_ok!(count <= max_string_count);
        self.m_offsets = StaticArray::default();
        self.m_buffer = Vec::new();
        for index in 0..count {
            let row: Vec<Option<&str>> = (0..k_language_count)
                .map(|language| strings[language].get(index).and_then(|string| string.as_deref()))
                .collect();
            if row[0].is_some() && row.iter().all(|string| *string == row[0]) {
                let offset = self.append(row[0].unwrap())?;
                for language in 0..k_language_count {
                    self.m_offsets[language].push(offset);
                }
                continue;
            }
            for (language, string) in row.into_iter().enumerate() {
                let offset = match string {
                    None => -1,
                    Some(string) => self.append(string)?,
                };
                self.m_offsets[language].push(offset);
            }
        }
        Ok(())
    }

    // The offset of `value` in the buffer, appending it unless it's already there.
    fn store(&mut self, value: &str) -> BLFLibResult<i32> {
        for language in 0..k_language_count {
            for &offset in &self.m_offsets[language] {
                if offset >= 0 && read_null_terminated_utf8(&self.m_buffer, offset as u64)? == value {
                    return Ok(offset);
                }
            }
        }
        self.append(value)
    }

    fn append(&mut self, value: &str) -> BLFLibResult<i32> {
        let offset = self.m_buffer.len();
        if offset + value.len() + 1 > max_string_length {
            return Err(format!(
                "string table buffer holds {max_string_length} bytes, can't add {} more to {offset}",
                value.len() + 1
            ).into());
        }
        self.m_buffer.extend_from_slice(value.as_bytes());
        self.m_buffer.push(0);
        Ok(offset as i32)
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_offsets = StaticArray::default();
        self.m_buffer = Vec::new();
        let string_count: usize = bitstream.read_integer("string-count", count_bit_length)?;

        if string_count == 0 {
            return Ok(());
        }

        for _ in 0..string_count {
            for language in 0..k_language_count {
                let offset = if bitstream.read_bool("exists")? {
                    bitstream.read_integer("index", offset_bit_length)?
                } else {
                    -1
                };
                self.m_offsets[language].push(offset);
            }
        }

        let buffer_size: usize = bitstream.read_integer("size", buffer_size_bit_length)?;

        self.m_buffer = if bitstream.read_bool("compressed")? {
            let compressed_length: usize = bitstream.read_integer("compressed-buffer-size", buffer_size_bit_length)?;
            let compressed_data = bitstream.read_raw_data(compressed_length * 8)?;
            let mut decompressed_data = Vec::with_capacity(buffer_size);

            runtime_data_decompress(&compressed_data, &mut decompressed_data, bitstream.get_byte_order())?;
            decompressed_data
        } else {
            bitstream.read_raw_data(buffer_size * 8)?
        };

        Ok(())
    }

    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        let string_count = self.string_count();
        assert_ok!(string_count <= max_string_count);
        bitstream.write_integer(string_count as u32, count_bit_length)?;

        if string_count == 0 {
            return Ok(());
        }

        for index in 0..string_count {
            for language in 0..k_language_count {
                let offset = self.m_offsets[language].get(index).copied().unwrap_or(-1);
                bitstream.write_bool(offset >= 0)?;
                if offset >= 0 {
                    bitstream.write_integer(offset as u32, offset_bit_length)?;
                }
            }
        }

        write_string_buffer_blob(bitstream, &self.m_buffer, buffer_size_bit_length)?;

        Ok(())
    }
}

impl<
    const max_string_count: usize,
    const max_string_length: usize,
    const offset_bit_length: usize,
    const buffer_size_bit_length: usize,
    const count_bit_length: usize,
> Serialize for c_string_table<
    max_string_count,
    max_string_length,
    offset_bit_length,
    buffer_size_bit_length,
    count_bit_length,
>
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let strings = self.strings().map_err(S::Error::custom)?;
        c_string_table_json { strings }.serialize(serializer)
    }
}

impl<
    'de,
    const max_string_count: usize,
    const max_string_length: usize,
    const offset_bit_length: usize,
    const buffer_size_bit_length: usize,
    const count_bit_length: usize,
> Deserialize<'de> for c_string_table<
    max_string_count,
    max_string_length,
    offset_bit_length,
    buffer_size_bit_length,
    count_bit_length,
>
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let json = c_string_table_json::deserialize(deserializer)?;
        let mut table = Self::default();
        table.set_strings(&json.strings).map_err(D::Error::custom)?;
        Ok(table)
    }
}
