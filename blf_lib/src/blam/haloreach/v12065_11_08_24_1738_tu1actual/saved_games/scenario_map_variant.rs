use std::cmp::min;
use binrw::{BinRead, BinWrite};
use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::{assert_ok, OPTION_TO_RESULT, TEST_BIT};
use crate::blam::common::math::real_math::{real_point3d, real_rectangle3d};
use blf_lib::types::array::StaticArray;
use crate::blam::common::math::real_math::real_vector3d;
use crate::blam::common::simulation::simulation_encoding::{simulation_read_quantized_position, simulation_write_position, simulation_write_quantized_position};
use serde_hex::{SerHex,StrictCap};
use blf_lib::blam::common::simulation::simulation_encoding::simulation_read_position;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::string_table;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::memory::bitstream_writer::c_bitstream_writer_extensions;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::saved_games::saved_game_files::s_content_item_metadata;
use blf_lib_derive::TestSize;
use blf_lib_derivable::result::BLFLibResult;
use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::string_table::c_single_language_string_table;
use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::memory::bitstream_reader::c_bitstream_reader_extensions;
use crate::types::bool::Bool;
use crate::types::numbers::Float32;

pub const k_maximum_variant_objects: usize = 651;
pub const k_maximum_variant_quotas: usize = 256;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
// This structure is not a direct representation of Halo: Reach's memory
// As such it can only be written packed via the encode / decode functions.
pub struct c_map_variant {
    pub m_metadata: s_content_item_metadata,
    pub m_map_variant_version: u16,
    pub m_number_of_placeable_object_quotas: u16,
    pub m_map_id: u32,
    pub m_world_bounds: real_rectangle3d,
    pub m_maximum_budget: u32,
    pub m_spent_budget: u32,
    pub m_helpers_enabled: Bool, // seems to still exist, though not packed.
    pub m_built_in: Bool,
    // #[brw(pad_after = 1)]
    pub m_built_from_xml: Bool,
    #[serde(with = "SerHex::<StrictCap>")]
    pub m_original_map_rsa_signature_hash: u32,
    #[serde(with = "SerHex::<StrictCap>")]
    pub m_scenario_palette_crc: u32,
    pub m_string_table: c_single_language_string_table<256, 4096, 12, 13, 9>,
    pub m_variant_objects: StaticArray<s_variant_object_datum, k_maximum_variant_objects>,
    pub m_quotas: StaticArray<s_variant_quota, k_maximum_variant_quotas>,
}

impl c_map_variant {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_metadata.encode(bitstream)?;
        bitstream.write_integer(self.m_map_variant_version, 8)?;
        bitstream.write_integer(self.m_original_map_rsa_signature_hash, 32)?;
        bitstream.write_integer(self.m_scenario_palette_crc, 32)?;
        bitstream.write_integer(self.m_number_of_placeable_object_quotas, 9)?;
        bitstream.write_integer(self.m_map_id, 32)?;
        bitstream.write_bool(self.m_built_in)?;
        bitstream.write_bool(self.m_built_from_xml)?;
        bitstream.write_raw(self.m_world_bounds, 0xC0)?;
        bitstream.write_integer(self.m_maximum_budget, 32)?;
        bitstream.write_integer(self.m_spent_budget, 32)?;
        self.m_string_table.encode(bitstream)?;

        for i in 0..k_maximum_variant_objects {
            self.m_variant_objects[i].encode(bitstream, &self.m_world_bounds)?;
        }

