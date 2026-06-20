/**
 * Convert game variants between Halo: Reach TU1 (Xbox 360) and MCC.
 *
 * Cross-build conversion uses AutoMapper (`@automapper/core` + `@automapper/classes`)
 * with profiles registered from the source instance graph, then applies build-specific
 * remaps in place on the mapped target instances.
 *
 * TU1 → MCC is always possible for supported variant kinds (custom / survival).
 * MCC → TU1 rejects MCC-only megalo features and fails when temporary object,
 * player, or team references cannot be mapped onto free global slots.
 */

import type { s_custom_game_engine_definition as s_custom_game_engine_definition_tu1 } from "../blam/haloreach/v12065_11_08_24_1738_tu1actual/game/game_variant";
import {
  type c_game_variant as c_game_variant_tu1,
  e_game_mode as e_game_mode_tu1,
} from "../blam/haloreach/v12065_11_08_24_1738_tu1actual/game/game_variant";
import { e_explicit_object_type as e_explicit_object_type_tu1 } from "../blam/haloreach/v12065_11_08_24_1738_tu1actual/game/megalogamengine/megalogamengine_explicit_object";
import { e_explicit_player_type as e_explicit_player_type_tu1 } from "../blam/haloreach/v12065_11_08_24_1738_tu1actual/game/megalogamengine/megalogamengine_explicit_player";
import { e_explicit_team_type as e_explicit_team_type_tu1 } from "../blam/haloreach/v12065_11_08_24_1738_tu1actual/game/megalogamengine/megalogamengine_explicit_team";
import {
  c_custom_variable_reference as c_custom_variable_reference_tu1,
  e_custom_variable_type as e_custom_variable_type_tu1,
} from "../blam/haloreach/v12065_11_08_24_1738_tu1actual/game/megalogamengine/megalogamengine_references";
import type { s_variable_metadata as s_variable_metadata_tu1 } from "../blam/haloreach/v12065_11_08_24_1738_tu1actual/game/megalogamengine/megalogamengine_variable_metadata";
import type { s_custom_game_engine_definition as s_custom_game_engine_definition_mcc } from "../blam/haloreach_mcc/v_untracked_25_08_16_1352/game/game_variant";
import {
  c_game_variant as c_game_variant_mcc,
  e_game_mode as e_game_mode_mcc,
} from "../blam/haloreach_mcc/v_untracked_25_08_16_1352/game/game_variant";
import {
  e_action_type as e_action_type_mcc,
  e_math_operation as e_math_operation_mcc,
} from "../blam/haloreach_mcc/v_untracked_25_08_16_1352/game/megalogamengine/megalogamengine_actions";
import { e_explicit_object_type as e_explicit_object_type_mcc } from "../blam/haloreach_mcc/v_untracked_25_08_16_1352/game/megalogamengine/megalogamengine_explicit_object";
import { e_explicit_player_type as e_explicit_player_type_mcc } from "../blam/haloreach_mcc/v_untracked_25_08_16_1352/game/megalogamengine/megalogamengine_explicit_player";
import { e_explicit_team_type as e_explicit_team_type_mcc } from "../blam/haloreach_mcc/v_untracked_25_08_16_1352/game/megalogamengine/megalogamengine_explicit_team";
import {
  c_custom_variable_reference as c_custom_variable_reference_mcc,
  e_custom_variable_type as e_custom_variable_type_mcc,
} from "../blam/haloreach_mcc/v_untracked_25_08_16_1352/game/megalogamengine/megalogamengine_references";
import type { s_variable_metadata as s_variable_metadata_mcc } from "../blam/haloreach_mcc/v_untracked_25_08_16_1352/game/megalogamengine/megalogamengine_variable_metadata";
import type { c_string_table as c_string_table_mcc } from "../blam/haloreach_mcc/v_untracked_25_08_16_1352/game/string_table";
import { BlfError } from "../error";
import { mccToTu1Mapper, tu1ToMccMapper } from "./reach_gametype_automap";
import {
  map_reach_gametype_mcc_to_tu1,
  map_reach_gametype_tu1_to_mcc,
} from "./reach_gametype_automapper";
import { e_reach_gametype_conversion_error } from "./reach_gametype_conversion_error";

export { e_reach_gametype_conversion_error } from "./reach_gametype_conversion_error";

