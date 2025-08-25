use std::io::{Cursor, Read, Seek, Write};
use binrw::{binrw, BinRead, BinReaderExt, BinResult, BinWrite, Endian};
use serde::{Deserialize, Serialize};
use blf_lib::BINRW_RESULT;
use blf_lib::types::array::StaticArray;
use blf_lib::types::c_string::StaticString;
use blf_lib::types::numbers::Float32;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
use crate::types::c_string::StaticWcharString;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("netc", 245.1)]
#[brw(big)]
pub struct s_blf_chunk_network_configuration
{
    pub config: s_network_configuration,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_network_file_download_configuration {
    pub master_catalog_file_download_interval_msec: i32,
    pub required_file_invalidation_check_interval_msec: i32,
    pub required_file_download_retry_interval_msec: i32,
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
pub struct s_simulation_zoom_relevance {
    pub zoom_0_tolerance: Float32,
    pub zoom_1_tolerance: Float32,
    pub zoom_0_relevance_bonus: Float32,
    pub zoom_1_relevance_bonus: Float32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_simulation_control_relevance {
    pub zero_relevance_distance: Float32,
    pub max_relevance: Float32,
    pub min_relevance: Float32,
    pub min_period: i32,
    pub max_period: i32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_simulation_position_relevance {
    pub distance_to_player_threshold: Float32,
    pub aiming_vector_high_tolerance: Float32,
    pub aiming_vector_medium_tolerance: Float32,
    pub distance_to_player_medium_tolerance: Float32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_simulation_netdebug_configuration {
    pub bar_maximum_count: i32,
    pub axis_bounds: StaticArray<StaticArray<i32, 2>, 4>
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_simulation_view_configuration {
    pub game_results_update_interval_msec: i32,
    pub synchronous_client_block_duration_msec: i32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_simulation_shared_configuration {
    pub action_persist_time: Float32,
    pub simulation_event_projectile_supercombine_request_fraction: Float32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_simulation_world_configuration {
    pub maximum_catchup_views: i32,
    pub join_timeout: i32,
    pub host_join_minimum_wait_time: i32,
    pub host_join_timeout: i32,
    pub join_total_wait_timeout: i32,
    pub pause_game_required_machines_fraction: Float32,
    pub join_activation_blocking_machines_fraction: Float32,
    pub maximum_catchup_attempts: i32,
    pub catchup_failure_timeout: i32,
    pub client_join_failure_count: i32,
    pub client_activation_failure_timeout: i32,
    pub game_simulation_queue_danger_allocation_size_percentage: Float32,
    pub game_simulation_queue_danger_allocation_count_percentage: Float32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_simulation_event_configuration {
    pub unknown1: i32,
    pub constant_priority: Float32,
    pub cancel_timer_milliseconds: i32,
    pub zero_relevance_distance: Float32,
    pub minimum_priority: Float32,
    pub maximum_priority: Float32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_simulation_entity_creation_configuration {
    pub unknown1: i32,
    pub constant_priority: Float32,
    pub creation_zero_relevance_distance: Float32,
    pub creation_minimum_priority: Float32,
    pub creation_maximum_priority: Float32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_simulation_entity_update_configuration {
    pub constant_priority: Float32,
    pub zero_relevance_distance: Float32,
    pub minimum_relevance: Float32,
    pub maximum_relevance: Float32,
    pub minimum_period: i32,
    pub maximum_period: i32,
    pub normal_minimum_priority: Float32,
    pub normal_maximum_priority: Float32,
    pub delayed_minimum_priority: Float32,
    pub delayed_maximum_priority: Float32,
    pub delayed_time_threshold: i32, // Low confidence - moved since h3
    pub maximum_priority: Float32,
    pub player_priority: Float32,
    pub dead_priority: Float32,
    pub in_motion_by_unit: Float32,
    pub unknown1: i32,
    pub unknown2: i32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_simulation_entity_configuration {
    pub creation_configuration: s_simulation_entity_creation_configuration,
    pub update_configuration: s_simulation_entity_update_configuration,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_simulation_warping_configuration {
    pub simulation_position_update_object_corrective_accept_tolerance: Float32,
    pub simulation_position_update_object_predicted_accept_tolerance: Float32,
    pub simulation_position_update_vehicle_corrective_accept_tolerance: Float32,
    pub simulation_position_update_vehicle_predicted_accept_tolerance: Float32,
    pub position_update_recent_seconds: Float32,
    pub position_update_minimum_distance: Float32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_simulation_weapon_configuration {
    pub trigger_recent_spew_time: Float32,
    pub prediction_delay_timer: Float32,
    pub predicted_fire_allow_ratio: Float32,
    pub predicted_fire_always_allow_threshold: Float32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_simulation_configuration {
    // pub zoom_relevance: s_simulation_zoom_relevance,
    // pub control_relevance: s_simulation_control_relevance,
    // pub position_relevance: s_simulation_position_relevance,
    // pub netdebug: s_simulation_netdebug_configuration,
    // pub view: s_simulation_view_configuration,
    // pub shared: s_simulation_shared_configuration,
    // pub world: s_simulation_world_configuration,
    pub events: StaticArray<s_simulation_event_configuration, 52>,
    pub entities: StaticArray<s_simulation_entity_configuration, 16>,
    // pub warping: s_simulation_warping_configuration,
    // pub weapon: s_simulation_weapon_configuration,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(big)]
pub struct s_network_configuration {
    pub config_download: s_network_file_download_configuration,
    pub unknown_data: StaticArray<u8, 2120>,
    // pub bandwidth: s_bandwidth_configuration,
    // pub life_cycle: s_life_cycle_configuration,
    // pub logic: s_logic_configuration,
    // pub banhammer: s_banhammer_configuration,
    pub simulation: s_simulation_configuration,
    pub unknown_data2: StaticArray<u8, 4002>,
    // pub replication: s_replication_configuration,
    // pub session: s_session_configuration,
    // pub observer: s_observer_configuration,
    // pub delivery: s_delivery_configuration,
    // pub transport: s_transport_configuration,
    // pub voice: s_voice_configuration,
    // pub data_mine: s_data_mine_configuration,
    // pub griefer_config: s_griefer_configuration,
    // pub memory: s_network_memory_configuration,
    // pub user_interface: s_user_interface,
    // pub skill_level_configuration: s_skill_level_configuration,
    // pub experience_configuration: s_experience_configuration,
    // pub hopper_experience_configuration: s_experience_configuration,
    // pub alpha_configuration: s_alpha_configuration,
    // pub crash_handling_configuration: s_crash_handling_configuration,
    // pub lsp_configuration: s_lsp_configuration,
    // pub map_configuration: s_map_configuration,
    pub unknown_domains: StaticArray<StaticWcharString<64>, 4>,
    pub dlc_paths: s_dlc_paths,
    // pub chicken_switches: s_chicken_switches,
    // pub determinism_configuration: s_determinism_configuration,
}


impl BlfChunkHooks for s_blf_chunk_network_configuration {}
