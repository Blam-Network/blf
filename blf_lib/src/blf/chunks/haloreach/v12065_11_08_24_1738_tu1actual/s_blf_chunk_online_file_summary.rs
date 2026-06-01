use binrw::{binrw, BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;

pub use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::networking::online::files::online_file_summary_listing::s_online_file_summary_listing_entry;

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
