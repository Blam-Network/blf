use std::io::{Read, Seek, Write};
use binrw::{BinRead, BinResult, BinWrite, Endian};
use serde::{Deserialize, Serialize};
use blf_lib::blam::common::memory::secure_signature::s_network_http_request_hash;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer, e_bitstream_byte_order};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::game_variant::c_game_variant;
use crate::blf::get_buffer_hash;

#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize,Default)]
#[Header("mpvr", 54.1)]
pub struct s_blf_chunk_game_variant
{
    pub game_variant: c_game_variant,
}

impl BinRead for s_blf_chunk_game_variant {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, args: Self::Args<'_>) -> BinResult<Self> {
        let mut data: Vec<u8> = Vec::new();
        reader.read_to_end(&mut data)?;

        let mut bitstream = c_bitstream_reader::new(data.as_slice(), e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        bitstream.begin_reading();

        let hash: s_network_http_request_hash = bitstream.read_raw(0x14 * 8)?;
        let unknown04: i16 = bitstream.read_integer("unknown04", 16)?;
        let unknown06: u16 = bitstream.read_integer("unknown06", 16)?;
        let variant_length: u32 = bitstream.read_integer("variant-length", 32)?;


        let mut game_variant = c_game_variant::default();
        game_variant.decode(&mut bitstream)?;

        Ok(Self {
            game_variant,
        })
    }
}

impl BinWrite for s_blf_chunk_game_variant {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(&self, writer: &mut W, endian: Endian, args: Self::Args<'_>) -> BinResult<()> {
        let mut bitstream_writer = c_bitstream_writer::new(0x5028, e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        bitstream_writer.begin_writing();
        self.game_variant.encode(&mut bitstream_writer)?;
        bitstream_writer.finish_writing();
        let gametype_data = bitstream_writer.get_data()?;
        let mut hashable_data: Vec<u8> = (gametype_data.len() as u32).to_be_bytes().to_vec();
        hashable_data.extend_from_slice(gametype_data.as_slice());

        let hash = get_buffer_hash(&hashable_data)?;
        let unknown04: i16 = -1;
        let unknown06: u16 = 0;
        let gametype_length = gametype_data.len() as u32;

        hash.write_options(writer, Endian::Big, args)?;
        unknown04.write_options(writer, Endian::Big, args)?;
        unknown06.write_options(writer, Endian::Big, args)?;
        gametype_length.write_options(writer, Endian::Big, args)?;
        gametype_data.write_options(writer, Endian::Big, args)?;

        Ok(())
    }
}


impl BlfChunkHooks for s_blf_chunk_game_variant {}

impl s_blf_chunk_game_variant {
    pub fn create(game_variant: c_game_variant) -> s_blf_chunk_game_variant {
        s_blf_chunk_game_variant {
            game_variant
        }
    }
}