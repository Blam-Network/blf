import { c } from "@craftycodie/cstruct";

export const e_purchase_state_flags = [
  "purchased",
  "forced_visible_and_purchasable",
  "bypassed",
  "granted_by_lsp",
  "banned",
] as const;

const purchase_state_bitfield = c.Bitfield(e_purchase_state_flags, c.u8());
export type e_purchase_state = c.infer<typeof purchase_state_bitfield>;

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

export const s_persistent_per_commendation_state = c.struct({
  progress: c.i16(),
});
export type s_persistent_per_commendation_state = c.infer<
  typeof s_persistent_per_commendation_state
>;
