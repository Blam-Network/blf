
use std::cmp::min;
use std::fmt::{Debug, Display};
use std::io::Cursor;
use binrw::BinRead;
use num_traits::FromPrimitive;
use widestring::U16CString;
use blf_lib::blam::common::math::real_math::{assert_valid_real_normal3d, cross_product3d, dot_product3d, k_real_epsilon, global_forward3d, global_left3d, global_up3d, normalize3d, valid_real_vector3d_axes3, arctangent, k_pi, rotate_vector_about_axis, valid_real_vector3d_axes2};
use blf_lib::{assert_ok, OPTION_TO_RESULT};
use blf_lib::io::bitstream::{e_bitstream_byte_fill_direction};
use blf_lib_derivable::result::{BLFLibError, BLFLibResult};
use crate::blam::common::math::integer_math::int32_point3d;
use crate::blam::common::math::real_math::real_vector3d;
use crate::blam::halo3::release::networking::transport::transport_security::s_transport_secure_address;
use crate::io::bitstream::{e_bitstream_byte_order, e_bitstream_state};
use crate::io::bitstream::e_bitstream_byte_fill_direction::{_bitstream_byte_fill_direction_msb_to_lsb, _bitstream_byte_fill_direction_lsb_to_msb};
use crate::types::numbers::Float32;

pub struct c_bitstream_reader<'a>
{
    m_data: &'a [u8],
    m_data_size_bytes: usize,
    m_state: e_bitstream_state,
    m_byte_order: e_bitstream_byte_order,
    /// Some old versions of halo will pack BE values in LE and visa versa.
    /// We keep "packed" byte order which is swapped when using legacy settings.
    /// This allows BE consumers (ie xenon builds) to pass BE and legacy config will swap if necessary.
    m_packed_byte_order: e_bitstream_byte_order,
    /// This is the direction in which bytes within the bitstream are filled.
    /// For example, when writing 5 bits 11111 then 3 bits 000 to a bitstream,
    /// this could be expressed as:
    /// 11111000 (most significant bit to least significant bit)
    /// 00011111 (least significant bit to most significant bit)
    /// In most cases, MSB to LSB is used, bit LSB to MSB is notably used in Halo 3 Beta and prior.
    m_byte_pack_direction: e_bitstream_byte_fill_direction,
    /// This is the direction in which bytes within the bitstream are put back together.
    /// Eg, given a buffer of 2 bytes, and an offset of bit 3
    /// when reading 8 bits, we will read 5 11111 from byte 1, 3 000 from byte 2.
    /// We can then return this as a byte
    /// When using MSB to LSB, the 5 bits read first would go first
    /// 11111000
    /// When using LSB to MSB, the 5 bits read first go last
    /// 00011111
    m_byte_unpack_direction: e_bitstream_byte_fill_direction,

    current_stream_bit_position: usize,
    current_stream_byte_position: usize,
}

impl<'a> c_bitstream_reader<'a> {
    pub fn new(data: &[u8], byte_order: e_bitstream_byte_order) -> c_bitstream_reader {
        let length = data.len();
        c_bitstream_reader {
            m_data: data,
            m_data_size_bytes: length,
            m_state: e_bitstream_state::_bitstream_state_initial,
            m_byte_order: byte_order,
            m_packed_byte_order: byte_order,
            m_byte_pack_direction: e_bitstream_byte_fill_direction::default(),
            m_byte_unpack_direction: e_bitstream_byte_fill_direction::default(),

            current_stream_bit_position: 0,
            current_stream_byte_position: 0,
        }
    }

    /// Use this when dealing with bitstream data from the Halo 3 Beta or prior.
    pub fn new_with_legacy_settings(data: &[u8], byte_order: e_bitstream_byte_order) -> c_bitstream_reader {
        let length = data.len();
        c_bitstream_reader {
            m_data: data,
            m_data_size_bytes: length,
            m_state: e_bitstream_state::_bitstream_state_initial,

            m_byte_order: byte_order,
            m_packed_byte_order: byte_order.swap(),
            m_byte_pack_direction: _bitstream_byte_fill_direction_lsb_to_msb,
            m_byte_unpack_direction: _bitstream_byte_fill_direction_lsb_to_msb,

            current_stream_byte_position: 0,
            current_stream_bit_position: 0,
        }
    }

