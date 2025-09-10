
use std::cmp::min;
use std::error::Error;
use std::io::Cursor;
use binrw::BinWrite;
use num_traits::{FromPrimitive, ToPrimitive};
use widestring::U16CString;
use blf_lib::{assert_ok, OPTION_TO_RESULT};
use blf_lib::blam::common::math::real_math::{assert_valid_real_normal3d, cross_product3d, dot_product3d, k_real_epsilon, global_forward3d, global_left3d, global_up3d, normalize3d, valid_real_vector3d_axes3, arctangent, quantize_normalized_vector3d, k_pi};
use blf_lib::io::bitstream::e_bitstream_byte_fill_direction;
use blf_lib::io::bitstream::e_bitstream_byte_fill_direction::{_bitstream_byte_fill_direction_lsb_to_msb, _bitstream_byte_fill_direction_msb_to_lsb};
use blf_lib_derivable::result::BLFLibResult;
use crate::blam::common::math::integer_math::int32_point3d;
use crate::blam::common::math::real_math::{quantize_real, real_vector3d};
use crate::blam::halo3::release::networking::transport::transport_security::s_transport_secure_address;
use crate::io::bitstream::{e_bitstream_byte_order, e_bitstream_state};

pub struct c_bitstream_writer
{
    m_data: Vec<u8>,
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

impl c_bitstream_writer {
    pub fn new(size: usize, byte_order: e_bitstream_byte_order) -> c_bitstream_writer {
        c_bitstream_writer {
            m_data: vec![0u8; size],
            m_data_size_bytes: size,
            m_state: e_bitstream_state::_bitstream_state_initial,

            m_byte_order: byte_order,
            m_packed_byte_order: byte_order,
            m_byte_pack_direction: e_bitstream_byte_fill_direction::default(),
            m_byte_unpack_direction: e_bitstream_byte_fill_direction::default(),

            current_stream_bit_position: 0,
            current_stream_byte_position: 0,
        }
    }

    pub fn new_with_legacy_settings(size: usize, byte_order: e_bitstream_byte_order) -> c_bitstream_writer {
        c_bitstream_writer {
            m_data: vec![0u8; size],
            m_data_size_bytes: size,
            m_state: e_bitstream_state::_bitstream_state_initial,

            m_byte_order: byte_order,
            m_packed_byte_order: byte_order.swap(),
            m_byte_pack_direction: _bitstream_byte_fill_direction_lsb_to_msb,
            m_byte_unpack_direction: _bitstream_byte_fill_direction_lsb_to_msb,

            current_stream_bit_position: 0,
            current_stream_byte_position: 0,
        }
    }

    pub fn get_byte_order(&self) -> e_bitstream_byte_order {
        self.m_byte_order
    }

    pub fn get_current_offset(&self) -> (usize, usize) {
        (self.current_stream_byte_position, self.current_stream_bit_position)
    }


    // WRITES

    pub fn write_enum<T: ToPrimitive + std::fmt::Debug>(&mut self, value: T, size_in_bits: usize) -> BLFLibResult {
        let value = OPTION_TO_RESULT!(value.to_u32(), format!("Failed to convert value {value:?} to an integer."))?;
        self.write_integer(value, size_in_bits)
    }

    pub fn write_integer(&mut self, value: impl Into<u32>, size_in_bits: usize) -> BLFLibResult {
        let value = value.into();

        match self.m_packed_byte_order {
            e_bitstream_byte_order::_bitstream_byte_order_little_endian => {
                self.write_bits_internal(&value.to_le_bytes(), size_in_bits)?;
            }
            e_bitstream_byte_order::_bitstream_byte_order_big_endian => {
                self.write_bits_internal(&value.to_be_bytes(), size_in_bits)?;
            }
        }

        Ok(())
    }

    pub fn write_signed_integer(&mut self, value: impl Into<i32>, size_in_bits: usize) -> BLFLibResult {
        let value = value.into();
        let max_value = ((1u32 << (size_in_bits - 1)) - 1) as i32; // Maximum positive value

        assert_ok!(self.writing(), "writing()");
        assert_ok!(size_in_bits <= 32, "size_in_bits>0 && size_in_bits<=LONG_BITS");
        assert_ok!(value > !max_value, "value>=minimum");
        assert_ok!(value < max_value, "value<=maximum");
        self.write_integer(value as u32, size_in_bits)?;

        Ok(())
    }