const MCC_ONLY_MATH_OPS: readonly number[] = [
  e_math_operation_mcc.lshift,
  e_math_operation_mcc.rshift,
];

const MCC_ONLY_ACTION_TYPES: readonly number[] = [
  e_action_type_mcc.begin,
  e_action_type_mcc.hs_function_call,
  e_action_type_mcc.get_button_time,
  e_action_type_mcc.team_set_vehicle_spawning,
  e_action_type_mcc.player_set_vehicle_spawning,
  e_action_type_mcc.set_player_respawn_vehicle,
  e_action_type_mcc.set_team_respawn_vehicle,
  e_action_type_mcc.hide_object,
];

const TU1_SURVIVAL_CAMPAIGN_DIFFICULTY = 1;
const MCC_SURVIVAL_ENCODING_VERSION = 2;
const HR_GVAR_PREFIX = "$hr_gvar_";

const MAX_OBJECT_GLOBALS = 16;
const MAX_PLAYER_GLOBALS = 8;
const MAX_TEAM_GLOBALS = 8;
const MAX_NUMERIC_GLOBALS = 16;

type JsonRecord = Record<string, unknown>;

interface TemporaryToGlobalMaps {
  numeric: Map<number, number>;
  object: Map<number, number>;
  player: Map<number, number>;
  team: Map<number, number>;
}

function remapTemporaryObjectType(
  obj: JsonRecord,
  maps: TemporaryToGlobalMaps
): void {
  const objectType = obj.m_explicit_object_type;
  if (
    typeof objectType !== "number" ||
    objectType < e_explicit_object_type_mcc.temporary_0 ||
    objectType > e_explicit_object_type_mcc.temporary_7
  ) {
    return;
  }
  const temporaryIndex = objectType - e_explicit_object_type_mcc.temporary_0;
  const globalIndex = maps.object.get(temporaryIndex);
  if (globalIndex !== undefined) {
    obj.m_explicit_object_type =
      e_explicit_object_type_tu1.global_0 + globalIndex;
  }
}

function remapTemporaryPlayerType(
  obj: JsonRecord,
  maps: TemporaryToGlobalMaps
): void {
  const playerType = obj.m_explicit_player_type;
  if (
    typeof playerType !== "number" ||
    playerType < e_explicit_player_type_mcc.temporary_0 ||
    playerType > e_explicit_player_type_mcc.temporary_2
  ) {
    return;
  }
  const temporaryIndex = playerType - e_explicit_player_type_mcc.temporary_0;
  const globalIndex = maps.player.get(temporaryIndex);
  if (globalIndex !== undefined) {
    obj.m_explicit_player_type =
      e_explicit_player_type_tu1.global_0 + globalIndex;
  }
}

function remapTemporaryTeamType(
  obj: JsonRecord,
  maps: TemporaryToGlobalMaps
): void {
  const teamType = obj.m_explicit_team_type;
  if (
    typeof teamType !== "number" ||
    teamType < e_explicit_team_type_mcc.temporary_0 ||
    teamType > e_explicit_team_type_mcc.temporary_5
  ) {
    return;
  }
  const temporaryIndex = teamType - e_explicit_team_type_mcc.temporary_0;
  const globalIndex = maps.team.get(temporaryIndex);
  if (globalIndex !== undefined) {
    obj.m_explicit_team_type = e_explicit_team_type_tu1.global_0 + globalIndex;
  }
}

function remapTemporaryNumericVariable(
  obj: JsonRecord,
  maps: TemporaryToGlobalMaps
): void {
  if (
    obj.m_type !== e_custom_variable_type_mcc.temporary_number ||
    typeof obj.m_variable_index !== "number"
  ) {
    return;
  }
  const globalIndex = maps.numeric.get(obj.m_variable_index);
  if (globalIndex !== undefined) {
    obj.m_type = e_custom_variable_type_tu1.global_number;
    obj.m_variable_index = globalIndex;
  }
}

function remapTemporaryExplicitRefsInObject(
  obj: JsonRecord,
  maps: TemporaryToGlobalMaps
): void {
  remapTemporaryObjectType(obj, maps);
  remapTemporaryPlayerType(obj, maps);
  remapTemporaryTeamType(obj, maps);
  remapTemporaryNumericVariable(obj, maps);
}

