use binrw::{binrw, BinResult, NullWideString};
use serde::{Deserialize, Serialize};
use serde_hex::{SerHex, StrictCap};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
use crate::BINRW_ERROR;
use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::networking::online::files::online_file_metadata::{
    s_online_file_metadata, s_online_file_tag,
};
use crate::types::c_string::StaticString;
use crate::types::u64::Unsigned64;

/// `message_length` on the wire is the message length in **Unicode characters**
/// (scalar values), not UTF-8 bytes or UTF-16 code units.
fn message_len_byte(message: &String) -> BinResult<u8> {
    let n = message.chars().count();
    u8::try_from(n).map_err(|_| {
        BINRW_ERROR!(String::from(
            "online file listing message is too long for u8 message_length",
        ))
    })
}

/// Wire layout matches `s_online_file_listing` in `blf.ts` (`#[brw(big)]`).
#[binrw]
#[derive(BlfChunk, Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[Header("fitm", 4.0)]
#[brw(big)]
pub struct s_blf_chunk_online_file_listing {
    #[serde(with = "SerHex::<StrictCap>")]
    pub xuid: Unsigned64,
    pub gamertag: StaticString<16>,
    pub unknown16: u8,
    pub unknown17: u8,
    pub unknown18: u8,
    pub unknown19: u8,
    pub quota_byte_count: u32,
    #[brw(pad_after = 1)]
    pub quota_slot_count: u8,
    #[bw(try_calc(u16::try_from(entries.len())))]
    #[br(temp)]
    pub slot_count: u16,
    #[bw(try_calc(u8::try_from(message.chars().count())))]
    #[brw(pad_after = 3)]
    pub message_length: u8,
    #[br(count = slot_count)]
    pub entries: Vec<s_online_file_metadata>,
    #[br(if(message_length > 0), map = |w: NullWideString| w.to_string())]
    #[bw(if(message_length > 0), map = |s: &String| NullWideString::from(s.as_str()))]
    pub message: String,
    #[br(if(slot_count == 1 && !entries.is_empty() && entries[0].screenshot_length != 0), count = entries[0].screenshot_length as usize)]
    #[bw(if(slot_count == 1 && !entries.is_empty() && entries[0].screenshot_length != 0))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub screenshot_data: Vec<u8>,
    #[br(if(slot_count == 1 && !entries.is_empty() && entries[0].general.tag_count != 0), count = entries[0].general.tag_count as usize)]
    #[bw(if(slot_count == 1 && !entries.is_empty() && entries[0].general.tag_count != 0))]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<s_online_file_tag>,
}

impl BlfChunkHooks for s_blf_chunk_online_file_listing {}
