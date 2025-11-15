use std::io::{Read, Seek, Write};
use binrw::{binrw, BinRead, BinResult, BinWrite, Endian};
use binrw::helpers::count;
use serde::{Deserialize, Serialize};
use blf_lib::types::c_string::StaticString;
use blf_lib::types::time::filetime;

#[cfg(feature = "napi")]
use napi_derive::napi;

#[cfg_attr(feature = "napi", napi(object, namespace = "common"))]
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
pub struct s_data_mine_header {
    pub byte_order_marker_fffe: u16,
    pub version_major: u16,
    pub version_minor: u16,
    pub sessionid: StaticString<128>,
    pub build_string: StaticString<32>,
    pub build_number: u32,
    pub systemid: StaticString<160>,
    pub title: StaticString<32>,
    pub session_start_date: filetime,
}

#[cfg_attr(feature = "napi", napi(object, namespace = "common"))]
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
pub struct c_datamine_game_info {
    pub game_instance: u64,
    pub map: StaticString<260>,
}

#[cfg_attr(feature = "napi", napi(object, namespace = "common"))]
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
pub struct s_datamine_event_header {
    pub total_size: u32,
    pub event_name: StaticString<512>,
    pub parameter_signature: StaticString<512>,
    pub priority: u32,
    pub event_index: u32,
    pub game_info: c_datamine_game_info,
    pub event_date: filetime,
}

#[cfg_attr(feature = "napi", napi(object, namespace = "common"))]
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(repr = u32)]
pub enum e_datamine_parameter_type {
    #[default]
    _datamine_parameter_type_long = 0,
    _datamine_parameter_type_int64 = 1,
    _datamine_parameter_type_float = 2,
    _datamine_parameter_type_string = 3,
}

#[cfg_attr(feature = "napi", napi(object, namespace = "common"))]
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
pub struct s_datamine_parameter_header {
    pub name: StaticString<32>,
    pub r#type: e_datamine_parameter_type,
}

// dont think this struct strictly exists in blam!, think it's anonymous usually.
#[cfg_attr(feature = "napi", napi(object, namespace = "common"))]
#[binrw]
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default)]
pub struct s_datamine_value_string {
    #[bw(try_calc(u32::try_from(string.len())))]
    pub string_length: u32,
    #[br(count = string_length, try_map = |s: Vec<u8>| String::from_utf8(s))]
    #[bw(map = |s: &String| s.as_bytes())]
    pub string: String,
}

#[cfg_attr(feature = "napi", napi(object, namespace = "common"))]
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default)]
pub struct s_datamine_parameter {
    pub name: StaticString<32>,
    pub r#type: e_datamine_parameter_type,

    // These values are supposed to exist in a separate struct named s_datamine_value
    // but due to rust union complexities I've pulled it here.
    pub value_long: Option<u32>,
    pub value_int64: Option<u64>,
    pub value_float: Option<f32>,
    pub value_string: Option<s_datamine_value_string>,
}

impl BinRead for s_datamine_parameter {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, args: Self::Args<'_>) -> BinResult<Self> {
        let mut read_param = Self::default();
        read_param.name = BinRead::read_options(reader, endian, args)?;
        read_param.r#type = BinRead::read_options(reader, endian, args)?;

        match read_param.r#type {
            e_datamine_parameter_type::_datamine_parameter_type_long => {
                read_param.value_long = Some(BinRead::read_options(reader, endian, args)?);
            }
            e_datamine_parameter_type::_datamine_parameter_type_int64 => {
                read_param.value_int64 = Some(BinRead::read_options(reader, endian, args)?);
            }
            e_datamine_parameter_type::_datamine_parameter_type_float => {
                read_param.value_float = Some(BinRead::read_options(reader, endian, args)?);
            }
            e_datamine_parameter_type::_datamine_parameter_type_string => {
                read_param.value_string = Some(BinRead::read_options(reader, endian, args)?);
            }
        }

        Ok(read_param)
    }
}

impl BinWrite for s_datamine_parameter {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(&self, writer: &mut W, endian: Endian, args: Self::Args<'_>) -> BinResult<()> {
        self.name.write_options(writer, endian, args)?;
        self.r#type.write_options(writer, endian, args)?;

        match self.r#type {
            e_datamine_parameter_type::_datamine_parameter_type_long => {
                self.value_long.write_options(writer, endian, args)?;
            }
            e_datamine_parameter_type::_datamine_parameter_type_int64 => {
                self.value_int64.write_options(writer, endian, args)?;
            }
            e_datamine_parameter_type::_datamine_parameter_type_float => {
                self.value_float.write_options(writer, endian, args)?;
            }
            e_datamine_parameter_type::_datamine_parameter_type_string => {
                self.value_string.write_options(writer, endian, args)?;
            }
        }

        Ok(())
    }
}