    pub fn get_byte_order(&self) -> e_bitstream_byte_order {
        self.m_packed_byte_order
    }

    pub fn seek_relative(&mut self, bits: usize) -> BLFLibResult {
        let current_bit_position = (self.current_stream_byte_position * 8) + self.current_stream_bit_position;
        let new_bit_position = current_bit_position + bits;
        assert_ok!(new_bit_position / 8 < self.m_data_size_bytes);

        self.current_stream_bit_position = new_bit_position % 8;
        self.current_stream_byte_position = new_bit_position / 8;

        Ok(())
    }

    pub fn seek(&mut self, byte: usize) -> BLFLibResult {
        assert_ok!(byte < self.m_data_size_bytes);

        self.current_stream_byte_position = byte;
        self.current_stream_bit_position = 0;

        Ok(())
    }

    pub fn seek_bit(&mut self, byte: usize, bit: usize) -> BLFLibResult {
        assert_ok!(bit < 8);

        self.seek(byte)?;
        self.current_stream_bit_position = bit;

        Ok(())
    }

    pub fn get_current_offset(&self) -> (usize, usize) {
        (self.current_stream_byte_position, self.current_stream_bit_position)
    }

    // READS

    pub fn read_raw_data(&mut self, size_in_bits: usize) -> BLFLibResult<Vec<u8>> {
        let mut buffer = vec![0u8; (size_in_bits as f32 / 8f32).ceil() as usize];
        self.read_bits_internal(buffer.as_mut_slice(), size_in_bits)?;
        Ok(buffer)
    }

    pub fn read_raw<T: BinRead>(&mut self, size_in_bits: usize) -> BLFLibResult<T> where for<'b> <T as BinRead>::Args<'b>: Default {
        let data = self.read_raw_data(size_in_bits)?;
        let mut reader = Cursor::new(data);

        Ok(match self.m_packed_byte_order {
            e_bitstream_byte_order::_bitstream_byte_order_little_endian => {
                T::read_le(&mut reader)?
            }
            e_bitstream_byte_order::_bitstream_byte_order_big_endian => {
                T::read_be(&mut reader)?
            }
        })
    }

    #[deprecated]
    pub fn read_unnamed_bool<T>(&mut self) -> BLFLibResult<T>
        where
            T: From<bool>,
    {
        self.read_bool("???")
    }

    pub fn read_bool<T>(&mut self, name: &str) -> BLFLibResult<T>
    where
        T: From<bool>,
    {
        Ok(T::from(self.read_integer::<u8>(name, 1)? == 1))
    }