function visit_object_tree(root: unknown, fn: (obj: JsonRecord) => void): void {
  if (root === null || typeof root !== "object") {
    return;
  }
  if (Array.isArray(root)) {
    for (const item of root) {
      visit_object_tree(item, fn);
    }
    return;
  }
  const obj = root as JsonRecord;
  fn(obj);
  for (const value of Object.values(obj)) {
    visit_object_tree(value, fn);
  }
}

function remap_temporary_explicit_refs(
  root: unknown,
  maps: TemporaryToGlobalMaps
): void {
  visit_object_tree(root, (obj) => {
    remapTemporaryExplicitRefsInObject(obj, maps);
  });
}

interface GlobalVariableMetadataSlots {
  m_numeric_variables: [c_custom_variable_reference_tu1, number][];
  m_object_variables: number[];
  m_player_variables: number[];
  m_team_variables: [number, number][];
}

function extend_global_metadata(
  metadata: GlobalVariableMetadataSlots,
  maps: TemporaryToGlobalMaps
): void {
  const maxObjectGlobal = Math.max(0, ...maps.object.values());
  while (metadata.m_object_variables.length <= maxObjectGlobal) {
    metadata.m_object_variables.push(0);
  }

  const maxPlayerGlobal = Math.max(0, ...maps.player.values());
  while (metadata.m_player_variables.length <= maxPlayerGlobal) {
    metadata.m_player_variables.push(0);
  }

  const maxTeamGlobal = Math.max(0, ...maps.team.values());
  while (metadata.m_team_variables.length <= maxTeamGlobal) {
    metadata.m_team_variables.push([0, 0]);
  }

  const maxNumericGlobal = Math.max(0, ...maps.numeric.values());
  while (metadata.m_numeric_variables.length <= maxNumericGlobal) {
    metadata.m_numeric_variables.push([
      new c_custom_variable_reference_tu1(),
      0,
    ]);
  }
}

function apply_survival_mcc_to_tu1(
  survival: c_game_variant_tu1["m_survival_variant"]
): void {
  if (survival === undefined) {
    return;
  }
  survival.m_campaign_difficulty_level = TU1_SURVIVAL_CAMPAIGN_DIFFICULTY;
  delete (survival as unknown as JsonRecord).m_encoding_version;
  delete (survival as unknown as JsonRecord).m_additional_flags;
}

function apply_survival_tu1_to_mcc(
  survival: c_game_variant_mcc["m_survival_variant"]
): void {
  if (survival === undefined) {
    return;
  }
  survival.m_encoding_version = MCC_SURVIVAL_ENCODING_VERSION;
  survival.m_additional_flags = 0;
  delete (survival as unknown as JsonRecord).m_campaign_difficulty_level;
}

function first_localized_string(
  table: c_string_table_mcc | undefined
): string | undefined {
  const value = table?.strings[0]?.[0];
  if (value === null || value === undefined || value.length === 0) {
    return;
  }
  return value;
}

function resolve_hr_gvar_name(
  value: string | undefined,
  localized_name: c_string_table_mcc | undefined
): string {
  if (value === undefined || !value.startsWith(HR_GVAR_PREFIX)) {
    return value ?? "";
  }
  const localized = first_localized_string(localized_name);
  if (localized !== undefined) {
    return localized;
  }
  return value.slice(HR_GVAR_PREFIX.length).replaceAll("_", " ");
}

function resolve_hr_gvar_description(
  value: string | undefined,
  localized_description: c_string_table_mcc | undefined
): string {
  if (value === undefined || !value.startsWith(HR_GVAR_PREFIX)) {
    return value ?? "";
  }
  return first_localized_string(localized_description) ?? "";
}

function apply_mcc_to_tu1_name_description(
  from: c_game_variant_mcc,
  to: c_game_variant_tu1
): void {
  const mccCustom = from.m_custom_variant;
  const tu1Custom = to.m_custom_variant;
  if (mccCustom === undefined || tu1Custom === undefined) {
    return;
  }
  const metadata = tu1Custom.m_base_variant.m_metadata;
  metadata.name = resolve_hr_gvar_name(
    mccCustom.m_base_variant.m_metadata.name,
    mccCustom.m_localized_name
  );
  metadata.description = resolve_hr_gvar_description(
    mccCustom.m_base_variant.m_metadata.description,
    mccCustom.m_localized_description
  );
}

