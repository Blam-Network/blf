import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../bitstream";
import {
  type real_vector3d,
  real_vector3d_default,
} from "../../../../bitstream/math";
import { BlfError } from "../../../../error";
import { AutoMap } from "../../../../helpers/automap";
import type {
  real_point3d,
  real_rectangle3d,
} from "../../../common/math/real_types";
import {
  point_in_rectangle3d,
  real_point3d_default,
  real_rectangle3d_default,
} from "../../../common/math/real_types";
import { c_single_language_string_table } from "../game/string_table";
import { read_axes, write_axes } from "../memory/bitstream_extensions";
import {
  simulation_read_position,
  simulation_write_position,
} from "../simulation/simulation_encoding";
import {
  content_item_metadata_decode,
  content_item_metadata_encode,
  s_content_item_display_metadata,
  s_content_item_general_metadata,
  s_content_item_history,
  s_content_item_metadata,
} from "./saved_game_files";

export const k_maximum_variant_objects = 651;
export const k_maximum_variant_quotas = 256;

/** Matches `e_boundary_shape` in blf_lib `scenario_map_variant.rs`. */
export enum e_boundary_shape {
  unused = 0,
  sphere = 1,
  cylinder = 2,
  box = 3,
}

export class s_variant_quota {
  @AutoMap(() => Number)
  minimum_count = 0;
  @AutoMap(() => Number)
  maximum_count = 0;
  @AutoMap(() => Number)
  placed_on_map = 0;

  decode(bitstream: c_bitstream_reader): void {
    this.minimum_count = bitstream.read_integer("minimum-count", 8);
    this.maximum_count = bitstream.read_integer("maximum-count", 8);
    this.placed_on_map = bitstream.read_integer("placed-on-map", 8);
  }

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.minimum_count, 8);
    bitstream.write_integer(this.maximum_count, 8);
    bitstream.write_integer(this.placed_on_map, 8);
  }
}

export class s_multiplayer_object_boundary {
  @AutoMap(() => e_boundary_shape)
  shape: e_boundary_shape = e_boundary_shape.unused;
  @AutoMap(() => Number)
  size = 0;
  @AutoMap(() => Number)
  box_length = 0;
  @AutoMap(() => Number)
  positive_height = 0;
  @AutoMap(() => Number)
  negative_height = 0;

  static decode(
    bitstream: c_bitstream_reader
  ): s_multiplayer_object_boundary | null {
    const boundary = new s_multiplayer_object_boundary();
    boundary.shape = bitstream.read_enum("shape", 2, e_boundary_shape);

    switch (boundary.shape) {
      case e_boundary_shape.unused:
        return null;
      case e_boundary_shape.sphere:
        boundary.size = bitstream.read_quantized_real(0, 200, 11, false, true);
        break;
      case e_boundary_shape.cylinder:
        boundary.size = bitstream.read_quantized_real(0, 200, 11, false, true);
        boundary.positive_height = bitstream.read_quantized_real(
          0,
          200,
          11,
          false,
          true
        );
        boundary.negative_height = bitstream.read_quantized_real(
          0,
          200,
          11,
          false,
          true
        );
        break;
      case e_boundary_shape.box:
        boundary.size = bitstream.read_quantized_real(0, 200, 11, false, true);
        boundary.box_length = bitstream.read_quantized_real(
          0,
          200,
          11,
          false,
          true
        );
        boundary.positive_height = bitstream.read_quantized_real(
          0,
          200,
          11,
          false,
          true
        );
        boundary.negative_height = bitstream.read_quantized_real(
          0,
          200,
          11,
          false,
          true
        );
        break;
    }

    return boundary;
  }

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_enum(this.shape, 2, e_boundary_shape);

    switch (this.shape) {
      case e_boundary_shape.unused:
        break;
      case e_boundary_shape.sphere:
        bitstream.write_quantized_real(this.size, 0, 200, 11, false, true);
        break;
      case e_boundary_shape.cylinder:
        bitstream.write_quantized_real(this.size, 0, 200, 11, false, true);
        bitstream.write_quantized_real(
          this.positive_height,
          0,
          200,
          11,
          false,
          true
        );
        bitstream.write_quantized_real(
          this.negative_height,
          0,
          200,
          11,
          false,
          true
        );
        break;
      case e_boundary_shape.box:
        bitstream.write_quantized_real(this.size, 0, 200, 11, false, true);
        bitstream.write_quantized_real(
          this.box_length,
          0,
          200,
          11,
          false,
          true
        );
        bitstream.write_quantized_real(
          this.positive_height,
          0,
          200,
          11,
          false,
          true
        );
        bitstream.write_quantized_real(
          this.negative_height,
          0,
          200,
          11,
          false,
          true
        );
        break;
    }
  }
}

