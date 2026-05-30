import { c_game_variant as c_game_variant_tu1 } from "../blam/haloreach/v12065_11_08_24_1738_tu1actual/game/c_game_variant";
import { c_game_variant as c_game_variant_mcc } from "../blam/haloreach_mcc/v_untracked_25_08_16_1352/game/c_game_variant";
import { mccToTu1Mapper, tu1ToMccMapper } from "./reach_gametype_automap";

export function map_reach_gametype_mcc_to_tu1(
  from: c_game_variant_mcc
): c_game_variant_tu1 {
  return mccToTu1Mapper.map(from, c_game_variant_mcc, c_game_variant_tu1);
}

export function map_reach_gametype_tu1_to_mcc(
  from: c_game_variant_tu1
): c_game_variant_mcc {
  return tu1ToMccMapper.map(from, c_game_variant_tu1, c_game_variant_mcc);
}
