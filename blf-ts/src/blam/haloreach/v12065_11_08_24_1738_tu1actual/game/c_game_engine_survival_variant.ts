import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../bitstream";
import {
  c_game_engine_base_variant,
  c_game_engine_respawn_options,
} from "./c_game_engine_default";
import { c_player_traits } from "./c_player_traits";
import { e_survival_variant_flags } from "./game_engine_enums";

export class c_ai_traits {
  m_vision = 0;
  m_sound = 0;
  m_luck = 0;
  m_weapon = 0;
  m_grenade = 0;
  m_equipment_drop_setting = 0;
  m_assasination_immunity_setting = 0;
  m_headshot_immunity_setting = 0;
  m_damage_resistance = 0;
  m_damage_modifier = 0;

  decode(bitstream: c_bitstream_reader): void {
    this.m_vision = bitstream.read_integer("vision", 3);
    this.m_sound = bitstream.read_integer("sound", 2);
    this.m_luck = bitstream.read_integer("luck", 3);
    this.m_weapon = bitstream.read_integer("weapon", 2);
    this.m_grenade = bitstream.read_integer("grenade", 2);
    this.m_equipment_drop_setting = bitstream.read_integer(
      "equipment-drop-setting",
      2
    );
    this.m_assasination_immunity_setting = bitstream.read_integer(
      "assasination-immunity-setting",
      2
    );
    this.m_headshot_immunity_setting = bitstream.read_integer(
      "headshot-immunity-setting",
      2
    );
    this.m_damage_resistance = bitstream.read_integer("damage-resistance", 4);
    this.m_damage_modifier = bitstream.read_integer("damage-modifier", 4);
  }

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_vision, 3);
    bitstream.write_integer(this.m_sound, 2);
    bitstream.write_integer(this.m_luck, 3);
    bitstream.write_integer(this.m_weapon, 2);
    bitstream.write_integer(this.m_grenade, 2);
    bitstream.write_integer(this.m_equipment_drop_setting, 2);
    bitstream.write_integer(this.m_assasination_immunity_setting, 2);
    bitstream.write_integer(this.m_headshot_immunity_setting, 2);
    bitstream.write_integer(this.m_damage_resistance, 4);
    bitstream.write_integer(this.m_damage_modifier, 4);
  }
}

export class s_survival_wave_properties {
  m_wave_flags = 0;
  m_wave_squad_advance_type = 0;
  m_wave_squad_count = 0;
  m_squads: number[] = Array.from({ length: 12 }, () => 0);

  decode(bitstream: c_bitstream_reader): void {
    this.m_wave_flags = bitstream.read_integer("wave_flags", 1);
    this.m_wave_squad_advance_type = bitstream.read_integer(
      "wave_squad_advance_type",
      1
    );
    this.m_wave_squad_count = bitstream.read_integer("wave-squad-count", 4);
    for (let i = 0; i < 12; i++) {
      this.m_squads[i] = bitstream.read_integer("possible-wave-squad", 8);
    }
  }

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_wave_flags, 1);
    bitstream.write_integer(this.m_wave_squad_advance_type, 1);
    bitstream.write_integer(this.m_wave_squad_count, 4);
    for (let i = 0; i < 12; i++) {
      bitstream.write_integer(this.m_squads[i]!, 8);
    }
  }
}

export class s_custom_skull {
  m_spartan_traits = new c_player_traits();
  m_elite_traits = new c_player_traits();
  m_wave_traits = new c_ai_traits();

  decode(bitstream: c_bitstream_reader): void {
    this.m_spartan_traits.decode(bitstream);
    this.m_elite_traits.decode(bitstream);
    this.m_wave_traits.decode(bitstream);
  }

  encode(bitstream: c_bitstream_writer): void {
    this.m_spartan_traits.encode(bitstream);
    this.m_elite_traits.encode(bitstream);
    this.m_wave_traits.encode(bitstream);
  }
}

export class s_survival_round_properties {
  m_skull_flags = 0;
  m_initial_wave_options = new s_survival_wave_properties();
  m_main_wave_options = new s_survival_wave_properties();
  m_boss_wave_options = new s_survival_wave_properties();

  decode(bitstream: c_bitstream_reader): void {
    this.m_skull_flags = bitstream.read_integer("skull-flags", 18);
    this.m_initial_wave_options.decode(bitstream);
    this.m_main_wave_options.decode(bitstream);
    this.m_boss_wave_options.decode(bitstream);
  }

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_skull_flags, 18);
    this.m_initial_wave_options.encode(bitstream);
    this.m_main_wave_options.encode(bitstream);
    this.m_boss_wave_options.encode(bitstream);
  }
}

