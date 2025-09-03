use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::TEST_BIT;
use crate::blam::common::math::real_math::{real_point3d, real_rectangle3d};
use crate::blam::halo3::release::saved_games::saved_game_files::s_content_item_metadata;
use blf_lib::types::array::StaticArray;
use crate::blam::common::math::real_math::real_vector3d;
use crate::blam::common::simulation::simulation_encoding::{simulation_read_quantized_position, simulation_write_quantized_position};
use serde_hex::{SerHex,StrictCap};
use blf_lib_derive::TestSize;
use blf_lib_derivable::result::BLFLibResult;
use crate::types::bool::Bool;
use crate::types::numbers::Float32;

const k_object_type_count: usize = 14;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite, TestSize)]
#[Size(0xE090)]
pub struct c_map_variant {
    pub m_metadata: s_content_item_metadata,
    pub m_map_variant_version: u16,
    pub m_number_of_scenario_objects: u16,
    pub m_number_of_variant_objects: u16,
    pub m_number_of_placeable_object_quotas: u16,
    pub m_map_id: u32,
    pub m_world_bounds: real_rectangle3d,
    pub m_game_engine_subtype: u32,
    pub m_maximum_budget: Float32,
    pub m_spent_budget: Float32,
    pub m_helpers_enabled: Bool,
    pub m_built_in: Bool,
    // #[serde(skip_serializing,skip_deserializing)]
    // __pad12A: [u8; 2],
    #[serde(with = "SerHex::<StrictCap>")]
    #[brw(align_before = 4)]
    pub m_original_map_rsa_signature_hash: u32,
    pub m_variant_objects: StaticArray<s_variant_object_datum, 640>, // 0x130
    pub m_object_type_start_index: StaticArray<i16, k_object_type_count>, // 0xD330 // Correct
    pub m_quotas: StaticArray<s_variant_quota, 256>,
    #[brw(pad_after = 4)]
    pub m_gamestate_indices: StaticArray<i32, 80>,
}

impl c_map_variant {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_metadata.encode(bitstream)?;
        bitstream.write_integer(self.m_map_variant_version as u32, 8)?;
        bitstream.write_integer(self.m_original_map_rsa_signature_hash, 32)?;
        bitstream.write_integer(self.m_number_of_scenario_objects as u32, 10)?;
        bitstream.write_integer(self.m_number_of_variant_objects as u32, 10)?;
        bitstream.write_integer(self.m_number_of_placeable_object_quotas as u32, 9)?;
        bitstream.write_integer(self.m_map_id, 32)?;
        bitstream.write_bool(self.m_built_in)?;
        bitstream.write_raw(self.m_world_bounds, 0xC0)?;
        bitstream.write_integer(self.m_game_engine_subtype, 4)?;
        bitstream.write_float(self.m_maximum_budget, 32)?;
        bitstream.write_float(self.m_spent_budget, 32)?;

