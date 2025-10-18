use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::TEST_BIT;
use crate::blam::common::math::real_math::{real_point3d, real_rectangle3d};
use crate::blam::halo3::v12070_08_09_05_2031_halo3_ship::saved_games::saved_game_files::s_content_item_metadata;
use blf_lib::types::array::StaticArray;
use crate::blam::common::math::real_math::real_vector3d;
use crate::blam::halo3::v12070_08_09_05_2031_halo3_ship::simulation::simulation_encoding::{simulation_write_quantized_position};
use serde_hex::{SerHex,StrictCap};
use blf_lib::blam::halo3::v12070_08_09_05_2031_halo3_ship::memory::bitstream_reader::c_bitstream_reader_extensions;
use blf_lib::blam::halo3::v12070_08_09_05_2031_halo3_ship::simulation::simulation_encoding::simulation_read_quantized_position;
use blf_lib::types::c_string::StaticWcharString;
use blf_lib_derive::TestSize;
use blf_lib_derivable::result::BLFLibResult;
use crate::blam::halo3::v12070_08_09_05_2031_halo3_ship::memory::bitstream_writer::c_bitstream_writer_extensions;
use crate::types::bool::Bool;
use crate::types::numbers::Float32;

const k_object_type_count: usize = 14;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
pub struct c_map_variant {
    pub m_name: StaticWcharString<32>,
    pub m_description: StaticWcharString<32>,
    pub m_map_variant_version: u16,
    pub m_number_of_scenario_objects: u16,
    pub m_number_of_variant_objects: u16,
    pub m_number_of_placeable_object_quotas: u16,
    pub m_map_id: u32,
    pub m_world_bounds: real_rectangle3d,
    pub m_game_engine_subtype: u32, // here up to m_variant_objects is guessed
    pub m_maximum_budget: Float32,
    pub m_spent_budget: Float32,
    pub m_helpers_enabled: Bool,
    #[brw(pad_after = 2)]
    pub m_built_in: Bool,
    pub m_variant_objects: StaticArray<s_variant_object_datum, 512>,
    pub m_object_type_start_index: StaticArray<i16, k_object_type_count>,
    pub m_quotas: StaticArray<s_variant_quota, 256>,
    // don't think this exists
    // pub m_gamestate_indices: StaticArray<i32, 80>,
}

impl c_map_variant {
    pub fn encode(&self, mut bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_map_variant_version as u32, 16)?;
        bitstream.write_string_wchar(&self.m_name.get_string(), 32)?;
        bitstream.write_string_wchar(&self.m_description.get_string(), 32)?;
        bitstream.write_integer(self.m_number_of_scenario_objects as u32, 10)?;
        bitstream.write_integer(self.m_number_of_variant_objects as u32, 10)?;
        bitstream.write_raw(self.m_world_bounds, 0xC0)?;

        for i in 0..k_object_type_count {
            bitstream.write_integer((self.m_object_type_start_index[i] + 1) as u32, 16)?;
        }

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

                if TEST_BIT!(variant_object.flags, 8) // spawns relative
                {
                    bitstream.write_bool(true)?; // parent-object-exists
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
                }
            }
        }

        Ok(())
    }

    pub fn decode(&mut self, mut bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_map_variant_version = bitstream.read_integer("map-variant-version", 16)?;
        self.m_name = StaticWcharString::from_string(bitstream.read_string_wchar(32)?)?;
        self.m_description = StaticWcharString::from_string(bitstream.read_string_wchar(32)?)?;
        self.m_number_of_scenario_objects = bitstream.read_integer("number_of_variant_objects", 10)?;
        self.m_number_of_variant_objects = bitstream.read_integer("number_of_scenario_objects", 10)?;
        self.m_world_bounds = bitstream.read_raw(0xC0)?;

        for i in 0..k_object_type_count {
            self.m_object_type_start_index.get_mut()[i] = bitstream.read_unnamed_integer::<i16>(16)? - 1;
        }

        for i in 0..self.m_number_of_variant_objects as usize {
            let variant_object = &mut self.m_variant_objects.get_mut()[i];

            if !bitstream.read_bool("variant_object_exists")? {
                continue;
            }

            variant_object.flags = bitstream.read_unnamed_integer(16)?;

            if !bitstream.read_bool("variant_object_position_exists")? {
                continue;
            }

            simulation_read_quantized_position(bitstream, &mut variant_object.position, 16, &self.m_world_bounds)?;
            bitstream.read_axes(&mut variant_object.forward, &mut variant_object.up)?;
        }

        Ok(())
    }
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
    pub unknown1: u16,
    pub unknown2: u16,
    pub unknown3: u32,
}

#[derive(Default, PartialEq, Debug, Clone, Copy, Serialize, Deserialize, BinRead, BinWrite, TestSize)]
#[Size(0x54)]
pub struct s_variant_object_datum {
    pub flags: u16,
    pub reuse_timeout: u16,
    pub object_datum_index: u32,
    pub editor_object_index: u32,
    pub variant_quota_index: u32,
    pub position: real_point3d,
    pub forward: real_vector3d,
    pub up: real_vector3d,
    pub multiplayer_game_object_properties: s_variant_multiplayer_object_properties_definition,
}

#[derive(Default, PartialEq, Debug, Clone, Copy, Serialize, Deserialize, BinRead, BinWrite)]
pub struct s_variant_quota {
    #[serde(with = "SerHex::<StrictCap>")]
    pub object_definition_index: u32,
    pub minimum_count: i8,
    pub maximum_count: i8,
    pub active_on_map: i8, // not sure but strings for placed_on_map and active_on_map are both present.
    pub placed_on_map: i8,
    #[brw(pad_after = 3)] // alignment
    pub maximum_allowed: i8,
    pub price_per_item: Float32,
}