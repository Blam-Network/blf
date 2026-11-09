use std::io::{Read, Seek, Write};
use binrw::{binrw, BinRead, BinResult, BinWrite, Endian};
#[cfg(feature = "napi")]
use napi_derive::napi;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use crate::blf::chunks::BlfChunkHooks;
use crate::{BlfChunk, TestSize};
use crate::blam::common::math::real_math::real_point3d;
use crate::types::array::StaticArray;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("mpev", 2.1)]
#[Size(0x8CA4)]
#[brw(big)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct s_blf_chunk_multiplayer_events {
    #[brw(pad_before = 4)]
    pub events: StaticArray<s_game_results_event, 1000>,
}

impl BlfChunkHooks for s_blf_chunk_multiplayer_events {}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(0x1C)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct s_game_results_data_kill {
    pub killer_position: real_point3d,
    pub dead_position: real_point3d,
    pub kill_type: i32,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(0x1C)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct s_game_results_data_carry {
    pub position: real_point3d,
    pub weapon_index: i32,
    #[brw(pad_after = 8)]
    pub carry_type: i32,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(0x1C)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct s_game_results_data_score {
    pub position: real_point3d,
    pub weapon_index: i32,
    #[brw(pad_after = 8)]
    pub score_type: i32,
}

#[binrw]
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default, TestSize)]
#[Size(0x1C)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct s_game_results_data_none {
    #[bw(calc = 0, pad_after = 27)]
    pad: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone, BinRead, BinWrite, FromPrimitive, Display, Default, PartialEq)]
#[brw(repr = u8)]
#[repr(u8)]
#[cfg_attr(feature = "napi", napi(namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub enum e_game_results_data_type {
    #[default]
    _no_data = 0,
    _kill = 1,
    _carry = 2,
    _score = 3,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default, TestSize)]
#[Size(0x24)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct s_game_results_event {
    pub type_: e_game_results_data_type,
    pub player_references: StaticArray<i8, 2>,
    pub kill_data: Option<s_game_results_data_kill>,
    pub carry_data: Option<s_game_results_data_carry>,
    pub score_data: Option<s_game_results_data_score>,
    pub time: u32,
}

impl BinRead for s_game_results_event {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, args: Self::Args<'_>) -> BinResult<Self> {
        let type_: e_game_results_data_type = e_game_results_data_type::from_u8(u8::read(reader)?).unwrap(); // TODO: don't unwrap
        let player_references = StaticArray::<i8, 2>::read_options(reader, endian, args)?;
        reader.seek_relative(1)?; // padding

        let mut kill: Option<s_game_results_data_kill> = None;
        let mut carry: Option<s_game_results_data_carry> = None;
        let mut score: Option<s_game_results_data_score> = None;

        match type_ {
            e_game_results_data_type::_no_data => {
                s_game_results_data_none::read_options(reader, endian, ());
            },
            e_game_results_data_type::_kill => {
                kill = Some(s_game_results_data_kill::read_options(reader, endian, ()).unwrap());
            },
            e_game_results_data_type::_carry => {
                carry = Some(s_game_results_data_carry::read_options(reader, endian, ())?);
            },
            e_game_results_data_type::_score => {
                score = Some(s_game_results_data_score::read_options(reader, endian, ())?);
            },
        };

        let time = match endian {
            Endian::Big => { u32::read_be(reader)? }
            Endian::Little => { u32::read_le(reader)? }
        };

        Ok(Self {
            type_,
            player_references,
            kill_data: kill,
            score_data: score,
            carry_data: carry,
            time,
        })
    }
}

impl BinWrite for s_game_results_event {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(&self, writer: &mut W, endian: Endian, args: Self::Args<'_>) -> BinResult<()> {
        self.type_.write_options(writer, endian, ())?;
        self.player_references.write_options(writer, endian, ())?;
        writer.seek_relative(1)?; //padding

        match &self.type_ {
            e_game_results_data_type::_no_data => s_game_results_data_none::default().write_options(writer, endian, ())?,
            e_game_results_data_type::_kill => self.kill_data.write_options(writer, endian, ())?,
            e_game_results_data_type::_score => self.score_data.write_options(writer, endian, ())?,
            e_game_results_data_type::_carry => self.carry_data.write_options(writer, endian, ())?,
        };

        match endian {
            Endian::Big => { u32::write_be(&self.time, writer)?; }
            Endian::Little => { u32::write_le(&self.time, writer)?; }
        }

        Ok(())
    }
}