use std::io::{Cursor, Read, Seek, Write};
use binrw::{BinRead, BinReaderExt, BinResult, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib_derive::TestSize;
use crate::blam::halo3::v12070_08_09_05_2031_halo3_ship::game::game_engine_variant::c_game_variant;
use crate::blam::halo3::v12070_08_09_05_2031_halo3_ship::game::players::{s_player_configuration_from_client, s_player_configuration_from_host};
use crate::blam::halo3::v12070_08_09_05_2031_halo3_ship::saved_games::scenario_map_variant::c_map_variant;
use crate::types::array::StaticArray;
use crate::types::bool::Bool;
use crate::types::string::{StaticString, StaticWcharString};
use crate::types::numbers::Float32;
use crate::types::u64::Unsigned64;
use crate::TEST_BIT;

#[derive(TestSize,BinRead,BinWrite,PartialEq,Debug,Default,Clone,Serialize,Deserialize)]
#[Size(0xF810)]
#[brw(big)]
pub struct game_options {
    pub game_mode: i32,
    pub game_simulation: u8,
    pub game_network_type: u8,
    pub game_tick_rate: u16,
    pub game_instance: Unsigned64,
    pub random_seed: u32,
    pub language: i32,
    pub determinism_version: i32,
    pub campaign_id: i32,
    pub map_id: i32,
    pub scenario_path: s_scenario_path,
    pub initial_zone_set_index: i16,
    pub load_level_only: Bool,
    pub dump_machine_index: u8,
    pub dump_object_log: Bool,
    pub dump_random_seeds: Bool,
    pub playtest_mode: Bool,
    #[brw(pad_before=1)]
    pub game_playback: i16,
    pub record_saved_film: Bool,
    #[brw(pad_before=1)]
    pub playback_start_tick: i32,
    pub playback_length_in_ticks: i32,
    pub campaign_difficulty: i16,
    pub campaign_insertion_point: i16,
    pub campaign_metagame_scoring: i16,
    pub campaign_metagame_enabled: Bool,
    pub campaign_allow_persistent_storage: Bool,
    pub campaign_active_primary_skulls: i32,
    pub campaign_active_secondary_skulls: i32,
    pub campaign_armaments: s_campaign_armaments,
    pub matchmade_game: Bool, // 0x1c4 Correct
    #[brw(pad_before=7)]
    pub game_matchmaking_options: s_game_matchmaking_options,
    pub multiplayer_variant: c_game_variant, // 0x228 correct
    #[brw(pad_before=4)]
    pub map_variant: c_map_variant, // 0x490
    pub machines: game_machine_options, // 0xE520
    #[brw(pad_before=4)]
    pub players: StaticArray<game_player_options, 16>
}


#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(0x78)]
pub struct s_campaign_armaments {
    pub player_armaments: StaticArray<s_campaign_armaments_player, 4>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(0x1E)]
pub struct s_campaign_armaments_player {
    #[brw(pad_after = 1)]
    pub valid: Bool,
    pub primary_weapon: s_campaign_armaments_weapon,
    pub backpack_weapon: s_campaign_armaments_weapon,
    pub secondary_weapon: s_campaign_armaments_weapon,
    pub frag_grenade_count: u8,
    pub plasma_grenade_count: u8,
    pub spike_grenade_count: u8,
    pub fire_grenade_count: u8,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
pub struct s_campaign_armaments_weapon {
    pub damage_reporting_type: u16,
    pub rounds_inventory: u16,
    pub rounds_loaded: u16,
    pub battery: u16,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
pub struct s_machine_identifier {
    pub identifier: StaticArray<u8, 6>
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(0x6C)]
pub struct game_machine_options {
    pub machine_valid_mask: u32,
    pub machine_identifiers: StaticArray<s_machine_identifier, 16>,
    pub local_machine_exists: Bool,
    #[brw(pad_before = 1)]
    pub local_machine_identifier: s_machine_identifier,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(0x128)]
pub struct game_player_options {
    pub valid: Bool,
    pub left_game: Bool,
    pub user_inex: i16,
    pub controller_index: i32,
    pub machine_identifier: s_machine_identifier,
    pub player_identifier: Unsigned64,
    #[brw(pad_before = 2)]
    pub configuration: s_player_configuration,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(0x110)]
pub struct s_player_configuration {
    pub client: s_player_configuration_from_client,
    pub host: s_player_configuration_from_host,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(0x5C)]
pub struct s_game_matchmaking_options {
    pub hopper_identifier: i16,
    pub xlast_index: i8,
    pub hopper_ranked: Bool,
    pub team_game: Bool,
    #[brw(pad_before = 1)]
    pub hopper_name: StaticWcharString<32>,
    #[brw(pad_before = 2)]
    pub draw_probability: i32,
    pub beta: Float32,
    pub tau: Float32,
    pub experience_base_increment: i32,
    pub experience_penalty_decrement: i32,
}

// DLC scenario paths set the top bit of the first byte, which isn't valid UTF-8.
// It's kept in `is_dlc` and cleared from `path`.
#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct s_scenario_path {
    pub is_dlc: bool,
    pub path: StaticString<260>,
}

impl BinRead for s_scenario_path {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: binrw::Endian, _args: Self::Args<'_>) -> BinResult<Self> {
        let mut buffer = vec![0u8; 260];
        reader.read_exact(&mut buffer)?;

        let is_dlc = TEST_BIT!(buffer[0], 7);
        buffer[0] &= 0b0111_1111;

        let path: StaticString<260> = Cursor::new(buffer).read_type(endian)?;

        Ok(s_scenario_path { path, is_dlc })
    }
}

impl BinWrite for s_scenario_path {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(&self, writer: &mut W, endian: binrw::Endian, _args: Self::Args<'_>) -> BinResult<()> {
        let mut buffer = Cursor::new(Vec::with_capacity(260));
        self.path.write_options(&mut buffer, endian, ())?;
        let mut buffer = buffer.into_inner();

        if self.is_dlc {
            buffer[0] |= 0b1000_0000;
        }

        writer.write_all(&buffer)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scenario_path_round_trips_dlc_bit() {
        let mut bytes = vec![0u8; 260];
        let path = b"A625E0D7ED51ADDA217AD5F150C843988717E9BD4D:\warehouse";
        bytes[..path.len()].copy_from_slice(path);
        bytes[0] |= 0b1000_0000;

        let scenario_path: s_scenario_path = Cursor::new(&bytes).read_be().unwrap();
        assert!(scenario_path.is_dlc);
        assert_eq!(scenario_path.path.get_string().unwrap(), "A625E0D7ED51ADDA217AD5F150C843988717E9BD4D:\warehouse");

        let mut written = Cursor::new(Vec::new());
        scenario_path.write_be(&mut written).unwrap();
        assert_eq!(written.into_inner(), bytes);
    }
}
