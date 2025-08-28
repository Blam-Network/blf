mod bitstream_reader;
mod bitstream_writer;

pub use bitstream_reader::c_bitstream_reader;
pub use bitstream_writer::c_bitstream_writer;
use blf_lib_derivable::result::BLFLibResult;

pub fn create_bitstream_writer(size: usize, endian: e_bitstream_byte_order) -> c_bitstream_writer {
    let mut bitstream = c_bitstream_writer::new(size, endian);
    bitstream.begin_writing();
    bitstream
}

pub fn create_bitstream_reader(buffer: &[u8], endian: e_bitstream_byte_order) -> c_bitstream_reader<'_> {
    let mut bitstream = c_bitstream_reader::new(buffer, e_bitstream_byte_order::_bitstream_byte_order_big_endian);
    bitstream.begin_reading();
    bitstream
}

pub fn close_bitstream_writer(bitstream: &mut c_bitstream_writer) -> BLFLibResult<Vec<u8>> {
    let mut bits_remaining: usize = 0;
    bitstream.finish_writing(&mut bits_remaining);
    let mut data_length: usize = 0;
    let data = bitstream.get_data(&mut data_length)?;
    Ok(data[0..data_length].to_vec())
}

#[derive(Default, PartialEq, Eq, Debug, Copy, Clone)]
pub enum e_bitstream_byte_order
{
    #[default]
    _bitstream_byte_order_little_endian,
    _bitstream_byte_order_big_endian
}

#[derive(Default, PartialEq, Eq, Debug)]
pub enum e_bitstream_byte_fill_direction
{
    #[default]
    _bitstream_byte_fill_direction_msb_to_lsb,
    _bitstream_byte_fill_direction_lsb_to_msb // Used by pre-release h3
}

impl e_bitstream_byte_order
{
    pub fn swap(&self) -> e_bitstream_byte_order {
        match self {
            e_bitstream_byte_order::_bitstream_byte_order_little_endian => {
                e_bitstream_byte_order::_bitstream_byte_order_big_endian
            }
            e_bitstream_byte_order::_bitstream_byte_order_big_endian => {
                e_bitstream_byte_order::_bitstream_byte_order_little_endian
            }
        }
    }

    pub fn from_binrw_endian(endian: binrw::endian::Endian) -> e_bitstream_byte_order {
        match endian {
            binrw::endian::Endian::Big => e_bitstream_byte_order::_bitstream_byte_order_big_endian,
            binrw::endian::Endian::Little => e_bitstream_byte_order::_bitstream_byte_order_little_endian
        }
    }
}

#[derive(Default, PartialEq, Eq, Debug)]
pub enum e_bitstream_state
{
    #[default]
    _bitstream_state_initial = 0,
    _bitstream_state_writing,
    _bitstream_state_write_finished,
    _bitstream_state_reading,
    _bitstream_state_read_only_for_consistency,
    _bitstream_state_read_finished,

    k_bitstream_state_count
}
