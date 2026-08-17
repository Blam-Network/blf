import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../bitstream";
import {
  e_file_type,
  e_game_mode,
  e_gui_game_mode,
  s_content_item_campaign_metadata,
  s_content_item_film_metadata,
  s_content_item_firefight_metadata,
  s_content_item_matchmaking_metadata,
  type s_content_item_metadata,
  content_item_metadata_set_defaults as tu1_content_item_metadata_set_defaults,
} from "../../v12065_11_08_24_1738_tu1actual/saved_games/saved_game_files";

export {
  e_file_type,
  e_game_mode,
  e_gui_game_mode,
  s_content_item_campaign_metadata,
  s_content_item_display_metadata,
  s_content_item_film_metadata,
  s_content_item_firefight_metadata,
  s_content_item_game_variant_metadata,
  s_content_item_general_metadata,
  s_content_item_history,
  s_content_item_matchmaking_metadata,
  s_content_item_metadata,
} from "../../v12065_11_08_24_1738_tu1actual/saved_games/saved_game_files";

/**
 * Omaha Delta/Beta content metadata bitstream layout (matches blf_lib delta
 * `s_content_item_metadata`). Differs from TU1: 64-bit display padding, no
 * game-variant icon payload, and different campaign/firefight tails.
 */
export function content_item_metadata_set_defaults(
  metadata: s_content_item_metadata
): void {
  tu1_content_item_metadata_set_defaults(metadata);
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
  // Alpha/Delta: 64-bit display padding (megalo category is not on the wire).
  metadata.display.padding = bitstream.read_qword(64);
  metadata.display.megalo_category_index = 0;
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
      const film = new s_content_item_film_metadata();
      film.seconds = bitstream.read_signed_integer("film-seconds", 32);
      metadata.file_type_data = film;
      break;
    }
    default:
      metadata.file_type_data = null;
      break;
  }

  if (metadata.general.activity === e_gui_game_mode.matchmaking) {
    const mm = new s_content_item_matchmaking_metadata();
    mm.hopper_identifier = bitstream.read_integer("hopper-identifier", 16);
    metadata.activity_data = mm;
  } else {
    metadata.activity_data = null;
  }

  switch (metadata.general.game_mode) {
    case e_game_mode.campaign: {
      const campaign = new s_content_item_campaign_metadata();
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
        2
      );
      const skullFlags = bitstream.read_integer("skull-flags", 32);
      campaign.campaign_primary_skulls = skullFlags & 0xffff;
      campaign.campaign_secondary_skulls = (skullFlags >>> 16) & 0xffff;
      metadata.game_mode_data = campaign;
      break;
    }
    case e_game_mode.survival: {
      const ff = new s_content_item_firefight_metadata();
      ff.firefight_difficulty = bitstream.read_integer("difficulty-level", 2);
      const skullFlags = bitstream.read_integer("skull-flags", 32);
      ff.firefight_primary_skulls = skullFlags & 0xffff;
      ff.firefight_secondary_skulls = (skullFlags >>> 16) & 0xffff;
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
  bitstream.write_qword(metadata.display.padding ?? 0n, 64);
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
      bitstream.write_integer(campaign.campaign_insertion_point, 2);
      bitstream.write_integer(
        (campaign.campaign_primary_skulls & 0xffff) |
          ((campaign.campaign_secondary_skulls & 0xffff) << 16),
        32
      );
      break;
    }
    case e_game_mode.survival: {
      const ff = metadata.game_mode_data as s_content_item_firefight_metadata;
      bitstream.write_integer(ff.firefight_difficulty, 2);
      bitstream.write_integer(
        (ff.firefight_primary_skulls & 0xffff) |
          ((ff.firefight_secondary_skulls & 0xffff) << 16),
        32
      );
      break;
    }
    default:
      break;
  }
}
