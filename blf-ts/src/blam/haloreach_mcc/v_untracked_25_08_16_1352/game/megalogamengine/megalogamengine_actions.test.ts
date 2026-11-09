import { describe, expect, it } from "vitest";
import {
  c_bitstream_reader,
  c_bitstream_writer,
  e_bitstream_byte_order,
} from "../../../../../bitstream";
import {
  c_action,
  e_action_type,
  e_scriptable_game_buttons,
  s_action_get_button_time_parameters,
  s_action_hide_object_parameters,
  s_action_hs_function_call_parameters,
  s_action_player_set_vehicle_spawning_parameters,
  s_action_set_player_respawn_vehicle_parameters,
  s_action_set_team_respawn_vehicle_parameters,
  s_action_team_set_vehicle_spawning_parameters,
} from "./megalogamengine_actions";
import {
  c_custom_variable_reference,
  c_explicit_object,
  c_explicit_player,
  c_explicit_team,
  c_object_reference,
  c_player_reference,
  c_team_reference,
} from "./megalogamengine_references";

const BE = e_bitstream_byte_order._bitstream_byte_order_big_endian;
const k_trailer = 0xa5;

function roundtrip(action: c_action): c_action {
  const writer = c_bitstream_writer.new(0x1000, BE);
  writer.begin_writing();
  action.encode(writer);
  writer.write_integer(k_trailer, 8);
  writer.finish_writing();

  const reader = c_bitstream_reader.new(writer.get_data(), BE);
  reader.begin_reading();
  const decoded = new c_action();
  decoded.decode(reader);
  expect(reader.read_integer("trailer", 8)).toBe(k_trailer);
  reader.finish_reading();
  return decoded;
}

function player_reference(): c_player_reference {
  const reference = new c_player_reference();
  reference.m_player = new c_explicit_player();
  return reference;
}

function team_reference(): c_team_reference {
  const reference = new c_team_reference();
  reference.m_team = new c_explicit_team();
  return reference;
}

function object_reference(): c_object_reference {
  const reference = new c_object_reference();
  reference.m_object = new c_explicit_object();
  return reference;
}

function constant_reference(value: number): c_custom_variable_reference {
  const reference = new c_custom_variable_reference();
  reference.m_immediate_value = value;
  return reference;
}

function make_action(
  type: e_action_type,
  assign: (action: c_action) => void
): c_action {
  const action = new c_action();
  action.m_type = type;
  assign(action);
  return action;
}

describe("c_action (MCC) MCC-only action types", () => {
  const cases: [string, c_action][] = [
    [
      "hs_function_call",
      make_action(e_action_type.hs_function_call, (action) => {
        const parameters = new s_action_hs_function_call_parameters();
        parameters.m_function_name_index = 5;
        action.m_hs_function_call_parameters = parameters;
      }),
    ],
    [
      "get_button_time",
      make_action(e_action_type.get_button_time, (action) => {
        const parameters = new s_action_get_button_time_parameters();
        parameters.m_player = player_reference();
        parameters.m_buttons = e_scriptable_game_buttons.crouch;
        parameters.m_variable = constant_reference(7);
        action.m_get_button_time_parameters = parameters;
      }),
    ],
    [
      "team_set_vehicle_spawning",
      make_action(e_action_type.team_set_vehicle_spawning, (action) => {
        const parameters = new s_action_team_set_vehicle_spawning_parameters();
        parameters.m_team = team_reference();
        parameters.m_enabled = true;
        action.m_team_set_vehicle_spawning_parameters = parameters;
      }),
    ],
    [
      "player_set_vehicle_spawning",
      make_action(e_action_type.player_set_vehicle_spawning, (action) => {
        const parameters =
          new s_action_player_set_vehicle_spawning_parameters();
        parameters.m_player = player_reference();
        parameters.m_enabled = true;
        action.m_player_set_vehicle_spawning_parameters = parameters;
      }),
    ],
    [
      "set_player_respawn_vehicle",
      make_action(e_action_type.set_player_respawn_vehicle, (action) => {
        const parameters = new s_action_set_player_respawn_vehicle_parameters();
        parameters.m_player = player_reference();
        action.m_set_player_respawn_vehicle_parameters = parameters;
      }),
    ],
    [
      "set_team_respawn_vehicle",
      make_action(e_action_type.set_team_respawn_vehicle, (action) => {
        const parameters = new s_action_set_team_respawn_vehicle_parameters();
        parameters.m_team = team_reference();
        action.m_set_team_respawn_vehicle_parameters = parameters;
      }),
    ],
    [
      "hide_object",
      make_action(e_action_type.hide_object, (action) => {
        const parameters = new s_action_hide_object_parameters();
        parameters.m_object = object_reference();
        parameters.m_should_hide = true;
        action.m_hide_object_parameters = parameters;
      }),
    ],
  ];

  it.each(cases)("%s round-trips", (_name, action) => {
    expect(roundtrip(action)).toEqual(action);
  });
});
