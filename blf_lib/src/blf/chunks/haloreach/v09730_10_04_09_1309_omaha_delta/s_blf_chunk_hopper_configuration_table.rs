use std::io::{Cursor, Read, Seek, Write};
use binrw::{BinRead, BinResult, BinWrite, BinWriterExt, Endian};
use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use serde::{Deserialize, Serialize};
use blf_lib::blam::common::memory::secure_signature::s_network_http_request_hash;
use blf_lib::io::bitstream::{c_bitstream_reader, close_bitstream_writer, create_bitstream_writer, e_bitstream_byte_order};
use blf_lib::types::array::StaticArray;
use crate::types::c_string::StaticString;
use blf_lib::types::time::{filetime};
use serde_hex::{SerHex,StrictCap};
use blf_lib::types::bool::Bool;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::{BlfChunk, TestSize};
use crate::types::numbers::Float32;

pub const k_hopper_maximum_category_count: usize = 16;
pub const k_hopper_maximum_hopper_count: usize = 32; // TODO: Check, this seems low.

#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("mhcf", 25.1)]
pub struct s_blf_chunk_hopper_configuration_table
{
    pub hopper_categories: Vec<s_game_hopper_custom_category>,
    pub hopper_configurations: Vec<c_hopper_configuration>,
}

impl BlfChunkHooks for s_blf_chunk_hopper_configuration_table {}

impl BinRead for s_blf_chunk_hopper_configuration_table {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, args: Self::Args<'_>) -> BinResult<Self> {
        // this chunk in this version is BE
        let endian = Endian::Big;

        // Hopper file is packed AND compressed. First create the bitstream for unpacking.
        let mut packed_buffer = Vec::<u8>::new();
        reader.read_to_end(&mut packed_buffer)?;
        let mut bitstream = c_bitstream_reader::new(packed_buffer.as_slice(), e_bitstream_byte_order::from_binrw_endian(endian));

        // Now decompress.
        let compressed_length= bitstream.read_integer(14)? - 4; // this -4 is necessary, but idk why
        let decompressed_length = bitstream.read_integer(32)?;
        let compressed_hopper_table_data: Vec<u8> = bitstream.read_raw_data((compressed_length * 8) as usize)?;
        let mut decompressed_hopper_table_data: Vec<u8> = Vec::with_capacity(decompressed_length as usize);
        let mut decoder = ZlibDecoder::new(Cursor::new(compressed_hopper_table_data));
        decoder.read_to_end(&mut decompressed_hopper_table_data)?;

        // Read the unpacked, decompressed chunk.
        let mut decompressed_hopper_reader = Cursor::new(decompressed_hopper_table_data);
        let mut hopper_table = Self::default();
        let hopper_configuration_count: u32 = BinRead::read_options(&mut decompressed_hopper_reader, endian, args)?;
        let hopper_category_count: u32 = BinRead::read_options(&mut decompressed_hopper_reader, endian, args)?;

        for i in 0..k_hopper_maximum_category_count {
            let category: s_game_hopper_custom_category = BinRead::read_options(&mut decompressed_hopper_reader, endian, args)?;
            // Seek through unused categories but don't keep them.
            if i >= hopper_category_count as usize { continue };
            hopper_table.hopper_categories.push(category);
        }

        for i in 0..hopper_configuration_count as usize {
            hopper_table.hopper_configurations.push(BinRead::read_options(&mut decompressed_hopper_reader, endian, args)?);
        }

        Ok(hopper_table)
    }
}

impl BinWrite for s_blf_chunk_hopper_configuration_table {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(&self, writer: &mut W, endian: Endian, args: Self::Args<'_>) -> BinResult<()> {
        // 2. Deflate via zlib
        // 3. Write packed chunk

        // this chunk in this version is BE
        let endian = Endian::Big;

        // 1. Encode chunk
        let mut encoded_chunk = Vec::<u8>::new();
        let mut encoded_writer = Cursor::new(&mut encoded_chunk);

        let configurations_count = self.hopper_configurations.len() as u32;
        let categories_count = self.hopper_categories.len() as u32;
        let categories: StaticArray<s_game_hopper_custom_category, k_hopper_maximum_category_count>
            = StaticArray::from_vec(&self.hopper_categories)?;
        let configurations: StaticArray<c_hopper_configuration, k_hopper_maximum_hopper_count>
            = StaticArray::from_vec(&self.hopper_configurations)?;

        configurations_count.write_options(&mut encoded_writer, endian, args)?;
        categories_count.write_options(&mut encoded_writer, endian, args)?;
        categories.write_options(&mut encoded_writer, endian, args)?;
        configurations.write_options(&mut encoded_writer, endian, args)?;

        // 2. Deflate
        let mut e = ZlibEncoder::new(Vec::new(), Compression::new(9));
        e.write_all(encoded_chunk.as_slice())?;
        let compressed_data = e.finish()?;

        // 3. Pack
        let compressed_length: u16 = compressed_data.len() as u16;
        let uncompressed_length: u32 = encoded_chunk.len() as u32;
        // TODO: allow the writer to grow if it runs out of space.
        let mut packed_writer = create_bitstream_writer(0x8A48, e_bitstream_byte_order::from_binrw_endian(endian));
        packed_writer.write_integer((compressed_length + 4) as u32, 14)?;
        packed_writer.write_integer(uncompressed_length, 32)?;
        packed_writer.write_raw_data(&compressed_data, (compressed_length * 8) as usize)?;
        writer.write_ne(&close_bitstream_writer(&mut packed_writer)?)?;

        Ok(())
    }
}


