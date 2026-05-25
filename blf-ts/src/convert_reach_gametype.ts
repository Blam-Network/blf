/**
 * Convert game variants between Halo Reach TU1 and MCC builds.
 *
 * JSON-clone the variant, reject MCC-only megalo features, remap what moved
 * between builds, then write the four top-level fields onto `to`.
 *
 * Differences handled here:
 * - MCC-only: temporary explicit refs, `<<=`, `>>=`
 * - `set_to_absolute` exists in both builds but moved (TU1 = 10, MCC = 12)
 * - Survival: unrelated fields — MCC→TU1 sets `m_campaign_difficulty_level` to 1;
 *   TU1→MCC sets `m_encoding_version` to 2; `m_additional_flags` only on MCC
 */

import {
  type c_game_variant as c_game_variant_tu1,
  e_game_mode as e_game_mode_tu1,
} from "./blam/haloreach/v12065_11_08_24_1738_tu1actual/game/c_game_variant";
import { e_math_operation as e_math_operation_tu1 } from "./blam/haloreach/v12065_11_08_24_1738_tu1actual/game/megalogamengine/megalogamengine_actions";
import {
  c_game_variant as c_game_variant_mcc,
  e_game_mode as e_game_mode_mcc,
} from "./blam/haloreach_mcc/v_untracked_25_08_19_1352/game/c_game_variant";
import { e_math_operation as e_math_operation_mcc } from "./blam/haloreach_mcc/v_untracked_25_08_19_1352/game/megalogamengine/megalogamengine_actions";
import { e_explicit_object_type as e_explicit_object_type_mcc } from "./blam/haloreach_mcc/v_untracked_25_08_19_1352/game/megalogamengine/megalogamengine_explicit_object";
import { e_explicit_player_type as e_explicit_player_type_mcc } from "./blam/haloreach_mcc/v_untracked_25_08_19_1352/game/megalogamengine/megalogamengine_explicit_player";
import { e_explicit_team_type as e_explicit_team_type_mcc } from "./blam/haloreach_mcc/v_untracked_25_08_19_1352/game/megalogamengine/megalogamengine_explicit_team";
import { BlfError } from "./error";

/** MCC math ops with no TU1 equivalent (`<<=`, `>>=`). */
const MCC_ONLY_MATH_OPS: readonly number[] = [
  e_math_operation_mcc.shift_left_with,
  e_math_operation_mcc.shift_right_with,
];

/** TU1 survival `m_campaign_difficulty_level` when converting from MCC. */
const TU1_SURVIVAL_CAMPAIGN_DIFFICULTY = 1;

/** MCC survival `m_encoding_version` when converting from TU1. */
const MCC_SURVIVAL_ENCODING_VERSION = 2;

// ---------------------------------------------------------------------------
// JSON helpers
// ---------------------------------------------------------------------------

function json_clone<T>(value: T): T {
  return JSON.parse(JSON.stringify(value));
}

interface GameVariantJson {
  m_campaign_variant?: unknown;
  m_custom_variant?: unknown;
  m_game_engine: number;
  m_survival_variant?: SurvivalJson;
}

type SurvivalJson = Record<string, unknown>;

function write_game_variant_json(
  to: c_game_variant_tu1 | c_game_variant_mcc,
  data: GameVariantJson
): void {
  to.m_game_engine = data.m_game_engine as never;
  to.m_campaign_variant = data.m_campaign_variant as never;
  to.m_custom_variant = data.m_custom_variant as never;
  to.m_survival_variant = data.m_survival_variant as never;
}

// ---------------------------------------------------------------------------
// Tree walk
// ---------------------------------------------------------------------------

function visit_object_tree(
  root: unknown,
  fn: (obj: Record<string, unknown>) => void
): void {
  if (root === null || typeof root !== "object") {
    return;
  }
  if (Array.isArray(root)) {
    for (const item of root) {
      visit_object_tree(item, fn);
    }
    return;
  }
  const obj = root as Record<string, unknown>;
  fn(obj);
  for (const value of Object.values(obj)) {
    visit_object_tree(value, fn);
  }
}

function remap_set_to_absolute_mcc_to_tu1(root: unknown): void {
  visit_object_tree(root, (obj) => {
    if (obj.m_operation === e_math_operation_mcc.set_to_absolute) {
      obj.m_operation = e_math_operation_tu1.set_to_absolute;
    }
    if (obj.m_math_operation === e_math_operation_mcc.set_to_absolute) {
      obj.m_math_operation = e_math_operation_tu1.set_to_absolute;
    }
  });
}

function remap_set_to_absolute_tu1_to_mcc(root: unknown): void {
  visit_object_tree(root, (obj) => {
    if (obj.m_operation === e_math_operation_tu1.set_to_absolute) {
      obj.m_operation = e_math_operation_mcc.set_to_absolute;
    }
    if (obj.m_math_operation === e_math_operation_tu1.set_to_absolute) {
      obj.m_math_operation = e_math_operation_mcc.set_to_absolute;
    }
  });
}