interface GlobalSlotUsage {
  numeric: Set<number>;
  object: Set<number>;
  player: Set<number>;
  team: Set<number>;
}

interface TemporarySlotUsage {
  numeric: Set<number>;
  object: Set<number>;
  player: Set<number>;
  team: Set<number>;
}

interface VariableMetadataSlots {
  m_numeric_variables: unknown[];
  m_object_variables: unknown[];
  m_player_variables: unknown[];
  m_team_variables: unknown[];
}

function object_global_index(value: number): number | undefined {
  if (
    value >= e_explicit_object_type_tu1.global_0 &&
    value <= e_explicit_object_type_tu1.global_15
  ) {
    return value - e_explicit_object_type_tu1.global_0;
  }
  return;
}

function object_temporary_index(value: number): number | undefined {
  if (
    value >= e_explicit_object_type_mcc.temporary_0 &&
    value <= e_explicit_object_type_mcc.temporary_7
  ) {
    return value - e_explicit_object_type_mcc.temporary_0;
  }
  return;
}

function player_global_index(value: number): number | undefined {
  if (
    value >= e_explicit_player_type_tu1.global_0 &&
    value <= e_explicit_player_type_tu1.global_7
  ) {
    return value - e_explicit_player_type_tu1.global_0;
  }
  return;
}

function player_temporary_index(value: number): number | undefined {
  if (
    value >= e_explicit_player_type_mcc.temporary_0 &&
    value <= e_explicit_player_type_mcc.temporary_2
  ) {
    return value - e_explicit_player_type_mcc.temporary_0;
  }
  return;
}

function team_global_index(value: number): number | undefined {
  if (
    value >= e_explicit_team_type_tu1.global_0 &&
    value <= e_explicit_team_type_tu1.global_7
  ) {
    return value - e_explicit_team_type_tu1.global_0;
  }
  return;
}

function team_temporary_index(value: number): number | undefined {
  if (
    value >= e_explicit_team_type_mcc.temporary_0 &&
    value <= e_explicit_team_type_mcc.temporary_5
  ) {
    return value - e_explicit_team_type_mcc.temporary_0;
  }
  return;
}

function reserve_metadata_slots(
  usage: GlobalSlotUsage,
  metadata: VariableMetadataSlots
): void {
  for (let i = 0; i < metadata.m_object_variables.length; i++) {
    usage.object.add(i);
  }
  for (let i = 0; i < metadata.m_player_variables.length; i++) {
    usage.player.add(i);
  }
  for (let i = 0; i < metadata.m_team_variables.length; i++) {
    usage.team.add(i);
  }
  for (let i = 0; i < metadata.m_numeric_variables.length; i++) {
    usage.numeric.add(i);
  }
}

function collect_slot_usage_from_object(
  obj: JsonRecord,
  globals: GlobalSlotUsage,
  temporaries: TemporarySlotUsage
): void {
  const objectType = obj.m_explicit_object_type;
  if (typeof objectType === "number") {
    const globalIndex = object_global_index(objectType);
    if (globalIndex !== undefined) {
      globals.object.add(globalIndex);
    }
    const temporaryIndex = object_temporary_index(objectType);
    if (temporaryIndex !== undefined) {
      temporaries.object.add(temporaryIndex);
    }
  }

  const playerType = obj.m_explicit_player_type;
  if (typeof playerType === "number") {
    const globalIndex = player_global_index(playerType);
    if (globalIndex !== undefined) {
      globals.player.add(globalIndex);
    }
    const temporaryIndex = player_temporary_index(playerType);
    if (temporaryIndex !== undefined) {
      temporaries.player.add(temporaryIndex);
    }
  }

  const teamType = obj.m_explicit_team_type;
  if (typeof teamType === "number") {
    const globalIndex = team_global_index(teamType);
    if (globalIndex !== undefined) {
      globals.team.add(globalIndex);
    }
    const temporaryIndex = team_temporary_index(teamType);
    if (temporaryIndex !== undefined) {
      temporaries.team.add(temporaryIndex);
    }
  }

  if (
    obj.m_type === e_custom_variable_type_tu1.global_number &&
    typeof obj.m_variable_index === "number"
  ) {
    globals.numeric.add(obj.m_variable_index);
  }

  if (
    obj.m_type === e_custom_variable_type_mcc.temporary_number &&
    typeof obj.m_variable_index === "number"
  ) {
    temporaries.numeric.add(obj.m_variable_index);
  }
}