#[derive(Clone, Default, PartialEq, Debug, Copy, Serialize, Deserialize, BinRead, BinWrite)]
pub struct s_game_hopper_custom_category {
    pub category_identifier: u16,
    pub category_name: StaticString<32>,
    pub unknown1: u16,
    pub unknown2: StaticString<32>,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
pub struct s_hopper_query_latency_desirability_configuration {
    pub unknown1: u32,
    pub unknown2: u32,
    pub unknown3: u32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
pub struct s_hopper_query_configuration {
    pub dword0: u32,
    pub gap4: u32,
    pub dword8: u32,
    pub gapc: u32,
    pub dword10: u32,
    pub gap14: u32,
    pub dword18: u32,
    pub dword1c: u32,
    pub dword20: u32,
    pub gap24: u32,
    pub dword28: u32,
    pub dword2c: u32,
    pub dword30: u32,
    pub unknown1: u32,
    pub latency_desirability_configurations: StaticArray<s_hopper_query_latency_desirability_configuration, 2>,
    pub unknown2: StaticArray<Float32, 17>,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
pub struct s_hopper_configuration_per_team_data {
    pub minimum_team_size: u32,
    pub maximum_team_size: u32,
    pub team_model_override: u32,
    pub team_allegiance: u32,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, TestSize)]
pub struct c_hopper_configuration {
    pub hopper_name: StaticString<32>,
    #[serde(skip_serializing,skip_deserializing)]
    pub game_set_hash: s_network_http_request_hash,
    pub identifier: u16,
    pub category_identifier: u16,
    pub category_index: u8,
    #[brw(pad_after = 2)]
    pub player_investment_category: u8,
    pub image_index: u32,
    pub xlast_index: u32,
    #[brw(pad_after = 3)]
    pub equivalency_id: u8, // this might be the wrong type.
    pub start_time: filetime,
    pub end_time: filetime,
    pub minimum_games_won: u32,
    pub maximum_games_won: u32,
    pub minimum_games_played: u32,
    pub maximum_games_played: u32,
    pub minimum_grade: u32,
    pub maximum_grade: u32,
    pub min_party_size: u32,
    pub max_party_size: u32,
    pub min_local_players: u32,
    pub max_local_players: u32,
    pub hopper_access_bit: u32,
    pub account_type_access: u32,
    pub require_all_party_members_meet_games_played_requirements: Bool,
    pub byte89: u8,
    pub require_all_party_members_meet_grade_requirements: Bool,
    pub require_all_party_members_meet_access_requirements: Bool,
    pub require_all_party_members_meet_live_account_access_requirements: Bool,
    pub hide_hopper_from_games_played_restricted_players: Bool,
    pub byte8e: u8,
    pub hide_hopper_from_grade_restricted_players: Bool,
    pub hide_hopper_from_access_restricted_players: Bool,
    pub hide_hopper_from_live_account_access_restricted_players: Bool,
    pub hide_hopper_due_to_time_restriction: Bool,
    pub requires_hard_drive: Bool,
    #[brw(pad_after = 3)]
    pub requires_local_party: Bool,
    pub dword98: u32,
    pub dword9c: u32,
    pub dworda0: u32,
    pub dworda4: u32,
    pub dworda8: u32,
    pub dwordac: u32,
    pub dwordb0: u32, // originally had this as a gap, but it does hold data. Might be noise.
    pub is_ranked: u8,
    pub is_arbitrated: u8,
    pub are_guests_allowed: u8,
    pub are_opponents_visible: u8,
    #[brw(pad_after = 3)]
    pub uses_arena_lsp_stats: u8,
    pub dwordbc: u32,
    pub dwordc0: u32,
    pub gapc4: u8, // unsure
    #[brw(pad_after = 2)]
    pub uses_high_score_leaderboard: u8,
    pub posse_formation: u32,
    pub post_match_countdown_time_seconds: u32,
    pub require_hosts_on_multiple_teams: u32, // definately a u32?
    pub repeated_opponents_to_consider_for_penalty: u32,
    pub repeated_opponents_skill_throttle_start: u32,
    pub repeated_opponents_skill_throttle_stop: u32,
    pub is_team_matching_enabled: u32,
    pub gather_start_threshold_seconds: u32,
    pub get_gather_start_game_early_seconds: u32,
    pub get_gather_give_up_seconds: u32,
    pub chance_of_gathering: StaticArray<u8, 16>,
    pub gapf0_5: u32, // sometimes there is data here, maybe not a gap.
    pub dword104: u32,
    pub dword108: u32,
    pub uses_ffa_scoring_for_leaderboard_writes: u8,
    #[brw(pad_after = 2)]
    pub should_modify_skill_update_weight_with_game_quality: u8,
    pub trueskill_sigma_multiplier: Float32,
    pub dword114: u32,
    pub trueskill_tau_dynamics_factor: u32,
    pub trueskill_draw_probability: u32,
    pub pre_match_voice_configuration: u32,
    pub in_match_voice_configuration: u32,
    pub post_match_voice_configuration: u32,
    pub restrict_open_channel: u32,
    pub dword130: u32,
    pub query_configurations: StaticArray<s_hopper_query_configuration, 4>,
    pub games_game_type: u32,
    pub minimum_player_count: u32,
    pub maximum_player_count: u32,
    pub ffa_model_override: u32,
    pub minimum_team_count: u32,
    pub maximum_team_count: u32,
    pub per_team_data: [s_hopper_configuration_per_team_data; 8],
    pub maximum_team_imbalance: u32,
    pub big_squad_size_threshold: u32,
    pub dword424: u32,
    pub gap428: u32, // unsure
    pub undersized_party_split_permissions: u32,
}