    pub fn write_bool<B: Sized + Into<bool>>(&mut self, value: B) -> BLFLibResult {
        self.write_integer(if value.into() { 1u8 } else { 0u8 }, 1)?;
        Ok(())
    }

    pub fn seek_relative(&mut self, bits: usize) -> BLFLibResult {
        self.write_raw_data(&*vec![0u8; (bits as f32 / 8f32).ceil() as usize], bits)
    }

    // Be careful using this.
    pub fn write_float(&mut self, value: impl Into<f32>, size_in_bits: usize) -> BLFLibResult {
        match self.m_packed_byte_order {
            e_bitstream_byte_order::_bitstream_byte_order_little_endian => {
                self.write_bits_internal(&value.into().to_le_bytes(), size_in_bits)?;
            }
            e_bitstream_byte_order::_bitstream_byte_order_big_endian => {
                self.write_bits_internal(&value.into().to_be_bytes(), size_in_bits)?;
            }
        }

        Ok(())
    }

    pub fn write_raw_data(&mut self, value: &[u8], size_in_bits: usize) -> BLFLibResult {
        assert_ok!(value.len() >= size_in_bits / 8);
        self.write_bits_internal(value, size_in_bits)?;

        Ok(())
    }

    pub fn write_raw<T: BinWrite>(&mut self, value: T, size_in_bits: usize) -> BLFLibResult where for<'a> <T as BinWrite>::Args<'a>: Default {
        let mut writer = Cursor::new(Vec::new());

        match self.m_packed_byte_order {
            e_bitstream_byte_order::_bitstream_byte_order_little_endian => {
                T::write_le(&value, &mut writer)?;
            }
            e_bitstream_byte_order::_bitstream_byte_order_big_endian => {
                T::write_be(&value, &mut writer)?;
            }
        }

        let value = writer.get_ref().clone();
        let value = value.as_slice();
        assert_ok!(value.len() >= size_in_bits / 8);
        self.write_bits_internal(value, size_in_bits)?;

        Ok(())
    }

    pub fn write_qword(&mut self, value: impl Into<u64>, size_in_bits: usize) -> BLFLibResult {
        match self.m_packed_byte_order {
            e_bitstream_byte_order::_bitstream_byte_order_little_endian => {
                self.write_bits_internal(&value.into().to_le_bytes(), size_in_bits)?;
            }
            e_bitstream_byte_order::_bitstream_byte_order_big_endian => {
                self.write_bits_internal(&value.into().to_be_bytes(), size_in_bits)?;
            }
        }

        Ok(())
    }

    fn write_value_internal(&mut self, data: &[u8], size_in_bits: usize) -> BLFLibResult {
        self.write_bits_internal(data, size_in_bits)?;
        Ok(())
    }

