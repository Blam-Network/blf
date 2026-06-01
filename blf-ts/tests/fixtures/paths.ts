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

/** Reach release LSP presence heartbeat upload (`phbt` 5.1, 455 bytes). */
export const reach_presence_heartbeat_1780307835656_fixture = join(
  import.meta.dirname,
  "reach_presence_heartbeat_1780307835656.blob"
);

/** Reach `UserUpdateImage` Spartan render (`_cmp` wrapping `auiu` 1.2, 320×704 RGBA). */
export const reach_spartan_render_auiu_cmp_fixture = join(
  import.meta.dirname,
  "reach_spartan_render_auiu_cmp.blf"
);

/** PNG snapshot produced from {@link reach_spartan_render_auiu_cmp_fixture}. */
export const reach_spartan_render_auiu_png_snapshot = join(
  import.meta.dirname,
  "reach_spartan_render_auiu_cmp.png"
);
