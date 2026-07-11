import { c } from "@craftycodie/cstruct";

export const e_purchase_state_flags = [
  "purchased",
  "forced_visible_and_purchasable",
  "bypassed",
  "granted_by_lsp",
  "banned",
] as const;

export type e_purchase_state = c.Bitfield<typeof e_purchase_state_flags>;

export function default_purchase_state(
  overrides: Partial<e_purchase_state> = {}
): e_purchase_state {
  return {
    purchased: false,
    forced_visible_and_purchasable: false,
    bypassed: false,
    granted_by_lsp: false,
    banned: false,
    ...overrides,
  };
}

@c.struct()
export class s_persistent_per_commendation_state {
  @c.field("i16")
  progress = 0;
}
