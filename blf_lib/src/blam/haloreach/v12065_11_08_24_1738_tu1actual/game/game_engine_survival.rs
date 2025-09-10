use serde::{Deserialize, Serialize};
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::game_engine_default::c_game_engine_base_variant;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::game_engine_traits::{c_game_engine_respawn_options};
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::types::array::StaticArray;
use blf_lib_derivable::result::BLFLibResult;
use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::game_engine_player_traits::c_player_traits;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_game_engine_survival_variant {
    pub m_base_variant: c_game_engine_base_variant,
    pub m_variant_flags: u8, // 5 bits
    pub m_campaign_difficulty_level: u8, // 3 bits
    pub m_set_count: u8, // 8 bits
    pub m_bonus_lives_awarded: u8, // 4 bits
    pub m_bonus_target: u16, // 15 bits
    pub m_bonus_lives_on_elite_player_death: u16, // 15 bits
    pub m_shared_team_life_count: u8, // 7 bits
    pub m_elite_life_count: u8, // 7 bits
    pub m_extra_life_score_target: u16, // 15 bits
    pub m_maximum_lives: u8, // 7 bits
    pub m_generator_count: u8, // 2 bits
    pub m_spartan_traits: c_player_traits,
    pub m_elite_traits: c_player_traits,
    pub m_ai_traits: c_ai_traits,
    pub m_red_skull: s_custom_skull,
    pub m_blue_skull: s_custom_skull,
    pub m_yellow_skull: s_custom_skull,
    pub m_elite_respawn_options: c_game_engine_respawn_options,
    pub m_round_1_properties: s_survival_round_properties,
    pub m_round_2_properties: s_survival_round_properties,
    pub m_round_3_properties: s_survival_round_properties,
    pub m_bonus_round_duration: u16, // 12 bits
    pub m_bonus_round_skull_flags: u32, // 18 bits
    pub m_bonus_round_properties: s_survival_wave_properties,
}

