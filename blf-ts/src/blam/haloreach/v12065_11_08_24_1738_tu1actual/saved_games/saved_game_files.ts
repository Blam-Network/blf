import { c } from "@craftycodie/cstruct";
import { c_bitstream_reader, c_bitstream_writer } from "@Blam-Network/blf/bitstream";

export enum e_file_type {
  Screenshot = 2,
  Film = 3,
  FilmClip = 4,
  MapVariant = 5,
  GameVariant = 6,
}

@c.struct()
export class s_content_item_history {
  @c.field(c.Time64())
  timestamp!: Date;

  @c.field("u64")
  xuid!: bigint;

  @c.field(c.String(16))
  name!: string;

  @c.field("u8", { pad_after: 3 })
  is_online!: number;
}

@c.struct()
export class s_content_item_general_metadata {
  @c.field(c.enum("i8", e_file_type), { pad_after: 3 })
  file_type!: e_file_type;

  @c.field("u32")
  size_in_bytes!: number;

  @c.field("u64")
  unique_id!: bigint;

  @c.field("u64")
  parent_unique_id!: bigint;

  @c.field("u64")
  root_unique_id!: bigint;

  @c.field("u64")
  game_id!: bigint;

  @c.field("i8")
  activity!: number;

  @c.field("u8")
  game_mode!: number;

  @c.field("u8", { pad_after: 1 })
  game_engine_type!: number;

  @c.field("i32")
  map_id!: number;
}

@c.struct()
export class s_content_item_display_metadata {
  @c.field("i8", { pad_after: 7 })
  megalo_category_index!: number;
}

@c.struct()
export class s_content_item_film_metadata {
  @c.field("i32")
  seconds!: number;
}

@c.struct()
export class s_content_item_game_variant_metadata {
  @c.field("i8")
  icon_index!: number;
}

@c.struct()
export class s_content_item_matchmaking_metadata {
  @c.field("u16")
  hopper_identifier!: number;
}

@c.struct()
export class s_content_item_campaign_metadata {
  @c.field("i32")
  campaign_id!: number;

  @c.field("i16")
  campaign_difficulty!: number;

  @c.field("i16")
  campaign_metagame_scoring!: number;

  @c.field("i32")
  campaign_insertion_point!: number;

  @c.field("i16")
  campaign_primary_skulls!: number;

  @c.field("i16")
  campaign_secondary_skulls!: number;
}

@c.struct()
export class s_content_item_firefight_metadata {
  @c.field("i16")
  firefight_difficulty!: number;

  @c.field("i16")
  firefight_primary_skulls!: number;

  @c.field("i16")
  firefight_secondary_skulls!: number;
}

/** Full `c_content_item_metadata` binrw layout (fixed prefix + conditional union tails). */
@c.struct()
export class s_content_item_metadata {
  @c.field(s_content_item_general_metadata)
  general!: s_content_item_general_metadata;

  @c.field(s_content_item_display_metadata)
  display!: s_content_item_display_metadata;

  @c.field(s_content_item_history)
  creation_history!: s_content_item_history;

  @c.field(s_content_item_history)
  modification_history!: s_content_item_history;

  @c.field(c.WString(0x80))
  name!: string;

  @c.field(c.WString(0x80))
  description!: string;

  @c.union(
    { size: 16 },
    c.when(
      e_file_type.Film,
      s_content_item_film_metadata,
      (m: s_content_item_metadata) => m.general.file_type,
    ),
    c.when(
      e_file_type.GameVariant,
      s_content_item_game_variant_metadata,
      (m: s_content_item_metadata) => m.general.file_type,
    ),
  )
  file_type_data:
    | s_content_item_film_metadata
    | s_content_item_game_variant_metadata
    | null = null;

  @c.union(
    { size: 16 },
    c.arm(s_content_item_matchmaking_metadata, (m: s_content_item_metadata) =>
      m.general.activity === 3,
    ),
  )
  activity_data: s_content_item_matchmaking_metadata | null = null;

