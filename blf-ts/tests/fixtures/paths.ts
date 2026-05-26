import { join } from "node:path";

/** Reach GEA0 map BLF used across integration and bisect tests. */
export const reach_12065_oddball_fixture = join(
  import.meta.dirname,
  "reach_12065_oddball.blf"
);

/** Reach MCC fileshare gametype with `_fsm` metadata appended after `_eof`. */
export const reachmcc_zb_slayer_team_dmr_054_fixture = join(
  import.meta.dirname,
  "reachmcc_zb_slayer_team_dmr_054.bin"
);