function collect_slot_usage(root: unknown): {
  globals: GlobalSlotUsage;
  temporaries: TemporarySlotUsage;
} {
  const globals: GlobalSlotUsage = {
    object: new Set(),
    player: new Set(),
    team: new Set(),
    numeric: new Set(),
  };
  const temporaries: TemporarySlotUsage = {
    object: new Set(),
    player: new Set(),
    team: new Set(),
    numeric: new Set(),
  };

  visit_object_tree(root, (obj) =>
    collect_slot_usage_from_object(obj, globals, temporaries)
  );

  return { globals, temporaries };
}

function allocate_global_slot(
  used: Set<number>,
  max: number
): number | undefined {
  for (let index = 0; index < max; index++) {
    if (!used.has(index)) {
      used.add(index);
      return index;
    }
  }
  return;
}

function build_temporary_to_global_maps(
  globals: GlobalSlotUsage,
  temporaries: TemporarySlotUsage
): TemporaryToGlobalMaps | undefined {
  const maps: TemporaryToGlobalMaps = {
    object: new Map(),
    player: new Map(),
    team: new Map(),
    numeric: new Map(),
  };

  for (const temporaryIndex of [...temporaries.object].sort((a, b) => a - b)) {
    const globalIndex = allocate_global_slot(
      globals.object,
      MAX_OBJECT_GLOBALS
    );
    if (globalIndex === undefined) {
      return;
    }
    maps.object.set(temporaryIndex, globalIndex);
  }

  for (const temporaryIndex of [...temporaries.player].sort((a, b) => a - b)) {
    const globalIndex = allocate_global_slot(
      globals.player,
      MAX_PLAYER_GLOBALS
    );
    if (globalIndex === undefined) {
      return;
    }
    maps.player.set(temporaryIndex, globalIndex);
  }

  for (const temporaryIndex of [...temporaries.team].sort((a, b) => a - b)) {
    const globalIndex = allocate_global_slot(globals.team, MAX_TEAM_GLOBALS);
    if (globalIndex === undefined) {
      return;
    }
    maps.team.set(temporaryIndex, globalIndex);
  }

  for (const temporaryIndex of [...temporaries.numeric].sort((a, b) => a - b)) {
    const globalIndex = allocate_global_slot(
      globals.numeric,
      MAX_NUMERIC_GLOBALS
    );
    if (globalIndex === undefined) {
      return;
    }
    maps.numeric.set(temporaryIndex, globalIndex);
  }

  return maps;
}

function get_custom_engine_tu1(
  variant: c_game_variant_tu1
): s_custom_game_engine_definition_tu1 | undefined {
  return variant.m_custom_variant?.m_game_engine;
}

function copy_variable_metadata_mcc_to_tu1(
  from: s_variable_metadata_mcc,
  to: s_variable_metadata_tu1
): void {
  to.m_numeric_variables = from.m_numeric_variables.map(([ref, state]) => [
    mccToTu1Mapper.map(
      ref,
      c_custom_variable_reference_mcc,
      c_custom_variable_reference_tu1
    ),
    state,
  ]);
  to.m_timer_variables = from.m_timer_variables.map((ref) =>
    mccToTu1Mapper.map(
      ref,
      c_custom_variable_reference_mcc,
      c_custom_variable_reference_tu1
    )
  );
  to.m_team_variables = from.m_team_variables.map(([value, state]) => [
    value,
    state,
  ]);
  to.m_player_variables = [...from.m_player_variables];
  to.m_object_variables = [...from.m_object_variables];
}

function copy_variable_metadata_tu1_to_mcc(
  from: s_variable_metadata_tu1,
  to: s_variable_metadata_mcc
): void {
  to.m_numeric_variables = from.m_numeric_variables.map(([ref, state]) => [
    tu1ToMccMapper.map(
      ref,
      c_custom_variable_reference_tu1,
      c_custom_variable_reference_mcc
    ),
    state,
  ]);
  to.m_timer_variables = from.m_timer_variables.map((ref) =>
    tu1ToMccMapper.map(
      ref,
      c_custom_variable_reference_tu1,
      c_custom_variable_reference_mcc
    )
  );
  to.m_team_variables = from.m_team_variables.map(([value, state]) => [
    value,
    state,
  ]);
  to.m_player_variables = [...from.m_player_variables];
  to.m_object_variables = [...from.m_object_variables];
}

