//! Halo 4-only base-variant option blocks (prototype / requisition / ordnance / map loadouts).
use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::game::game_engine_loadout_traits::c_loadout_traits;
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::memory::bitstream_reader::c_bitstream_reader_extensions;
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::memory::bitstream_writer::c_bitstream_writer_extensions;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::types::array::StaticArray;
use blf_lib_derivable::result::BLFLibResult;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive, Default, Serialize, Deserialize)]
pub enum e_custom_game_map_size {
    #[default]
    small = 0,
    medium = 1,
    large = 2,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct MapLoadoutInfo {
    pub m_size: e_custom_game_map_size,
    pub m_loadout: c_loadout_traits,
}

impl MapLoadoutInfo {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_enum_raw(self.m_size, 2)?;
        self.m_loadout.encode(bitstream)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_size = bitstream.read_enum_raw("size", 2)?;
        self.m_loadout.decode(bitstream)?;
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_game_engine_prototype_options {
    pub m_mode: u8,
    pub m_promethean_energy_kill: u8,
    pub m_promethean_energy_time: u8,
    pub m_promethean_energy_medal: u8,
    pub m_promethean_duration: u8,
    pub m_class_color_override: bool,
}

impl c_game_engine_prototype_options {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_mode, 2)?;
        bitstream.write_integer(self.m_promethean_energy_kill, 3)?;
        bitstream.write_integer(self.m_promethean_energy_time, 3)?;
        bitstream.write_integer(self.m_promethean_energy_medal, 3)?;
        bitstream.write_integer(self.m_promethean_duration, 4)?;
        bitstream.write_bool(self.m_class_color_override)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_mode = bitstream.read_integer("prototype-options-mode", 2)?;
        self.m_promethean_energy_kill =
            bitstream.read_integer("prototype-options-promethean-energy-kill", 3)?;
        self.m_promethean_energy_time =
            bitstream.read_integer("prototype-options-promethean-energy-time", 3)?;
        self.m_promethean_energy_medal =
            bitstream.read_integer("prototype-options-promethean-energy-medal", 3)?;
        self.m_promethean_duration =
            bitstream.read_integer("prototype-options-promethean-duration", 4)?;
        self.m_class_color_override =
            bitstream.read_bool("prototype-options-class-color-override")?;
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_game_engine_requisition_palette_item {
    pub m_global_palette_index: u8,
    pub m_locked: bool,
    pub m_designer_id: u32,
    pub m_max_instances: u32,
    pub m_price: f32,
    pub m_model_variant_name: u32,
    pub m_starting_ammo: u32,
    pub m_warm_up: f32,
    pub m_purchase_frequency_player: f32,
    pub m_purchase_frequency_team: f32,
    pub m_price_increase_factor: f32,
    pub m_max_buy_player: u8,
    pub m_max_buy_team: u8,
}

impl c_game_engine_requisition_palette_item {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_global_palette_index, 6)?;
        bitstream.write_bool(self.m_locked)?;
        bitstream.write_integer(self.m_designer_id, 32)?;
        // e_requisition_sub_menu: 0 bits on wire
        bitstream.write_integer(self.m_max_instances, 30)?;
        bitstream.write_float(self.m_price, 32)?;
        bitstream.write_integer(self.m_model_variant_name, 30)?;
        bitstream.write_integer(self.m_starting_ammo, 30)?;
        bitstream.write_float(self.m_warm_up, 32)?;
        bitstream.write_float(self.m_purchase_frequency_player, 32)?;
        bitstream.write_float(self.m_purchase_frequency_team, 32)?;
        bitstream.write_float(self.m_price_increase_factor, 32)?;
        bitstream.write_integer(self.m_max_buy_player, 8)?;
        bitstream.write_integer(self.m_max_buy_team, 8)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_global_palette_index =
            bitstream.read_integer("requisition-item-global-palette-index", 6)?;
        self.m_locked = bitstream.read_bool("requisition-item-locked")?;
        self.m_designer_id = bitstream.read_integer("requisition-item-designer-id", 32)?;
        self.m_max_instances = bitstream.read_integer("requisition-item-max-instances", 30)?;
        self.m_price = bitstream.read_float("requisition-item-price", 32)?;
        self.m_model_variant_name =
            bitstream.read_integer("requisition-item-model-variant-name", 30)?;
        self.m_starting_ammo = bitstream.read_integer("requisition-item-starting-ammo", 30)?;
        self.m_warm_up = bitstream.read_float("requisition-item-warm-up", 32)?;
        self.m_purchase_frequency_player =
            bitstream.read_float("requisition-item-purchase-frequency-player", 32)?;
        self.m_purchase_frequency_team =
            bitstream.read_float("requisition-item-purchase-frequency-team", 32)?;
        self.m_price_increase_factor =
            bitstream.read_float("requisition-item-price-increase-factor", 32)?;
        self.m_max_buy_player = bitstream.read_integer("requisition-item-max-buy-player", 8)?;
        self.m_max_buy_team = bitstream.read_integer("requisition-item-max-buy-team", 8)?;
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_game_engine_requisition_options {
    pub m_player_frequency: f32,
    pub m_initial_currency: u32,
    pub m_items: Vec<c_game_engine_requisition_palette_item>,
}

impl c_game_engine_requisition_options {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_float(self.m_player_frequency, 32)?;
        bitstream.write_integer(self.m_initial_currency, 32)?;
        bitstream.write_integer(self.m_items.len() as u32, 7)?;
        for item in &self.m_items {
            item.encode(bitstream)?;
        }
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player_frequency =
            bitstream.read_float("requisition-options-player-frequency", 32)?;
        self.m_initial_currency =
            bitstream.read_integer("requisition-options-initial-game-currency", 32)?;
        let count: u8 = bitstream.read_integer("requisition-item-count", 7)?;
        for _ in 0..count {
            let mut item = c_game_engine_requisition_palette_item::default();
            item.decode(bitstream)?;
            self.m_items.push(item);
        }
        Ok(())
    }
}

const k_ordnance_quantized_bits: usize = 30;
const k_ordnance_quantized_max: f32 = 10000.0;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct GameEngineOrdnanceSlotItem {
    pub m_name: String,
    pub m_weight: f32,
}

impl GameEngineOrdnanceSlotItem {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_string_extended_ascii(&self.m_name, 32)?;
        bitstream.write_quantized_real(
            self.m_weight,
            0.0,
            k_ordnance_quantized_max,
            k_ordnance_quantized_bits,
            false,
            true,
        )?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_name = bitstream.read_string_extended_ascii(32)?;
        self.m_weight = bitstream
            .read_quantized_real(0.0, k_ordnance_quantized_max, k_ordnance_quantized_bits, false, true)?
            .0;
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct GameEngineOrdnanceOptions {
    pub m_initial_enabled: bool,
    pub m_random_enabled: bool,
    pub m_objective_enabled: bool,
    pub m_player_enabled: bool,
    pub m_custom_player_ordnance_enabled: bool,
    pub m_non_player_drop_enabled: bool,
    pub m_random_drop_count: u8,
    pub m_random_drop_delay_min: u16,
    pub m_random_drop_delay_max: u16,
    pub m_random_drop_fanfare_duration: u16,
    pub m_initial_drop_name: String,
    pub m_initial_drop_delay: u16,
    pub m_initial_drop_fanfare_duration: u16,
    pub m_normal_drop_name: String,
    pub m_player_drop_name: String,
    pub m_remapping_table_name: String,
    pub m_custom_banks: StaticArray<StaticArray<GameEngineOrdnanceSlotItem, 8>, 4>,
    pub m_cost: f32,
    pub m_cost_multiplier: f32,
}

impl GameEngineOrdnanceOptions {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_bool(self.m_initial_enabled)?;
        bitstream.write_bool(self.m_random_enabled)?;
        bitstream.write_bool(self.m_objective_enabled)?;
        bitstream.write_bool(self.m_player_enabled)?;
        bitstream.write_bool(self.m_non_player_drop_enabled)?;
        bitstream.write_signed_integer(self.m_random_drop_count as i32, 8)?;
        bitstream.write_signed_integer(self.m_random_drop_delay_min as i32, 16)?;
        bitstream.write_signed_integer(self.m_random_drop_delay_max as i32, 16)?;
        bitstream.write_signed_integer(self.m_random_drop_fanfare_duration as i32, 16)?;
        bitstream.write_string_extended_ascii(&self.m_initial_drop_name, 32)?;
        bitstream.write_signed_integer(self.m_initial_drop_delay as i32, 16)?;
        bitstream.write_signed_integer(self.m_initial_drop_fanfare_duration as i32, 16)?;
        bitstream.write_string_extended_ascii(&self.m_normal_drop_name, 32)?;
        bitstream.write_string_extended_ascii(&self.m_player_drop_name, 32)?;
        bitstream.write_string_extended_ascii(&self.m_remapping_table_name, 32)?;
        bitstream.write_bool(self.m_custom_player_ordnance_enabled)?;
        for bank in self.m_custom_banks.get().iter() {
            for slot in bank.get().iter() {
                slot.encode(bitstream)?;
            }
        }
        bitstream.write_quantized_real(
            self.m_cost,
            0.0,
            k_ordnance_quantized_max,
            k_ordnance_quantized_bits,
            false,
            true,
        )?;
        bitstream.write_quantized_real(
            self.m_cost_multiplier,
            0.0,
            k_ordnance_quantized_max,
            k_ordnance_quantized_bits,
            false,
            true,
        )?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_initial_enabled = bitstream.read_bool("ordnanceInitialEnabled")?;
        self.m_random_enabled = bitstream.read_bool("ordnanceRandomEnabled")?;
        self.m_objective_enabled = bitstream.read_bool("ordnanceObjectiveEnabled")?;
        self.m_player_enabled = bitstream.read_bool("ordnancePlayerEnabled")?;
        self.m_non_player_drop_enabled = bitstream.read_bool("nonPlayerDropEnabled")?;
        self.m_random_drop_count = bitstream.read_signed_integer::<i32>("randomDropCount", 8)? as u8;
        self.m_random_drop_delay_min =
            bitstream.read_signed_integer::<i32>("randomDropDelayMin", 16)? as u16;
        self.m_random_drop_delay_max =
            bitstream.read_signed_integer::<i32>("randomDropDelayMax", 16)? as u16;
        self.m_random_drop_fanfare_duration =
            bitstream.read_signed_integer::<i32>("randomDropFanfareDuration", 16)? as u16;
        self.m_initial_drop_name = bitstream.read_string_extended_ascii(32)?;
        self.m_initial_drop_delay =
            bitstream.read_signed_integer::<i32>("initialDropDelay", 16)? as u16;
        self.m_initial_drop_fanfare_duration =
            bitstream.read_signed_integer::<i32>("initialDropFanfareDuration", 16)? as u16;
        self.m_normal_drop_name = bitstream.read_string_extended_ascii(32)?;
        self.m_player_drop_name = bitstream.read_string_extended_ascii(32)?;
        self.m_remapping_table_name = bitstream.read_string_extended_ascii(32)?;
        self.m_custom_player_ordnance_enabled =
            bitstream.read_bool("customPlayerOrdnanceEnabled")?;
        for bank in self.m_custom_banks.get_mut().iter_mut() {
            for slot in bank.get_mut().iter_mut() {
                slot.decode(bitstream)?;
            }
        }
        self.m_cost = bitstream
            .read_quantized_real(0.0, k_ordnance_quantized_max, k_ordnance_quantized_bits, false, true)?
            .0;
        self.m_cost_multiplier = bitstream
            .read_quantized_real(0.0, k_ordnance_quantized_max, k_ordnance_quantized_bits, false, true)?
            .0;
        Ok(())
    }
}
