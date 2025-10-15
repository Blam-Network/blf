use std::error::Error;
use std::io::{Read, Seek, Write};
use binrw::{BinRead, BinResult, BinWrite, BinWriterExt, Endian};
use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use blf_lib::BINRW_RESULT;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer, e_bitstream_byte_order};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
use crate::types::c_string::StaticString;

#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("mhdf", 2.1)]
pub struct s_blf_chunk_hopper_description_table {
    description_count: usize,
    descriptions: Vec<s_game_hopper_description>,
}

impl BlfChunkHooks for s_blf_chunk_hopper_description_table {}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct s_game_hopper_description {
    pub identifier: u16,
    pub r#type: e_hopper_descripton_type,
    pub description: StaticString<256>,
}

#[derive(BinRead, BinWrite, Serialize, Deserialize, Default, PartialEq, Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
#[brw(big, repr = u8)]
pub enum e_hopper_descripton_type {
    #[default]
    _hopper_description_type_selected = 0,
    _hopper_description_type_restricted,   // 0.0
}


pub const MAX_DESCRIPTIONS: usize = 63;

impl s_blf_chunk_hopper_description_table {
    pub fn get_descriptions(&self) -> Vec<s_game_hopper_description> {
        self.descriptions.as_slice()[0..self.description_count].to_vec()
    }

    pub fn add_description(&mut self, config: (u16, &String)) -> Result<(), Box<dyn Error>> {
        if self.description_count >= MAX_DESCRIPTIONS {
            return Err("The hopper desciptions chunk is full!".into());
        }
        self.description_count += 1;
        self.descriptions.push(s_game_hopper_description {
            identifier: config.0,
            r#type: e_hopper_descripton_type::_hopper_description_type_selected, // seems unused
            description: StaticString::from_string(config.1)?,
        });
        Ok(())
    }
}


impl BinRead for s_blf_chunk_hopper_description_table {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, args: Self::Args<'_>) -> BinResult<Self> {
        let mut buffer = Vec::<u8>::new();
        reader.read_to_end(&mut buffer)?;

        let mut bitstream = c_bitstream_reader::new_with_legacy_settings(buffer.as_slice(), e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        bitstream.begin_reading();

        let mut mhdf = Self::default();

        mhdf.description_count = bitstream.read_unnamed_integer(6)?;
        mhdf.descriptions.resize(mhdf.description_count, s_game_hopper_description::default());

        for i in 0..mhdf.description_count {
            let description = &mut mhdf.descriptions[i];
            description.identifier = bitstream.read_unnamed_integer(16)?;
            description.r#type = if bitstream.read_unnamed_bool()? {
                e_hopper_descripton_type::_hopper_description_type_restricted
            } else {
                e_hopper_descripton_type::_hopper_description_type_selected
            };
            BINRW_RESULT!(description.description.set_string(&BINRW_RESULT!(bitstream.read_string_utf8(256))?))?;
        }

        Ok(mhdf)
    }
}

impl BinWrite for s_blf_chunk_hopper_description_table {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(&self, writer: &mut W, endian: Endian, args: Self::Args<'_>) -> BinResult<()> {
        let mut bitstream = c_bitstream_writer::new_with_legacy_settings(0x4000, e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        bitstream.begin_writing();

        bitstream.write_integer(self.description_count as u32, 6)?;

        for i in 0..self.description_count {
            let description = &self.descriptions[i];
            bitstream.write_integer(description.identifier as u32, 16)?;
            bitstream.write_bool(description.r#type == e_hopper_descripton_type::_hopper_description_type_restricted)?;
            bitstream.write_string_utf8(&description.description.get_string()?, 256)?;
        }

        bitstream.finish_writing();
        writer.write_ne(&bitstream.get_data()?)
    }
}