export class s_variant_multiplayer_object_properties_definition_location_data {
  @AutoMap(() => Number)
  location_name_index = 0;
}

export class s_variant_multiplayer_object_properties_definition_teleporter_data {
  @AutoMap(() => Number)
  channel = 0;
  @AutoMap(() => Number)
  passability = 0;
}

export class s_variant_multiplayer_object_properties_definition_weapon_data {
  @AutoMap(() => Number)
  spare_clips = 0;
}

export class s_variant_multiplayer_object_properties_definition {
  @AutoMap(() => s_multiplayer_object_boundary)
  boundary?: s_multiplayer_object_boundary;
  @AutoMap(() => Number)
  game_engine_flags = 0;
  @AutoMap(() => Number)
  user_data = 0;
  @AutoMap(() => Number)
  spawn_time = 0;
  @AutoMap(() => Number)
  cached_type = 0;
  @AutoMap(() => Number)
  label_index = 0;
  @AutoMap(() => Number)
  placement_flags = 0;
  @AutoMap(() => Number)
  team = 0;
  @AutoMap(() => Number)
  primary_change_color_index = 0;
  @AutoMap(
    () => s_variant_multiplayer_object_properties_definition_location_data
  )
  location_data?: s_variant_multiplayer_object_properties_definition_location_data;
  @AutoMap(
    () => s_variant_multiplayer_object_properties_definition_teleporter_data
  )
  teleporter_data?: s_variant_multiplayer_object_properties_definition_teleporter_data;
  @AutoMap(() => s_variant_multiplayer_object_properties_definition_weapon_data)
  weapon_data?: s_variant_multiplayer_object_properties_definition_weapon_data;

  decode(bitstream: c_bitstream_reader): void {
    this.boundary =
      s_multiplayer_object_boundary.decode(bitstream) ?? undefined;
    this.user_data = bitstream.read_integer("user-data", 8);
    this.spawn_time = bitstream.read_integer("spawn-time", 8);
    this.cached_type = bitstream.read_integer("cached-type", 5);
    this.label_index = bitstream.read_index("label-index", 256, 8);
    this.placement_flags = bitstream.read_integer("placement-flags", 8);
    this.team = bitstream.read_integer("team", 4) - 1;
    this.primary_change_color_index = bitstream.read_index(
      "primary-change-color-index",
      8,
      3
    );

    this.location_data = undefined;
    this.teleporter_data = undefined;
    this.weapon_data = undefined;

    switch (this.cached_type) {
      case 1:
        this.weapon_data =
          new s_variant_multiplayer_object_properties_definition_weapon_data();
        this.weapon_data.spare_clips = bitstream.read_integer("spare-clips", 8);
        break;
      case 12:
      case 13:
      case 14:
        this.teleporter_data =
          new s_variant_multiplayer_object_properties_definition_teleporter_data();
        this.teleporter_data.channel = bitstream.read_integer("channel", 5);
        this.teleporter_data.passability = bitstream.read_integer(
          "passability",
          5
        );
        break;
      case 19:
        this.location_data =
          new s_variant_multiplayer_object_properties_definition_location_data();
        this.location_data.location_name_index = bitstream.read_index(
          "location-name-index",
          255,
          8
        );
        break;
    }
  }

  encode(bitstream: c_bitstream_writer): void {
    (this.boundary ?? new s_multiplayer_object_boundary()).encode(bitstream);
    bitstream.write_integer(this.user_data, 8);
    bitstream.write_integer(this.spawn_time, 8);
    bitstream.write_integer(this.cached_type, 5);
    bitstream.write_index(this.label_index, 256, 8);
    bitstream.write_integer(this.placement_flags, 8);
    bitstream.write_integer(this.team + 1, 4);
    bitstream.write_index(this.primary_change_color_index, 8, 3);

    switch (this.cached_type) {
      case 1:
        if (!this.weapon_data) {
          throw new BlfError(
            "Tried to encode a weapon with no weapon data provided."
          );
        }
        bitstream.write_integer(this.weapon_data.spare_clips, 8);
        break;
      case 12:
      case 13:
      case 14:
        if (!this.teleporter_data) {
          throw new BlfError(
            "Tried to encode a teleporter with no teleporter data provided."
          );
        }
        bitstream.write_integer(this.teleporter_data.channel, 5);
        bitstream.write_integer(this.teleporter_data.passability, 5);
        break;
      case 19:
        if (!this.location_data) {
          throw new BlfError(
            "Tried to encode a location name with no name provided."
          );
        }
        bitstream.write_index(this.location_data.location_name_index, 255, 8);
        break;
    }
  }
}

