import { c } from "@craftycodie/cstruct";
import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../bitstream";

export enum e_file_type {
  Screenshot = 2,
  Film = 3,
  FilmClip = 4,
  MapVariant = 5,
  GameVariant = 6,
}

export enum e_gui_game_mode {
  none = -1,
  activities = 0,
  campaign = 1,
  matchmaking = 2,
  multiplayer = 3,
  mapeditor = 4,
  theater = 5,
  survival = 6,
}

export enum e_game_mode {
  none = 0,
  campaign = 1,
  survival = 2,
  multiplayer = 3,
}

export const s_content_item_history = c.struct({
  timestamp: c.Time64(),
  xuid: c.u64(),
  name: c.string(16, "latin1"),
  is_online: c.bool().padAfter(3),
});
export type s_content_item_history = c.infer<typeof s_content_item_history>;

export const s_content_item_general_metadata = c.struct({
  /** Starts as 0, which is not a file type: set it before writing. */
  file_type: c
    .enum(e_file_type, c.i8())
    .default(0 as e_file_type)
    .padAfter(3),
  size_in_bytes: c.u32(),
  unique_id: c.u64(),
  parent_unique_id: c.u64(),
  root_unique_id: c.u64(),
  game_id: c.u64(),
  activity: c.enum(e_gui_game_mode, c.i8()).default(e_gui_game_mode.none),
  game_mode: c.enum(e_game_mode, c.u8()),
  game_engine_type: c.u8().padAfter(1),
  map_id: c.i32(),
});
export type s_content_item_general_metadata = c.infer<
  typeof s_content_item_general_metadata
>;

export const s_content_item_display_metadata = c.struct({
  megalo_category_index: c.i8().padAfter(7),
});
export type s_content_item_display_metadata = c.infer<
  typeof s_content_item_display_metadata
> & {
  /**
   * Alpha/Delta bitstream only: 64-bit pad after map_id.
   * Not present on TU1 wire — leave unset so TU1/MCC snapshots stay clean.
   */
  padding?: bigint;
};

export const s_content_item_film_metadata = c.struct({
  seconds: c.i32(),
});
export type s_content_item_film_metadata = c.infer<
  typeof s_content_item_film_metadata
>;

export const s_content_item_game_variant_metadata = c.struct({
  icon_index: c.i8(),
});
export type s_content_item_game_variant_metadata = c.infer<
  typeof s_content_item_game_variant_metadata
>;

export const s_content_item_matchmaking_metadata = c.struct({
  hopper_identifier: c.u16(),
});
export type s_content_item_matchmaking_metadata = c.infer<
  typeof s_content_item_matchmaking_metadata
>;

export const s_content_item_campaign_metadata = c.struct({
  campaign_id: c.i32(),
  campaign_difficulty: c.i16(),
  campaign_metagame_scoring: c.i16(),
  campaign_insertion_point: c.i32(),
  campaign_primary_skulls: c.i16(),
  campaign_secondary_skulls: c.i16(),
});
export type s_content_item_campaign_metadata = c.infer<
  typeof s_content_item_campaign_metadata
>;

export const s_content_item_firefight_metadata = c.struct({
  firefight_difficulty: c.i16(),
  firefight_primary_skulls: c.i16(),
  firefight_secondary_skulls: c.i16(),
});
export type s_content_item_firefight_metadata = c.infer<
  typeof s_content_item_firefight_metadata
>;

/** Full `c_content_item_metadata` binrw layout (fixed prefix + conditional union tails). */
export const s_content_item_metadata = c.struct({
  general: s_content_item_general_metadata,
  display: s_content_item_display_metadata,
  creation_history: s_content_item_history,
  modification_history: s_content_item_history,
  name: c.wstring(0x80),
  description: c.wstring(0x80),
  file_type_data: c.match(
    "general.file_type",
    {
      [e_file_type.Film]: s_content_item_film_metadata,
      [e_file_type.FilmClip]: s_content_item_film_metadata,
      [e_file_type.GameVariant]: s_content_item_game_variant_metadata,
    },
    { size: 16 }
  ),
  activity_data: c.match(
    "general.activity",
    {
      [e_gui_game_mode.matchmaking]: s_content_item_matchmaking_metadata,
    },
    { size: 16 }
  ),
  game_mode_data: c.match(
    "general.game_mode",
    {
      [e_game_mode.campaign]: s_content_item_campaign_metadata,
      [e_game_mode.survival]: s_content_item_firefight_metadata,
    },
    { size: 16 }
  ),
});
export type s_content_item_metadata = Omit<
  c.infer<typeof s_content_item_metadata>,
  "display"
> & { display: s_content_item_display_metadata };

export function content_item_metadata_set_defaults(
  metadata: s_content_item_metadata
): void {
  metadata.general = Object.assign(s_content_item_general_metadata.zero(), {
    file_type: -1 as e_file_type,
    activity: e_gui_game_mode.none,
    game_mode: e_game_mode.none,
    game_engine_type: 0,
    map_id: -1,
  });
  metadata.display = Object.assign(s_content_item_display_metadata.zero(), {
    megalo_category_index: -1,
  });
  metadata.creation_history = s_content_item_history.zero();
  metadata.modification_history = s_content_item_history.zero();
  metadata.name = "";
  metadata.description = "";
  metadata.file_type_data = null;
  metadata.activity_data = null;
  metadata.game_mode_data = null;
}