    pub fn read_bits_internal(&mut self, output: &mut [u8], size_in_bits: usize) -> BLFLibResult {
        let end_memory_position = output.len();
        let end_stream_position = self.m_data.len();
        let remaining_stream_bytes = end_stream_position - (self.current_stream_byte_position + 1);
        let remaining_stream_bits = (8 - self.current_stream_bit_position) + (remaining_stream_bytes * 8);

        // println!("memory:bitstream:bitstream_reader read_bits_internal: reading {size_in_bits} bits");

        let size_in_bytes = size_in_bits.div_ceil(8);
        if end_memory_position < size_in_bytes {
            return Err(format!("Tried to read {size_in_bytes} bytes ({size_in_bits} bits) into a {end_memory_position} byte buffer!").into())
        }

        if remaining_stream_bits < size_in_bits {
            return Err(format!("Tried to read {size_in_bits} bits but the stream only has {remaining_stream_bits} bits left!").into())
        }

        if size_in_bits == 0 {
            return Err("Tried to read zero bits.".into())
        }

        let mut remaining_bits_to_read = size_in_bits;

        let mut output_byte_index = 0;
        while remaining_bits_to_read > 0 {
            // println!("memory:bitstream:bitstream_reader reading byte {output_byte_index}");

            let mut output_byte = 0u8;
            let mut bits_read = 0;

            // 1. Read any remaining bits on the current byte.
            if self.current_stream_bit_position != 0 {
                let remaining_bits_at_position = 8 - self.current_stream_bit_position;
                let reading_bits_at_position = min(remaining_bits_at_position, remaining_bits_to_read);

                // 1.1 Grab the next 8 bits.
                let mut bits = self.m_data[self.current_stream_byte_position];

                // 1.2. We shift the bits we've read to the MSB and discard any previously read.
                match self.m_byte_pack_direction {
                    _bitstream_byte_fill_direction_msb_to_lsb => {
                        bits <<= self.current_stream_bit_position;
                    }
                    _bitstream_byte_fill_direction_lsb_to_msb => {
                        bits <<= remaining_bits_at_position - reading_bits_at_position;
                    }
                }

                // 1.3. Mask off any excess
                bits &= 0xff << (8 - reading_bits_at_position);

                // println!("memory:bitstream:bitstream_reader 1. Read {:0width$b}", bits >> 8 - reading_bits_at_position, width=reading_bits_at_position);

                // 1.4 If we're unpacking LSB to MSB, we need to shift what we've read.
                if self.m_byte_unpack_direction == _bitstream_byte_fill_direction_lsb_to_msb {
                    bits >>= 8 - reading_bits_at_position;
                }

                // push the byte onto the window.
                output_byte = bits;
                bits_read += reading_bits_at_position;

                // If we read all the remaining bits at this byte, move to the next.
                if remaining_bits_at_position > remaining_bits_at_position {
                    panic!("bitstream reader believes it has read more bits than available. This should never happen.")
                } else if reading_bits_at_position == remaining_bits_at_position {
                    self.current_stream_bit_position = 0;
                    self.current_stream_byte_position += 1;
                } else {
                    self.current_stream_bit_position += reading_bits_at_position;
                }

                remaining_bits_to_read -= reading_bits_at_position;
            }

            // 2. Read more bits from the next byte.
            if remaining_bits_to_read > 0 {
                let reading_bits_at_position = min(8 - bits_read, remaining_bits_to_read);
                // 2.1 Grab the next 8 bits.
                let mut bits = self.m_data[self.current_stream_byte_position];

                // 2.2. If we're reading LSB to MSB, we push the bits we've read to the MSB
                //      If reading MSB to LSB, they're already there.
                if self.m_byte_pack_direction == _bitstream_byte_fill_direction_lsb_to_msb {
                    bits <<= 8 - reading_bits_at_position;
                }

                // 2.3. Mask off any excess
                bits &= 0xff << (8 - reading_bits_at_position);

                // println!("memory:bitstream:bitstream_reader 2. Read {:0width$b}", bits >> 8 - reading_bits_at_position, width=reading_bits_at_position);

                // 2.4 Shift what we've read according to unpacking order.
                match self.m_byte_unpack_direction {
                    _bitstream_byte_fill_direction_msb_to_lsb => {
                        bits >>= bits_read
                    }
                    _bitstream_byte_fill_direction_lsb_to_msb => {
                        bits >>= 8 - (bits_read + reading_bits_at_position)
                    }
                }

                // Stick it on the output.
                output_byte |= bits;
                bits_read += reading_bits_at_position;

                remaining_bits_to_read -= reading_bits_at_position;

                // If we read a full byte, move onto the next
                if reading_bits_at_position == 8 {
                    self.current_stream_bit_position = 0;
                    self.current_stream_byte_position += 1;
                } else {
                    self.current_stream_bit_position += reading_bits_at_position;
                }
            }

            // If we read less than a byte and we've unpacked LSB to MSB, we shift the bits to the MSB.
            if bits_read < 8 && self.m_byte_unpack_direction == _bitstream_byte_fill_direction_lsb_to_msb {
                output_byte <<= 8 - bits_read;
            }

            // println!("memory:bitstream:bitstream_reader read {bits_read} bits {:0width$b}", output_byte >> 8 - bits_read, width = bits_read);
            // 3. We now have n bits loaded into the byte, with any unused bits at the LSB.
            // Depending on byte order, we shift this to relocate surplus bits.
            match self.m_packed_byte_order {
                e_bitstream_byte_order::_bitstream_byte_order_big_endian => {
                    // 1111111 11111000 >> 00011111 11111111
                    let surplus_bits = (8 - (size_in_bits % 8)) % 8;
                    let [left_output, right_output] = ((output_byte as u16) << (8 - surplus_bits)).to_be_bytes();
                    output[output_byte_index] |= left_output;

                    if right_output != 0 {
                        output[output_byte_index + 1] |= right_output;
                    }
                    output_byte_index += 1;

                }
                e_bitstream_byte_order::_bitstream_byte_order_little_endian => {
                    // 1111111 11111000 >> 11111111 00011111
                    if bits_read < 8 {
                        output_byte >>= 8 - bits_read;
                    }

                    output[output_byte_index] = output_byte;
                    output_byte_index += 1;
                }
            }

            // println!("memory:bitstream:bitstream_reader byte {output_byte_index} is {output_byte:08b}");
        }

        Ok(())
    }

