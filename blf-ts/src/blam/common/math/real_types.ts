export interface real_bounds {
  lower: number;
  upper: number;
}

export interface real_point3d {
  x: number;
  y: number;
  z: number;
}

export interface real_rectangle3d {
  x: real_bounds;
  y: real_bounds;
  z: real_bounds;
}

export function real_point3d_default(): real_point3d {
  return { x: 0, y: 0, z: 0 };
}

export function real_bounds_default(): real_bounds {
  return { lower: 0, upper: 0 };
}

export function real_rectangle3d_default(): real_rectangle3d {
  return {
    x: real_bounds_default(),
    y: real_bounds_default(),
    z: real_bounds_default(),
  };
}

export function point_in_rectangle3d(
  point: real_point3d,
  rect: real_rectangle3d
): boolean {
  return (
    rect.x.lower <= point.x &&
    point.x <= rect.x.upper &&
    rect.y.lower <= point.y &&
    point.y <= rect.y.upper &&
    rect.z.lower <= point.z &&
    point.z <= rect.z.upper
  );
}