  @c.union(
    { size: 16 },
    c.arm(s_content_item_campaign_metadata, (m: s_content_item_metadata) =>
      m.general.game_mode === 1,
    ),
    c.arm(s_content_item_firefight_metadata, (m: s_content_item_metadata) =>
      m.general.game_mode === 2,
    ),
  )
  game_mode_data:
    | s_content_item_campaign_metadata
    | s_content_item_firefight_metadata
    | null = null;
}



export function content_item_metadata_decode(
  bitstream: c_bitstream_reader,
  metadata: s_content_item_metadata,
): void {
  metadata.general.file_type =
    (bitstream.read_integer("type", 4) - 1) as e_file_type;
  metadata.general.size_in_bytes = bitstream.read_integer("file-size", 32);
  metadata.general.unique_id = bitstream.read_qword(64);
  metadata.general.parent_unique_id = bitstream.read_qword(64);
  metadata.general.root_unique_id = bitstream.read_qword(64);
  metadata.general.game_id = bitstream.read_qword(64);
  metadata.general.activity = bitstream.read_integer("activity", 3) - 1;
  metadata.general.game_mode = bitstream.read_integer("game-mode", 3);
  metadata.general.game_engine_type = bitstream.read_integer(
    "game-engine-type",
    3,
  );
  metadata.general.map_id = bitstream.read_signed_integer("map-id", 32);
  metadata.display.megalo_category_index = bitstream.read_signed_integer(
    "megalo-category-index",
    8,
  );
  metadata.creation_history.timestamp = new Date(
    Number(bitstream.read_qword(64)) * 1000,
  );
  metadata.creation_history.xuid = bitstream.read_qword(64);
  metadata.creation_history.name = bitstream.read_string_extended_ascii(16);
  metadata.creation_history.is_online = bitstream.read_bool("author-flags")
    ? 1
    : 0;
  metadata.modification_history.timestamp = new Date(
    Number(bitstream.read_qword(64)) * 1000,
  );
  metadata.modification_history.xuid = bitstream.read_qword(64);
  metadata.modification_history.name = bitstream.read_string_extended_ascii(16);
  metadata.modification_history.is_online = bitstream.read_bool("author-flags")
    ? 1
    : 0;
  metadata.name = bitstream.read_string_wchar(128);
  metadata.description = bitstream.read_string_wchar(128);

  switch (metadata.general.file_type) {
    case e_file_type.Film:
    case e_file_type.FilmClip: {
      const film = new s_content_item_film_metadata();
      film.seconds = bitstream.read_signed_integer("film-seconds", 32);
      metadata.file_type_data = film;
      break;
    }
    case e_file_type.GameVariant: {
      const gv = new s_content_item_game_variant_metadata();
      gv.icon_index = bitstream.read_signed_integer("icon-index", 8);
      metadata.file_type_data = gv;
      break;
    }
    default:
      metadata.file_type_data = null;
      break;
  }

  if (metadata.general.activity === 2) {
    const mm = new s_content_item_matchmaking_metadata();
    mm.hopper_identifier = bitstream.read_integer("hopper-identifier", 16);
    metadata.activity_data = mm;
  } else {
    metadata.activity_data = null;
  }

  switch (metadata.general.game_mode) {
    case 1: {
      const campaign = new s_content_item_campaign_metadata();
      campaign.campaign_id = bitstream.read_integer("campaign-id", 8);
      campaign.campaign_difficulty = bitstream.read_integer(
        "difficulty-level",
        2,
      );
      campaign.campaign_metagame_scoring = bitstream.read_integer(
        "metagame-scoring",
        2,
      );
      campaign.campaign_insertion_point = bitstream.read_integer(
        "insertion-point",
        8,
      );
      campaign.campaign_primary_skulls = bitstream.read_integer(
        "skull-flags",
        16,
      );
      campaign.campaign_secondary_skulls = bitstream.read_integer(
        "skull-flags",
        16,
      );
      metadata.game_mode_data = campaign;
      break;
    }
    case 2: {
      const ff = new s_content_item_firefight_metadata();
      ff.firefight_difficulty = bitstream.read_integer("difficulty-level", 2);
      ff.firefight_primary_skulls = bitstream.read_integer("skull-flags", 16);
      ff.firefight_secondary_skulls = bitstream.read_integer("skull-flags", 16);
      metadata.game_mode_data = ff;
      break;
    }
    default:
      metadata.game_mode_data = null;
      break;
  }
}

