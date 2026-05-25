use serde::{Deserialize, Serialize};
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_game_engine_player_rating_parameters {
    pub m_rating_scale: f32,
    pub m_kill_weight: f32,
    pub m_assist_weight: f32,
    pub m_betrayal_weight: f32,
    pub m_death_weight: f32,
    pub m_normalize_by_max_kills: f32,
    pub m_base: f32,
    pub m_range: f32,
    pub m_loss_scalar: f32,
    pub m_custom_stat_0: f32,
    pub m_custom_stat_1: f32,
    pub m_custom_stat_2: f32,
    pub m_custom_stat_3: f32,
    pub m_expansion_0: f32,
    pub m_expansion_1: f32,
    pub m_show_in_scoreboard: bool,
}

impl s_game_engine_player_rating_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_float(self.m_rating_scale, 32)?;
        bitstream.write_float(self.m_kill_weight, 32)?;
        bitstream.write_float(self.m_assist_weight, 32)?;
        bitstream.write_float(self.m_betrayal_weight, 32)?;
        bitstream.write_float(self.m_death_weight, 32)?;
        bitstream.write_float(self.m_normalize_by_max_kills, 32)?;
        bitstream.write_float(self.m_base, 32)?;
        bitstream.write_float(self.m_range, 32)?;
        bitstream.write_float(self.m_loss_scalar, 32)?;
        bitstream.write_float(self.m_custom_stat_0, 32)?;
        bitstream.write_float(self.m_custom_stat_1, 32)?;
        bitstream.write_float(self.m_custom_stat_2, 32)?;
        bitstream.write_float(self.m_custom_stat_3, 32)?;
        bitstream.write_float(self.m_expansion_0, 32)?;
        bitstream.write_float(self.m_expansion_1, 32)?;
        bitstream.write_bool(self.m_show_in_scoreboard)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_rating_scale = bitstream.read_float("player-rating-parameter", 32)?;
        self.m_kill_weight = bitstream.read_float("player-rating-parameter", 32)?;
        self.m_assist_weight = bitstream.read_float("player-rating-parameter", 32)?;
        self.m_betrayal_weight = bitstream.read_float("player-rating-parameter", 32)?;
        self.m_death_weight = bitstream.read_float("player-rating-parameter", 32)?;
        self.m_normalize_by_max_kills = bitstream.read_float("player-rating-parameter", 32)?;
        self.m_base = bitstream.read_float("player-rating-parameter", 32)?;
        self.m_range = bitstream.read_float("player-rating-parameter", 32)?;
        self.m_loss_scalar = bitstream.read_float("player-rating-parameter", 32)?;
        self.m_custom_stat_0 = bitstream.read_float("player-rating-parameter", 32)?;
        self.m_custom_stat_1 = bitstream.read_float("player-rating-parameter", 32)?;
        self.m_custom_stat_2 = bitstream.read_float("player-rating-parameter", 32)?;
        self.m_custom_stat_3 = bitstream.read_float("player-rating-parameter", 32)?;
        self.m_expansion_0 = bitstream.read_float("player-rating-parameter", 32)?;
        self.m_expansion_1 = bitstream.read_float("player-rating-parameter", 32)?;
        self.m_show_in_scoreboard = bitstream.read_bool("flags")?;

        Ok(())
    }
}