    fn write_bits_internal(&mut self, data: &[u8], size_in_bits: usize) -> Result<(), Box<dyn Error>> {
        if data.len() < (size_in_bits as f32 / 8f32).ceil() as usize {
            return Err(format!("Tried to write {size_in_bits} bits but only {} were provided!", (data.len() * 8)).into())
        }

        // println!("memory:bitstream:bitstream_writer write_bits_internal: writing {size_in_bits}");

        let surplus_bytes = data.len() - (size_in_bits as f32 / 8f32).ceil() as usize;
        // This isn't the total surplus bit count but instead, bits in addition to the surplus bytes.
        let surplus_bits = ((data.len() * 8) - size_in_bits) % 8;

        let mut remaining_bits_to_write = size_in_bits;

        while remaining_bits_to_write > 0 {
            let bytes_written = (size_in_bits - remaining_bits_to_write) / 8;

            // Bits of the current byte written.
            let mut bits_written = 0;

            // 1. Get the next bits to write. We shift bits to the MSB.
            let mut writing_byte = match self.m_packed_byte_order {
                e_bitstream_byte_order::_bitstream_byte_order_little_endian => {
                    // if we were writing -1 in 13 bits, we would have 3 surplus bits.
                    // in LE these surplus bits are at the last byte
                    // 11111111 00011111
                    // If we have less than a byte remaining to write, we're at the end, we can shift left.
                    if remaining_bits_to_write < 8 {
                        data[bytes_written] << (8 - remaining_bits_to_write)
                    } else {
                        data[bytes_written]
                    }
                }
                e_bitstream_byte_order::_bitstream_byte_order_big_endian => {
                    let mut bits = data[bytes_written + surplus_bytes] << surplus_bits;

                    // if after shifting, we no longer have enough bits to write...
                    // grab more bits from the next source byte.
                    if surplus_bits != 0 && remaining_bits_to_write > 8 - surplus_bits {
                        bits |= data[bytes_written + surplus_bytes + 1] >> 8 - surplus_bits;
                    }

                    bits
                }
            };

            // 2. If we're writing less than a byte, mask off any excess at the LSB.
            writing_byte &= 0xff << (8 - min(8, remaining_bits_to_write));

            // 3. Write remaining bits at the current byte.
            let writing_bits_at_position = min(8 - self.current_stream_bit_position, remaining_bits_to_write);
            let remaining_bits_at_position = 8 - self.current_stream_bit_position;

            // The bits we're writing at the current byte are first shifted to MSB / masked if necessary.
            let mut bits = match &self.m_byte_unpack_direction {
                _bitstream_byte_fill_direction_lsb_to_msb => {
                    writing_byte << min(8, remaining_bits_to_write) - writing_bits_at_position
                }
                _bitstream_byte_fill_direction_msb_to_lsb => {
                    writing_byte & (0xff << 8 - writing_bits_at_position)
                }
            };

            // Shift from MSB to current stream position
            match self.m_byte_pack_direction {
                _bitstream_byte_fill_direction_lsb_to_msb => {
                    bits >>= 8 - (self.current_stream_bit_position + writing_bits_at_position);
                }
                _bitstream_byte_fill_direction_msb_to_lsb => {
                    bits >>= self.current_stream_bit_position;
                }
            }

            self.m_data[self.current_stream_byte_position] |= bits;
            bits_written += writing_bits_at_position;

            if writing_bits_at_position == remaining_bits_at_position {
                self.current_stream_bit_position = 0;
                self.current_stream_byte_position += 1;
            } else {
                self.current_stream_bit_position += writing_bits_at_position;
            }

            remaining_bits_to_write -= writing_bits_at_position;

            // 4. Write any more bits to the next byte.
            if remaining_bits_to_write > 0 && bits_written < 8 {
                let writing_bits_at_position = min(remaining_bits_to_write, 8 - bits_written);

                // The bits we're writing at the current byte are first shifted to MSB / masked if necessary.
                let mut bits = match &self.m_byte_unpack_direction {
                    _bitstream_byte_fill_direction_lsb_to_msb => {
                        writing_byte & (0xff << 8 - writing_bits_at_position)
                    }
                    _bitstream_byte_fill_direction_msb_to_lsb => {
                        writing_byte << bits_written
                    }
                };

                if self.m_byte_pack_direction == _bitstream_byte_fill_direction_lsb_to_msb {
                    bits >>= 8 - writing_bits_at_position
                }

                self.m_data[self.current_stream_byte_position] |= bits;
                bits_written += writing_bits_at_position;

                self.current_stream_bit_position = writing_bits_at_position;

                remaining_bits_to_write -= writing_bits_at_position;
            }
        }

        Ok(())
    }

    pub fn write_identifier(identifier: String) {
        unimplemented!()
    }

    pub fn write_point3d(&mut self, point: &int32_point3d, axis_encoding_size_in_bits: usize) -> BLFLibResult {
        assert_ok!(axis_encoding_size_in_bits > 0 && axis_encoding_size_in_bits <= 32);

        assert_ok!(point.x < 1 << axis_encoding_size_in_bits);
        assert_ok!(point.y < 1 << axis_encoding_size_in_bits);
        assert_ok!(point.z < 1 << axis_encoding_size_in_bits);

        self.write_integer(point.x as u32, axis_encoding_size_in_bits)?;
        self.write_integer(point.y as u32, axis_encoding_size_in_bits)?;
        self.write_integer(point.z as u32, axis_encoding_size_in_bits)?;

        Ok(())
    }