    pub fn read_unnamed_enum<T: FromPrimitive>(&mut self, size_in_bits: usize) -> BLFLibResult<T> {
        self.read_enum("???", size_in_bits)
    }

    pub fn read_enum<T: FromPrimitive>(&mut self, name: &str, size_in_bits: usize) -> BLFLibResult<T> {
        let integer: u32 = self.read_integer(name, size_in_bits)?;
        OPTION_TO_RESULT!(FromPrimitive::from_u32(integer), format!("Unexpected enum value: {}", integer).into())
    }

    pub fn read_integer<T>(&mut self, name: &str, size_in_bits: usize) -> BLFLibResult<T>
    where
        T: TryFrom<u32> + Display + Debug, <T as TryFrom<u32>>::Error: Display + Debug
    {
        assert_ok!(size_in_bits > 0);
        assert_ok!(size_in_bits <= 32);
        let size_in_bytes = (size_in_bits as f32 / 8f32 ).ceil() as usize;
        let mut bytes_vec = vec![0u8; size_in_bytes];
        self.read_bits_internal(&mut bytes_vec, size_in_bits)?;
        let bytes_slice = bytes_vec.as_slice();

        let mut byte_array = [0u8; 4];

        Ok(T::try_from(match self.m_packed_byte_order {
            e_bitstream_byte_order::_bitstream_byte_order_little_endian => {
                byte_array[0..bytes_slice.len()].copy_from_slice(bytes_slice);
                u32::from_le_bytes(byte_array)
            }
            e_bitstream_byte_order::_bitstream_byte_order_big_endian => {
                byte_array[4 - bytes_slice.len()..4].copy_from_slice(bytes_slice);
                // println!("read {} ({} bits) = {}", name, size_in_bits, u32::from_be_bytes(byte_array));
                u32::from_be_bytes(byte_array)
            }
        }).map_err(|e|BLFLibError::from(format!("\
            read_integer failed to convert u32 to type. size = {} data = {:?}",
            size_in_bits,
            byte_array,
        )))?)
    }

    #[deprecated]
    pub fn read_unnamed_integer<T>(&mut self, size_in_bits: usize) -> BLFLibResult<T>
        where
            T: TryFrom<u32> + Display + Debug, <T as TryFrom<u32>>::Error: Display + Debug
    {
        self.read_integer("???", size_in_bits)
    }

    #[deprecated]
    pub fn read_unnamed_index<const max: usize>(&mut self, size_in_bits: usize) -> BLFLibResult<i32> {
        if self.read_unnamed_bool()? {
            Ok(-1)
        } else {
            let value = self.read_unnamed_integer(size_in_bits)?;
            assert_ok!(value < max as i32);
            Ok(value)
        }
    }

    pub fn read_index<const max: usize>(&mut self, name: &str, size_in_bits: usize) -> BLFLibResult<i32> {
        if self.read_bool("index-exists")? {
            Ok(-1)
        } else {
            let value = self.read_integer(name, size_in_bits)?;
            assert_ok!(value < max as i32);
            Ok(value)
        }
    }

