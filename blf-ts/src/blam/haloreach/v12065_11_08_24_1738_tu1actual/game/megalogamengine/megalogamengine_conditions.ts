import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../../bitstream";
import {
  c_custom_timer_reference,
  c_object_reference,
  c_object_type_reference,
  c_player_reference,
  c_team_reference,
} from "./megalogamengine_references";
import { s_variant_variable } from "./s_variant_variable";

export class s_condition_if_parameters {
  m_left = new s_variant_variable();
  m_right = new s_variant_variable();
  m_comparison = 0;

  decode(bitstream: c_bitstream_reader): void {
    this.m_left.decode(bitstream);
    this.m_right.decode(bitstream);
    this.m_comparison = bitstream.read_integer("comparison", 3);
  }

  encode(bitstream: c_bitstream_writer): void {
    this.m_left.encode(bitstream);
    this.m_right.encode(bitstream);
    bitstream.write_integer(this.m_comparison, 3);
  }

}

export class s_condition_player_died_parameters {
  m_player = new c_player_reference();
  m_killer_type = 0;

  decode(bitstream: c_bitstream_reader): void {
    this.m_player.decode(bitstream);
    this.m_killer_type = bitstream.read_integer("killer-type", 5);
  }

  encode(bitstream: c_bitstream_writer): void {
    this.m_player.encode(bitstream);
    bitstream.write_integer(this.m_killer_type, 5);
  }

}

export class s_condition_team_disposition_parameters {
  m_team_1 = new c_team_reference();
  m_team_2 = new c_team_reference();
  m_disposition = 0;

  decode(bitstream: c_bitstream_reader): void {
    this.m_team_1.decode(bitstream);
    this.m_team_2.decode(bitstream);
    this.m_disposition = bitstream.read_integer("disposition", 2);
  }

  encode(bitstream: c_bitstream_writer): void {
    this.m_team_1.encode(bitstream);
    this.m_team_2.encode(bitstream);
    bitstream.write_integer(this.m_disposition, 2);
  }

}

export class s_condition_object_matches_filter_parameters {
  m_object = new c_object_reference();
  m_filter_index = 0;

  decode(bitstream: c_bitstream_reader): void {
    this.m_object.decode(bitstream);
    this.m_filter_index = bitstream.read_index("filter-index", 16, 4);
  }

  encode(bitstream: c_bitstream_writer): void {
    this.m_object.encode(bitstream);
        bitstream.write_index(this.m_filter_index, 16, 4);
  }

}

export class c_condition {
  m_type = 0;
  m_negated = false;
  m_union_group = 0;
  m_execute_before_action = 0;
  m_if_parameters?: s_condition_if_parameters;
  m_object_reference_1?: c_object_reference;
  m_object_reference_2?: c_object_reference;
  m_player_died_parameters?: s_condition_player_died_parameters;
  m_timer?: c_custom_timer_reference;
  m_team_disposition_parameters?: s_condition_team_disposition_parameters;
  m_object_type_reference?: c_object_type_reference;
  m_team_reference?: c_team_reference;
  m_player_reference_1?: c_player_reference;
  m_player_reference_2?: c_player_reference;
  m_object_matches_filter_parameters?: s_condition_object_matches_filter_parameters;

  decode(bitstream: c_bitstream_reader): void {
    this.m_type = bitstream.read_integer("type", 5);
    if (this.m_type === 0) {
      return;
    }

    this.m_negated = bitstream.read_bool("negated");
    this.m_union_group = bitstream.read_integer("union-group", 9);
    this.m_execute_before_action = bitstream.read_integer(
      "execute-before-action",
      10,
    );

    switch (this.m_type) {
      case 1: {
        const params = new s_condition_if_parameters();
        params.decode(bitstream);
        this.m_if_parameters = params;
        break;
      }
      case 2: {
        const object1 = new c_object_reference();
        const object2 = new c_object_reference();
        object1.decode(bitstream);
        object2.decode(bitstream);
        this.m_object_reference_1 = object1;
        this.m_object_reference_2 = object2;
        break;
      }
      case 3: {
        const params = new s_condition_player_died_parameters();
        params.decode(bitstream);
        this.m_player_died_parameters = params;
        break;
      }
      case 4: {
        const params = new s_condition_team_disposition_parameters();
        params.decode(bitstream);
        this.m_team_disposition_parameters = params;
        break;
      }
      case 5: {
        const timer = new c_custom_timer_reference();
        timer.decode(bitstream);
        this.m_timer = timer;
        break;
      }
      case 6: {
        const object = new c_object_reference();
        const object_type = new c_object_type_reference();
        object.decode(bitstream);
        object_type.decode(bitstream);
        this.m_object_reference_1 = object;
        this.m_object_type_reference = object_type;
        break;
      }
      case 7: {
        const team = new c_team_reference();
        team.decode(bitstream);
        this.m_team_reference = team;
        break;
      }
      case 8:
      case 13: {
        const object = new c_object_reference();
        object.decode(bitstream);
        this.m_object_reference_1 = object;
        break;
      }
      case 9:
      case 12:
      case 14:
      case 15:
      case 16: {
        const player = new c_player_reference();
        player.decode(bitstream);
        this.m_player_reference_1 = player;
        break;
      }
      case 10: {
        const player1 = new c_player_reference();
        const player2 = new c_player_reference();
        player1.decode(bitstream);
        player2.decode(bitstream);
        this.m_player_reference_1 = player1;
        this.m_player_reference_2 = player2;
        break;
      }
      case 11: {
        const params = new s_condition_object_matches_filter_parameters();
        params.decode(bitstream);
        this.m_object_matches_filter_parameters = params;
        break;
      }
    }
  }

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_type, 5);
    if (this.m_type === 0) {
      return;
    }
    bitstream.write_bool(this.m_negated);
    bitstream.write_integer(this.m_union_group, 9);
    bitstream.write_integer(this.m_execute_before_action, 10);
    switch (this.m_type) {
      case 1:
        this.m_if_parameters!.encode(bitstream);
        break;
      case 2:
        this.m_object_reference_1!.encode(bitstream);
        this.m_object_reference_2!.encode(bitstream);
        break;
      case 3:
        this.m_player_died_parameters!.encode(bitstream);
        break;
      case 4:
        this.m_team_disposition_parameters!.encode(bitstream);
        break;
      case 5:
        this.m_timer!.encode(bitstream);
        break;
      case 6:
        this.m_object_reference_1!.encode(bitstream);
        this.m_object_type_reference!.encode(bitstream);
        break;
      case 7:
        this.m_team_reference!.encode(bitstream);
        break;
      case 8:
      case 13:
        this.m_object_reference_1!.encode(bitstream);
        break;
      case 9:
      case 12:
      case 14:
      case 15:
      case 16:
        this.m_player_reference_1!.encode(bitstream);
        break;
      case 10:
        this.m_player_reference_1!.encode(bitstream);
        this.m_player_reference_2!.encode(bitstream);
        break;
      case 11:
        this.m_object_matches_filter_parameters!.encode(bitstream);
        break;
    }
  }

}