    pub fn write_point3d_efficient(
        &mut self,
        point: &int32_point3d,
        axis_encoding_size_in_bits: &int32_point3d,
    ) -> BLFLibResult {
        assert_ok!(axis_encoding_size_in_bits.x > 0 && axis_encoding_size_in_bits.x <= 32);
        assert_ok!(axis_encoding_size_in_bits.y > 0 && axis_encoding_size_in_bits.y <= 32);
        assert_ok!(axis_encoding_size_in_bits.z > 0 && axis_encoding_size_in_bits.z <= 32);

        assert_ok!((point.x as u32) < (1u32 << axis_encoding_size_in_bits.x));
        assert_ok!((point.y as u32) < (1u32 << axis_encoding_size_in_bits.y));
        assert_ok!((point.z as u32) < (1u32 << axis_encoding_size_in_bits.z));

        self.write_integer(point.x as u32, axis_encoding_size_in_bits.x as usize)?;
        self.write_integer(point.y as u32, axis_encoding_size_in_bits.y as usize)?;
        self.write_integer(point.z as u32, axis_encoding_size_in_bits.z as usize)?;

        Ok(())
    }

    pub fn write_index<const max_value: usize>(&mut self, value: impl Into<i32>, bit_size: usize) -> BLFLibResult {
        let value = value.into();

        assert_ok!(value <= max_value as i32);

        if value == -1 {
            self.write_bool(true)?;
        } else {
            self.write_bool(false)?;
            self.write_integer(value as u32, bit_size)?;
        }

        Ok(())
    }

    pub fn write_quantized_real(
        &mut self,
        value: impl Into<f32>,
        min_value: f32,
        max_value: f32,
        size_in_bits: usize,
        exact_midpoint: bool,
        exact_endpoints: bool
    ) -> BLFLibResult {
        assert_ok!(self.writing());
        self.write_integer(
            quantize_real(
                value.into(),
                min_value,
                max_value,
                size_in_bits,
                exact_midpoint,
                exact_endpoints
            ) as u32,
            size_in_bits
        )
    }

    pub fn write_secure_address(address: &s_transport_secure_address) -> BLFLibResult {
        unimplemented!()
    }

    pub fn write_string(_string: &String, max_string_size: u32) -> BLFLibResult {
        unimplemented!()
    }

    pub fn write_string_utf8(&mut self, char_string: &String, max_string_size: u32) -> BLFLibResult {
        assert_ok!(self.writing());
        assert_ok!(max_string_size > 0);
        assert_ok!(char_string.len() <= max_string_size as usize);

        for byte in char_string.as_bytes() {
            self.write_value_internal(&[*byte], 8)?;
        }

        // null terminate
        self.write_value_internal(&0u8.to_ne_bytes(), 8)?;

        Ok(())
    }

    pub fn write_string_extended_ascii(&mut self, char_string: &String, max_string_size: u32) -> BLFLibResult {
        assert_ok!(self.writing());
        assert_ok!(max_string_size > 0);
        assert_ok!(char_string.len() <= max_string_size as usize);

        for ch in char_string.chars() {
            let code = ch as u32;
            if code > 0xFF {
                return Err(format!("Cannot encode non-Latin-1 char: {ch:?}").into());
            }
            let byte = code as u8;
            self.write_value_internal(&[byte], 8)?;
        }

        // null terminate
        self.write_value_internal(&0u8.to_ne_bytes(), 8)?;

        Ok(())
    }

    pub fn write_string_wchar(&mut self, value: &String, max_string_size: usize) -> BLFLibResult {
        assert_ok!(self.writing());
        assert_ok!(max_string_size > 0);

        let wchar_string = U16CString::from_str(value).map_err(|e|e.to_string())?;
        let characters = wchar_string.as_slice();

        assert_ok!(characters.len() <= max_string_size);

        for char in characters {
            match self.m_packed_byte_order {
                e_bitstream_byte_order::_bitstream_byte_order_little_endian => {
                    self.write_value_internal(&char.to_le_bytes(), 16)?;
                }
                e_bitstream_byte_order::_bitstream_byte_order_big_endian => {
                    self.write_value_internal(&char.to_be_bytes(), 16)?;
                }
            }
        }

        // null terminate
        self.write_value_internal(&0u16.to_ne_bytes(), 16)?;

        Ok(())
    }

