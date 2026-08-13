use std::collections::HashMap;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use binrw::{binrw, BinRead, BinResult, BinWrite, Endian};
use serde::{Deserialize, Serialize};
use blf_lib::types::string::StaticString;
use blf_lib::types::time::filetime;
use blf_lib::types::u64::Unsigned64;
use blf_lib::types::numbers::Float32;

#[cfg(feature = "napi")]
use napi_derive::napi;

#[binrw]
#[cfg_attr(feature = "napi", napi(object, namespace = "common"))]
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default)]
pub struct s_data_mine_header_v1 {
    pub byte_order_marker_fffe: u16,
    pub version_major: u16,
    pub sessionid: StaticString<128>,
    pub build_string: StaticString<32>,
    pub build_number: i32,
    pub systemid: StaticString<160>,
    pub title: StaticString<32>,
    pub session_start_date: filetime,
}

#[binrw]
#[cfg_attr(feature = "napi", napi(object, namespace = "common"))]
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default)]
pub struct s_data_mine_header_v2 {
    pub byte_order_marker_fffe: u16,
    pub version_major: u16,
    pub sessionid: StaticString<128>,
    pub build_string: StaticString<32>,
    pub build_number: i32,
    pub systemid: StaticString<160>,
    pub title: StaticString<32>,
    pub session_start_date: filetime,
    // Seemingly set by data_mine_set_header_flag
    // "sets the source flags of the data mine header (used for backend bucketing)"
    // Seems to be a 1 flag bitfield.
    pub source_flag: u8,
}

// v3: major=3, minor=1 (extra u16 vs v1/v2).
#[binrw]
#[cfg_attr(feature = "napi", napi(object, namespace = "common"))]
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default)]
pub struct s_data_mine_header_v3 {
    pub byte_order_marker_fffe: u16,
    pub version_major: u16,
    pub version_minor: u16,
    pub sessionid: StaticString<128>,
    pub build_string: StaticString<32>,
    pub build_number: i32,
    pub systemid: StaticString<160>,
    pub title: StaticString<32>,
    pub session_start_date: filetime,
    pub source_flag: u8,
    pub source_flag_pad: u8,
    pub unknown0: StaticString<64>,
    pub application_name: StaticString<32>,
    pub unknown1: u32,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum s_data_mine_header {
    V1(s_data_mine_header_v1),
    V2(s_data_mine_header_v2),
    V3(s_data_mine_header_v3),
}

impl s_data_mine_header {
    pub fn version(&self) -> u16 {
        match self {
            Self::V1(_) => 1,
            Self::V2(_) => 2,
            Self::V3(_) => 3,
        }
    }
}

#[cfg_attr(feature = "napi", napi(object, namespace = "common"))]
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
pub struct c_datamine_game_info {
    pub game_instance: Unsigned64,
    pub map: StaticString<260>,
}

/// v1/v2 monolithic event header (definition + values in one record).
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

#[binrw]
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default)]
pub struct s_datamine_v3_definition_prefix {
    pub total_size: u32,
    pub format_major: u8,
    pub priority: u8,
    pub event_name: StaticString<512>,
    pub parameter_signature: StaticString<512>,
    /// Registered-report id; occurrences join by this value.
    pub definition_id: u32,
}

/// v3 occurrence prefix: size + type=2 + index + definition_id + filetime.
#[binrw]
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default)]
pub struct s_datamine_v3_occurrence_prefix {
    pub total_size: u32,
    pub record_type: u8,
    pub event_index: u32,
    pub definition_id: u32,
    pub event_date: filetime,
}

#[cfg_attr(feature = "napi", napi(namespace = "common"))]
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
    pub parameter_type: e_datamine_parameter_type,
}

// dont think this struct strictly exists in blam!, think it's anonymous usually.
#[binrw]
#[cfg_attr(feature = "napi", napi(object, namespace = "common"))]
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default)]
pub struct s_datamine_value_string {
    #[br(temp)]
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
    pub parameter_type: e_datamine_parameter_type,

    // These values are supposed to exist in a separate struct named s_datamine_value
    // but due to rust union complexities I've pulled it here.
    pub value_long: Option<u32>,
    pub value_int64: Option<Unsigned64>,
    pub value_float: Option<Float32>,
    pub value_string: Option<s_datamine_value_string>,
}

