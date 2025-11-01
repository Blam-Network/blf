use binrw::{binrw, BinRead, BinWrite};
use blf_lib::blf::chunks::BlfChunkHooks;
use blf_lib::{bitfield, BlfChunk};
use serde::{Deserialize, Serialize};
use blf_lib::types::array::StaticArray;
#[cfg(feature = "napi")]
use napi_derive::napi;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::player_rewards::e_purchase_state;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::player_rewards::player_commendations::s_persistent_per_commendation_state;
use blf_lib::types::time::time64_t;

#[binrw]
#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize,Default)]
#[Header("rpdl", 2.1)]
#[Size(0x21B)]
#[brw(big)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_blf_chunk_rewards_persistance {
    // TODO: Map
    pub credits: u32,
    pub unknown1: u32, // maybe bonus credits
    pub commendations: StaticArray<s_persistent_per_commendation_state, 128>, // commendation state structs
    pub purchased_items: StaticArray<e_purchase_state, 256>,
    pub unknown2: u16, //hopper id?
    pub unknown3: u32,
    pub unknown4: time64_t, // maybe lsp modified at
    pub awarded_credits: u32,
    pub unknown6: u8
}

impl BlfChunkHooks for s_blf_chunk_rewards_persistance {}