export function content_item_metadata_decode(
  bitstream: c_bitstream_reader,
  metadata: s_content_item_metadata
): void {
  metadata.general.file_type = (bitstream.read_integer("type", 4) -
    1) as e_file_type;
  metadata.general.size_in_bytes = bitstream.read_integer("file-size", 32);
  metadata.general.unique_id = bitstream.read_qword(64);
  metadata.general.parent_unique_id = bitstream.read_qword(64);
  metadata.general.root_unique_id = bitstream.read_qword(64);
  metadata.general.game_id = bitstream.read_qword(64);
  metadata.general.activity = (bitstream.read_integer("activity", 3) -
    1) as e_gui_game_mode;
  metadata.general.game_mode = bitstream.read_integer(
    "game-mode",
    3
  ) as e_game_mode;
  metadata.general.game_engine_type = bitstream.read_integer(
    "game-engine-type",
    3
  );
  metadata.general.map_id = bitstream.read_signed_integer("map-id", 32);
  metadata.display.megalo_category_index = bitstream.read_signed_integer(
    "megalo-category-index",
    8
  );
  metadata.creation_history.timestamp = new Date(
    Number(bitstream.read_qword(64)) * 1000
  );
  metadata.creation_history.xuid = bitstream.read_qword(64);
  metadata.creation_history.name = bitstream.read_string_extended_ascii(16);
  metadata.creation_history.is_online = bitstream.read_bool("author-flags");
  metadata.modification_history.timestamp = new Date(
    Number(bitstream.read_qword(64)) * 1000
  );
  metadata.modification_history.xuid = bitstream.read_qword(64);
  metadata.modification_history.name = bitstream.read_string_extended_ascii(16);
  metadata.modification_history.is_online = bitstream.read_bool("author-flags");
  metadata.name = bitstream.read_string_wchar(128);
  metadata.description = bitstream.read_string_wchar(128);

  switch (metadata.general.file_type) {
    case e_file_type.Film:
    case e_file_type.FilmClip: {
      const film = s_content_item_film_metadata.zero();
      film.seconds = bitstream.read_signed_integer("film-seconds", 32);
      metadata.file_type_data = film;
      break;
    }
    case e_file_type.GameVariant: {
      const gv = s_content_item_game_variant_metadata.zero();
      gv.icon_index = bitstream.read_signed_integer("icon-index", 8);
      metadata.file_type_data = gv;
      break;
    }
    default:
      metadata.file_type_data = null;
      break;
  }

  if (metadata.general.activity === e_gui_game_mode.matchmaking) {
    const mm = s_content_item_matchmaking_metadata.zero();
    mm.hopper_identifier = bitstream.read_integer("hopper-identifier", 16);
    metadata.activity_data = mm;
  } else {
    metadata.activity_data = null;
  }

  switch (metadata.general.game_mode) {
    case e_game_mode.campaign: {
      const campaign = s_content_item_campaign_metadata.zero();
      campaign.campaign_id = bitstream.read_integer("campaign-id", 8);
      campaign.campaign_difficulty = bitstream.read_integer(
        "difficulty-level",
        2
      );
      campaign.campaign_metagame_scoring = bitstream.read_integer(
        "metagame-scoring",
        2
      );
      campaign.campaign_insertion_point = bitstream.read_integer(
        "insertion-point",
        8
      );
      campaign.campaign_primary_skulls = bitstream.read_integer(
        "skull-flags",
        16
      );
      campaign.campaign_secondary_skulls = bitstream.read_integer(
        "skull-flags",
        16
      );
      metadata.game_mode_data = campaign;
      break;
    }
    case e_game_mode.survival: {
      const ff = s_content_item_firefight_metadata.zero();
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
  metadata: s_content_item_metadata
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
    64
  );
  bitstream.write_qword(metadata.creation_history.xuid, 64);
  bitstream.write_string_extended_ascii(metadata.creation_history.name, 16);
  bitstream.write_bool(metadata.creation_history.is_online);
  bitstream.write_qword(
    BigInt(
      Math.floor(metadata.modification_history.timestamp.getTime() / 1000)
    ),
    64
  );
  bitstream.write_qword(metadata.modification_history.xuid, 64);
  bitstream.write_string_extended_ascii(metadata.modification_history.name, 16);
  bitstream.write_bool(metadata.modification_history.is_online);
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

  if (metadata.general.activity === e_gui_game_mode.matchmaking) {
    const mm = metadata.activity_data as s_content_item_matchmaking_metadata;
    bitstream.write_integer(mm.hopper_identifier, 16);
  }

  switch (metadata.general.game_mode) {
    case e_game_mode.campaign: {
      const campaign =
        metadata.game_mode_data as s_content_item_campaign_metadata;
      bitstream.write_integer(campaign.campaign_id, 8);
      bitstream.write_integer(campaign.campaign_difficulty, 2);
      bitstream.write_integer(campaign.campaign_metagame_scoring, 2);
      bitstream.write_integer(campaign.campaign_insertion_point, 8);
      bitstream.write_integer(campaign.campaign_primary_skulls, 16);
      bitstream.write_integer(campaign.campaign_secondary_skulls, 16);
      break;
    }
    case e_game_mode.survival: {
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
