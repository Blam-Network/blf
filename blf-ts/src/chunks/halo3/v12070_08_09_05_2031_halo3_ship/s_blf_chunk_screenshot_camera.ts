import { c } from "@craftycodie/cstruct";
import {
  int16_point2d,
  int16_rectangle2d,
} from "../../../blam/common/math/integer_math";
import {
  real_matrix4x3,
  real_plane3d,
  real_point2d,
  real_point3d,
  real_rectangle2d,
  real_vector2d,
  real_vector3d,
} from "../../../blam/common/math/real_math";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

/** 0x88 bytes. */
export const render_camera = c.struct({
  position: real_point3d,
  forward: real_vector3d,
  up: real_vector3d,
  mirrored: c.bool().padAfter(3),
  vertical_field_of_view: c.f32(),
  field_of_view_scale: c.f32(),
  window_pixel_bounds: int16_rectangle2d,
  window_title_safe_pixel_bounds: int16_rectangle2d,
  window_final_location: int16_point2d,
  render_pixel_bounds: int16_rectangle2d,
  render_title_safe_pixel_bounds: int16_rectangle2d,
  display_pixel_bounds: int16_rectangle2d,
  z_near: c.f32(),
  z_far: c.f32(),
  mirror_plane: real_plane3d,
  enlarge_view: c.bool().padAfter(3),
  enlarge_center: real_point2d,
  enlarge_size_x: c.f32(),
  enlarge_size_y: c.f32(),
});
export type render_camera = c.infer<typeof render_camera>;

/** 0xC0 bytes. */
export const render_projection = c.struct({
  world_to_view: real_matrix4x3,
  view_to_world: real_matrix4x3,
  projection_bounds: real_rectangle2d,
  projection_matrix: c.f32().array(4).array(4),
  world_to_screen_size: real_vector2d,
});
export type render_projection = c.infer<typeof render_projection>;

/** 0x158 bytes. */
export const s_saved_camera = c.struct({
  camera: render_camera,
  frustum_bounds: real_rectangle2d,
  projection: render_projection,
});
export type s_saved_camera = c.infer<typeof s_saved_camera>;

/** Halo 3 screenshot camera (`scnc` 2.1, 0x164 bytes). */
@blf.chunk("scnc", 2.1)
export class s_blf_chunk_screenshot_camera extends CStructBLFChunk(
  c.struct({
    /** Length of `jpeg_data` in the following `scnd`. */
    jpeg_data_length: c.u32(),
    camera: s_saved_camera,
    game_tick: c.u32(),
    film_tick: c.u32(),
  })
) {}
