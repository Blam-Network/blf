/** Result codes for Reach TU1 ↔ MCC gametype conversion. */
export enum e_reach_gametype_conversion_error {
  ok = 0,
  forge_variant = 1,
  campaign_variant = 2,
  mcc_exclusive_action = 3,
  mcc_exclusive_math_operator = 4,
  mcc_survival_additional_flags = 5,
  insufficient_global_slots = 6,
}
