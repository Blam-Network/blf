use binrw::{binrw, BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
use crate::BINRW_ERROR;

/// Matches `s_online_file_summary_listing_entry` in `blf.ts` (big-endian fields).
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_online_file_summary_listing_entry {
    pub share_id: u64,
    pub screenshots_count: u32,
    pub films_count: u32,
    pub map_variants_count: u32,
    pub game_variants_count: u32,
    pub new_items_count: u32,
    pub unknown1C: u32,
    pub unknown20: u32,
}

/// `finf` 1.0 — matches `fileshare.service.ts` (`entry_count`, 2-byte pad, `entries`).
#[binrw]
#[derive(BlfChunk, Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[Header("finf", 1.0)]
#[brw(big)]
pub struct s_blf_chunk_online_file_summary {
    #[brw(pad_after = 2)]
    #[bw(try_calc(u16::try_from(entries.len())))]
    #[br(temp)]
    pub entry_count: u16,
    #[br(count = entry_count)]
    pub entries: Vec<s_online_file_summary_listing_entry>,
}

impl BlfChunkHooks for s_blf_chunk_online_file_summary {}
