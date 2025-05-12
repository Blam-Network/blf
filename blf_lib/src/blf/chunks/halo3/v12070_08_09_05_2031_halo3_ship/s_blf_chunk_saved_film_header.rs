use binrw::binrw;
use serde::{Deserialize, Serialize};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
use crate::blam::halo_3::release::game::game_options::game_options;
use crate::types::array::StaticArray;
use crate::types::bool::Bool;
use crate::types::c_string::StaticString;
use crate::types::time::time32_t;

#[binrw]
#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize,Default)]
#[Header("flmh", 10.1)]
#[brw(big)]
pub struct s_blf_chunk_saved_film_header {
    #[brw(pad_before = 4)]
    pub build_number: StaticString<32>,
    pub executable_type: i32,
    pub network_executable_version: i32,
    pub network_compatible_version: i32,
    pub map_language: i32, // check
    pub map_minor_version: i32,
    pub map_minor_version_is_tracked: Bool,
    #[brw(pad_before=11)]
    pub map_signature_size: i32,
    pub map_signature_bytes: StaticArray<u8, 60>,

    pub is_host_film: Bool,
    pub contains_gamestate: Bool,
    pub is_snippet: Bool,
    #[brw(pad_before=5)]
    pub session_id: StaticArray<u8, 128>,
    pub options: game_options,
    pub recorded_time: time32_t,
    pub length_in_ticks: i32,
    pub snippet_start_tick: i32,
    pub padding_to_align_for_utility_drive: StaticArray<u8, 0x538>, // this is a guess.
}

impl BlfChunkHooks for s_blf_chunk_saved_film_header {

}
