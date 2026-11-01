import { c } from "@craftycodie/cstruct";
import {
  real_point3d,
  real_rectangle3d,
  real_vector3d,
} from "../../../common/math/real_math";
import { s_content_item_metadata } from "./saved_game_files";

export const k_object_type_count = 14;
export const k_maximum_variant_objects = 640;
export const k_maximum_variant_quotas = 256;
export const k_maximum_variant_gamestate_indices = 80;

export const c_object_identifier = c.struct({
  m_unique_id: c.i32(),
  m_origin_bsp_index: c.i16(),
  m_type: c.i8(),
  m_source: c.i8(),
});
export type c_object_identifier = c.infer<typeof c_object_identifier>;

/** 0x18 bytes. */
export const s_variant_multiplayer_object_properties_definition = c.struct({
  game_engine_flags: c.u16(),
  symmetry_placement_flags: c.u8(),
  owner_team: c.i8(),
  shared_storage: c.u8(),
  spawn_time: c.u8(),
  object_type: c.i8(),
  boundary_shape: c.u8(),
  boundary_size: c.f32(),
  boundary_box_length: c.f32(),
  boundary_positive_height: c.f32(),
  boundary_negative_height: c.f32(),
});
export type s_variant_multiplayer_object_properties_definition = c.infer<
  typeof s_variant_multiplayer_object_properties_definition
>;

/** 0x54 bytes. */
export const s_variant_object_datum = c.struct({
  flags: c.u16(),
  reuse_timeout: c.u16(),
  object_datum_index: c.i32(),
  editor_object_index: c.i32(),
  variant_quota_index: c.i32(),
  position: real_point3d,
  forward: real_vector3d,
  up: real_vector3d,
  parent_object_identifier: c_object_identifier,
  multiplayer_game_object_properties:
    s_variant_multiplayer_object_properties_definition,
});
export type s_variant_object_datum = c.infer<typeof s_variant_object_datum>;

export const s_variant_quota = c.struct({
  object_definition_index: c.u32(),
  minimum_count: c.u8(),
  maximum_count: c.u8(),
  placed_on_map: c.u8(),
  maximum_allowed: c.i8(),
  price_per_item: c.f32(),
});
export type s_variant_quota = c.infer<typeof s_variant_quota>;

/**
 * Unpacked Halo 3 map variant (0xE090 bytes).
 *
 * blf_lib aligns `m_original_map_rsa_signature_hash` to 4 bytes of the stream;
 * every place this is embedded starts 4-aligned, so that is 2 bytes of padding.
 */
export const c_map_variant = c.struct({
  m_metadata: s_content_item_metadata,
  m_map_variant_version: c.u16(),
  m_number_of_scenario_objects: c.u16(),
  m_number_of_variant_objects: c.u16(),
  m_number_of_placeable_object_quotas: c.u16(),
  m_map_id: c.u32(),
  m_world_bounds: real_rectangle3d,
  m_game_engine_subtype: c.u32(),
  m_maximum_budget: c.f32(),
  m_spent_budget: c.f32(),
  m_helpers_enabled: c.bool(),
  m_built_in: c.bool(),
  m_original_map_rsa_signature_hash: c.u32().padBefore(2),
  m_variant_objects: s_variant_object_datum.array(k_maximum_variant_objects),
  m_object_type_start_index: c.i16().array(k_object_type_count),
  m_quotas: s_variant_quota.array(k_maximum_variant_quotas),
  m_gamestate_indices: c
    .i32()
    .array(k_maximum_variant_gamestate_indices)
    .padAfter(4),
});
export type c_map_variant = c.infer<typeof c_map_variant>;
