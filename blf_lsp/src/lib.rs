use std::io::{Cursor, Read};
use bytes::Bytes;
use flate2::read::ZlibDecoder;

mod halo3;
mod haloreach;

pub fn zlib_decompress(data: &Bytes) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut uncompressed_buffer = Vec::<u8>::new();
    let (unknown, mut data) = data.split_at(12);
    let data = Vec::from(data);
    let mut decoder = ZlibDecoder::new(Cursor::new(&data));
    decoder.read_to_end(&mut uncompressed_buffer)?;
    Ok(uncompressed_buffer)
}