        for i in 0..k_maximum_variant_quotas {
            self.m_quotas[i].encode(bitstream)?;
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_metadata.decode(bitstream)?;
        self.m_map_variant_version = bitstream.read_unnamed_integer(8)?;
        self.m_original_map_rsa_signature_hash = bitstream.read_unnamed_integer(32)?;
        self.m_scenario_palette_crc = bitstream.read_unnamed_integer(32)?;
        self.m_number_of_placeable_object_quotas = bitstream.read_unnamed_integer(9)?;
        self.m_map_id = bitstream.read_unnamed_integer(32)?;
        self.m_built_in = bitstream.read_unnamed_bool()?;
        self.m_built_from_xml = bitstream.read_unnamed_bool()?;
        self.m_world_bounds = bitstream.read_raw(0xC0)?;
        self.m_maximum_budget = bitstream.read_unnamed_integer(32)?;
        self.m_spent_budget = bitstream.read_unnamed_integer(32)?;
        self.m_string_table.decode(bitstream)?;

        for i in 0..k_maximum_variant_objects {
            &mut self.m_variant_objects.get_mut()[i].decode(bitstream, &self.m_world_bounds)?;
        }

        for i in 0..min(k_maximum_variant_quotas, self.m_number_of_placeable_object_quotas as usize) {
            &mut self.m_quotas.get_mut()[i].decode(bitstream)?;
        }

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Copy, Serialize, Deserialize, BinRead, BinWrite)]
pub struct s_variant_quota {
    // #[serde(with = "SerHex::<StrictCap>")]
    // pub object_definition_index: u32,
    pub minimum_count: u8,
    pub maximum_count: u8,
    pub placed_on_map: u8,
    // pub maximum_allowed: i8,
    // pub price_per_item: Float32,
}

impl s_variant_quota {
    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.minimum_count = bitstream.read_unnamed_integer(8)?;
        self.maximum_count = bitstream.read_unnamed_integer(8)?;
        self.placed_on_map = bitstream.read_unnamed_integer(8)?;
        Ok(())
    }

    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.minimum_count, 8)?;
        bitstream.write_integer(self.maximum_count, 8)?;
        bitstream.write_integer(self.placed_on_map, 8)?;
        Ok(())
    }
}

#[derive(BinRead, BinWrite, Serialize, Deserialize, Default, PartialEq, Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
#[repr(u8)]
#[brw(repr = u8)]
pub enum e_boundary_shape {
    #[default]
    unused = 0,
    sphere = 1,
    cylinder = 2,
    r#box = 3,
}

#[derive(Default, PartialEq, Debug, Clone, Copy, Serialize, Deserialize, BinRead, BinWrite)]
pub struct s_multiplayer_object_boundary {
    pub shape: e_boundary_shape,
    pub size: Float32,
    pub box_length: Float32,
    pub positive_height: Float32,
    pub negative_height: Float32,
}

impl s_multiplayer_object_boundary {
    pub fn decode(bitstream: &mut c_bitstream_reader) -> BLFLibResult<Option<s_multiplayer_object_boundary>> {
        let mut boundary = Self::default();
        boundary.shape = bitstream.read_enum(2)?;

        match boundary.shape {
            e_boundary_shape::unused => return Ok(None),
            e_boundary_shape::sphere => {
                boundary.size = bitstream.read_quantized_real(0f32, 200.0f32, 11, false, true)?;
            }
            e_boundary_shape::cylinder => {
                boundary.size = bitstream.read_quantized_real(0f32, 200.0f32, 11, false, true)?;
                boundary.positive_height = bitstream.read_quantized_real(0f32, 200.0f32, 11, false, true)?;
                boundary.negative_height = bitstream.read_quantized_real(0f32, 200.0f32, 11, false, true)?;
            }
            e_boundary_shape::r#box => {
                boundary.size = bitstream.read_quantized_real(0f32, 200.0f32, 11, false, true)?;
                boundary.box_length = bitstream.read_quantized_real(0f32, 200.0f32, 11, false, true)?;
                boundary.positive_height = bitstream.read_quantized_real(0f32, 200.0f32, 11, false, true)?;
                boundary.negative_height = bitstream.read_quantized_real(0f32, 200.0f32, 11, false, true)?;
            }
        };