function copy_engine_variable_metadata_mcc_to_tu1(
  from: s_custom_game_engine_definition_mcc,
  to: s_custom_game_engine_definition_tu1
): void {
  copy_variable_metadata_mcc_to_tu1(
    from.m_global_variable_metadata,
    to.m_global_variable_metadata
  );
  copy_variable_metadata_mcc_to_tu1(
    from.m_player_variable_metadata,
    to.m_player_variable_metadata
  );
  copy_variable_metadata_mcc_to_tu1(
    from.m_object_variable_metadata,
    to.m_object_variable_metadata
  );
  copy_variable_metadata_mcc_to_tu1(
    from.m_team_variable_metadata,
    to.m_team_variable_metadata
  );
}

function copy_engine_variable_metadata_tu1_to_mcc(
  from: s_custom_game_engine_definition_tu1,
  to: s_custom_game_engine_definition_mcc
): void {
  copy_variable_metadata_tu1_to_mcc(
    from.m_global_variable_metadata,
    to.m_global_variable_metadata
  );
  copy_variable_metadata_tu1_to_mcc(
    from.m_player_variable_metadata,
    to.m_player_variable_metadata
  );
  copy_variable_metadata_tu1_to_mcc(
    from.m_object_variable_metadata,
    to.m_object_variable_metadata
  );
  copy_variable_metadata_tu1_to_mcc(
    from.m_team_variable_metadata,
    to.m_team_variable_metadata
  );
}

function get_custom_engine(
  variant: c_game_variant_mcc
): s_custom_game_engine_definition_mcc | undefined {
  return variant.m_custom_variant?.m_game_engine;
}

