import { type c_bitstream_reader, c_bitstream_writer } from "../../../../../bitstream";

export class s_game_engine_player_rating_parameters {
  m_rating_scale = 0;
  m_kill_weight = 0;
  m_assist_weight = 0;
  m_betrayal_weight = 0;
  m_death_weight = 0;
  m_normalize_by_max_kills = 0;
  m_base = 0;
  m_range = 0;
  m_loss_scalar = 0;
  m_custom_stat_0 = 0;
  m_custom_stat_1 = 0;
  m_custom_stat_2 = 0;
  m_custom_stat_3 = 0;
  m_expansion_0 = 0;
  m_expansion_1 = 0;
  m_show_in_scoreboard = false;

  decode(bitstream: c_bitstream_reader): void {
    this.m_rating_scale = bitstream.read_float("player-rating-parameter", 32);
    this.m_kill_weight = bitstream.read_float("player-rating-parameter", 32);
    this.m_assist_weight = bitstream.read_float("player-rating-parameter", 32);
    this.m_betrayal_weight = bitstream.read_float("player-rating-parameter", 32);
    this.m_death_weight = bitstream.read_float("player-rating-parameter", 32);
    this.m_normalize_by_max_kills = bitstream.read_float(
      "player-rating-parameter",
      32,
    );
    this.m_base = bitstream.read_float("player-rating-parameter", 32);
    this.m_range = bitstream.read_float("player-rating-parameter", 32);
    this.m_loss_scalar = bitstream.read_float("player-rating-parameter", 32);
    this.m_custom_stat_0 = bitstream.read_float("player-rating-parameter", 32);
    this.m_custom_stat_1 = bitstream.read_float("player-rating-parameter", 32);
    this.m_custom_stat_2 = bitstream.read_float("player-rating-parameter", 32);
    this.m_custom_stat_3 = bitstream.read_float("player-rating-parameter", 32);
    this.m_expansion_0 = bitstream.read_float("player-rating-parameter", 32);
    this.m_expansion_1 = bitstream.read_float("player-rating-parameter", 32);
    this.m_show_in_scoreboard = bitstream.read_bool("flags");
  }

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_float(this.m_rating_scale, 32);
            bitstream.write_float(this.m_kill_weight, 32);
            bitstream.write_float(this.m_assist_weight, 32);
            bitstream.write_float(this.m_betrayal_weight, 32);
            bitstream.write_float(this.m_death_weight, 32);
            bitstream.write_float(this.m_normalize_by_max_kills, 32);
            bitstream.write_float(this.m_base, 32);
            bitstream.write_float(this.m_range, 32);
            bitstream.write_float(this.m_loss_scalar, 32);
            bitstream.write_float(this.m_custom_stat_0, 32);
            bitstream.write_float(this.m_custom_stat_1, 32);
            bitstream.write_float(this.m_custom_stat_2, 32);
            bitstream.write_float(this.m_custom_stat_3, 32);
            bitstream.write_float(this.m_expansion_0, 32);
            bitstream.write_float(this.m_expansion_1, 32);
            bitstream.write_bool(this.m_show_in_scoreboard);
  }
}