        Ok(Some(boundary))
    }

    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_enum(self.shape, 2)?;

        match self.shape {
            e_boundary_shape::unused => {}
            e_boundary_shape::sphere => {
                bitstream.write_quantized_real(self.size, 0f32, 200f32, 11, false, true)?;
            }
            e_boundary_shape::cylinder => {
                bitstream.write_quantized_real(self.size, 0f32, 200f32, 11, false, true)?;
                bitstream.write_quantized_real(self.positive_height, 0f32, 200f32, 11, false, true)?;
                bitstream.write_quantized_real(self.negative_height, 0f32, 200f32, 11, false, true)?;
            }
            e_boundary_shape::r#box => {
                bitstream.write_quantized_real(self.size, 0f32, 200f32, 11, false, true)?;
                bitstream.write_quantized_real(self.box_length, 0f32, 200f32, 11, false, true)?;
                bitstream.write_quantized_real(self.positive_height, 0f32, 200f32, 11, false, true)?;
                bitstream.write_quantized_real(self.negative_height, 0f32, 200f32, 11, false, true)?;
            }
        }

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Copy, Serialize, Deserialize, BinRead, BinWrite)]
pub struct s_variant_multiplayer_object_properties_definition_location_data {
    pub location_name_index: i8, // 8
}

#[derive(Default, PartialEq, Debug, Clone, Copy, Serialize, Deserialize, BinRead, BinWrite)]
pub struct s_variant_multiplayer_object_properties_definition_teleporter_data {
    pub channel: u8, // 5
    pub passability: u8, // 5
}

#[derive(Default, PartialEq, Debug, Clone, Copy, Serialize, Deserialize, BinRead, BinWrite)]
pub struct s_variant_multiplayer_object_properties_definition_weapon_data {
    pub spare_clips: u8, // 8
}


// TODO: When implementing binrw for c_map_variant, manually impl BinRead and BinWrite here for optionals.
#[derive(Default, PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct s_variant_multiplayer_object_properties_definition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boundary: Option<s_multiplayer_object_boundary>,
    pub game_engine_flags: u16,
    pub user_data: u8,
    pub spawn_time: u8,
    pub cached_type: u8,
    pub label_index: i8,
    pub placement_flags: u8,
    pub team: i8,
    pub primary_change_color_index: i8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_data: Option<s_variant_multiplayer_object_properties_definition_location_data>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teleporter_data: Option<s_variant_multiplayer_object_properties_definition_teleporter_data>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weapon_data: Option<s_variant_multiplayer_object_properties_definition_weapon_data>,
}

impl s_variant_multiplayer_object_properties_definition {
    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.boundary = s_multiplayer_object_boundary::decode(bitstream)?;
        self.user_data = bitstream.read_unnamed_integer(8)?;
        self.spawn_time = bitstream.read_unnamed_integer(8)?;
        self.cached_type = bitstream.read_unnamed_integer(5)?;
        self.label_index = bitstream.read_unnamed_index::<256>(8)? as i8;
        self.placement_flags = bitstream.read_unnamed_integer(8)?;
        self.team = bitstream.read_unnamed_integer::<i8>(4)? - 1;
        self.primary_change_color_index = bitstream.read_unnamed_index::<8>(3)? as i8;

        match self.cached_type {
            1 => {
                self.weapon_data = Some(s_variant_multiplayer_object_properties_definition_weapon_data {
                    spare_clips: bitstream.read_unnamed_integer(8)?,
                })
            }
            12 | 13 | 14 => {
                self.teleporter_data = Some(s_variant_multiplayer_object_properties_definition_teleporter_data {
                    channel: bitstream.read_unnamed_integer(5)?,
                    passability: bitstream.read_unnamed_integer(5)?,
                })
            }
            19 => {
                self.location_data = Some(s_variant_multiplayer_object_properties_definition_location_data {
                    location_name_index: bitstream.read_unnamed_index::<255>(8)? as i8,
                })
            }
            _ => {}
        }