    pub fn write_unit_vector(unit_vector: &real_vector3d, size_in_bits: u8) {
        unimplemented!()
    }

    pub fn write_vector(vector: &real_vector3d, min_value: f32, max_value: f32, step_count_size_in_bits: u32, size_in_bits: u8) { unimplemented!() }

    // GUTS

    pub fn append(stream: &c_bitstream_writer) {
        unimplemented!()
    }

    pub fn begin_consistency_check() -> bool {
        unimplemented!()
    }

    pub fn begin_writing(&mut self) {
        self.reset(e_bitstream_state::_bitstream_state_writing);
    }

    pub fn data_is_untrusted(is_untrusted: bool) {
        unimplemented!()
    }

    pub fn discard_remaining_data() {
        unimplemented!()
    }

    fn encode_qword_to_memory(value: u64, size_in_bits: u8) {
        unimplemented!()
    }

    pub fn overflowed() -> bool {
        unimplemented!()
    }

    pub fn error_occurred() -> bool {
        unimplemented!()
    }

    pub fn writing(&self) -> bool {
        self.m_state == e_bitstream_state::_bitstream_state_writing
    }



    pub fn finish_writing(&mut self) {
        self.m_state = e_bitstream_state::_bitstream_state_write_finished;
        self.m_data_size_bytes = (((self.current_stream_byte_position * 8) + self.current_stream_bit_position) as f32 / 8f32).ceil() as usize;
    }

    pub fn get_data(&self) -> BLFLibResult<Vec<u8>> {
        assert_ok!(!self.writing());

        Ok(self.m_data[0..self.m_data_size_bytes].to_vec())
    }

    fn reset(&mut self, state: e_bitstream_state) {
        self.m_state = state;
    }

    // fn set_data(&mut self, data: &'a mut [u8]) {
    //     let length = data.len();
    //     self.m_data = data;
    //     self.m_data_size_bytes = length;
    //     self.reset(e_bitstream_state::_bitstream_state_initial);
    // }

    fn skip(bits_to_skip: u32) {
        unimplemented!()
    }

    fn would_overflow(size_in_bits: u32) -> bool {
        unimplemented!()
    }

    // fn write_accumulator_to_memory(a1: u64, a2: u32) {
    //     unimplemented!()
    // }

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

        assert_ok!(valid_real_vector3d_axes3(forward_reference, left_reference, up));

        Ok(())
    }

    pub(crate) fn axes_to_angle_internal(forward: &real_vector3d, up: &real_vector3d) -> BLFLibResult<f32> {
        let mut forward_reference: real_vector3d = real_vector3d::default();
        let mut left_reference: real_vector3d = real_vector3d::default();
        c_bitstream_writer::axes_compute_reference_internal(up, &mut forward_reference, &mut left_reference)?;
        Ok(arctangent(dot_product3d(&left_reference, forward), dot_product3d(&forward_reference, forward)))
    }
}

#[cfg(test)]
mod bitstream_writer_tests {
    use super::*;

