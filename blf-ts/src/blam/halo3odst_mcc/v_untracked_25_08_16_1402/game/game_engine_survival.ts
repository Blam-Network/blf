import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../bitstream";
import {
  type BitfieldOf,
  bitfieldFromRawQword,
  bitfieldToRawQword,
} from "../../../../bitstream";
import { AutoMap } from "../../../../helpers/automap";
import { c_game_engine_base_variant } from "./game_engine_default";
import { c_player_traits } from "./game_engine_player_traits";

export const e_game_skulls = [
  // primary (0–8)
  "iron",
  "black_eye",
  "tough_luck",
  "catch",
  "fog",
  "famine",
  "thunderstorm",
  "tilt",
  "mythic",
  // secondary (9–15)
  "assassin",
  "blind",
  "superman",
  "grunt_birthday_party",
  "iwhbyd",
  "third_person",
  "directors_cut",
  // custom (16–21)
  "custom_red",
  "custom_yellow",
  "custom_blue",
  "custom_green",
  "custom_white",
  "custom_black",
  // mcc (22–45)
  "anger",
  "bandanna",
  "bonded_pair",
  "boom",
  "envy",
  "eye_patch",
  "feather",
  "foreign",
  "ghost",
  "grunt_funeral",
  "jacked",
  "malfunction",
  "masterblaster",
  "pinata",
  "prophet_birthday_party",
  "recession",
  "scarab",
  "so_angry",
  "sputnik",
  "streaking",
  "swarm",
  "thats_just_wrong",
  "they_come_back",
  "boots_off_the_ground",
] as const;

export type e_game_skulls = BitfieldOf<typeof e_game_skulls>;

function defaultGameSkulls(): e_game_skulls {
  return bitfieldFromRawQword(0n, e_game_skulls);
}

function readGameSkulls(bitstream: c_bitstream_reader): e_game_skulls {
  return bitfieldFromRawQword(bitstream.read_qword(64), e_game_skulls);
}

function writeGameSkulls(
  bitstream: c_bitstream_writer,
  value: e_game_skulls
): void {
  bitstream.write_qword(bitfieldToRawQword(value, e_game_skulls), 64);
}

export class s_survival_wave_properties {
  @AutoMap(() => Number)
  m_wave_flags = 0;
  @AutoMap(() => Number)
  m_wave_squad_advance_type = 0;
  @AutoMap(() => Number)
  m_wave_squad_count = 0;
  @AutoMap(() => [Number])
  m_squads: number[] = Array.from({ length: 5 }, () => -1);

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_wave_flags, 8);
    bitstream.write_integer(this.m_wave_squad_advance_type, 1);
    bitstream.write_signed_integer(this.m_wave_squad_count, 8);
    for (let i = 0; i < 5; i++) {
      const squad = this.m_squads[i] ?? -1;
      const unused = squad === -1;
      bitstream.write_bool(unused);
      if (!unused) {
        bitstream.write_integer(squad, 7);
      }
    }
  }

  decode(bitstream: c_bitstream_reader): void {
    this.m_wave_flags = bitstream.read_integer("wave_flags", 8);
    this.m_wave_squad_advance_type = bitstream.read_integer(
      "wave_squad_advance_type",
      1
    );
    this.m_wave_squad_count = bitstream.read_signed_integer(
      "wave-squad-count",
      8
    );
    for (let i = 0; i < 5; i++) {
      const unused = bitstream.read_bool("wave-squad-unused");
      this.m_squads[i] = unused
        ? -1
        : bitstream.read_integer("possible-wave-squads", 7);
    }
  }
}

export class s_survival_round_properties {
  @AutoMap(() => Object)
  m_skulls: e_game_skulls = defaultGameSkulls();
  @AutoMap(() => s_survival_wave_properties)
  m_initial_wave = new s_survival_wave_properties();
  @AutoMap(() => s_survival_wave_properties)
  m_primary_wave = new s_survival_wave_properties();
  @AutoMap(() => s_survival_wave_properties)
  m_boss_wave = new s_survival_wave_properties();