// ---------------------------------------------------------------------------
// MCC → TU1 compatibility checks
// ---------------------------------------------------------------------------

function variant_uses_temporary_explicit_refs(
  variant: c_game_variant_mcc
): boolean {
  let found = false;
  visit_object_tree(variant, (obj) => {
    if (found) {
      return;
    }
    const team = obj.m_explicit_team_type;
    if (
      typeof team === "number" &&
      team >= e_explicit_team_type_mcc.temporary_0 &&
      team <= e_explicit_team_type_mcc.temporary_5
    ) {
      found = true;
      return;
    }
    const player = obj.m_explicit_player_type;
    if (
      typeof player === "number" &&
      player >= e_explicit_player_type_mcc.temporary_0 &&
      player <= e_explicit_player_type_mcc.temporary_2
    ) {
      found = true;
      return;
    }
    const object = obj.m_explicit_object_type;
    if (
      typeof object === "number" &&
      object >= e_explicit_object_type_mcc.temporary_0 &&
      object <= e_explicit_object_type_mcc.temporary_7
    ) {
      found = true;
    }
  });
  return found;
}

function variant_uses_mcc_only_math_ops(variant: c_game_variant_mcc): boolean {
  let found = false;
  visit_object_tree(variant, (obj) => {
    if (found) {
      return;
    }
    const op = obj.m_operation ?? obj.m_math_operation;
    if (typeof op === "number" && MCC_ONLY_MATH_OPS.includes(op)) {
      found = true;
    }
  });
  return found;
}

/** Returns false when MCC features cannot be represented in TU1. */
function can_convert_mcc_game_variant_to_tu1(
  from: c_game_variant_mcc
): boolean {
  if (
    from.m_game_engine === e_game_mode_mcc.survival &&
    from.m_survival_variant !== undefined &&
    from.m_survival_variant.m_additional_flags !== 0
  ) {
    return false;
  }
  if (variant_uses_temporary_explicit_refs(from)) {
    return false;
  }
  if (variant_uses_mcc_only_math_ops(from)) {
    return false;
  }
  return true;
}

// ---------------------------------------------------------------------------
// Survival (MCC and TU1 use different field names for unrelated data)
// ---------------------------------------------------------------------------

function survival_json_mcc_to_tu1(survival: SurvivalJson): void {
  survival.m_campaign_difficulty_level = TU1_SURVIVAL_CAMPAIGN_DIFFICULTY;
  survival.m_encoding_version = undefined;
  survival.m_additional_flags = undefined;
}

function survival_json_tu1_to_mcc(survival: SurvivalJson): void {
  survival.m_encoding_version = MCC_SURVIVAL_ENCODING_VERSION;
  survival.m_campaign_difficulty_level = undefined;
  survival.m_additional_flags = 0;
}

// ---------------------------------------------------------------------------
// Conversion (cross-version only)
// ---------------------------------------------------------------------------

function convert_mcc_to_tu1(
  from: c_game_variant_mcc,
  to: c_game_variant_tu1
): boolean {
  if (!can_convert_mcc_game_variant_to_tu1(from)) {
    return false;
  }

  const data = json_clone(from) as GameVariantJson;
  remap_set_to_absolute_mcc_to_tu1(data);
  if (data.m_survival_variant !== undefined) {
    survival_json_mcc_to_tu1(data.m_survival_variant);
  }
  write_game_variant_json(to, data);
  return true;
}

function convert_tu1_to_mcc(
  from: c_game_variant_tu1,
  to: c_game_variant_mcc
): boolean {
  const data = json_clone(from) as GameVariantJson;
  remap_set_to_absolute_tu1_to_mcc(data);
  if (data.m_survival_variant !== undefined) {
    survival_json_tu1_to_mcc(data.m_survival_variant);
  }
  write_game_variant_json(to, data);
  return true;
}

export function convert_reach_gametype(
  from: c_game_variant_mcc,
  to: c_game_variant_tu1
): boolean;
export function convert_reach_gametype(
  from: c_game_variant_tu1,
  to: c_game_variant_mcc
): boolean;
export function convert_reach_gametype(
  from: c_game_variant_tu1 | c_game_variant_mcc,
  to: c_game_variant_tu1 | c_game_variant_mcc
): boolean {
  if (from.m_game_engine === e_game_mode_tu1.sandbox) {
    throw new BlfError(
      "Forge game variants are not supported. Where did you even get this?"
    );
  }

  const from_is_mcc = from instanceof c_game_variant_mcc;
  const to_is_mcc = to instanceof c_game_variant_mcc;

  if (from_is_mcc === to_is_mcc) {
    throw new BlfError(
      "convert_reach_gametype only supports cross-version conversion between Reach TU1 and Reach MCC"
    );
  }
  if (from_is_mcc) {
    return convert_mcc_to_tu1(from, to as c_game_variant_tu1);
  }
  return convert_tu1_to_mcc(
    from as c_game_variant_tu1,
    to as c_game_variant_mcc
  );
}
