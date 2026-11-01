import { c } from "@craftycodie/cstruct";
import { ipv4_address } from "../../cseries/cseries_types";

/** Halo 3 `s_transport_secure_address` (0x24 bytes). */
export const s_transport_secure_address = c.struct({
  ina: ipv4_address(),
  ina_online: ipv4_address(),
  data: c.u8().array(28),
});
export type s_transport_secure_address = c.infer<
  typeof s_transport_secure_address
>;