        for i in 0..self.m_number_of_variant_objects as usize {
            let variant_object = self.m_variant_objects[i];

            if variant_object.flags & 0x3FF == 0 // 0x3FF is 10 bits, there's 10 flags. If none are set...
            {
                bitstream.write_bool(false)?; // variant_object_exists
            }
            else
            {
                bitstream.write_bool(true)?; // variant_object_exists
                bitstream.write_integer(variant_object.flags as u32, 16)?;
                bitstream.write_integer(variant_object.variant_quota_index as u32, 32)?;

                if TEST_BIT!(variant_object.flags, 8) // spawns relative
                {
                    bitstream.write_bool(true)?; // parent-object-exists
                    bitstream.write_raw(variant_object.parent_object_identifier, 64)?;
                }
                else
                {
                    bitstream.write_bool(false)?; // parent-object-exists
                }

                if !TEST_BIT!(variant_object.flags, 1) && i < self.m_number_of_scenario_objects as usize  //edited
                {
                    bitstream.write_bool(false)?;
                }
                else
                {
                    bitstream.write_bool(true)?;
                    simulation_write_quantized_position(bitstream, &variant_object.position, 16, false, &self.m_world_bounds)?;
                    bitstream.write_axes(&variant_object.forward, &variant_object.up)?;
                    bitstream.write_integer(variant_object.multiplayer_game_object_properties.object_type as u32, 8)?;
                    bitstream.write_integer(variant_object.multiplayer_game_object_properties.symmetry_placement_flags as u32, 8)?;
                    bitstream.write_integer(variant_object.multiplayer_game_object_properties.game_engine_flags as u32, 16)?;
                    bitstream.write_integer(variant_object.multiplayer_game_object_properties.shared_storage as u32, 8)?;
                    bitstream.write_integer(variant_object.multiplayer_game_object_properties.spawn_time as u32, 8)?;
                    bitstream.write_integer(variant_object.multiplayer_game_object_properties.owner_team as u32, 8)?;
                    bitstream.write_integer(variant_object.multiplayer_game_object_properties.boundary_shape as u32, 8)?;

                    match variant_object.multiplayer_game_object_properties.boundary_shape {
                        1 => { // sphere
                            bitstream.write_quantized_real(variant_object.multiplayer_game_object_properties.boundary_size, 0.0, 60.0, 16, false, false)?;
                            bitstream.write_quantized_real(variant_object.multiplayer_game_object_properties.boundary_negative_height, 0.0, 60.0, 16, false, false)?;
                        }
                        2 => { // cylinder
                            bitstream.write_quantized_real(variant_object.multiplayer_game_object_properties.boundary_size, 0.0, 60.0, 16, false, false)?;
                            bitstream.write_quantized_real(variant_object.multiplayer_game_object_properties.boundary_box_length, 0.0, 60.0, 16, false, false)?;
                            bitstream.write_quantized_real(variant_object.multiplayer_game_object_properties.boundary_positive_height, 0.0, 60.0, 16, false, false)?;
                        }
                        3 => { // box
                            bitstream.write_quantized_real(variant_object.multiplayer_game_object_properties.boundary_size, 0.0, 60.0, 16, false, false)?;
                            bitstream.write_quantized_real(variant_object.multiplayer_game_object_properties.boundary_box_length, 0.0, 60.0, 16, false, false)?;
                            bitstream.write_quantized_real(variant_object.multiplayer_game_object_properties.boundary_positive_height, 0.0, 60.0, 16, false, false)?;
                            bitstream.write_quantized_real(variant_object.multiplayer_game_object_properties.boundary_negative_height, 0.0, 60.0, 16, false, false)?;
                        }
                        _ => { }
                    }
                }
            }
        }

        for i in 0..k_object_type_count {
            bitstream.write_integer((self.m_object_type_start_index[i] + 1) as u32, 9)?;
        }

