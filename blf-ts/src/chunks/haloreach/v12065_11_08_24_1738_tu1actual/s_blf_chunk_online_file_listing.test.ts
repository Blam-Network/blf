// fitm/idls against an independent copy of the wire layout web_private shipped with (pack 1, big endian).
import { c } from "@craftycodie/cstruct";
import { expect, it } from "vitest";
import { s_blf_chunk_online_file_listing } from "./s_blf_chunk_online_file_listing";
import { s_blf_chunk_xuid_list } from "./s_blf_chunk_xuid_list";

const be = { pack: 1, endian: "big" } as const;
const s_content_item_history = c.struct(
  {
    timestamp: c.Time64(),
    xuid: c.u64(),
    name: c.string(16, "latin1"),
    is_online: c.u8(),
    pad: c.pad(3),
  },
  be
);
const gv = c.struct({ icon_index: c.i8() }, be);
const film = c.struct({ seconds: c.i32() }, be);
const mm = c.struct({ hopper_identifier: c.u16() }, be);
const camp = c.struct(
  {
    campaign_id: c.i32(),
    campaign_difficulty: c.i16(),
    campaign_metagame_scoring: c.i16(),
    campaign_insertion_point: c.i32(),
    campaign_primary_skulls: c.i16(),
    campaign_secondary_skulls: c.i16(),
  },
  be
);
const ff = c.struct(
  {
    firefight_difficulty: c.i16(),
    firefight_primary_skulls: c.i16(),
    firefight_secondary_skulls: c.i16(),
    pad: c.pad(10),
  },
  be
);
const tag = c.struct({ tag: c.string(23), unknown: c.u32() }, be);
const general = c.struct(
  {
    id: c.u64(),
    file_type: c.u8(),
    tag_count: c.u8(),
    megalo_category_index: c.u8(),
    pad: c.pad(1),
    size_in_bytes: c.u32(),
    activity: c.u8(),
    game_mode: c.u8(),
    game_engine_type: c.u8(),
    pad2: c.pad(1),
    unknown3: c.u8().array(8),
    map_id: c.i32(),
  },
  be
);
const pad16 = c.struct({ pad: c.pad(16) }, be);
const metadata = c.struct(
  {
    general,
    created: s_content_item_history,
    modified: s_content_item_history,
    name: c.wstring(128),
    description: c.wstring(128),
    game_variant_or_film: c.union({ game_variant: gv, film, pad: pad16 }),
    matchmaking: c.union({ metadata: mm, pad: pad16 }),
    campaign_or_firefight: c.union({
      campaign: camp,
      firefight: ff,
      pad: pad16,
    }),
    screenshot_length: c.u32(),
  },
  be
);
const head = {
  xuid: c.u64(),
  gamertag: c.string(16),
  unknown16: c.u8(),
  unknown17: c.u8(),
  unknown18: c.u8(),
  unknown19: c.u8(),
  quota_byte_count: c.u32(),
  quota_slot_count: c.u8(),
  pad: c.pad(1),
};
const old_listing = (fileCount: number, messageLength: number) =>
  c.struct(
    {
      ...head,
      slot_count: c.u16(),
      message_length: c.MagicNumber(messageLength, c.u8()),
      pad2: c.pad(3),
      entries: metadata.array(fileCount),
      message: c.wstring(messageLength + 1),
    },
    be
  );
const old_details = (screenshotLength: number, tagCount: number) =>
  c.struct(
    {
      ...head,
      slot_count: c.MagicNumber(1, c.u16()),
      message_length: c.MagicNumber(0, c.u8()),
      pad2: c.pad(3),
      entries: metadata,
      screenshot: c.u8().array(screenshotLength),
      tags: tag.array(tagCount),
    },
    be
  );