    pub fn read_unnamed_float(&mut self, size_in_bits: usize) -> BLFLibResult<Float32> {
        self.read_float("???", size_in_bits)
    }

    pub fn read_float<T>(&mut self, name: &str, size_in_bits: usize) -> BLFLibResult<T>
        where
            T: TryFrom<f32> + Display + Debug, <T as TryFrom<f32>>::Error: Display + Debug
    {
        assert_ok!(size_in_bits > 0);
        assert_ok!(size_in_bits <= 32);
        let mut bytes = [0u8; 4];
        self.read_bits_internal(&mut bytes, size_in_bits)?;

        Ok(T::try_from(match self.m_packed_byte_order {
            e_bitstream_byte_order::_bitstream_byte_order_little_endian => { f32::from_le_bytes(bytes) }
            e_bitstream_byte_order::_bitstream_byte_order_big_endian => { f32::from_be_bytes(bytes) }
        }).map_err(|e|BLFLibError::from(format!("\
            read_integer failed to convert u32 to type. size = {} data = {:?}",
            size_in_bits,
            bytes,
        )))?)
    }

    pub fn read_i16(&mut self, size_in_bits: usize) -> BLFLibResult<i16> {
        assert_ok!(size_in_bits > 0);
        assert_ok!(size_in_bits <= 16);
        let mut bytes = [0u8; 2];
        self.read_bits_internal(&mut bytes, size_in_bits)?;

        Ok(match self.m_packed_byte_order {
            e_bitstream_byte_order::_bitstream_byte_order_little_endian => { i16::from_le_bytes(bytes) }
            e_bitstream_byte_order::_bitstream_byte_order_big_endian => { i16::from_be_bytes(bytes) }
        })
    }

    #[deprecated]
    pub fn read_unnamed_signed_integer<T>(&mut self, size_in_bits: usize) -> BLFLibResult<T>
        where
            T: TryFrom<i32>,
    {
        self.read_signed_integer("???", size_in_bits)
    }

    pub fn read_signed_integer<T>(&mut self, name: &str, size_in_bits: usize) -> BLFLibResult<T>
    where
        T: TryFrom<i32>,
    {
        let mut result: u32 = self.read_integer(name, size_in_bits)?;

        if size_in_bits < 32 && (result & (1 << (size_in_bits - 1))) != 0 {
            result |= !((1 << size_in_bits) - 1);
        }

        Ok(
            T::try_from(result as i32)
                .map_err(|e|BLFLibError::from("read_signed_integer failed to convert i32 to type."))?
        )
    }

    pub fn read_qword<T>(&mut self, size_in_bits: usize) -> BLFLibResult<T>
        where
            T: TryFrom<u64>,
    {
        assert_ok!(size_in_bits > 0);
        assert_ok!(size_in_bits <= 64);
        let mut bytes = [0u8; 8];
        self.read_bits_internal(&mut bytes, size_in_bits)?;

        Ok(T::try_from(
            match self.m_packed_byte_order {
                e_bitstream_byte_order::_bitstream_byte_order_little_endian => {
                    u64::from_le_bytes(bytes)
                }
                e_bitstream_byte_order::_bitstream_byte_order_big_endian => {
                    u64::from_be_bytes(bytes)
                }
            }
        ).map_err(|e|BLFLibError::from("read_qword failed to convert u64 to type."))?)
    }

    pub fn read_identifier(identifier: String) { // param may be wrong.
        unimplemented!()
    }

    pub fn read_point3d(&mut self, point: &mut int32_point3d, axis_encoding_size_in_bits: usize) -> BLFLibResult {
        assert_ok!(0 < axis_encoding_size_in_bits && axis_encoding_size_in_bits <= 32);

        point.x = self.read_unnamed_integer(axis_encoding_size_in_bits)?;
        point.y = self.read_unnamed_integer(axis_encoding_size_in_bits)?;
        point.z = self.read_unnamed_integer(axis_encoding_size_in_bits)?;

        Ok(())
    }

