use std::io::{Cursor, Read, Seek, Write};
use binrw::{binrw, BinRead, BinReaderExt, BinResult, BinWrite, Endian};
use serde::{Deserialize, Serialize};
use blf_lib::BINRW_RESULT;
use blf_lib::types::array::StaticArray;
use blf_lib::types::c_string::StaticString;
use blf_lib::types::numbers::Float32;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::{BlfChunk, TestSize};
use crate::types::c_string::StaticWcharString;
use serde_hex::{SerHex,StrictCapPfx};
use blf_lib::types::bool::Bool;
use crate::blf::versions::haloreach::v11860_10_07_24_0147_omaha_release::{e_map_status, s_active_roster_configuration, s_alpha_configuration, s_bandwidth_configuration, s_banhammer_configuration, s_chicken_switches, s_crash_handling_configuration, s_data_mine_configuration, s_delivery_configuration, s_determinism_configuration, s_experience_and_credits_configuration, s_griefer_configuration, s_life_cycle_configuration, s_logic_configuration, s_lsp_configuration, s_network_file_download_configuration, s_network_files_configuration, s_network_memory_configuration, s_network_status_configuration, s_observer_configuration, s_replication_configuration, s_session_configuration, s_simulation_configuration, s_transport_configuration, s_user_interface, s_voice_configuration};

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Size(0x25B0)]
#[Header("netc", 245.1)]
#[brw(big)]
pub struct s_blf_chunk_network_configuration
{
    pub config: s_network_configuration,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(big, repr = u32)]
pub enum e_dlc_pack {
    #[default]
    dlc_pack_none = 0,
    dlc_pack_1 = 1,
    dlc_pack_2 = 2,
    dlc_pack_3 = 3,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_map_information {
    pub map_id: i32,
    pub map_status: e_map_status,
    pub dlc_path_index: i32,
    pub dlc_pack: e_dlc_pack,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_map_configuration {
    pub map_list: StaticArray<s_map_information, 64>,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct s_dlc_path {
    pub path: StaticString<43>,
}


impl BinRead for s_dlc_path {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: binrw::Endian, _args: Self::Args<'_>) -> BinResult<Self> {
        // Read the `StaticString` field, ignoring the first bit
        let mut buffer = vec![0u8; 43];
        reader.read_exact(&mut buffer)?;

        // Ignore the first bit of the first byte
        buffer[0] &= 0b0111_1111;
        let mut cursor = Cursor::new(buffer);

        // Convert the buffer to a StaticString
        let path: StaticString<43> = cursor.read_type(endian)?;

        Ok(s_dlc_path { path })
    }
}

impl BinWrite for s_dlc_path {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(&self, writer: &mut W, endian: binrw::Endian, _args: Self::Args<'_>) -> BinResult<()> {
        // Convert the StaticString to bytes
        let string: String = BINRW_RESULT!(self.path.get_string())?;
        let mut buffer = string.into_bytes();
        buffer.resize(43, 0);

        // Ensure the first bit is set in the first byte
        buffer[0] |= 0b1000_0000;

        // Write the buffer to the output
        writer.write_all(buffer.as_slice())?;
        Ok(())
    }
}


#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_dlc_paths {
    pub paths: StaticArray<s_dlc_path, 8>,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_network_configuration {
    pub config_download: s_network_file_download_configuration,
    pub bandwidth: s_bandwidth_configuration,
    pub life_cycle: s_life_cycle_configuration,
    pub logic: s_logic_configuration,
    pub banhammer: s_banhammer_configuration,
    pub simulation: s_simulation_configuration,
    pub replication: s_replication_configuration,
    pub session: s_session_configuration,
    pub observer: s_observer_configuration,
    pub delivery: s_delivery_configuration,
    pub transport: s_transport_configuration,
    pub voice: s_voice_configuration,
    pub unknown17D4: f32,
    pub unknown17DC: f32,
    pub unknown17E0: f32,
    pub unknown17E4: f32,
    pub unknown17E8: f32,
    pub unknown17EC: f32,
    pub data_mine: s_data_mine_configuration,
    pub griefer_config: s_griefer_configuration,
    pub memory: s_network_memory_configuration,
    pub user_interface: s_user_interface,
    pub alpha_configuration: s_alpha_configuration,
    pub crash_handling_configuration: s_crash_handling_configuration,
    pub lsp_configuration: s_lsp_configuration,
    pub map_configuration: s_map_configuration,
    pub chicken_switches: s_chicken_switches,
    pub unknown1b08: i32,
    pub unknown1b0c: i32,
    pub active_roster_configuration: s_active_roster_configuration,
    pub unknown1b8c: i32, // Added in TU 1
    pub unknown1b80_lsp_leaderboard_time: i32,
    pub unknown1b84_lsp_leaderboard_time: i32,
    pub unknown1b88_lsp_leaderboard_time: i32,
    pub determinism_configuration: s_determinism_configuration,
    pub network_files_configuration: s_network_files_configuration,
    pub network_status_configuration: s_network_status_configuration,
    pub experience_and_credits_configuration: s_experience_and_credits_configuration,
    #[brw(pad_after = 1)]
    pub allow_bungie_pro_file_share_without_gold: Bool,
    pub router_url: StaticWcharString<64>,
    pub arena_url: StaticWcharString<64>,
    pub invasion_url: StaticWcharString<64>,
    #[brw(pad_after = 2)]
    pub network_details_url: StaticWcharString<64>,
    pub dlc_paths: s_dlc_paths,
}


impl BlfChunkHooks for s_blf_chunk_network_configuration {}
