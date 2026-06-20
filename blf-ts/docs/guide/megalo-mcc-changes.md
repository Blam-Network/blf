# Megalo MCC changes

Halo: Reach on MCC uses a newer megalo scripting build than Xbox 360 Title Update 1. This library models both under separate version bundles and documents the differences here.

**Import paths**

| Build | Bundle |
|-------|--------|
| Xbox 360 TU1 | `@blamnetwork/blf/haloreach/v12065_11_08_24_1738_tu1actual` |
| Reach MCC (Aug 2025) | `@blamnetwork/blf/haloreach_mcc/v_untracked_25_08_16_1352` |

Cross-build conversion: [`convert_reach_gametype`](/guide/converting-reach-gametypes) on `@blamnetwork/blf/helpers`.

## MCC-only megalo features (vs Xbox 360)

These exist in the MCC build but not in Xbox 360:

### Math operators

MCC adds bit-shift assignment operators on megalo variables:

- `shift_left_with` (`<<=`)
- `shift_right_with` (`>>=`)

`set_to_absolute` exists in both builds but its enum value moved (TU1 = `10`, MCC = `12`).

### Temporary explicit references

MCC adds temporary object, player, and team reference types used in conditions and actions.
These can often be mapped to unused Global variables for Xbox 360, if enough space is available.

### Survival / firefight

- MCC survival variants may set `m_additional_flags` (e.g. **Network Test 1**). Non-zero values block MCC → TU1 conversion.
- Survival encoding differs between builds; the converter sets TU1/MCC-specific defaults for unrelated fields.

### Action types

Reach MCC adds eight action types at the end of the megalo action table. They are **not** present in TU1 and will block MCC → TU1 conversion if used in a gametype.

| `e_action_type` | Payload |
|-----------------|---------|
| `begin` | *(none)* |
| `hs_function_call` | `function-name-index` (8 bits) |
| `get_button_time` | player reference, `e_scriptable_game_buttons` (5 bits), custom variable reference |
| `team_set_vehicle_spawning` | team reference, `enabled` bool |
| `player_set_vehicle_spawning` | player reference, `enabled` bool |
| `set_player_respawn_vehicle` | object type reference, player reference |
| `set_team_respawn_vehicle` | object type reference, team reference |
| `hide_object` | object reference, `should hide` bool |