const entry = (i: number, screenshot_length = 0, tag_count = 0) => ({
  general: {
    id: 1000n + BigInt(i),
    file_type: [3, 6, 5][i % 3],
    tag_count,
    megalo_category_index: i,
    size_in_bytes: 4096 * i,
    activity: i % 4,
    game_mode: i % 3,
    game_engine_type: 2,
    unknown3: [1, 2, 3, 4, 5, 6, 7, 8],
    map_id: 1035 + i,
  },
  created: {
    timestamp: new Date(1_300_000_000_000 + i * 1000),
    xuid: 0x0009000000000000n + BigInt(i),
    name: `creator${i}`,
    is_online: 1,
  },
  modified: {
    timestamp: new Date(1_300_000_500_000),
    xuid: 0x0009000000000001n,
    name: "modder",
    is_online: 0,
  },
  name: `File ${i} é`,
  description: "a description",
  game_variant_or_film: [
    { film: { seconds: 90 + i } },
    { game_variant: { icon_index: 3 } },
    { pad: {} },
  ][i % 3],
  matchmaking: i % 2 ? { metadata: { hopper_identifier: 101 } } : { pad: {} },
  campaign_or_firefight: [
    {
      campaign: {
        campaign_id: 1,
        campaign_difficulty: 2,
        campaign_metagame_scoring: 1,
        campaign_insertion_point: 3,
        campaign_primary_skulls: 5,
        campaign_secondary_skulls: 6,
      },
    },
    {
      firefight: {
        firefight_difficulty: 3,
        firefight_primary_skulls: 7,
        firefight_secondary_skulls: 8,
      },
    },
    { pad: {} },
  ][i % 3],
  screenshot_length,
});
const top = {
  xuid: 0x0009000001234567n,
  gamertag: "Gamer Tag",
  unknown16: 1,
  unknown17: 2,
  unknown18: 3,
  unknown19: 4,
  quota_byte_count: 1 << 20,
  quota_slot_count: 50,
};
const asNew = (e: ReturnType<typeof entry>) => ({
  ...e,
  created: { ...e.created, is_online: e.created.is_online === 1 },
  modified: { ...e.modified, is_online: e.modified.is_online === 1 },
});

it("listings match web_private's bytes", () => {
  for (const [count, message] of [
    [0, ""],
    [0, "File share is unavailable."],
    [1, ""],
    [3, "hi"],
    [12, ""],
  ] as const) {
    const entries = Array.from({ length: count }, (_, i) => entry(i));
    const old = old_listing(count, message.length).write(
      {
        ...top,
        slot_count: count,
        message_length: message.length,
        entries,
        message,
      } as never,
      "big"
    );
    const chunk = new s_blf_chunk_online_file_listing();
    Object.assign(chunk, top, { entries: entries.map(asNew), message });
    const body = chunk.write_body("big");
    expect(Buffer.from(body).toString("hex")).toBe(
      Buffer.from(old).toString("hex")
    );
    const back = new s_blf_chunk_online_file_listing();
    back.read_body(body, "big");
    expect(back.message).toBe(message);
    expect(back.entries.length).toBe(count);
  }
});

it("details match web_private's bytes", () => {
  for (const [shot, tags] of [
    [0, 0],
    [300, 0],
    [0, 2],
    [1234, 3],
  ] as const) {
    const e = entry(1, shot, tags);
    const screenshot = Array.from({ length: shot }, (_, i) => (i * 7) & 0xff);
    const tagList = Array.from({ length: tags }, (_, i) => ({
      tag: `tag${i}`,
      unknown: 0,
    }));
    const old = old_details(shot, tags).write(
      {
        ...top,
        slot_count: 1,
        message_length: 0,
        entries: e,
        screenshot,
        tags: tagList,
      } as never,
      "big"
    );
    const chunk = new s_blf_chunk_online_file_listing();
    Object.assign(chunk, top, {
      entries: [asNew(e)],
      message: null,
      screenshot,
      tags: tagList,
    });
    const body = chunk.write_body("big");
    expect(Buffer.from(body).toString("hex")).toBe(
      Buffer.from(old).toString("hex")
    );
    const back = new s_blf_chunk_online_file_listing();
    back.read_body(body, "big");
    expect(back.message).toBe(null);
    expect(back.screenshot).toEqual(screenshot);
    expect(back.tags).toEqual(tagList);
  }
});

it("idls matches web_private's layout", () => {
  const xuids = [0x0009000000000001n, 0x0009000000000002n, 0x00090000ffffffffn];
  const old = c
    .struct({ friend_count: c.u32(), friend_xuid: c.u64().array(3) }, be)
    .write({ friend_count: 3, friend_xuid: xuids }, "big");
  const chunk = new s_blf_chunk_xuid_list();
  chunk.xuids = xuids;
  expect(Buffer.from(chunk.write_body("big")).toString("hex")).toBe(
    Buffer.from(old).toString("hex")
  );
  const back = new s_blf_chunk_xuid_list();
  back.read_body(old, "big");
  expect(back.xuids).toEqual(xuids);
});