impl BinRead for s_datamine_parameter {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, args: Self::Args<'_>) -> BinResult<Self> {
        let mut read_param = Self::default();
        read_param.name = BinRead::read_options(reader, endian, args)?;
        read_param.parameter_type = BinRead::read_options(reader, endian, args)?;
        read_param_value(reader, endian, &mut read_param)?;
        Ok(read_param)
    }
}

impl BinWrite for s_datamine_parameter {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(&self, writer: &mut W, endian: Endian, args: Self::Args<'_>) -> BinResult<()> {
        self.name.write_options(writer, endian, args)?;
        self.parameter_type.write_options(writer, endian, args)?;

        match self.parameter_type {
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

fn read_param_value<R: Read + Seek>(
    reader: &mut R,
    endian: Endian,
    param: &mut s_datamine_parameter,
) -> BinResult<()> {
    match param.parameter_type {
        e_datamine_parameter_type::_datamine_parameter_type_long => {
            param.value_long = Some(BinRead::read_options(reader, endian, ())?);
        }
        e_datamine_parameter_type::_datamine_parameter_type_int64 => {
            param.value_int64 = Some(BinRead::read_options(reader, endian, ())?);
        }
        e_datamine_parameter_type::_datamine_parameter_type_float => {
            param.value_float = Some(BinRead::read_options(reader, endian, ())?);
        }
        e_datamine_parameter_type::_datamine_parameter_type_string => {
            param.value_string = Some(BinRead::read_options(reader, endian, ())?);
        }
    }
    Ok(())
}

#[binrw]
#[cfg_attr(feature = "napi", napi(object, namespace = "common"))]
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default)]
pub struct s_datamine_event {
    pub header: s_datamine_event_header,

    #[br(temp)]
    #[bw(try_calc(u32::try_from(categories.len())))]
    category_count: u32,
    #[br(count = category_count)]
    pub categories: Vec<StaticString<32>>,

    #[br(temp)]
    #[bw(try_calc(u32::try_from(parameters.len())))]
    parameter_count: u32,
    #[br(count = parameter_count)]
    pub parameters: Vec<s_datamine_parameter>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default)]
pub struct s_datamine_v3_definition {
    pub definition_id: u32,
    pub priority: u8,
    pub event_name: StaticString<512>,
    pub parameter_signature: StaticString<512>,
    pub categories: Vec<StaticString<32>>,
    pub parameters: Vec<s_datamine_parameter_header>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct s_datamine_file {
    pub header: s_data_mine_header,
    pub events: Vec<s_datamine_event>,
}

fn read_u16<R: Read + Seek>(reader: &mut R, endian: Endian) -> BinResult<u16> {
    BinRead::read_options(reader, endian, ())
}

fn read_u32<R: Read + Seek>(reader: &mut R, endian: Endian) -> BinResult<u32> {
    BinRead::read_options(reader, endian, ())
}

fn read_u8<R: Read + Seek>(reader: &mut R) -> BinResult<u8> {
    let mut b = [0u8; 1];
    reader.read_exact(&mut b)?;
    Ok(b[0])
}

fn read_counted_strings<R: Read + Seek, const N: usize>(
    reader: &mut R,
    endian: Endian,
) -> BinResult<Vec<StaticString<N>>> {
    let count: u32 = read_u32(reader, endian)?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(BinRead::read_options(reader, endian, ())?);
    }
    Ok(out)
}

fn read_v3_definition<R: Read + Seek>(
    reader: &mut R,
    endian: Endian,
) -> BinResult<s_datamine_v3_definition> {
    let prefix: s_datamine_v3_definition_prefix = BinRead::read_options(reader, endian, ())?;
    if prefix.format_major != 1 {
        return Err(binrw::Error::AssertFail {
            pos: reader.stream_position().unwrap_or(0),
            message: format!("unexpected definition format_major {}", prefix.format_major),
        });
    }
    let categories = read_counted_strings::<_, 32>(reader, endian)?;
    let parameter_count: u32 = read_u32(reader, endian)?;
    let mut parameters = Vec::with_capacity(parameter_count as usize);
    for _ in 0..parameter_count {
        parameters.push(BinRead::read_options(reader, endian, ())?);
    }
    Ok(s_datamine_v3_definition {
        definition_id: prefix.definition_id,
        priority: prefix.priority,
        event_name: prefix.event_name,
        parameter_signature: prefix.parameter_signature,
        categories,
        parameters,
    })
}

