pub(crate) mod halo3;
pub(crate) mod halo3odst;
pub(crate) mod haloreach;
pub(crate) mod ares;
pub(crate) mod haloonline;

use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{Cursor, Read, Seek};
use binrw::BinReaderExt;
use serde::Deserialize;
use blf_lib::BINRW_RESULT;
use blf_lib::blf::s_blf_header;
pub use blf_lib_derivable::blf::chunks::*;
use blf_lib_derivable::result::{BLFLibError, BLFLibResult};
use blf_lib_derivable::types::chunk_signature::chunk_signature;
use crate::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_end_of_file, s_blf_chunk_end_of_file_with_sha1, s_blf_chunk_end_of_file_with_rsa, s_blf_chunk_end_of_file_with_crc};

pub fn find_and_validate_eof(buffer: &[u8]) -> BLFLibResult {
    let mut cursor = Cursor::new(buffer);
    let mut header_bytes = [0u8; s_blf_header::size()];
    let mut header: s_blf_header;

    while cursor.read_exact(&mut header_bytes).is_ok() {
        header = s_blf_header::decode(&header_bytes)?;
        let mut body_bytes = vec![0u8; (header.chunk_size as usize) - s_blf_header::size()];
        if header.signature == chunk_signature::from_string("_eof") && header.version.major == 1 {
            cursor.read_exact(body_bytes.as_mut_slice())?;
            let authentication_type: u8 = body_bytes[4];
            let mut chunk_position = (cursor.position() as usize) - body_bytes.len() - s_blf_header::size();

            match authentication_type {
                0 => { s_blf_chunk_end_of_file::read(body_bytes.clone(), Some(header.clone()), &buffer[0..chunk_position])?; }
                1 => { s_blf_chunk_end_of_file_with_crc::read(body_bytes.clone(), Some(header.clone()), &buffer[0..chunk_position])?; }
                2 => { s_blf_chunk_end_of_file_with_sha1::read(body_bytes.clone(), Some(header.clone()), &buffer[0..chunk_position])?; }
                3 => { s_blf_chunk_end_of_file_with_rsa::read(body_bytes.clone(), Some(header.clone()), &buffer[0..chunk_position])?; }
                _ => { return Err(format!("Unknown _eof authentication type {}", authentication_type).into()); }
            }
        }
        if header.chunk_size == 0 {
            break;
        }
        cursor.seek_relative((header.chunk_size - s_blf_header::size() as u32) as i64)?;
    }

    Ok(())
}

pub fn find_chunk<'a, T: BlfChunk + SerializableBlfChunk + ReadableBlfChunk>(buffer: &[u8]) -> Result<T, Box<dyn Error>> {
    let mut cursor = Cursor::new(buffer);
    let mut headerBytes = [0u8; s_blf_header::size()];
    let mut header: s_blf_header;

    find_and_validate_eof(buffer)?;

    while cursor.read_exact(&mut headerBytes).is_ok() {
        header = s_blf_header::decode(&headerBytes)?;
        if header.signature == T::get_signature() && header.version == T::get_version() {
            let mut body_bytes = vec![0u8; (header.chunk_size as usize) - s_blf_header::size()];
            cursor.read_exact(body_bytes.as_mut_slice())?;
            return Ok(BINRW_RESULT!(T::read(body_bytes, Some(header), &Vec::new()))?);
        }
        if header.chunk_size == 0 {
            break;
        }
        cursor.seek_relative((header.chunk_size - s_blf_header::size() as u32) as i64)?;
    }
    Err(format!("{} Chunk not found!", T::get_signature()).into())
}

pub fn find_chunk_in_file<T: BlfChunk + SerializableBlfChunk + ReadableBlfChunk>(path: impl Into<String>) -> BLFLibResult<T> {
    let path = path.into();
    let mut file = File::open(&path)?;
    let mut headerBytes = [0u8; s_blf_header::size()];
    let mut header: s_blf_header;

    find_and_validate_eof(&fs::read(&path)?)?;

    while file.read_exact(&mut headerBytes).is_ok() {
        header = s_blf_header::decode(&headerBytes)?;
        if header.signature == T::get_signature() && header.version == T::get_version() {
            let mut body_bytes = vec![0u8; (header.chunk_size as usize) - s_blf_header::size()];
            file.read_exact(body_bytes.as_mut_slice())?;
            return Ok(T::read(body_bytes, Some(header), &Vec::new())?);
        }
        if header.chunk_size == 0 {
            break;
        }
        file.seek_relative((header.chunk_size - s_blf_header::size() as u32) as i64)?;
    }
    Err(format!("{} chunk not found in file {}", T::get_signature(), &path).into())
}

pub fn search_for_chunk<'a, T: BlfChunk + SerializableBlfChunk + ReadableBlfChunk>(buffer: Vec<u8>) -> BLFLibResult<Option<T>> {
    for i in 0..(buffer.len() - 0xC) {
        let header_bytes = &buffer.as_slice()[i..i+0xC];
        let header = s_blf_header::decode(header_bytes)?;

        if header.signature == T::get_signature() && header.version == T::get_version() {
            let body_bytes = buffer.as_slice()[i+0xC..i+header.chunk_size as usize].to_vec();
            return Ok(Some(T::read(body_bytes, Some(header), &Vec::new())?));
        }
    }

    Ok(None)
}

pub fn search_for_chunk_in_file<T: BlfChunk + SerializableBlfChunk + ReadableBlfChunk>(path: impl Into<String>) -> BLFLibResult<Option<T>> {
    let mut fileBytes = Vec::<u8>::new();
    File::open(path.into())?.read_to_end(&mut fileBytes)?;

    for i in 0..(fileBytes.len() - 0xC) {
        let header_bytes = &fileBytes.as_slice()[i..i+0xC];
        let header = s_blf_header::decode(header_bytes)?;

        if header.signature == T::get_signature() && header.version == T::get_version() {
            let body_bytes = fileBytes.as_slice()[i+0xC..i+header.chunk_size as usize].to_vec();
            return Ok(Some(T::read(body_bytes, Some(header), &Vec::new())?));
        }
    }

    Ok(None)
}

pub fn read_chunk_json<T: BlfChunk + for<'d> Deserialize<'d>>(path: &str) -> BLFLibResult<T> {
    let mut file = File::open(path)?;
    Ok(serde_json::from_reader(&mut file)?)
}