    #[test]
    fn write_legacy_be() {
        let expected: [u8; 2] = [
            0b10110_001, 0b00001001
        ];

        let mut sut = c_bitstream_writer::new_with_legacy_settings(2, e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        sut.begin_writing();

        sut.write_integer(0b001u32, 3).unwrap();
        sut.write_integer(310u32, 13).unwrap();

        sut.finish_writing();
        let actual = sut.get_data().unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn write_legacy_le() {
        let expected: [u8; 2] = [
            0b011111_001, 0b11111111
        ];

        let mut sut = c_bitstream_writer::new_with_legacy_settings(2, e_bitstream_byte_order::_bitstream_byte_order_little_endian);
        sut.begin_writing();

        sut.write_integer(0b001u32, 3).unwrap();
        sut.write_integer(8191u32, 13).unwrap();

        sut.finish_writing();
        let actual = sut.get_data().unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn write_be() {
        let expected: [u8; 2] = [
            0b001_11111, 0b11111111
        ];

        let mut sut = c_bitstream_writer::new(expected.len(), e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        sut.begin_writing();

        sut.write_integer(0b001u32, 3).unwrap();
        sut.write_integer(8191u32, 13).unwrap();

        sut.finish_writing();
        let actual = sut.get_data().unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn write_le() {
        let expected: [u8; 2] = [
            0b001_10000, 0b10000001
        ];

        let mut sut = c_bitstream_writer::new(expected.len(), e_bitstream_byte_order::_bitstream_byte_order_little_endian);
        sut.begin_writing();

        sut.write_integer(0b001u32, 3).unwrap();
        sut.write_integer(388u32, 13).unwrap();

        sut.finish_writing();
        let actual = sut.get_data().unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn write_with_msb_to_lsb_byte_pack_direction() {
        let expected: [u8; 1] = [
            0b00011111
        ];

        let mut sut = c_bitstream_writer::new(1, e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        sut.begin_writing();

        sut.write_integer(0b000u32, 3).unwrap();
        sut.write_integer(0b11111u32, 5).unwrap();

        sut.finish_writing();
        let actual = sut.get_data().unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn write_with_lsb_to_msb_byte_pack_direction() {
        let expected: [u8; 1] = [
            0b11111000
        ];

        let mut sut = c_bitstream_writer::new_with_legacy_settings(1, e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        sut.begin_writing();

        sut.write_integer(0b000u32, 3).unwrap();
        sut.write_integer(0b11111u32, 5).unwrap();

        sut.finish_writing();
        let actual = sut.get_data().unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn write_9_at_2() {
        let expected: [u8; 2] = [
            0b00000010, 0b01100000
        ];

        let mut sut = c_bitstream_writer::new(2, e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        sut.begin_writing();

        sut.write_integer(0b00u32, 2).unwrap();
        sut.write_integer(19u32, 9).unwrap();

        sut.finish_writing();
        let actual = sut.get_data().unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn write_h3_06481_game_set_data() {
        let expected: [u8; 13] = [
            0b10010100, 0b00000001, 0b00000000, 0b00000000,
            0b00000000, 0b10110001, 0b00001001, 0b00000000,
            0b00000000, 0b10010000, 0b10101011, 0b00000_011, 0
        ];

        let mut sut = c_bitstream_writer::new_with_legacy_settings(13, e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        sut.begin_writing();

        // game entries count
        sut.write_integer(20u32, 6).unwrap();
        // game entry 1 weight
        sut.write_integer(6u32, 32).unwrap();
        // game entry 1 minimum players
        sut.write_integer(4u32, 4).unwrap();
        // game entry 1 skip after veto
        sut.write_bool(false).unwrap();
        // game entry 1 map id
        sut.write_integer(310u32, 32).unwrap();
        // game entry 1 game variant (truncated)
        sut.write_string_utf8(&String::from("ru"), 3).unwrap();

        sut.finish_writing();
        let actual = sut.get_data().unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn write_h3_12070_game_set_data() {
        let expected: Vec<u8> = [
            0b11011100, 0b00000000, 0b00000000, 0b00000000,
            0b00000100, 0b01000000, 0b00000000, 0b00000000,
            0b00100000, 0b10000011, 0b01010101, 0b1111_0000, 0
        ].to_vec();

        let mut sut = c_bitstream_writer::new(13, e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        sut.begin_writing();

        // game entries count
        sut.write_integer(55u32, 6).unwrap();
        // game entry 1 weight
        sut.write_integer(1u32, 32).unwrap();
        // game entry 1 minimum players
        sut.write_integer(1u32, 4).unwrap();
        // game entry 1 skip after veto
        sut.write_bool(false).unwrap();
        // game entry 1 optional
        sut.write_bool(false).unwrap();
        // game entry 1 map id
        sut.write_integer(520u32, 32).unwrap();
        // game entry 1 game variant (truncated)
        sut.write_string_utf8(&String::from("5_"), 3).unwrap();

        sut.finish_writing();
        let actual = sut.get_data().unwrap();
        assert_eq!(actual, expected);
    }
}