impl c_game_engine_survival_variant {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_base_variant.encode(bitstream)?;
        bitstream.write_integer(self.m_variant_flags, 5)?;
        bitstream.write_integer(self.m_campaign_difficulty_level, 3)?;
        bitstream.write_integer(self.m_set_count, 8)?;
        bitstream.write_integer(self.m_bonus_lives_awarded, 4)?;
        bitstream.write_integer(self.m_bonus_target, 15)?;
        bitstream.write_integer(self.m_bonus_lives_on_elite_player_death, 15)?;
        bitstream.write_integer(self.m_shared_team_life_count, 7)?;
        bitstream.write_integer(self.m_elite_life_count, 7)?;
        bitstream.write_integer(self.m_extra_life_score_target, 15)?;
        bitstream.write_integer(self.m_maximum_lives, 7)?;
        bitstream.write_integer(self.m_generator_count, 2)?;
        self.m_spartan_traits.encode(bitstream)?;
        self.m_elite_traits.encode(bitstream)?;
        self.m_ai_traits.encode(bitstream)?;
        self.m_red_skull.encode(bitstream)?;
        self.m_blue_skull.encode(bitstream)?;
        self.m_yellow_skull.encode(bitstream)?;
        self.m_elite_respawn_options.encode(bitstream)?;
        self.m_round_1_properties.encode(bitstream)?;
        self.m_round_2_properties.encode(bitstream)?;
        self.m_round_3_properties.encode(bitstream)?;
        bitstream.write_integer(self.m_bonus_round_duration, 12)?;
        bitstream.write_integer(self.m_bonus_round_skull_flags, 18)?;
        self.m_bonus_round_properties.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_base_variant.decode(bitstream)?;
        self.m_variant_flags = bitstream.read_integer("m_variant_flags", 5)?;
        self.m_campaign_difficulty_level = bitstream.read_integer("campaign-difficulty-level", 3)?;
        self.m_set_count = bitstream.read_integer("set-count", 8)?;
        self.m_bonus_lives_awarded = bitstream.read_integer("bonus-lives-awarded", 4)?;
        self.m_bonus_target = bitstream.read_integer("bonus-target", 15)?;
        self.m_bonus_lives_on_elite_player_death = bitstream.read_integer("bonus-lives-on-elite-player-death", 15)?;
        self.m_shared_team_life_count = bitstream.read_integer("shared-team-life-count", 7)?;
        self.m_elite_life_count = bitstream.read_integer("elite-life-count", 7)?;
        self.m_extra_life_score_target = bitstream.read_integer("extra-life-score-target", 15)?;
        self.m_maximum_lives = bitstream.read_integer("maximum-lives", 7)?;
        self.m_generator_count = bitstream.read_integer("generator-count", 2)?;
        self.m_spartan_traits.decode(bitstream)?;
        self.m_elite_traits.decode(bitstream)?;
        self.m_ai_traits.decode(bitstream)?;
        self.m_red_skull.decode(bitstream)?;
        self.m_blue_skull.decode(bitstream)?;
        self.m_yellow_skull.decode(bitstream)?;
        self.m_elite_respawn_options.decode(bitstream)?;
        self.m_round_1_properties.decode(bitstream)?;
        self.m_round_2_properties.decode(bitstream)?;
        self.m_round_3_properties.decode(bitstream)?;
        self.m_bonus_round_duration = bitstream.read_integer("duration-seconds", 12)?;
        self.m_bonus_round_skull_flags = bitstream.read_integer("skull-flags", 18)?;
        self.m_bonus_round_properties.decode(bitstream)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_ai_traits {
    pub m_vision: u8, // 3 bits
    pub m_sound: u8, // 2 bits
    pub m_luck: u8, // 3 bits
    pub m_weapon: u8, // 2 bits
    pub m_grenade: u8, // 2 bits
    pub m_equipment_drop_setting: u8, // 2 bits
    pub m_assasination_immunity_setting: u8, // 2 bits
    pub m_headshot_immunity_setting: u8, // 2 bits
    pub m_damage_resistance: u8, // 4 bits
    pub m_damage_modifier: u8, // 4 bits
}

impl c_ai_traits {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_vision, 3)?;
        bitstream.write_integer(self.m_sound, 2)?;
        bitstream.write_integer(self.m_luck, 3)?;
        bitstream.write_integer(self.m_weapon, 2)?;
        bitstream.write_integer(self.m_grenade, 2)?;
        bitstream.write_integer(self.m_equipment_drop_setting, 2)?;
        bitstream.write_integer(self.m_assasination_immunity_setting, 2)?;
        bitstream.write_integer(self.m_headshot_immunity_setting, 2)?;
        bitstream.write_integer(self.m_damage_resistance, 4)?;
        bitstream.write_integer(self.m_damage_modifier, 4)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_vision = bitstream.read_integer("vision", 3)?;
        self.m_sound = bitstream.read_integer("sound", 2)?;
        self.m_luck = bitstream.read_integer("luck", 3)?;
        self.m_weapon = bitstream.read_integer("weapon", 2)?;
        self.m_grenade = bitstream.read_integer("grenade", 2)?;
        self.m_equipment_drop_setting = bitstream.read_integer("equipment-drop-setting", 2)?;
        self.m_assasination_immunity_setting = bitstream.read_integer("assasination-immunity-setting", 2)?;
        self.m_headshot_immunity_setting = bitstream.read_integer("headshot-immunity-setting", 2)?;
        self.m_damage_resistance = bitstream.read_integer("damage-resistance", 4)?;
        self.m_damage_modifier = bitstream.read_integer("damage-modifier", 4)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_survival_wave_properties {
    pub m_wave_flags: u8, // 1 bit
    pub m_wave_squad_advance_type: u8, // 1 bits
    pub m_wave_squad_count: u8, // 4 bits
    pub m_squads: StaticArray<u8, 12>,
}

impl s_survival_wave_properties {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_wave_flags, 1)?;
        bitstream.write_integer(self.m_wave_squad_advance_type, 1)?;
        bitstream.write_integer(self.m_wave_squad_count, 4)?;

        for i in 0..12 {
            bitstream.write_integer(self.m_squads[i], 8)?;
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_wave_flags = bitstream.read_integer("wave_flags", 1)?;
        self.m_wave_squad_advance_type = bitstream.read_integer("wave_squad_advance_type", 1)?;
        self.m_wave_squad_count = bitstream.read_integer("wave-squad-count", 4)?;
        for i in 0..12 {
            self.m_squads[i] = bitstream.read_integer("possible-wave-squad", 8)?;
        }

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_custom_skull {
    pub m_spartan_traits: c_player_traits,
    pub m_elite_traits: c_player_traits,
    pub m_wave_traits: c_ai_traits,
}

impl s_custom_skull {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_spartan_traits.encode(bitstream)?;
        self.m_elite_traits.encode(bitstream)?;
        self.m_wave_traits.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_spartan_traits.decode(bitstream)?;
        self.m_elite_traits.decode(bitstream)?;
        self.m_wave_traits.decode(bitstream)?;

        Ok(())
    }
}

// idk if this is an actual struct in blam
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_survival_round_properties {
    pub m_skull_flags: u32, // 18 bits
    pub m_initial_wave_options: s_survival_wave_properties,
    pub m_main_wave_options: s_survival_wave_properties,
    pub m_boss_wave_options: s_survival_wave_properties,
}

impl s_survival_round_properties {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_skull_flags, 18)?;
        self.m_initial_wave_options.encode(bitstream)?;
        self.m_main_wave_options.encode(bitstream)?;
        self.m_boss_wave_options.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_skull_flags = bitstream.read_integer("skull-flags", 18)?;
        self.m_initial_wave_options.decode(bitstream)?;
        self.m_main_wave_options.decode(bitstream)?;
        self.m_boss_wave_options.decode(bitstream)?;

        Ok(())
    }
}