    pub fn read_point3d_efficient(&mut self, point: &mut int32_point3d, axis_encoding_size_in_bits: int32_point3d) -> BLFLibResult {
        assert_ok!(0 < axis_encoding_size_in_bits.x && axis_encoding_size_in_bits.x <= 32);
        assert_ok!(0 < axis_encoding_size_in_bits.y && axis_encoding_size_in_bits.y <= 32);
        assert_ok!(0 < axis_encoding_size_in_bits.z && axis_encoding_size_in_bits.z <= 32);

        point.x = self.read_unnamed_integer(axis_encoding_size_in_bits.x as usize)?;
        point.y = self.read_unnamed_integer(axis_encoding_size_in_bits.y as usize)?;
        point.z = self.read_unnamed_integer(axis_encoding_size_in_bits.z as usize)?;

        Ok(())
    }

    pub fn read_qword_internal(size_in_bits: u8) -> u64 {
        unimplemented!()
    }

    pub fn read_secure_address(address: &mut s_transport_secure_address) {
        unimplemented!()
    }

    pub fn angle_to_axes_internal(up: &real_vector3d, angle: impl Into<f32>, forward: &mut real_vector3d) -> BLFLibResult {
        let angle = angle.into();

        let mut forward_reference = real_vector3d::default();
        let mut left_reference = real_vector3d::default();

        c_bitstream_reader::axes_compute_reference_internal(up, &mut forward_reference, &mut left_reference)?;

        forward.i = forward_reference.i;
        forward.j = forward_reference.j;
        forward.k = forward_reference.k;

        let u: f32;
        let v: f32;

        if angle == k_pi || angle == -k_pi {
            u = 0.0;
            v = -1.0;
        }
        else {
            u = angle.sin();
            v = angle.cos();
        }

        rotate_vector_about_axis(forward, up, u, v);
        normalize3d(forward);

        assert_ok!(valid_real_vector3d_axes2(forward, up));

        Ok(())
    }

    pub fn read_string(_string: &mut String, max_string_size: u8) {
        unimplemented!()
    }

    // differs from blam API
    pub fn read_string_utf8(&mut self, max_string_size: usize) -> BLFLibResult<String> {
        assert_ok!(self.reading());
        assert_ok!(max_string_size > 0);

        let mut bytes = vec![0u8; max_string_size];

        for i in 0..max_string_size {
            let byte = self.read_unnamed_integer(8)?;
            bytes[i] = byte;

            if byte == 0 {
                return Ok(String::from_utf8(bytes)?)
            }
        }

        Err("Exceeded max string size reading utf8 string.".into())
    }

    pub fn read_string_extended_ascii(&mut self, max_string_size: usize) -> BLFLibResult<String> {
        assert_ok!(self.reading());
        assert_ok!(max_string_size > 0);

        let mut bytes = vec![0u8; max_string_size];

        for i in 0..max_string_size {
            let byte = self.read_unnamed_integer(8)?;
            bytes[i] = byte;

            if byte == 0 {
                // manually parse the string because halo sometimes uses extended char sets.
                let s: String = bytes.iter().map(|&b| b as char).collect();
                return Ok(s);
            }
        }

        Err("Exceeded max string size reading utf8 string.".into())
    }

    // differs from blam API
    pub fn read_string_whar(&mut self, max_string_size: usize) -> BLFLibResult<String> {
        assert_ok!(self.reading());
        assert_ok!(max_string_size > 0);

        let mut characters = vec![0u16; max_string_size];

        for i in 0..max_string_size {
            let character = self.read_unnamed_integer(16)?;

            if character == 0 {
                return Ok(U16CString::from_vec(&mut characters[0..i]).map_err(|e|e.to_string())?.to_string().map_err(|e|e.to_string())?);
            } else {
                characters[i] = character;
            }
        }

        Err("Exceeded max string size reading wchar string.".into())
    }

    pub fn read_unit_vector(unit_vector: &mut real_vector3d, size_in_bits: u8) {
        unimplemented!()
    }

    pub fn read_vector(vector: &mut real_vector3d, min_value: f32, max_value: f32, step_count_size_in_bits: f32, size_in_bits: u8) {
        unimplemented!()
    }