export class s_variant_object_datum {
  @AutoMap(() => Number)
  flags = 0;
  @AutoMap(() => Number)
  reuse_timeout = 0;
  @AutoMap(() => Number)
  object_datum_index = 0;
  @AutoMap(() => Number)
  editor_object_index = 0;
  @AutoMap(() => Number)
  variant_quota_index = 0;
  @AutoMap(() => Number)
  variant_index = 0;
  @AutoMap(() => Object)
  position: real_point3d = real_point3d_default();
  @AutoMap(() => Object)
  forward: real_vector3d = real_vector3d_default();
  @AutoMap(() => Object)
  up: real_vector3d = real_vector3d_default();
  @AutoMap(() => Number)
  spawn_relative_to = 0;
  @AutoMap(() => s_variant_multiplayer_object_properties_definition)
  multiplayer_game_object_properties =
    new s_variant_multiplayer_object_properties_definition();

  decode(bitstream: c_bitstream_reader, world_bounds: real_rectangle3d): void {
    if (bitstream.read_bool("variant_object_exists")) {
      this.flags = bitstream.read_integer("flags", 2);
      this.variant_quota_index = bitstream.read_index(
        "variant-quota-index",
        k_maximum_variant_quotas,
        8
      );
      this.variant_index = bitstream.read_index("variant-index", 32, 5);
      simulation_read_position(bitstream, this.position, 21, world_bounds);
      read_axes(bitstream, this.forward, this.up, 14, 20);
      this.spawn_relative_to =
        bitstream.read_integer("spawn-relative-to", 10) - 1;
      this.multiplayer_game_object_properties.decode(bitstream);
    }
  }

  encode(bitstream: c_bitstream_writer, world_bounds: real_rectangle3d): void {
    if ((this.flags & 0x3ff) === 0) {
      bitstream.write_bool(false);
      return;
    }

    bitstream.write_bool(true);
    bitstream.write_integer(this.flags, 2);
    bitstream.write_index(
      this.variant_quota_index,
      k_maximum_variant_quotas,
      8
    );
    bitstream.write_index(this.variant_index, 32, 5);
    simulation_write_position(bitstream, this.position, 21, world_bounds);
    write_axes(bitstream, this.forward, this.up, 14, 20);
    bitstream.write_integer(this.spawn_relative_to + 1, 10);
    this.multiplayer_game_object_properties.encode(bitstream);
  }
}

export class c_object_identifier {
  m_unique_id = 0;
  m_origin_bsp_index = 0;
  m_type = 0;
  m_source = 0;
}

function read_real_rectangle3d(
  bitstream: c_bitstream_reader
): real_rectangle3d {
  const raw = bitstream.read_raw_data(0xc0);
  const view = new DataView(raw.buffer, raw.byteOffset, raw.byteLength);
  return {
    x: { lower: view.getFloat32(0, false), upper: view.getFloat32(4, false) },
    y: { lower: view.getFloat32(8, false), upper: view.getFloat32(12, false) },
    z: { lower: view.getFloat32(16, false), upper: view.getFloat32(20, false) },
  };
}

function write_real_rectangle3d(
  bitstream: c_bitstream_writer,
  bounds: real_rectangle3d
): void {
  const raw = new Uint8Array(24);
  const view = new DataView(raw.buffer);
  view.setFloat32(0, bounds.x.lower, false);
  view.setFloat32(4, bounds.x.upper, false);
  view.setFloat32(8, bounds.y.lower, false);
  view.setFloat32(12, bounds.y.upper, false);
  view.setFloat32(16, bounds.z.lower, false);
  view.setFloat32(20, bounds.z.upper, false);
  bitstream.write_raw_data(raw, 0xc0);
}

/**
 * Reach map variant packed body.
 * Mirrors `c_map_variant` in blf_lib — not a direct memory layout.
 */
