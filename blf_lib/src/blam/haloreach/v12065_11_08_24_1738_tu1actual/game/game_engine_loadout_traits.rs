use serde::{Deserialize, Serialize};
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::TEST_BIT;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::string_table::c_single_language_string_table;
use blf_lib_derivable::result::BLFLibResult;
use crate::types::array::StaticArray;
use serde_hex::{SerHex,StrictCap};

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_loadout_traits {
    pub m_visible: bool,
    pub m_name: i8,
    pub m_initial_primary_weapon_absolute_index: i8,
    pub m_initial_secondary_weapon_absolute_index: i8,
    pub m_initial_equipment_absolute_index: i8,
    pub m_initial_grenade_count_setting: u8,
}

impl c_loadout_traits {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_bool(self.m_visible)?;
        bitstream.write_index::<128>(self.m_name, 7)?;
        bitstream.write_signed_integer(self.m_initial_primary_weapon_absolute_index, 8)?;
        bitstream.write_signed_integer(self.m_initial_secondary_weapon_absolute_index, 8)?;
        bitstream.write_signed_integer(self.m_initial_equipment_absolute_index, 8)?;
        bitstream.write_integer(self.m_initial_grenade_count_setting, 4)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_visible = bitstream.read_bool("flags")?;
        self.m_name = bitstream.read_index::<128>("name", 7)? as i8;
        self.m_initial_primary_weapon_absolute_index = bitstream.read_signed_integer("initial-primary-weapon", 8)?;
        self.m_initial_secondary_weapon_absolute_index = bitstream.read_signed_integer("initial-secondary-weapon", 8)?;
        self.m_initial_equipment_absolute_index = bitstream.read_signed_integer("initial-equipment", 8)?;
        self.m_initial_grenade_count_setting = bitstream.read_integer("initial-grenade-count", 4)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_loadout_palette_traits {
    pub m_loadouts: StaticArray<c_loadout_traits, 5>,
}

impl c_loadout_palette_traits {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        for i in 0..5 {
            self.m_loadouts.get()[i].encode(bitstream)?;
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        for i in 0..5 {
            self.m_loadouts.get_mut()[i].decode(bitstream)?;
        }

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_game_engine_loadout_traits {
    pub m_flags: u8,
    pub m_loadout_palettes: StaticArray<c_loadout_palette_traits, 6>,
}

impl c_game_engine_loadout_traits {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_flags, 2)?;
        for i in 0..6 {
            self.m_loadout_palettes.get()[i].encode(bitstream)?;
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_flags = bitstream.read_integer("flags", 2)?;
        for i in 0..6 {
            self.m_loadout_palettes.get_mut()[i].decode(bitstream)?;
        }

        Ok(())
    }
}
