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

/** Reach MCC hopper gametype (3nvasion DLC). */
export const reach_mcc_3nvasion_dlc_fixture = join(
  import.meta.dirname,
  "3nvasion_dlc_054.bin"
);

/** Reach MCC hopper map variant (Forge World). */
export const reach_mcc_skeeball_court_xtreme_fixture = join(
  import.meta.dirname,
  "skeeball_court_xtreme_031.mvar"
);
