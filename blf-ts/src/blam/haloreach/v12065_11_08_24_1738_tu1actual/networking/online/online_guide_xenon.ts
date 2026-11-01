import { c } from "@craftycodie/cstruct";

export enum e_network_session_privacy {
  none = 0,
  open = 1,
  friends_only = 2,
  invitation_only = 3,
}

export enum e_network_session_closed {
  none = 0,
  open = 1,
  campaign_in_progress = 2,
  film_in_progress = 3,
  matchmaking_in_progress = 4,
  survival_in_progress = 5,
}

export const s_network_session_privacy_mode = c.struct({
  privacy_mode: c.u8(),
  closed_mode: c.u8(),
  mode_based_maximum_player_count: c.u8(),
  closed_by_user: c.bool(),
});
export type s_network_session_privacy_mode = c.infer<
  typeof s_network_session_privacy_mode
>;