  encode(bitstream: c_bitstream_writer): void {
    writeGameSkulls(bitstream, this.m_skulls);
    this.m_initial_wave.encode(bitstream);
    this.m_primary_wave.encode(bitstream);
    this.m_boss_wave.encode(bitstream);
  }

  decode(bitstream: c_bitstream_reader): void {
    this.m_skulls = readGameSkulls(bitstream);
    this.m_initial_wave.decode(bitstream);
    this.m_primary_wave.decode(bitstream);
    this.m_boss_wave.decode(bitstream);
  }
}

export class c_game_engine_survival_variant {
  @AutoMap(() => Number)
  m_encoding_version = 0;
  @AutoMap(() => c_game_engine_base_variant)
  m_base_variant = new c_game_engine_base_variant();
  @AutoMap(() => Number)
  m_flags = 0;
  @AutoMap(() => Number)
  m_maximum_lives = 0;
  @AutoMap(() => Number)
  m_set_count = 0;
  @AutoMap(() => Number)
  m_shared_team_life_count = 0;
  @AutoMap(() => Object)
  m_initial_skulls: e_game_skulls = defaultGameSkulls();
  @AutoMap(() => c_player_traits)
  m_player_traits = new c_player_traits();
  @AutoMap(() => [s_survival_round_properties])
  m_rounds: s_survival_round_properties[] = Array.from(
    { length: 3 },
    () => new s_survival_round_properties()
  );
  @AutoMap(() => [Object])
  m_tier_skulls: e_game_skulls[] = Array.from({ length: 4 }, () =>
    defaultGameSkulls()
  );
  @AutoMap(() => Number)
  m_bonus_duration_seconds = 0;
  @AutoMap(() => Object)
  m_bonus_skulls: e_game_skulls = defaultGameSkulls();
  @AutoMap(() => s_survival_wave_properties)
  m_bonus_wave = new s_survival_wave_properties();

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_encoding_version, 8);
    this.m_base_variant.encode(bitstream);
    bitstream.write_integer(this.m_flags, 16);
    bitstream.write_signed_integer(this.m_maximum_lives, 8);
    bitstream.write_integer(this.m_set_count, 8);
    bitstream.write_signed_integer(this.m_shared_team_life_count, 8);
    writeGameSkulls(bitstream, this.m_initial_skulls);
    this.m_player_traits.encode(bitstream);

    if (this.m_encoding_version >= 2) {
      for (let i = 0; i < 3; i++) {
        this.m_rounds[i]!.encode(bitstream);
      }
      for (let i = 0; i < 4; i++) {
        writeGameSkulls(bitstream, this.m_tier_skulls[i]!);
      }
      bitstream.write_signed_integer(this.m_bonus_duration_seconds, 16);
      writeGameSkulls(bitstream, this.m_bonus_skulls);
      this.m_bonus_wave.encode(bitstream);
    }
  }

  decode(bitstream: c_bitstream_reader): void {
    this.m_encoding_version = bitstream.read_integer("encoding-version", 8);
    this.m_base_variant.decode(bitstream);
    this.m_flags = bitstream.read_integer("flags", 16);
    this.m_maximum_lives = bitstream.read_signed_integer("maximum-lives", 8);
    this.m_set_count = bitstream.read_integer("set-count", 8);
    this.m_shared_team_life_count = bitstream.read_signed_integer(
      "shared-team-life-count",
      8
    );
    this.m_initial_skulls = readGameSkulls(bitstream);
    this.m_player_traits.decode(bitstream);

    if (this.m_encoding_version >= 2) {
      for (let i = 0; i < 3; i++) {
        this.m_rounds[i]!.decode(bitstream);
      }
      for (let i = 0; i < 4; i++) {
        this.m_tier_skulls[i] = readGameSkulls(bitstream);
      }
      this.m_bonus_duration_seconds = bitstream.read_signed_integer(
        "duration-seconds",
        16
      );
      this.m_bonus_skulls = readGameSkulls(bitstream);
      this.m_bonus_wave.decode(bitstream);
    }
  }
}