function apply_mcc_to_tu1_megalo_remapping(
  variant: c_game_variant_tu1,
  maps: TemporaryToGlobalMaps
): void {
  const engine = variant.m_custom_variant?.m_game_engine;
  if (!engine) {
    return;
  }
  remap_temporary_explicit_refs(engine, maps);
  extend_global_metadata(engine.m_global_variable_metadata, maps);
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

function variant_uses_mcc_only_action_types(
  variant: c_game_variant_mcc
): boolean {
  let found = false;
  visit_object_tree(variant, (obj) => {
    if (found) {
      return;
    }
    const actionType = obj.m_type;
    if (
      typeof actionType === "number" &&
      MCC_ONLY_ACTION_TYPES.includes(actionType)
    ) {
      found = true;
    }
  });
  return found;
}

function plan_temporary_relocation(
  variant: c_game_variant_mcc
): TemporaryToGlobalMaps | undefined {
  const engine = get_custom_engine(variant);
  if (!engine) {
    return {
      object: new Map(),
      player: new Map(),
      team: new Map(),
      numeric: new Map(),
    };
  }

  const { globals, temporaries } = collect_slot_usage(engine);
  reserve_metadata_slots(globals, engine.m_global_variable_metadata);

  if (
    temporaries.object.size === 0 &&
    temporaries.player.size === 0 &&
    temporaries.team.size === 0 &&
    temporaries.numeric.size === 0
  ) {
    return {
      object: new Map(),
      player: new Map(),
      team: new Map(),
      numeric: new Map(),
    };
  }

  return build_temporary_to_global_maps(globals, temporaries);
}

function check_mcc_game_variant_to_tu1(
  from: c_game_variant_mcc
): e_reach_gametype_conversion_error {
  if (from.m_game_engine === e_game_mode_mcc.sandbox) {
    return e_reach_gametype_conversion_error.forge_variant;
  }
  if (from.m_game_engine === e_game_mode_mcc.campaign) {
    return e_reach_gametype_conversion_error.campaign_variant;
  }

  if (
    from.m_game_engine === e_game_mode_mcc.survival &&
    from.m_survival_variant !== undefined &&
    from.m_survival_variant.m_additional_flags !== 0
  ) {
    return e_reach_gametype_conversion_error.mcc_survival_additional_flags;
  }

  if (variant_uses_mcc_only_action_types(from)) {
    return e_reach_gametype_conversion_error.mcc_exclusive_action;
  }
  if (variant_uses_mcc_only_math_ops(from)) {
    return e_reach_gametype_conversion_error.mcc_exclusive_math_operator;
  }
  if (plan_temporary_relocation(from) === undefined) {
    return e_reach_gametype_conversion_error.insufficient_global_slots;
  }

  return e_reach_gametype_conversion_error.ok;
}

function check_tu1_game_variant_to_mcc(
  from: c_game_variant_tu1
): e_reach_gametype_conversion_error {
  if (from.m_game_engine === e_game_mode_tu1.sandbox) {
    return e_reach_gametype_conversion_error.forge_variant;
  }
  if (from.m_game_engine === e_game_mode_tu1.campaign) {
    return e_reach_gametype_conversion_error.campaign_variant;
  }
  return e_reach_gametype_conversion_error.ok;
}

function convert_mcc_to_tu1(
  from: c_game_variant_mcc,
  to: c_game_variant_tu1
): e_reach_gametype_conversion_error {
  const error = check_mcc_game_variant_to_tu1(from);
  if (error !== e_reach_gametype_conversion_error.ok) {
    return error;
  }

  const maps = plan_temporary_relocation(from);
  if (maps === undefined) {
    return e_reach_gametype_conversion_error.insufficient_global_slots;
  }
  const converted = map_reach_gametype_mcc_to_tu1(from);
  const sourceEngine = get_custom_engine(from);
  const targetEngine = converted.m_custom_variant?.m_game_engine;
  if (sourceEngine !== undefined && targetEngine !== undefined) {
    copy_engine_variable_metadata_mcc_to_tu1(sourceEngine, targetEngine);
  }
  apply_mcc_to_tu1_megalo_remapping(converted, maps);
  apply_mcc_to_tu1_name_description(from, converted);
  apply_survival_mcc_to_tu1(converted.m_survival_variant);

  Object.assign(to, converted);
  return e_reach_gametype_conversion_error.ok;
}

function convert_tu1_to_mcc(
  from: c_game_variant_tu1,
  to: c_game_variant_mcc
): e_reach_gametype_conversion_error {
  const error = check_tu1_game_variant_to_mcc(from);
  if (error !== e_reach_gametype_conversion_error.ok) {
    return error;
  }

  const converted = map_reach_gametype_tu1_to_mcc(from);
  const sourceEngine = get_custom_engine_tu1(from);
  const targetEngine = converted.m_custom_variant?.m_game_engine;
  if (sourceEngine !== undefined && targetEngine !== undefined) {
    copy_engine_variable_metadata_tu1_to_mcc(sourceEngine, targetEngine);
  }
  apply_survival_tu1_to_mcc(converted.m_survival_variant);

  Object.assign(to, converted);
  return e_reach_gametype_conversion_error.ok;
}

function assert_cross_version(
  from: c_game_variant_tu1 | c_game_variant_mcc,
  to: c_game_variant_tu1 | c_game_variant_mcc
): void {
  const from_is_mcc = from instanceof c_game_variant_mcc;
  const to_is_mcc = to instanceof c_game_variant_mcc;
  if (from_is_mcc === to_is_mcc) {
    throw new BlfError(
      "convert_reach_gametype only supports cross-version conversion between Reach TU1 and Reach MCC"
    );
  }
}

export function convert_reach_gametype(
  from: c_game_variant_mcc,
  to: c_game_variant_tu1
): e_reach_gametype_conversion_error;
export function convert_reach_gametype(
  from: c_game_variant_tu1,
  to: c_game_variant_mcc
): e_reach_gametype_conversion_error;
export function convert_reach_gametype(
  from: c_game_variant_tu1 | c_game_variant_mcc,
  to: c_game_variant_tu1 | c_game_variant_mcc
): e_reach_gametype_conversion_error {
  assert_cross_version(from, to);
  if (from instanceof c_game_variant_mcc) {
    return convert_mcc_to_tu1(from, to as c_game_variant_tu1);
  }
  return convert_tu1_to_mcc(
    from as c_game_variant_tu1,
    to as c_game_variant_mcc
  );
}
