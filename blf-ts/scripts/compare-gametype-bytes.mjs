import { readFileSync } from "node:fs";
import { search_for_chunk } from "../dist/blf_chunk.js";
import { s_blf_chunk_game_variant } from "../dist/chunks/haloreach/v12065_11_08_24_1738_tu1actual/s_blf_chunk_game_variant.js";

const GEA0 = new URL(
  "../tests/fixtures/haloreach_gea0_map.blf",
  import.meta.url
);
const file = new Uint8Array(readFileSync(GEA0));
const original = new s_blf_chunk_game_variant();
search_for_chunk(file, original, "big");
const origGt = file.subarray(0); // wrong

function mpvrPayload(buf) {
  for (let o = 0; o + 12 <= buf.length; o++) {
    if (
      buf[o] === 0x6d &&
      buf[o + 1] === 0x70 &&
      buf[o + 2] === 0x76 &&
      buf[o + 3] === 0x72
    ) {
      const cs = new DataView(buf.buffer, buf.byteOffset).getUint32(
        o + 4,
        false
      );
      return buf.subarray(o + 12, o + cs);
    }
  }
  throw new Error("no mpvr");
}

const payload = mpvrPayload(file);
const vl = new DataView(payload.buffer, payload.byteOffset + 24).getUint32(
  0,
  false
);
const origGametype = payload.subarray(28, 28 + vl);
const encoded = original.write_body("big").subarray(28);

console.log({ vl, origLen: origGametype.length, encLen: encoded.length });
let firstDiff = -1;
for (let i = 0; i < Math.max(origGametype.length, encoded.length); i++) {
  if (origGametype[i] !== encoded[i]) {
    firstDiff = i;
    console.log("first diff at", i, origGametype[i], encoded[i]);
    console.log("orig slice", [...origGametype.subarray(i, i + 32)]);
    console.log("enc slice", [...encoded.subarray(i, i + 32)]);
    break;
  }
}
if (firstDiff < 0) {
  console.log("byte-identical gametype");
}
