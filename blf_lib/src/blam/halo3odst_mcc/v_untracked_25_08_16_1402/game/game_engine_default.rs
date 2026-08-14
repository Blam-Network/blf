use binrw::{BinRead, BinWrite};
#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use blf_lib::blam::halo3::v12070_08_09_05_2031_halo3_ship::game::game_engine_default::{
    c_game_engine_map_override_options, c_game_engine_social_options,
};
use blf_lib::blam::halo3::v12070_08_09_05_2031_halo3_ship::game::game_engine_traits::{
    c_game_engine_miscellaneous_options, c_game_engine_respawn_options,
};
use crate::blam::halo3odst_mcc::v_untracked_25_08_16_1402::saved_games::saved_game_files::s_content_item_metadata;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;

pub const k_game_engine_type_count: usize = 12;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3odst_mcc_v_untracked_25_08_16_1402"))]
pub struct c_game_engine_base_variant {
    #[serde(skip_serializing, skip_deserializing)]
    pub m_checksum: u32,
    #[brw(pad_before = 4)]
    pub m_metadata: s_content_item_metadata,
    pub m_miscellaneous_options: c_game_engine_miscellaneous_options,
    pub m_respawn_options: c_game_engine_respawn_options,
    pub m_social_options: c_game_engine_social_options,
    pub m_map_override_options: c_game_engine_map_override_options,
    #[brw(pad_before = 2)]
    pub m_flags: u16,
    #[brw(pad_after = 2)]
    pub m_team_scoring_method: u16,
}

impl c_game_engine_base_variant {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_metadata.encode(bitstream)?;
        bitstream.write_integer(self.m_flags as u32, 1)?;
        self.m_miscellaneous_options.encode(bitstream)?;
        self.m_respawn_options.encode(bitstream)?;
        self.m_social_options.encode(bitstream)?;
        self.m_map_override_options.encode(bitstream)?;
        bitstream.write_integer(self.m_team_scoring_method as u32, 3)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_metadata.decode(bitstream)?;
        self.m_flags = bitstream.read_unnamed_integer(1)?;
        self.m_miscellaneous_options.decode(bitstream)?;
        self.m_respawn_options.decode(bitstream)?;
        self.m_social_options.decode(bitstream)?;
        self.m_map_override_options.decode(bitstream)?;
        self.m_team_scoring_method = bitstream.read_unnamed_integer(3)?;
        Ok(())
    }
}
