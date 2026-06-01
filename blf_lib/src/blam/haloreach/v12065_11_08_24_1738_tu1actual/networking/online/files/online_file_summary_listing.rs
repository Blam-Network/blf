use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_online_file_summary_listing_entry {
    pub share_id: u64,
    pub screenshots_count: u32,
    pub films_count: u32,
    pub game_variants_count: u32,
    pub map_variants_count: u32,
    pub new_items_count: u32,
    pub unknown1C: u32,
    pub unknown20: u32,
}