    pub fn begin_reading(&mut self) {
        self.reset(e_bitstream_state::_bitstream_state_reading);
    }

    pub fn reading(&self) -> bool {
        self.m_state == e_bitstream_state::_bitstream_state_reading ||
        self.m_state == e_bitstream_state::_bitstream_state_read_only_for_consistency
    }

    pub fn finish_reading(&mut self) {
        self.m_state = e_bitstream_state::_bitstream_state_read_finished;
    }

    pub fn get_data(&self, data_length: &mut usize) -> BLFLibResult<&[u8]> {
        *data_length = self.m_data_size_bytes;
        Ok(self.m_data)
    }

    fn reset(&mut self, state: e_bitstream_state) {
        self.m_state = state;
        self.current_stream_bit_position = 0;
        self.current_stream_byte_position = 0;
    }

    fn set_data(&mut self, data: &'a [u8]) {
        let length = data.len();
        self.m_data = data;
        self.m_data_size_bytes = length;
        self.reset(e_bitstream_state::_bitstream_state_initial);
    }

    fn skip(bits_to_skip: u32) {
        unimplemented!()
    }

    fn would_overflow(size_in_bits: u32) -> bool {
        unimplemented!()
    }

    pub fn axes_compute_reference_internal(
        up: &real_vector3d,
        forward_reference: &mut real_vector3d,
        left_reference: &mut real_vector3d
    ) -> BLFLibResult {
        assert_ok!(assert_valid_real_normal3d(up));

        let v10 = dot_product3d(up, &global_forward3d).abs();
        let v9 = dot_product3d(up, &global_left3d).abs();

        if v10 >= v9 {
            cross_product3d(&global_left3d, up, forward_reference);
        } else {
            cross_product3d(up, &global_forward3d, forward_reference);
        }

        let forward_magnitude = normalize3d(forward_reference);
        assert_ok!(forward_magnitude > k_real_epsilon, "forward_magnitude>k_real_epsilon");

        cross_product3d(up, forward_reference, left_reference);

        let left_magnitude = normalize3d(left_reference);
        assert_ok!(left_magnitude > k_real_epsilon, "left_magnitude>k_real_epsilon");

        assert_ok!(valid_real_vector3d_axes3(forward_reference, left_reference, up)); // Failing

        Ok(())
    }

    fn axes_to_angle_internal(forward: &real_vector3d, up: &real_vector3d) -> BLFLibResult<f32> {
        let mut forward_reference: real_vector3d = real_vector3d::default();
        let mut left_reference: real_vector3d = real_vector3d::default();
        c_bitstream_reader::axes_compute_reference_internal(up, &mut forward_reference, &mut left_reference)?;
        Ok(arctangent(dot_product3d(&left_reference, forward), dot_product3d(&forward_reference, forward)))
    }
}

#[cfg(test)]
mod bitstream_reader_tests {
    use super::*;