export class c_map_variant {
  @AutoMap(() => s_content_item_metadata)
  m_metadata = (() => {
    const metadata = new s_content_item_metadata();
    metadata.general = new s_content_item_general_metadata();
    metadata.display = new s_content_item_display_metadata();
    metadata.creation_history = new s_content_item_history();
    metadata.modification_history = new s_content_item_history();
    return metadata;
  })();
  @AutoMap(() => Number)
  m_map_variant_version = 0;
  @AutoMap(() => Number)
  m_number_of_placeable_object_quotas = 0;
  @AutoMap(() => Number)
  m_map_id = 0;
  @AutoMap(() => Object)
  m_world_bounds: real_rectangle3d = real_rectangle3d_default();
  @AutoMap(() => Number)
  m_maximum_budget = 0;
  @AutoMap(() => Number)
  m_spent_budget = 0;
  @AutoMap(() => Boolean)
  m_helpers_enabled = false;
  @AutoMap(() => Boolean)
  m_built_in = false;
  @AutoMap(() => Boolean)
  m_built_from_xml = false;
  @AutoMap(() => Number)
  m_original_map_rsa_signature_hash = 0;
  @AutoMap(() => Number)
  m_scenario_palette_crc = 0;
  @AutoMap(() => c_single_language_string_table)
  m_string_table = new c_single_language_string_table(256, 4096, 12, 13, 9);
  @AutoMap(() => [s_variant_object_datum])
  m_variant_objects: s_variant_object_datum[] = Array.from(
    { length: k_maximum_variant_objects },
    () => new s_variant_object_datum()
  );
  @AutoMap(() => [s_variant_quota])
  m_quotas: s_variant_quota[] = Array.from(
    { length: k_maximum_variant_quotas },
    () => new s_variant_quota()
  );

  decode(bitstream: c_bitstream_reader): void {
    content_item_metadata_decode(bitstream, this.m_metadata);
    this.m_map_variant_version = bitstream.read_integer(
      "map-variant-version",
      8
    );
    this.m_original_map_rsa_signature_hash = bitstream.read_integer(
      "original-map-rsa-signature-hash",
      32
    );
    this.m_scenario_palette_crc = bitstream.read_integer(
      "scenario-palette-crc",
      32
    );
    this.m_number_of_placeable_object_quotas = bitstream.read_integer(
      "number-of-placeable-object-quotas",
      9
    );
    this.m_map_id = bitstream.read_integer("map-id", 32);
    this.m_built_in = bitstream.read_bool("built-in");
    this.m_built_from_xml = bitstream.read_bool("built-from-xml");
    this.m_world_bounds = read_real_rectangle3d(bitstream);
    this.m_maximum_budget = bitstream.read_integer("maximum-budget", 32);
    this.m_spent_budget = bitstream.read_integer("spent-budget", 32);
    this.m_string_table.decode(bitstream);

    for (let i = 0; i < k_maximum_variant_objects; i++) {
      this.m_variant_objects[i]!.decode(bitstream, this.m_world_bounds);
    }

    const quota_count = Math.min(
      k_maximum_variant_quotas,
      this.m_number_of_placeable_object_quotas
    );
    for (let i = 0; i < quota_count; i++) {
      this.m_quotas[i]!.decode(bitstream);
    }
  }

  encode(bitstream: c_bitstream_writer): void {
    content_item_metadata_encode(bitstream, this.m_metadata);
    bitstream.write_integer(this.m_map_variant_version, 8);
    bitstream.write_integer(this.m_original_map_rsa_signature_hash, 32);
    bitstream.write_integer(this.m_scenario_palette_crc, 32);
    bitstream.write_integer(this.m_number_of_placeable_object_quotas, 9);
    bitstream.write_integer(this.m_map_id, 32);
    bitstream.write_bool(this.m_built_in);
    bitstream.write_bool(this.m_built_from_xml);
    write_real_rectangle3d(bitstream, this.m_world_bounds);
    bitstream.write_integer(this.m_maximum_budget, 32);
    bitstream.write_integer(this.m_spent_budget, 32);
    this.m_string_table.encode(bitstream);

    for (let i = 0; i < k_maximum_variant_objects; i++) {
      this.m_variant_objects[i]!.encode(bitstream, this.m_world_bounds);
    }

    const quota_count = Math.min(
      k_maximum_variant_quotas,
      this.m_number_of_placeable_object_quotas
    );
    for (let i = 0; i < quota_count; i++) {
      this.m_quotas[i]!.encode(bitstream);
    }
  }
}

export function create_default_map_variant(): c_map_variant {
  return new c_map_variant();
}

/** Convenience helper used by tests and tooling. */
export function map_variant_point_in_bounds(
  map_variant: c_map_variant,
  position: real_point3d
): boolean {
  return point_in_rectangle3d(position, map_variant.m_world_bounds);
}
