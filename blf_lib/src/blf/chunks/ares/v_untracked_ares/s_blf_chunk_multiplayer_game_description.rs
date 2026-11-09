use std::io::{Cursor, Read, Seek};
use binrw::{binrw, BinRead, BinReaderExt, BinResult, BinWrite};
#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use crate::blf::chunks::BlfChunkHooks;
use crate::{BlfChunk, TEST_BIT};
use crate::types::bool::Bool;
use crate::types::string::{StaticString, StaticWcharString};
use crate::types::time::time32_t;
use crate::types::u64::Unsigned64;
use serde_hex::{SerHex, StrictCap};

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("mpgd", 2.1)]
#[Size(0x174)]
#[brw(little)]
#[cfg_attr(feature = "napi", napi(object, namespace = "ares_untracked"))]
pub struct s_blf_chunk_multiplayer_game_description {
    pub team_game: Bool,
    #[serde(with = "SerHex::<StrictCap>")]
    pub game_id: Unsigned64,
    #[brw(big)] // not sure why!
    pub map_id: u32,
    pub scenario_path: s_scenario_path,
    pub map_variant_name: StaticWcharString<0x20>,
    #[serde(with = "SerHex::<StrictCap>")]
    pub game_variant_unique_id: Unsigned64,
    #[serde(with = "SerHex::<StrictCap>")]
    pub map_variant_unique_id: Unsigned64,
    pub started: Bool,
    #[brw(big)]
    pub start_time: time32_t,
    pub finished: Bool,
    #[brw(big)]
    pub finish_time: time32_t,
    pub migrated_to_group: Bool,
    pub migrated_solo: Bool,
    #[brw(pad_after = 6)]
    pub simulation_aborted: Bool
}

impl BlfChunkHooks for s_blf_chunk_multiplayer_game_description { }

// This type just handles DLC paths which aren't UTF-8, but we want them to be.
#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinWrite)]
#[brw(big)]
#[cfg_attr(feature = "napi", napi(object, namespace = "ares_untracked"))]
pub struct s_scenario_path {
    #[brw(ignore)]
    pub is_dlc: bool,
    pub path: StaticString<260>,
}

impl BinRead for s_scenario_path {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: binrw::Endian, _args: Self::Args<'_>) -> BinResult<Self> {
        // Read the `StaticString` field, ignoring the first bit
        let mut buffer = vec![0u8; 260];
        reader.read_exact(&mut buffer)?;

        // Ignore the first bit of the first byte
        let is_dlc = TEST_BIT!(buffer[0], 7);
        buffer[0] &= 0b0111_1111;
        let mut cursor = Cursor::new(buffer);

        // Convert the buffer to a StaticString
        let path: StaticString<260> = cursor.read_type(endian)?;

        Ok(s_scenario_path { path, is_dlc })
    }
}