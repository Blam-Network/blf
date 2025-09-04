use std::io::{Cursor, Read, Write};
use binrw::BinRead;
use flate2::{Compress, Compression, Decompress};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use blf_lib_derivable::result::BLFLibResult;
use crate::io::bitstream::e_bitstream_byte_order;

// Signature differes from original.
pub fn runtime_data_decompress(
    source_buffer: &[u8],
    decompressed_buffer: &mut Vec<u8>,
    endian: e_bitstream_byte_order,
) -> BLFLibResult {
    let mut cursor = Cursor::new(source_buffer);
    let compressed_buffer_size = u32::read_options(&mut cursor, endian.into(), ())?;
    let mut decoder = ZlibDecoder::new_with_decompress(
        cursor,
        Decompress::new(true)
    );
    decoder.read_to_end(decompressed_buffer)?;

    Ok(())
}

pub fn runtime_data_compress(
    source_buffer: &Vec<u8>,
    compressed_buffer: &mut Vec<u8>,
    endian: e_bitstream_byte_order,
) -> BLFLibResult {
    let mut e = ZlibEncoder::new_with_compress(Vec::new(), Compress::new_with_window_bits(Compression::new(9), true, 15));
    e.write_all(source_buffer)?;
    let mut compressed_data = e.finish()?;

    compressed_buffer.append(&mut match endian {
        e_bitstream_byte_order::_bitstream_byte_order_big_endian => {
            u32::to_be_bytes(compressed_data.len() as u32)
        }
        e_bitstream_byte_order::_bitstream_byte_order_little_endian => {
            u32::to_le_bytes(compressed_data.len() as u32)
        }
    }.to_vec());

    compressed_buffer.append(&mut compressed_data);

    Ok(())
}