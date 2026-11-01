import { c } from "@craftycodie/cstruct";

/** cstruct layouts for blf_lib `blam::common::math::real_math`. */

export const real_vector2d = c.struct({
  i: c.f32(),
  j: c.f32(),
});
export type real_vector2d = c.infer<typeof real_vector2d>;

export const real_vector3d = c.struct({
  i: c.f32(),
  j: c.f32(),
  k: c.f32(),
});
export type real_vector3d = c.infer<typeof real_vector3d>;

export const real_point2d = c.struct({
  x: c.f32(),
  y: c.f32(),
});
export type real_point2d = c.infer<typeof real_point2d>;

export const real_point3d = c.struct({
  x: c.f32(),
  y: c.f32(),
  z: c.f32(),
});
export type real_point3d = c.infer<typeof real_point3d>;

export const real_bounds = c.struct({
  lower: c.f32(),
  upper: c.f32(),
});
export type real_bounds = c.infer<typeof real_bounds>;

export const real_rectangle2d = c.struct({
  x: real_bounds,
  y: real_bounds,
});
export type real_rectangle2d = c.infer<typeof real_rectangle2d>;

export const real_rectangle3d = c.struct({
  x: real_bounds,
  y: real_bounds,
  z: real_bounds,
});
export type real_rectangle3d = c.infer<typeof real_rectangle3d>;

export const real_matrix3x3 = c.struct({
  forward: real_vector3d,
  left: real_vector3d,
  up: real_vector3d,
});
export type real_matrix3x3 = c.infer<typeof real_matrix3x3>;

export const real_matrix4x3 = c.struct({
  scale: c.f32(),
  matrix: real_matrix3x3,
  center: real_point3d,
});
export type real_matrix4x3 = c.infer<typeof real_matrix4x3>;

export const real_plane3d = c.struct({
  n: real_vector3d,
  d: c.f32(),
});
export type real_plane3d = c.infer<typeof real_plane3d>;