    #[test]
    fn read_be() {
        let test_data: [u8; 2] = [
            0b001_11111, 0b11111111
        ];

        let mut sut = c_bitstream_reader::new(&test_data, e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        sut.begin_reading();

        assert_eq!(sut.read_unnamed_integer::<u32>(3).unwrap(), 0b001);
        assert_eq!(sut.read_unnamed_integer::<u32>(13).unwrap(), 8191);
    }

    #[test]
    fn read_le() {
        let test_data: [u8; 2] = [
            0b001_11111, 0b11111111
        ];

        let mut sut = c_bitstream_reader::new(&test_data, e_bitstream_byte_order::_bitstream_byte_order_little_endian);
        sut.begin_reading();

        assert_eq!(sut.read_unnamed_integer::<u32>(3).unwrap(), 0b001);
        assert_eq!(sut.read_unnamed_integer::<u32>(13).unwrap(), 8191);
    }

    #[test]
    fn read_legacy_be() {
        let test_data: [u8; 2] = [
            0b10110_001, 0b00001001
        ];

        let mut sut = c_bitstream_reader::new_with_legacy_settings(&test_data, e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        sut.begin_reading();

        assert_eq!(sut.read_unnamed_integer::<u32>(3).unwrap(), 0b001);
        assert_eq!(sut.read_unnamed_integer::<u32>(13).unwrap(), 310);
    }

    #[test]
    fn read_legacy_le() {
        let test_data: [u8; 2] = [
            0b01001_001, 0b10110000
        ];

        let mut sut = c_bitstream_reader::new_with_legacy_settings(&test_data, e_bitstream_byte_order::_bitstream_byte_order_little_endian);
        sut.begin_reading();

        assert_eq!(sut.read_unnamed_integer::<u32>(3).unwrap(), 0b001);
        assert_eq!(sut.read_unnamed_integer::<u32>(13).unwrap(), 310);
    }


    #[test]
    fn read_with_msb_to_lsb_byte_pack_direction() {
        let test_data: [u8; 1] = [
            0b00011111
        ];

        let mut sut = c_bitstream_reader::new(&test_data, e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        sut.begin_reading();

        assert_eq!(sut.read_unnamed_integer::<u32>(3).unwrap(), 0b000);
        assert_eq!(sut.read_unnamed_integer::<u32>(5).unwrap(), 0b11111);
    }

    #[test]
    fn read_with_lsb_to_msb_byte_pack_direction() {
        let test_data: [u8; 1] = [
            0b10010100
        ];

        let mut sut = c_bitstream_reader::new_with_legacy_settings(&test_data, e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        sut.begin_reading();

        assert_eq!(sut.read_unnamed_integer::<u32>(6).unwrap(), 20);
        assert_eq!(sut.read_unnamed_integer::<u32>(2).unwrap(), 0b10);
    }

    #[test]
    fn read_h3_06481_game_set_data() {
        let test_data: [u8; 13] = [
            0b10010100, 0b00000001, 0b00000000, 0b00000000,
            0b00000000, 0b10110001, 0b00001001, 0b00000000,
            0b00000000, 0b10010000, 0b10101011, 0b00000_011, 0
        ];

        let mut sut = c_bitstream_reader::new_with_legacy_settings(&test_data, e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        sut.begin_reading();
        // game entries count
        assert_eq!(sut.read_unnamed_integer::<u32>(6).unwrap(), 20);
        // game entry 1 weight
        assert_eq!(sut.read_unnamed_integer::<u32>(32).unwrap(), 6);
        // game entry 1 minimum players
        assert_eq!(sut.read_unnamed_integer::<u32>(4).unwrap(), 4);
        // game entry 1 skip after veto
        assert_eq!(sut.read_unnamed_bool::<bool>().unwrap(), false);
        // game entry 1 map id
        assert_eq!(sut.read_unnamed_integer::<u32>(32).unwrap(), 310);
        // game entry 1 game variant (truncated)
        assert_eq!(sut.read_string_utf8(3).unwrap(), "ru\0");
    }

    #[test]
    fn read_h3_12070_game_set_data() {
        let test_data: [u8; 13] = [
            0b11011100, 0b00000000, 0b00000000, 0b00000000,
            0b00000100, 0b01000000, 0b00000000, 0b00000000,
            0b00100000, 0b10000011, 0b01010101, 0b1111_0000, 0
        ];

        let mut sut = c_bitstream_reader::new(&test_data, e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        sut.begin_reading();
        // game entries count
        assert_eq!(sut.read_unnamed_integer::<u32>(6).unwrap(), 55);
        // game entry 1 weight
        assert_eq!(sut.read_unnamed_integer::<u32>(32).unwrap(), 1);
        // game entry 1 minimum players
        assert_eq!(sut.read_unnamed_integer::<u32>(4).unwrap(), 1);
        // game entry 1 skip after veto
        assert_eq!(sut.read_unnamed_bool::<bool>().unwrap(), false);
        // game entry 1 optional
        assert_eq!(sut.read_unnamed_bool::<bool>().unwrap(), false);
        // game entry 1 map id
        assert_eq!(sut.read_unnamed_integer::<u32>(32).unwrap(), 520);
        // game entry 1 game variant (truncated)
        assert_eq!(sut.read_string_utf8(3).unwrap(), "5_\0");
    }
}