        for i in 0..self.m_number_of_placeable_object_quotas as usize {
            let object_quota = self.m_quotas[i];
            bitstream.write_integer(object_quota.object_definition_index, 32)?;
            bitstream.write_integer(object_quota.minimum_count as u32, 8)?;
            bitstream.write_integer(object_quota.maximum_count as u32, 8)?;
            bitstream.write_integer(object_quota.placed_on_map as u32, 8)?;
            bitstream.write_integer(object_quota.maximum_allowed as u32, 8)?;
            bitstream.write_float(object_quota.price_per_item, 32)?;
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_metadata.decode(bitstream)?;
        self.m_map_variant_version = bitstream.read_integer(8)? as u16;
        self.m_original_map_rsa_signature_hash = bitstream.read_integer(32)?;
        self.m_number_of_scenario_objects = bitstream.read_u16(10)?;
        self.m_number_of_variant_objects = bitstream.read_u16(10)?;
        self.m_number_of_placeable_object_quotas = bitstream.read_u16(9)?;
        self.m_map_id = bitstream.read_integer(32)?;
        self.m_built_in = Bool::from(bitstream.read_bool()?);
        self.m_world_bounds = bitstream.read_raw(0xC0)?;
        self.m_game_engine_subtype = bitstream.read_integer(4)?;
        self.m_maximum_budget = bitstream.read_float(32)?;
        self.m_spent_budget = bitstream.read_float(32)?;

        for i in 0..self.m_number_of_variant_objects as usize {
            let variant_object = &mut self.m_variant_objects.get_mut()[i];

            let variant_object_exists = bitstream.read_bool()?;

            if !variant_object_exists {
                continue;
            }

            variant_object.flags = bitstream.read_u16(16)?;
            variant_object.variant_quota_index = bitstream.read_signed_integer(32)?;

            let parent_object_exists = bitstream.read_bool()?;
            if parent_object_exists {
                variant_object.parent_object_identifier = bitstream.read_raw(64)?;
            }

            let position_exists = bitstream.read_bool()?;

            if !position_exists {
                continue;
            }

            simulation_read_quantized_position(bitstream, &mut variant_object.position, 16, &self.m_world_bounds)?;
            bitstream.read_axis::<8, 19>(&mut variant_object.forward, &mut variant_object.up)?;
            variant_object.multiplayer_game_object_properties.object_type = bitstream.read_signed_integer(8)? as i8;
            variant_object.multiplayer_game_object_properties.symmetry_placement_flags = bitstream.read_u8(8)?;
            variant_object.multiplayer_game_object_properties.game_engine_flags = bitstream.read_u16(16)?;
            variant_object.multiplayer_game_object_properties.shared_storage = bitstream.read_u8(8)?;
            variant_object.multiplayer_game_object_properties.spawn_time = bitstream.read_u8(8)?;
            variant_object.multiplayer_game_object_properties.owner_team = bitstream.read_u8(8)? as i8;
            variant_object.multiplayer_game_object_properties.boundary_shape = bitstream.read_u8(8)?;

            match variant_object.multiplayer_game_object_properties.boundary_shape {
                1 => { // sphere
                    variant_object.multiplayer_game_object_properties.boundary_size = bitstream.read_quantized_real(0.0, 60.0, 16, false, false)?;
                    variant_object.multiplayer_game_object_properties.boundary_negative_height = bitstream.read_quantized_real(0.0, 60.0, 16, false, false)?;
                }
                2 => { // cylinder
                    variant_object.multiplayer_game_object_properties.boundary_size = bitstream.read_quantized_real(0.0, 60.0, 16, false, false)?;
                    variant_object.multiplayer_game_object_properties.boundary_box_length = bitstream.read_quantized_real(0.0, 60.0, 16, false, false)?;
                    variant_object.multiplayer_game_object_properties.boundary_positive_height = bitstream.read_quantized_real(0.0, 60.0, 16, false, false)?;
                }
                3 => { // box
                    variant_object.multiplayer_game_object_properties.boundary_size = bitstream.read_quantized_real(0.0, 60.0, 16, false, false)?;
                    variant_object.multiplayer_game_object_properties.boundary_box_length = bitstream.read_quantized_real(0.0, 60.0, 16, false, false)?;
                    variant_object.multiplayer_game_object_properties.boundary_positive_height = bitstream.read_quantized_real(0.0, 60.0, 16, false, false)?;
                    variant_object.multiplayer_game_object_properties.boundary_negative_height = bitstream.read_quantized_real(0.0, 60.0, 16, false, false)?;
                }
                _ => { }
            }
        }

        for i in 0..k_object_type_count {
            self.m_object_type_start_index.get_mut()[i] = bitstream.read_integer(9)? as i16 - 1;
        }

        for i in 0..self.m_number_of_placeable_object_quotas as usize {
            let object_quota = &mut self.m_quotas.get_mut()[i];
            object_quota.object_definition_index = bitstream.read_integer(32)?;
            object_quota.minimum_count = bitstream.read_integer(8)? as u8;
            object_quota.maximum_count = bitstream.read_integer(8)? as u8;
            object_quota.placed_on_map = bitstream.read_integer(8)? as u8;
            object_quota.maximum_allowed = bitstream.read_integer(8)? as i8;
            object_quota.price_per_item = bitstream.read_float(32)?;
        }

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Copy, Serialize, Deserialize, BinRead, BinWrite)]
pub struct s_variant_quota {
    #[serde(with = "SerHex::<StrictCap>")]
    pub object_definition_index: u32,
    pub minimum_count: u8,
    pub maximum_count: u8,
    pub placed_on_map: u8,
    pub maximum_allowed: i8,
    pub price_per_item: Float32,
}

#[derive(Default, PartialEq, Debug, Clone, Copy, Serialize, Deserialize, BinRead, BinWrite, TestSize)]
#[Size(0x18)]
pub struct s_variant_multiplayer_object_properties_definition {
    pub game_engine_flags: u16,
    pub symmetry_placement_flags: u8, // foo
    pub owner_team: i8, // byte?
    pub shared_storage: u8, // spare_clips, teleporter_channel, spawn_rate
    pub spawn_time: u8,
    pub object_type: i8,
    pub boundary_shape: u8,
    pub boundary_size: Float32, // width or radius
    pub boundary_box_length: Float32,
    pub boundary_positive_height: Float32,
    pub boundary_negative_height: Float32,
}

#[derive(Default, PartialEq, Debug, Clone, Copy, Serialize, Deserialize, BinRead, BinWrite, TestSize)]
#[Size(0x54)]
pub struct s_variant_object_datum {
    pub flags: u16,
    pub reuse_timeout: u16,
    pub object_datum_index: i32,
    pub editor_object_index: i32,
    pub variant_quota_index: i32,
    pub position: real_point3d,
    pub forward: real_vector3d,
    pub up: real_vector3d,
    pub parent_object_identifier: c_object_identifier,
    pub multiplayer_game_object_properties: s_variant_multiplayer_object_properties_definition,
}

#[derive(Default, PartialEq, Debug, Clone, Copy, Serialize, Deserialize, BinRead, BinWrite, TestSize)]
#[Size(0x8)]
pub struct c_object_identifier {
    m_unique_id: i32,
    m_origin_bsp_index: i16,
    m_type: i8,
    m_source: i8,
}