export class c_game_engine_survival_variant {
  m_base_variant = new c_game_engine_base_variant();
  m_variant_flags = new e_survival_variant_flags();
  m_campaign_difficulty_level = 0;
  m_set_count = 0;
  m_bonus_lives_awarded = 0;
  m_bonus_target = 0;
  m_bonus_lives_on_elite_player_death = 0;
  m_shared_team_life_count = 0;
  m_elite_life_count = 0;
  m_extra_life_score_target = 0;
  m_maximum_lives = 0;
  m_generator_count = 0;
  m_spartan_traits = new c_player_traits();
  m_elite_traits = new c_player_traits();
  m_ai_traits = new c_ai_traits();
  m_red_skull = new s_custom_skull();
  m_blue_skull = new s_custom_skull();
  m_yellow_skull = new s_custom_skull();
  m_elite_respawn_options = new c_game_engine_respawn_options();
  m_round_1_properties = new s_survival_round_properties();
  m_round_2_properties = new s_survival_round_properties();
  m_round_3_properties = new s_survival_round_properties();
  m_bonus_round_duration = 0;
  m_bonus_round_skull_flags = 0;
  m_bonus_round_properties = new s_survival_wave_properties();

  decode(bitstream: c_bitstream_reader): void {
    this.m_base_variant.decode(bitstream);
    this.m_variant_flags = e_survival_variant_flags.from_raw(
      bitstream.read_integer("m_variant_flags", 5)
    );
    this.m_campaign_difficulty_level = bitstream.read_integer(
      "campaign-difficulty-level",
      3
    );
    this.m_set_count = bitstream.read_integer("set-count", 8);
    this.m_bonus_lives_awarded = bitstream.read_integer(
      "bonus-lives-awarded",
      4
    );
    this.m_bonus_target = bitstream.read_integer("bonus-target", 15);
    this.m_bonus_lives_on_elite_player_death = bitstream.read_integer(
      "bonus-lives-on-elite-player-death",
      15
    );
    this.m_shared_team_life_count = bitstream.read_integer(
      "shared-team-life-count",
      7
    );
    this.m_elite_life_count = bitstream.read_integer("elite-life-count", 7);
    this.m_extra_life_score_target = bitstream.read_integer(
      "extra-life-score-target",
      15
    );
    this.m_maximum_lives = bitstream.read_integer("maximum-lives", 7);
    this.m_generator_count = bitstream.read_integer("generator-count", 2);
    this.m_spartan_traits.decode(bitstream);
    this.m_elite_traits.decode(bitstream);
    this.m_ai_traits.decode(bitstream);
    this.m_red_skull.decode(bitstream);
    this.m_blue_skull.decode(bitstream);
    this.m_yellow_skull.decode(bitstream);
    this.m_elite_respawn_options.decode(bitstream);
    this.m_round_1_properties.decode(bitstream);
    this.m_round_2_properties.decode(bitstream);
    this.m_round_3_properties.decode(bitstream);
    this.m_bonus_round_duration = bitstream.read_integer(
      "duration-seconds",
      12
    );
    this.m_bonus_round_skull_flags = bitstream.read_integer("skull-flags", 18);
    this.m_bonus_round_properties.decode(bitstream);
  }

  encode(bitstream: c_bitstream_writer): void {
    this.m_base_variant.encode(bitstream);
    bitstream.write_integer(this.m_variant_flags.to_raw(), 5);
    bitstream.write_integer(this.m_campaign_difficulty_level, 3);
    bitstream.write_integer(this.m_set_count, 8);
    bitstream.write_integer(this.m_bonus_lives_awarded, 4);
    bitstream.write_integer(this.m_bonus_target, 15);
    bitstream.write_integer(this.m_bonus_lives_on_elite_player_death, 15);
    bitstream.write_integer(this.m_shared_team_life_count, 7);
    bitstream.write_integer(this.m_elite_life_count, 7);
    bitstream.write_integer(this.m_extra_life_score_target, 15);
    bitstream.write_integer(this.m_maximum_lives, 7);
    bitstream.write_integer(this.m_generator_count, 2);
    this.m_spartan_traits.encode(bitstream);
    this.m_elite_traits.encode(bitstream);
    this.m_ai_traits.encode(bitstream);
    this.m_red_skull.encode(bitstream);
    this.m_blue_skull.encode(bitstream);
    this.m_yellow_skull.encode(bitstream);
    this.m_elite_respawn_options.encode(bitstream);
    this.m_round_1_properties.encode(bitstream);
    this.m_round_2_properties.encode(bitstream);
    this.m_round_3_properties.encode(bitstream);
    bitstream.write_integer(this.m_bonus_round_duration, 12);
    bitstream.write_integer(this.m_bonus_round_skull_flags, 18);
    this.m_bonus_round_properties.encode(bitstream);
  }
}
