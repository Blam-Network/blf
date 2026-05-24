import { c } from "@craftycodie/cstruct";
import { describe, expect, it } from "vitest";
import { search_for_chunk } from "../../../blf_chunk";
import { write_blffile } from "../../../index";
import { s_blf_chunk_end_of_file } from "../../halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_end_of_file";
import { s_blf_chunk_start_of_file } from "../../halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_start_of_file";
import {
  s_blf_chunk_author,
} from "./s_blf_chunk_author";

describe("s_blf_chunk_author", () => {
  it("has a 0x44-byte body (big-endian)", () => {
    expect(c.size(s_blf_chunk_author)).toBe(0x44);
  });

  it("round-trips known field values", () => {
    const original = new s_blf_chunk_author();
    original.program_name = "GameData.Reach";
    original.build_number = 12065;
    original.build_number_sequence = 2;
    original.build_string = "12065.11.08.24.1738.tu1actual";
    original.author_name = "dagasca";

    const payload = original.write_body("big");
    expect(payload.length).toBe(0x44);

    const parsed = new s_blf_chunk_author();
    parsed.read_body(payload, "big");

    expect(parsed.program_name).toBe("GameData.Reach");
    expect(parsed.build_number).toBe(12065);
    expect(parsed.build_number_sequence).toBe(2);
    expect(parsed.build_string).toBe("12065.11.08.24.1738.tu1actua");
    expect(parsed.author_name).toBe("dagasca");
  });

  it("forBuild() matches blf_lib Reach TU1 defaults", () => {
    const chunk = s_blf_chunk_author.forBuild({
      buildNumber: 12065,
      buildString: "12065.11.08.24.1738.tu1actual",
      buildNumberSequence: 2,
      authorName: "",
      programName: "blf-ts",
    });
    const payload = chunk.write_body("big");

    expect(payload.length).toBe(0x44);
    expect(chunk.program_name).toBe("blf-ts");
    expect(chunk.build_number).toBe(12065);
    expect(chunk.build_number_sequence).toBe(2);
    expect(chunk.build_string).toBe("12065.11.08.24.1738.tu1actua");
    expect(chunk.author_name).toBe("");
  });

  it("finds and reads athr in a BLF file", () => {
    const author = s_blf_chunk_author.forBuild({
      programName: "GameData.Reach",
      authorName: "blf_ts",
    });
    const blf = write_blffile("big", [
      s_blf_chunk_start_of_file.create("athr-test"),
      author,
      new s_blf_chunk_end_of_file(),
    ]);

    const found = new s_blf_chunk_author();
    expect(search_for_chunk(blf, found, "big")).toBe(true);
    expect(found.program_name).toBe("GameData.Reach");
    expect(found.build_number).toBe(0);
    expect(found.author_name).toBe("blf_ts");
  });
});