fn read_v3_occurrence<R: Read + Seek>(
    reader: &mut R,
    endian: Endian,
    definitions: &HashMap<u32, s_datamine_v3_definition>,
) -> BinResult<s_datamine_event> {
    let prefix: s_datamine_v3_occurrence_prefix = BinRead::read_options(reader, endian, ())?;
    if prefix.record_type != 2 {
        return Err(binrw::Error::AssertFail {
            pos: reader.stream_position().unwrap_or(0),
            message: format!("unexpected occurrence record_type {}", prefix.record_type),
        });
    }
    let def = definitions.get(&prefix.definition_id);

    let mut parameters = Vec::new();
    if let Some(def) = def {
        for schema in &def.parameters {
            let mut param = s_datamine_parameter {
                name: schema.name.clone(),
                parameter_type: schema.parameter_type.clone(),
                ..Default::default()
            };
            read_param_value(reader, endian, &mut param)?;
            parameters.push(param);
        }
    }

    let (event_name, parameter_signature, categories, priority) = match def {
        Some(d) => (
            d.event_name.clone(),
            d.parameter_signature.clone(),
            d.categories.clone(),
            u32::from(d.priority),
        ),
        None => Default::default(),
    };

    Ok(s_datamine_event {
        header: s_datamine_event_header {
            total_size: prefix.total_size,
            event_name,
            parameter_signature,
            priority,
            event_index: prefix.event_index,
            game_info: c_datamine_game_info::default(),
            event_date: prefix.event_date,
        },
        categories,
        parameters,
    })
}

fn read_records<R: Read + Seek>(
    reader: &mut R,
    endian: Endian,
    version: u16,
) -> Vec<s_datamine_event> {
    let mut events = Vec::new();
    let mut definitions: HashMap<u32, s_datamine_v3_definition> = HashMap::new();

    loop {
        let start = match reader.stream_position() {
            Ok(p) => p,
            Err(_) => break,
        };

        let total_size: u32 = match read_u32(reader, endian) {
            Ok(s) if s >= 5 => s,
            _ => break,
        };

        // Rewind so record readers see total_size again.
        if reader.seek(SeekFrom::Start(start)).is_err() {
            break;
        }

        let end = start + u64::from(total_size);

        if version == 3 {
            // Peek record kind after total_size.
            if reader.seek(SeekFrom::Start(start + 4)).is_err() {
                break;
            }
            let kind = match read_u8(reader) {
                Ok(k) => k,
                Err(_) => break,
            };
            if reader.seek(SeekFrom::Start(start)).is_err() {
                break;
            }

            match kind {
                1 => match read_v3_definition(reader, endian) {
                    Ok(def) => {
                        definitions.insert(def.definition_id, def);
                    }
                    Err(_) => break,
                },
                2 => match read_v3_occurrence(reader, endian, &definitions) {
                    Ok(event) => events.push(event),
                    Err(_) => break,
                },
                _ => break,
            }
        } else {
            match s_datamine_event::read_options(reader, endian, ()) {
                Ok(event) => events.push(event),
                Err(_) => break,
            }
        }

        if reader.seek(SeekFrom::Start(end)).is_err() {
            break;
        }
    }

    events
}

/// Parse a raw `compressed.dat` buffer. Returns `None` if the header is unrecognized.
pub fn read_datamine_file(buffer: &[u8]) -> Option<s_datamine_file> {
    if buffer.len() < 4 {
        return None;
    }

    let mut endian = Endian::Big;
    let bom = u16::from_be_bytes([buffer[0], buffer[1]]);
    if bom == 0xfeff {
        endian = Endian::Little;
    } else if bom != 0xfffe {
        return None;
    }

    let major = match endian {
        Endian::Big => u16::from_be_bytes([buffer[2], buffer[3]]),
        Endian::Little => u16::from_le_bytes([buffer[2], buffer[3]]),
    };

    let mut reader = Cursor::new(buffer);
    let header = match major {
        1 => s_data_mine_header::V1(
            s_data_mine_header_v1::read_options(&mut reader, endian, ()).ok()?,
        ),
        2 => s_data_mine_header::V2(
            s_data_mine_header_v2::read_options(&mut reader, endian, ()).ok()?,
        ),
        3 => s_data_mine_header::V3(
            s_data_mine_header_v3::read_options(&mut reader, endian, ()).ok()?,
        ),
        _ => return None,
    };

    let events = read_records(&mut reader, endian, header.version());
    Some(s_datamine_file { header, events })
}
