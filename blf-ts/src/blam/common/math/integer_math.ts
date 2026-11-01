import { c } from "@craftycodie/cstruct";

/** cstruct layouts for blf_lib `blam::common::math::integer_math`. */

export const int16_point2d = c.struct({
  x: c.i16(),
  y: c.i16(),
});
export type int16_point2d = c.infer<typeof int16_point2d>;

/** Blam's `short_rectangle2d`: edges in pixels, so width is `right - left`. */
export const int16_rectangle2d = c.struct({
  top: c.i16(),
  left: c.i16(),
  bottom: c.i16(),
  right: c.i16(),
});
export type int16_rectangle2d = c.infer<typeof int16_rectangle2d>;