export function content_item_metadata_encode(
  bitstream: c_bitstream_writer,
  metadata: s_content_item_metadata,
): void {
  bitstream.write_integer(metadata.general.file_type + 1, 4);
  bitstream.write_integer(metadata.general.size_in_bytes, 32);
  bitstream.write_qword(metadata.general.unique_id, 64);
  bitstream.write_qword(metadata.general.parent_unique_id, 64);
  bitstream.write_qword(metadata.general.root_unique_id, 64);
  bitstream.write_qword(metadata.general.game_id, 64);
  bitstream.write_integer(metadata.general.activity + 1, 3);
  bitstream.write_integer(metadata.general.game_mode, 3);
  bitstream.write_integer(metadata.general.game_engine_type, 3);
  bitstream.write_signed_integer(metadata.general.map_id, 32);
  bitstream.write_signed_integer(metadata.display.megalo_category_index, 8);
  bitstream.write_qword(
    BigInt(Math.floor(metadata.creation_history.timestamp.getTime() / 1000)),
    64,
  );
  bitstream.write_qword(metadata.creation_history.xuid, 64);
  bitstream.write_string_extended_ascii(metadata.creation_history.name, 16);
  bitstream.write_bool(metadata.creation_history.is_online !== 0);
  bitstream.write_qword(
    BigInt(Math.floor(metadata.modification_history.timestamp.getTime() / 1000)),
    64,
  );
  bitstream.write_qword(metadata.modification_history.xuid, 64);
  bitstream.write_string_extended_ascii(metadata.modification_history.name, 16);
  bitstream.write_bool(metadata.modification_history.is_online !== 0);
  bitstream.write_string_wchar(metadata.name, 128);
  bitstream.write_string_wchar(metadata.description, 128);

  switch (metadata.general.file_type) {
    case e_file_type.Film:
    case e_file_type.FilmClip: {
      const film = metadata.file_type_data as s_content_item_film_metadata;
      bitstream.write_signed_integer(film.seconds, 32);
      break;
    }
    case e_file_type.GameVariant: {
      const gv =
        metadata.file_type_data as s_content_item_game_variant_metadata;
      bitstream.write_signed_integer(gv.icon_index, 8);
      break;
    }
    default:
      break;
  }

  if (metadata.general.activity === 2) {
    const mm = metadata.activity_data as s_content_item_matchmaking_metadata;
    bitstream.write_integer(mm.hopper_identifier, 16);
  }

  switch (metadata.general.game_mode) {
    case 1: {
      const campaign = metadata.game_mode_data as s_content_item_campaign_metadata;
      bitstream.write_integer(campaign.campaign_id, 8);
      bitstream.write_integer(campaign.campaign_difficulty, 2);
      bitstream.write_integer(campaign.campaign_metagame_scoring, 2);
      bitstream.write_integer(campaign.campaign_insertion_point, 8);
      bitstream.write_integer(campaign.campaign_primary_skulls, 16);
      bitstream.write_integer(campaign.campaign_secondary_skulls, 16);
      break;
    }
    case 2: {
      const ff = metadata.game_mode_data as s_content_item_firefight_metadata;
      bitstream.write_integer(ff.firefight_difficulty, 2);
      bitstream.write_integer(ff.firefight_primary_skulls, 16);
      bitstream.write_integer(ff.firefight_secondary_skulls, 16);
      break;
    }
    default:
      break;
  }
}
