# Megalo MCC changes

Halo: Reach on MCC uses a newer megalo scripting build than Xbox 360 Title Update 1. This library models both under separate version bundles and documents the differences here.

**Import paths**

| Build | Bundle |
|-------|--------|
| Xbox 360 TU1 | `@blamnetwork/blf/haloreach/v12065_11_08_24_1738_tu1actual` |
| Reach MCC (Aug 2025) | `@blamnetwork/blf/haloreach_mcc/v_untracked_25_08_16_1352` |

Cross-build conversion: [`convert_reach_gametype`](/guide/converting-reach-gametypes) on `@blamnetwork/blf/helpers`.

## Version bundle renames

If you were on older `@blamnetwork/blf` import paths, update as follows:

| Old | New |
|-----|-----|
| `@blamnetwork/blf/haloreach_mcc/v_untracked_25_08_19_1352` | `@blamnetwork/blf/haloreach_mcc/v_untracked_25_08_16_1352` |
| `@blamnetwork/blf/mcc/v_25_08_16` | `@blamnetwork/blf/mcc/v2025_08_16_178512_1_release` |

The Reach MCC rename reflects the build date (16 Aug 2025). The MCC menu bundle now uses the full MCC menu build string (`2025.08.16.178512.1-Release`).

## `e_action_type` naming

Action type enum members use the **in-game string table** names (snake_case), not internal C++ struct names. For example:

| Value | Name |
|------:|------|
| 2 | `create_object` (not `place_at_me`) |
| 9 | `set` (not `modify_variable`) |
| 23 | `object_destroy` (not `kill_object_instantly`) |
| 46 | `hud_widget_set_text` (not `set_text`) |

Both Rust (`blf_lib`) and TypeScript export `e_action_type` from `megalogamengine_actions`. Values `0`–`98` match between TU1 and MCC; MCC adds `99`–`106` (see below).

TypeScript example:

```ts
import { e_action_type } from "@blamnetwork/blf/haloreach_mcc/v_untracked_25_08_16_1352";

if (action.m_type === e_action_type.object_destroy) {
  // ...
}
```

## MCC-only megalo features (vs TU1)

These exist in the MCC build but not in TU1:

### Math operators

MCC adds bit-shift assignment operators on megalo variables:

- `shift_left_with` (`<<=`)
- `shift_right_with` (`>>=`)

`set_to_absolute` exists in both builds but its enum value moved (TU1 = `10`, MCC = `12`). `convert_reach_gametype` remaps this automatically.

### Temporary explicit references

MCC adds temporary object, player, and team reference types used in conditions and actions. Gametypes that reference them cannot be converted to TU1.

### Survival / firefight

- MCC survival variants may set `m_additional_flags` (e.g. **Network Test 1**). Non-zero values block MCC → TU1 conversion.
- Survival encoding differs between builds; the converter sets TU1/MCC-specific defaults for unrelated fields.

### Action types

Reach MCC adds eight action types at the end of the megalo action table. They are **not** present in TU1 and will block MCC → TU1 conversion if used in a gametype.

| `e_action_type` | Payload |
|-----------------|---------|
| `begin` | *(none)* |
| `hs_function_call` | `function-name-index` (8 bits, stored as wire value − 1) |
| `get_button_time` | player reference, `buttons` (5 bits), custom variable reference |
| `team_set_vehicle_spawning` | team reference, `enabled` bool |
| `player_set_vehicle_spawning` | player reference, `enabled` bool |
| `set_player_respawn_vehicle` | object type reference, player reference |
| `set_team_respawn_vehicle` | object type reference, team reference |
| `hide_object` | object reference, `should hide` bool |

Parameter structs live in `megalogamengine_actions` (`s_action_hs_function_call_parameters`, etc.) and are wired through `c_action` decode/encode in the MCC bundle only.

## Related types

Megalo types for a given build live under:

- `src/blam/haloreach/.../game/megalogamengine/` (TU1)
- `src/blam/haloreach_mcc/.../game/megalogamengine/` (MCC)

Key exports: `c_action`, `c_condition`, `c_trigger`, `c_game_variant` (via `c_game_engine_custom_variant`).