        Ok(())
    }

    pub fn encode(&self, mut bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.boundary.unwrap_or_default().encode(&mut bitstream)?;
        bitstream.write_integer(self.user_data, 8)?;
        bitstream.write_integer(self.spawn_time, 8)?;
        bitstream.write_integer(self.cached_type, 5)?;
        bitstream.write_index::<256>(self.label_index, 8)?;
        bitstream.write_integer(self.placement_flags, 8)?;
        bitstream.write_integer((self.team + 1) as u32, 4)?;
        bitstream.write_index::<8>(self.primary_change_color_index, 3)?;

        match self.cached_type {
            1 => {
                let weapon_data = OPTION_TO_RESULT!(
                    self.weapon_data,
                    "Tried to encode a weapon with no weapon data provided."
                )?;

                bitstream.write_integer(weapon_data.spare_clips, 8)?;
            }
            12 | 13 | 14 => {
                let teleporter_data = OPTION_TO_RESULT!(
                    self.teleporter_data,
                    "Tried to encode a teleporter with no teleporter data provided."
                )?;

                bitstream.write_integer(teleporter_data.channel, 5)?;
                bitstream.write_integer(teleporter_data.passability, 5)?;
            }
            19 => {
                let location_data = OPTION_TO_RESULT!(
                    self.location_data,
                    "Tried to encode a location name with no name provided."
                )?;

                bitstream.write_index::<255>(location_data.location_name_index, 8)?;
            }
            _ => {}
        }

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct s_variant_object_datum {
    pub flags: u16,
    pub reuse_timeout: u16,
    pub object_datum_index: i32,
    pub editor_object_index: i32,
    pub variant_quota_index: i32,
    pub variant_index: i32,  // not sure on unpacked layout
    pub position: real_point3d,
    pub forward: real_vector3d,
    pub up: real_vector3d,
    pub spawn_relative_to: i32, // not sure on unpacked layout
    pub multiplayer_game_object_properties: s_variant_multiplayer_object_properties_definition,
}

impl s_variant_object_datum {
    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader, world_bounds: &real_rectangle3d) -> BLFLibResult {
        if bitstream.read_unnamed_bool()? { // exists
            self.flags = bitstream.read_unnamed_integer(2)?;
            self.variant_quota_index = bitstream.read_unnamed_index::<k_maximum_variant_quotas>(8)?;
            self.variant_index = bitstream.read_unnamed_index::<32>(5)?;
            simulation_read_position(bitstream, &mut self.position, 21, false, true, &world_bounds)?;
            bitstream.read_axes::<14, 20>(&mut self.forward, &mut self.up)?;
            self.spawn_relative_to = bitstream.read_unnamed_integer::<i32>(10)? - 1;
            self.multiplayer_game_object_properties.decode(bitstream)?;
        }

        Ok(())
    }

    pub fn encode(&self, mut bitstream: &mut c_bitstream_writer, world_bounds: &real_rectangle3d) -> BLFLibResult {
        if self.flags & 0x3FF == 0 {
            bitstream.write_bool(false)?;
            return Ok(());
        }

        bitstream.write_bool(true)?;
        bitstream.write_integer(self.flags, 2)?;
        bitstream.write_index::<k_maximum_variant_quotas>(self.variant_quota_index, 8)?;
        bitstream.write_index::<32>(self.variant_index, 5)?;
        simulation_write_position(bitstream, &self.position, 21, world_bounds)?;
        bitstream.write_axes::<14, 20>(&self.forward, &self.up)?;
        bitstream.write_integer((self.spawn_relative_to + 1) as u32, 10)?;
        self.multiplayer_game_object_properties.encode(bitstream)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Copy, Serialize, Deserialize, BinRead, BinWrite, TestSize)]
#[Size(0x8)]
pub struct c_object_identifier {
    m_unique_id: i32,
    m_origin_bsp_index: i16,
    m_type: i8,
    m_source: i8,
}