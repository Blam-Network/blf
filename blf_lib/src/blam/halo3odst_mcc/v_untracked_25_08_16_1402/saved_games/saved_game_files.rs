use binrw::{BinRead, BinWrite};
#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use crate::types::string::StaticString;
use crate::types::string::StaticWcharString;
use serde_hex::{SerHex, StrictCap};
use blf_lib_derivable::result::BLFLibResult;
use blf_lib::types::time::time64_t;
use blf_lib_derive::TestSize;
use crate::types::bool::Bool;
use crate::types::u64::Unsigned64;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite, TestSize)]
#[Size(0xF8)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3odst_mcc_v_untracked_25_08_16_1402"))]
pub struct s_content_item_metadata {
    pub unique_id: Unsigned64,
    pub name: StaticWcharString<0x10>,
    pub description: StaticString<128>,
    pub author: StaticString<16>,
    pub file_type: i32,
    #[brw(pad_after = 3)]
    pub author_is_xuid_online: Bool,
    #[serde(with = "SerHex::<StrictCap>")]
    pub author_id: Unsigned64,
    pub size_in_bytes: Unsigned64,
    pub date: time64_t,
    pub length_seconds: u32,
    pub campaign_id: i32,
    pub map_id: i32,
    pub game_engine_type: u32,
    pub campaign_difficulty: i32,
    pub campaign_insertion_point: u8,
    pub campaign_survival_enabled: Bool,
    #[brw(pad_before = 2)]
    pub game_id: Unsigned64,
}

impl s_content_item_metadata {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_qword(self.unique_id, 64)?;
        bitstream.write_string_wchar(&self.name.get_string(), 16)?;
        bitstream.write_string_utf8(&self.description.get_string()?, 128)?;
        bitstream.write_string_utf8(&self.author.get_string()?, 16)?;
        bitstream.write_signed_integer(self.file_type + 1, 5)?;
        bitstream.write_bool(self.author_is_xuid_online)?;
        bitstream.write_qword(self.author_id, 64)?;
        bitstream.write_qword(self.size_in_bytes, 64)?;
        bitstream.write_qword(self.date, 64)?;
        bitstream.write_integer(self.length_seconds, 32)?;
        bitstream.write_signed_integer(self.campaign_id, 32)?;
        bitstream.write_signed_integer(self.map_id, 32)?;
        bitstream.write_integer(self.game_engine_type, 4)?;
        bitstream.write_signed_integer(self.campaign_difficulty + 1, 3)?;
        bitstream.write_integer(self.campaign_insertion_point as u32, 4)?;
        bitstream.write_bool(self.campaign_survival_enabled)?;
        bitstream.write_qword(self.game_id, 64)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.unique_id = bitstream.read_qword(64)?;
        self.name.set_string(&bitstream.read_string_wchar(16)?)?;
        self.description.set_string(&bitstream.read_string_utf8(128)?)?;
        self.author.set_string(&bitstream.read_string_utf8(16)?)?;
        self.file_type = bitstream.read_unnamed_signed_integer::<i32>(5)? - 1;
        self.author_is_xuid_online = bitstream.read_unnamed_bool()?;
        self.author_id = bitstream.read_qword(64)?;
        self.size_in_bytes = bitstream.read_qword(64)?;
        self.date = bitstream.read_qword(64)?;
        self.length_seconds = bitstream.read_unnamed_integer(32)?;
        self.campaign_id = bitstream.read_unnamed_signed_integer(32)?;
        self.map_id = bitstream.read_unnamed_signed_integer(32)?;
        self.game_engine_type = bitstream.read_unnamed_integer(4)?;
        self.campaign_difficulty = bitstream.read_unnamed_signed_integer::<i32>(3)? - 1;
        self.campaign_insertion_point = bitstream.read_unnamed_integer(4)?;
        self.campaign_survival_enabled = bitstream.read_unnamed_bool()?;
        self.game_id = bitstream.read_qword(64)?;
        Ok(())
    }
}
