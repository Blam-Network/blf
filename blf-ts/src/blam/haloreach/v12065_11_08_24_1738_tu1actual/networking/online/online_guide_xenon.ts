import { c } from "@craftycodie/cstruct";

@c.struct()
export class s_network_session_privacy_mode {
  @c.field("u8")
  network_session_privacy = 0;

  @c.field("u8")
  network_session_closed_status = 0;

  @c.field("u8")
  unknown2 = 0;

  @c.field("u8")
  unknown3 = 0;
}
