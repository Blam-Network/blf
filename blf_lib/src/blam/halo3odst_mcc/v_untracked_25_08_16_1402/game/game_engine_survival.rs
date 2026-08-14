use binrw::{BinRead, BinWrite};
#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use blf_lib::bitfield;
use blf_lib::blam::halo3::v12070_08_09_05_2031_halo3_ship::game::game_engine_player_traits::c_player_traits;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::types::array::StaticArray;
use blf_lib_derivable::result::BLFLibResult;
use crate::blam::halo3odst_mcc::v_untracked_25_08_16_1402::game::game_engine_default::c_game_engine_base_variant;

bitfield! {
    #[derive(Serialize, Deserialize)]
    #[cfg_attr(feature = "napi", napi(object, namespace = "halo3odst_mcc_v_untracked_25_08_16_1402"))]
    pub struct e_game_skulls: u64 {
        // primary (0–8)
        iron,
        black_eye,
        tough_luck,
        catch,
        fog,
        famine,
        thunderstorm,
        tilt,
        mythic,
        // secondary (9–15)
        assassin,
        blind,
        superman,
        grunt_birthday_party,
        iwhbyd,
        third_person,
        directors_cut,
        // custom (16–21)
        custom_red,
        custom_yellow,
        custom_blue,
        custom_green,
        custom_white,
        custom_black,
        // mcc (22–45)
        anger,
        bandanna,
        bonded_pair,
        boom,
        envy,
        eye_patch,
        feather,
        foreign,
        ghost,
        grunt_funeral,
        jacked,
        malfunction,
        masterblaster,
        pinata,
        prophet_birthday_party,
        recession,
        scarab,
        so_angry,
        sputnik,
        streaking,
        swarm,
        thats_just_wrong,
        they_come_back,
        boots_off_the_ground,
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3odst_mcc_v_untracked_25_08_16_1402"))]
pub struct s_survival_wave_properties {
    pub m_wave_flags: u8,
    pub m_wave_squad_advance_type: u8,
    pub m_wave_squad_count: i8,
    pub m_squads: StaticArray<i8, 5>,
}

impl s_survival_wave_properties {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_wave_flags as u32, 8)?;
        bitstream.write_integer(self.m_wave_squad_advance_type as u32, 1)?;
        bitstream.write_signed_integer(self.m_wave_squad_count as i32, 8)?;
        for i in 0..5 {
            let squad = self.m_squads[i];
            let unused = squad == -1;
            bitstream.write_bool(unused)?;
            if !unused {
                bitstream.write_integer(squad as u32, 7)?;
            }
        }
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_wave_flags = bitstream.read_integer("wave_flags", 8)?;
        self.m_wave_squad_advance_type = bitstream.read_integer("wave_squad_advance_type", 1)?;
        self.m_wave_squad_count = bitstream.read_signed_integer("wave-squad-count", 8)?;
        for i in 0..5 {
            let unused = bitstream.read_unnamed_bool()?;
            self.m_squads[i] = if unused {
                -1
            } else {
                bitstream.read_integer::<u8>("possible-wave-squads", 7)? as i8
            };
        }
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3odst_mcc_v_untracked_25_08_16_1402"))]
pub struct s_survival_round_properties {
    pub m_skulls: e_game_skulls,
    pub m_initial_wave: s_survival_wave_properties,
    pub m_primary_wave: s_survival_wave_properties,
    pub m_boss_wave: s_survival_wave_properties,
}

impl s_survival_round_properties {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_qword(self.m_skulls.to_raw(), 64)?;
        self.m_initial_wave.encode(bitstream)?;
        self.m_primary_wave.encode(bitstream)?;
        self.m_boss_wave.encode(bitstream)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_skulls = e_game_skulls::from_raw(bitstream.read_qword(64)?);
        self.m_initial_wave.decode(bitstream)?;
        self.m_primary_wave.decode(bitstream)?;
        self.m_boss_wave.decode(bitstream)?;
        Ok(())
    }
}

/// ODST firefight / survival variant — atlas `sub_1406760F0` / `sub_140675F00`.
///
/// Packed layout after the 4-bit engine index:
/// `encoding-version` (8) → base variant → survival fields.
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3odst_mcc_v_untracked_25_08_16_1402"))]
pub struct c_game_engine_survival_variant {
    pub m_encoding_version: u8,
    pub m_base_variant: c_game_engine_base_variant,
    pub m_flags: u16,
    pub m_maximum_lives: i8,
    pub m_set_count: u8,
    pub m_shared_team_life_count: i8,
    pub m_initial_skulls: e_game_skulls,
    pub m_player_traits: c_player_traits,
    pub m_rounds: StaticArray<s_survival_round_properties, 3>,
    pub m_tier_skulls: StaticArray<e_game_skulls, 4>,
    pub m_bonus_duration_seconds: i16,
    pub m_bonus_skulls: e_game_skulls,
    pub m_bonus_wave: s_survival_wave_properties,
}

impl c_game_engine_survival_variant {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_encoding_version as u32, 8)?;
        self.m_base_variant.encode(bitstream)?;
        bitstream.write_integer(self.m_flags as u32, 16)?;
        bitstream.write_signed_integer(self.m_maximum_lives as i32, 8)?;
        bitstream.write_integer(self.m_set_count as u32, 8)?;
        bitstream.write_signed_integer(self.m_shared_team_life_count as i32, 8)?;
        bitstream.write_qword(self.m_initial_skulls.to_raw(), 64)?;
        self.m_player_traits.encode(bitstream)?;

        if self.m_encoding_version >= 2 {
            for i in 0..3 {
                self.m_rounds[i].encode(bitstream)?;
            }
            for i in 0..4 {
                bitstream.write_qword(self.m_tier_skulls[i].to_raw(), 64)?;
            }
            bitstream.write_signed_integer(self.m_bonus_duration_seconds as i32, 16)?;
            bitstream.write_qword(self.m_bonus_skulls.to_raw(), 64)?;
            self.m_bonus_wave.encode(bitstream)?;
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_encoding_version = bitstream.read_integer("encoding-version", 8)?;
        self.m_base_variant.decode(bitstream)?;
        self.m_flags = bitstream.read_integer("flags", 16)?;
        self.m_maximum_lives = bitstream.read_signed_integer("maximum-lives", 8)?;
        self.m_set_count = bitstream.read_integer("set-count", 8)?;
        self.m_shared_team_life_count =
            bitstream.read_signed_integer("shared-team-life-count", 8)?;
        self.m_initial_skulls = e_game_skulls::from_raw(bitstream.read_qword(64)?);
        self.m_player_traits.decode(bitstream)?;

        if self.m_encoding_version >= 2 {
            for i in 0..3 {
                self.m_rounds[i].decode(bitstream)?;
            }
            for i in 0..4 {
                self.m_tier_skulls[i] = e_game_skulls::from_raw(bitstream.read_qword(64)?);
            }
            self.m_bonus_duration_seconds =
                bitstream.read_signed_integer("duration-seconds", 16)?;
            self.m_bonus_skulls = e_game_skulls::from_raw(bitstream.read_qword(64)?);
            self.m_bonus_wave.decode(bitstream)?;
        }

        Ok(())
    }
}
