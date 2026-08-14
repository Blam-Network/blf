import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../bitstream";
import { AutoMap } from "../../../../helpers/automap";

export class s_content_item_metadata {
  @AutoMap(() => BigInt)
  unique_id = 0n;
  @AutoMap(() => String)
  name = "";
  @AutoMap(() => String)
  description = "";
  @AutoMap(() => String)
  author = "";
  @AutoMap(() => Number)
  file_type = 0;
  @AutoMap(() => Boolean)
  author_is_xuid_online = false;
  @AutoMap(() => BigInt)
  author_id = 0n;
  @AutoMap(() => BigInt)
  size_in_bytes = 0n;
  @AutoMap(() => BigInt)
  date = 0n;
  @AutoMap(() => Number)
  length_seconds = 0;
  @AutoMap(() => Number)
  campaign_id = 0;
  @AutoMap(() => Number)
  map_id = 0;
  @AutoMap(() => Number)
  game_engine_type = 0;
  @AutoMap(() => Number)
  campaign_difficulty = 0;
  @AutoMap(() => Number)
  campaign_insertion_point = 0;
  @AutoMap(() => Boolean)
  campaign_survival_enabled = false;
  @AutoMap(() => BigInt)
  game_id = 0n;

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_qword(this.unique_id, 64);
    bitstream.write_string_wchar(this.name, 16);
    bitstream.write_string_utf8(this.description, 128);
    bitstream.write_string_utf8(this.author, 16);
    bitstream.write_signed_integer(this.file_type + 1, 5);
    bitstream.write_bool(this.author_is_xuid_online);
    bitstream.write_qword(this.author_id, 64);
    bitstream.write_qword(this.size_in_bytes, 64);
    bitstream.write_qword(this.date, 64);
    bitstream.write_integer(this.length_seconds, 32);
    bitstream.write_signed_integer(this.campaign_id, 32);
    bitstream.write_signed_integer(this.map_id, 32);
    bitstream.write_integer(this.game_engine_type, 4);
    bitstream.write_signed_integer(this.campaign_difficulty + 1, 3);
    bitstream.write_integer(this.campaign_insertion_point, 4);
    bitstream.write_bool(this.campaign_survival_enabled);
    bitstream.write_qword(this.game_id, 64);
  }

  decode(bitstream: c_bitstream_reader): void {
    this.unique_id = bitstream.read_qword(64);
    this.name = bitstream.read_string_wchar(16);
    this.description = bitstream.read_string_utf8(128);
    this.author = bitstream.read_string_utf8(16);
    this.file_type = bitstream.read_signed_integer("file-type", 5) - 1;
    this.author_is_xuid_online = bitstream.read_bool("author-is-xuid-online");
    this.author_id = bitstream.read_qword(64);
    this.size_in_bytes = bitstream.read_qword(64);
    this.date = bitstream.read_qword(64);
    this.length_seconds = bitstream.read_integer("length-seconds", 32);
    this.campaign_id = bitstream.read_signed_integer("campaign-id", 32);
    this.map_id = bitstream.read_signed_integer("map-id", 32);
    this.game_engine_type = bitstream.read_integer("game-engine-type", 4);
    this.campaign_difficulty =
      bitstream.read_signed_integer("campaign-difficulty", 3) - 1;
    this.campaign_insertion_point = bitstream.read_integer(
      "campaign-insertion-point",
      4
    );
    this.campaign_survival_enabled = bitstream.read_bool(
      "campaign-survival-enabled"
    );
    this.game_id = bitstream.read_qword(64);
  }
}
