use std::io::{Cursor, Read, Write};
use binrw::{BinRead, BinWrite};
use flate2::{Compress, Compression, Decompress};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use blf_lib_derivable::result::{BLFLibError, BLFLibResult};
use crate::io::bitstream::e_bitstream_byte_order;

// Signature differes from original.
pub fn runtime_data_decompress(
    source_buffer: &[u8],
    decompressed_buffer: &mut Vec<u8>,
    endian: e_bitstream_byte_order,
) -> BLFLibResult {
    let mut cursor = Cursor::new(source_buffer);
    let decompressed_buffer_size = u32::read_options(&mut cursor, endian.into(), ())?;
    let mut decoder = ZlibDecoder::new_with_decompress(
        cursor,
        Decompress::new(true)
    );

    decoder.read_to_end(decompressed_buffer)
        .map_err(|e| BLFLibError::from(
            format!("runtime_data_decompress: failed to decompress {} - {}",
                    decompressed_buffer_size, e.to_string()
        ))
    )?;

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

    let mut writer = Cursor::new(compressed_buffer);
    let decompressed_size = source_buffer.len() as u32;

    decompressed_size.write_options(&mut writer, endian.into(), ())?;
    writer.write_all(compressed_data.as_slice())?;

    Ok(())
}