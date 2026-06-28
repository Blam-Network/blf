use blf_lib::bitfield;
use blf_lib::big_bitfield;
use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use blf_lib::blam::haloreach_mcc::v_untracked_25_08_16_1352::game::game_engine_campaign::c_game_engine_campaign_variant;
use blf_lib::blam::haloreach_mcc::v_untracked_25_08_16_1352::game::game_engine_default::c_game_engine_base_variant;
use blf_lib::blam::haloreach_mcc::v_untracked_25_08_16_1352::game::game_engine_player_rating_parameters::s_game_engine_player_rating_parameters;
use blf_lib::blam::haloreach_mcc::v_untracked_25_08_16_1352::game::game_engine_survival::c_game_engine_survival_variant;
use crate::blam::haloreach_mcc::v_untracked_25_08_16_1352::game::game_engine_sandbox::c_game_engine_sandbox_variant;
use blf_lib::blam::haloreach_mcc::v_untracked_25_08_16_1352::game::game_engine_traits::s_player_trait_option;
use blf_lib::blam::haloreach_mcc::v_untracked_25_08_16_1352::game::megalogamengine::megalogamengine_actions::c_action;
use blf_lib::blam::haloreach_mcc::v_untracked_25_08_16_1352::game::megalogamengine::megalogamengine_conditions::c_condition;
use blf_lib::blam::haloreach_mcc::v_untracked_25_08_16_1352::game::megalogamengine::megalogamengine_map_objects::c_object_filter;
use blf_lib::blam::haloreach_mcc::v_untracked_25_08_16_1352::game::megalogamengine::megalogamengine_statistics::c_megalo_game_statistic;
use blf_lib::blam::haloreach_mcc::v_untracked_25_08_16_1352::game::megalogamengine::megalogamengine_trigger::c_trigger;
use blf_lib::blam::haloreach_mcc::v_untracked_25_08_16_1352::game::megalogamengine::megalogamengine_user_defined_options::s_user_defined_option;
use blf_lib::blam::haloreach_mcc::v_untracked_25_08_16_1352::game::megalogamengine::megalogamengine_variable_metadata::s_variable_metadata;
use blf_lib::blam::haloreach_mcc::v_untracked_25_08_16_1352::game::string_table::c_string_table;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::memory::bitstream_reader::c_bitstream_reader_extensions;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::memory::bitstream_writer::c_bitstream_writer_extensions;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::OPTION_TO_RESULT;
use blf_lib::types::numbers::Float32;
use blf_lib_derivable::result::{BLFLibError, BLFLibResult};
use crate::blam::haloreach_mcc::v_untracked_25_08_16_1352::game::megalogamengine::megalogamengine_map_permissions::c_megalogamengine_map_permissions;
use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::saved_games::saved_game_files::c_content_item_metadata;
use crate::types::array::StaticArray;

big_bitfield! {
    // c_big_flags_typed<enum e_game_variant_parameter,1273>
    pub struct s_game_variant_parameter_flags {        
        game_misc_score_to_win_round,
        game_misc_teams,
        game_misc_round_reset_map,
        game_misc_round_time_limit,
        game_misc_sudden_death_time_limit,
        game_misc_perfection_enabled,
        game_misc_round_limit,
        game_misc_early_win_count,
        game_respawn_inherit_timer,
        game_respawn_with_teammate,
        game_respawn_at_location,
        game_respawn_on_kill,
        game_respawn_lives_per_round,
        game_respawn_team_lives_per_round,
        game_respawn_respawn_time,
        game_respawn_suicide_penalty,
        game_respawn_betrayal_penalty,
        game_respawn_growth,
        game_respawn_initial_loadout_selection_time,
        game_respawn_traits_duration,
        respawn_shld_dmg_resistance,
        respawn_body_multiplier,
        respawn_body_recharge_rate,
        respawn_shld_multiplier,
        respawn_shld_recharge_rate,
        respawn_overshld_recharge_rate,
        respawn_headshot_immunity,
        respawn_assassination_immunity,
        respawn_deathless,
        respawn_vampirism,
        respawn_dmg_modifier_percentage,
        respawn_melee_dmg_modifier_percentage,
        respawn_initial_primary_weapon,
        respawn_initial_secondary_weapon,
        respawn_initial_equipment,
        respawn_initial_grenade_count,
        respawn_recharging_grenades,
        respawn_infinite_ammo,
        respawn_infinite_equipment,
        respawn_weapon_pickup,
        respawn_equipment_usage,
        respawn_equipment_drops,
        respawn_walking_speed,
        respawn_personal_gravity,
        respawn_vehicle_use,
        respawn_jump_modifier,
        respawn_double_jump,
        respawn_motion_tracker,
        respawn_motion_tracker_range,
        respawn_directional_damage,
        respawn_waypoint,
        respawn_active_camo,
        respawn_visual,
        respawn_forced_change_colors,
        respawn_gamertag_visibility,
        game_social_observers,
        game_social_team_changing,
        game_social_friendly_fire,
        game_social_betrayal_booting,
        game_social_enemy_voice,
        game_social_open_channel_voice,
        game_social_dead_player_voice,
        game_map_grenades,
        game_map_equipment,
        game_map_turrets,
        game_map_shortcuts,
        game_map_powerups,
        game_map_indestructible_vehicles,
        player_shld_dmg_resistance,
        player_body_multiplier,
        player_body_recharge_rate,
        player_shld_multiplier,
        player_shld_recharge_rate,
        player_overshld_recharge_rate,
        player_headshot_immunity,
        player_assassination_immunity,
        player_deathless,
        player_vampirism,
        player_dmg_modifier_percentage,
        player_melee_dmg_modifier_percentage,
        player_initial_primary_weapon,
        player_initial_secondary_weapon,
        player_initial_equipment,
        player_initial_grenade_count,
        player_recharging_grenades,
        player_infinite_ammo,
        player_infinite_equipment,
        player_weapon_pickup,
        player_equipment_usage,
        player_equipment_drops,
        player_walking_speed,
        player_personal_gravity,
        player_vehicle_use,
        player_jump_modifier,
        player_double_jump,
        player_motion_tracker,
        player_motion_tracker_range,
        player_directional_damage,
        player_waypoint,
        player_active_camo,
        player_visual,
        player_forced_change_colors,
        player_gamertag_visibility,
        game_map_weapon_set,
        game_map_vehicle_set,
        red_powerup_shld_dmg_resistance,
        red_powerup_body_multiplier,
        red_powerup_body_recharge_rate,
        red_powerup_shld_multiplier,
        red_powerup_shld_recharge_rate,
        red_powerup_overshld_recharge_rate,
        red_powerup_headshot_immunity,
        red_powerup_assassination_immunity,
        red_powerup_deathless,
        red_powerup_vampirism,
        red_powerup_dmg_modifier_percentage,
        red_powerup_melee_dmg_modifier_percentage,
        red_powerup_initial_primary_weapon,
        red_powerup_initial_secondary_weapon,
        red_powerup_initial_equipment,
        red_powerup_initial_grenade_count,
        red_powerup_recharging_grenades,
        red_powerup_infinite_ammo,
        red_powerup_infinite_equipment,
        red_powerup_weapon_pickup,
        red_powerup_equipment_usage,
        red_powerup_equipment_drops,
        red_powerup_walking_speed,
        red_powerup_personal_gravity,
        red_powerup_vehicle_use,
        red_powerup_jump_modifier,
        red_powerup_double_jump,
        red_powerup_motion_tracker,
        red_powerup_motion_tracker_range,
        red_powerup_directional_damage,
        red_powerup_waypoint,
        red_powerup_active_camo,
        red_powerup_visual,
        red_powerup_forced_change_colors,
        red_powerup_gamertag_visibility,
        blue_powerup_shld_dmg_resistance,
        blue_powerup_body_multiplier,
        blue_powerup_body_recharge_rate,
        blue_powerup_shld_multiplier,
        blue_powerup_shld_recharge_rate,
        blue_powerup_overshld_recharge_rate,
        blue_powerup_headshot_immunity,
        blue_powerup_assassination_immunity,
        blue_powerup_deathless,
        blue_powerup_vampirism,
        blue_powerup_dmg_modifier_percentage,
        blue_powerup_melee_dmg_modifier_percentage,
        blue_powerup_initial_primary_weapon,
        blue_powerup_initial_secondary_weapon,
        blue_powerup_initial_equipment,
        blue_powerup_initial_grenade_count,
        blue_powerup_recharging_grenades,
        blue_powerup_infinite_ammo,
        blue_powerup_infinite_equipment,
        blue_powerup_weapon_pickup,
        blue_powerup_equipment_usage,
        blue_powerup_equipment_drops,
        blue_powerup_walking_speed,
        blue_powerup_personal_gravity,
        blue_powerup_vehicle_use,
        blue_powerup_jump_modifier,
        blue_powerup_double_jump,
        blue_powerup_motion_tracker,
        blue_powerup_motion_tracker_range,
        blue_powerup_directional_damage,
        blue_powerup_waypoint,
        blue_powerup_active_camo,
        blue_powerup_visual,
        blue_powerup_forced_change_colors,
        blue_powerup_gamertag_visibility,
        yellow_powerup_shld_dmg_resistance,
        yellow_powerup_body_multiplier,
        yellow_powerup_body_recharge_rate,
        yellow_powerup_shld_multiplier,
        yellow_powerup_shld_recharge_rate,
        yellow_powerup_overshld_recharge_rate,
        yellow_powerup_headshot_immunity,
        yellow_powerup_assassination_immunity,
        yellow_powerup_deathless,
        yellow_powerup_vampirism,
        yellow_powerup_dmg_modifier_percentage,
        yellow_powerup_melee_dmg_modifier_percentage,
        yellow_powerup_initial_primary_weapon,
        yellow_powerup_initial_secondary_weapon,
        yellow_powerup_initial_equipment,
        yellow_powerup_initial_grenade_count,
        yellow_powerup_recharging_grenades,
        yellow_powerup_infinite_ammo,
        yellow_powerup_infinite_equipment,
        yellow_powerup_weapon_pickup,
        yellow_powerup_equipment_usage,
        yellow_powerup_equipment_drops,
        yellow_powerup_walking_speed,
        yellow_powerup_personal_gravity,
        yellow_powerup_vehicle_use,
        yellow_powerup_jump_modifier,
        yellow_powerup_double_jump,
        yellow_powerup_motion_tracker,
        yellow_powerup_motion_tracker_range,
        yellow_powerup_directional_damage,
        yellow_powerup_waypoint,
        yellow_powerup_active_camo,
        yellow_powerup_visual,
        yellow_powerup_forced_change_colors,
        yellow_powerup_gamertag_visibility,
        game_map_red_powerup_duration,
        game_map_blue_powerup_duration,
        game_map_yellow_powerup_duration,
        sandbox_open_voice_channel,
        sandbox_enter_editor_mode,
        sandbox_respawn_time,
        sandbox_editor_shld_dmg_resistance,
        sandbox_editor_body_multiplier,
        sandbox_editor_body_recharge_rate,
        sandbox_editor_shld_multiplier,
        sandbox_editor_shld_recharge_rate,
        sandbox_editor_overshld_recharge_rate,
        sandbox_editor_headshot_immunity,
        sandbox_editor_assassination_immunity,
        sandbox_editor_deathless,
        sandbox_editor_vampirism,
        sandbox_editor_dmg_modifier_percentage,
        sandbox_editor_melee_dmg_modifier_percentage,
        sandbox_editor_initial_primary_weapon,
        sandbox_editor_initial_secondary_weapon,
        sandbox_editor_initial_equipment,
        sandbox_editor_initial_grenade_count,
        sandbox_editor_recharging_grenades,
        sandbox_editor_infinite_ammo,
        sandbox_editor_infinite_equipment,
        sandbox_editor_weapon_pickup,
        sandbox_editor_equipment_usage,
        sandbox_editor_equipment_drops,
        sandbox_editor_walking_speed,
        sandbox_editor_personal_gravity,
        sandbox_editor_vehicle_use,
        sandbox_editor_jump_modifier,
        sandbox_editor_double_jump,
        sandbox_editor_motion_tracker,
        sandbox_editor_motion_tracker_range,
        sandbox_editor_directional_damage,
        sandbox_editor_waypoint,
        sandbox_editor_active_camo,
        sandbox_editor_visual,
        sandbox_editor_forced_change_colors,
        sandbox_editor_gamertag_visibility,
        megalo_custom_1_shld_dmg_resistance,
        megalo_custom_1_body_multiplier,
        megalo_custom_1_body_recharge_rate,
        megalo_custom_1_shld_multiplier,
        megalo_custom_1_shld_recharge_rate,
        megalo_custom_1_overshld_recharge_rate,
        megalo_custom_1_headshot_immunity,
        megalo_custom_1_assassination_immunity,
        megalo_custom_1_deathless,
        megalo_custom_1_vampirism,
        megalo_custom_1_dmg_modifier_percentage,
        megalo_custom_1_melee_dmg_modifier_percentage,
        megalo_custom_1_initial_primary_weapon,
        megalo_custom_1_initial_secondary_weapon,
        megalo_custom_1_initial_equipment,
        megalo_custom_1_initial_grenade_count,
        megalo_custom_1_recharging_grenades,
        megalo_custom_1_infinite_ammo,
        megalo_custom_1_infinite_equipment,
        megalo_custom_1_weapon_pickup,
        megalo_custom_1_equipment_usage,
        megalo_custom_1_equipment_drops,
        megalo_custom_1_walking_speed,
        megalo_custom_1_personal_gravity,
        megalo_custom_1_vehicle_use,
        megalo_custom_1_jump_modifier,
        megalo_custom_1_double_jump,
        megalo_custom_1_motion_tracker,
        megalo_custom_1_motion_tracker_range,
        megalo_custom_1_directional_damage,
        megalo_custom_1_waypoint,
        megalo_custom_1_active_camo,
        megalo_custom_1_visual,
        megalo_custom_1_forced_change_colors,
        megalo_custom_1_gamertag_visibility,
        megalo_custom_2_shld_dmg_resistance,
        megalo_custom_2_body_multiplier,
        megalo_custom_2_body_recharge_rate,
        megalo_custom_2_shld_multiplier,
        megalo_custom_2_shld_recharge_rate,
        megalo_custom_2_overshld_recharge_rate,
        megalo_custom_2_headshot_immunity,
        megalo_custom_2_assassination_immunity,
        megalo_custom_2_deathless,
        megalo_custom_2_vampirism,
        megalo_custom_2_dmg_modifier_percentage,
        megalo_custom_2_melee_dmg_modifier_percentage,
        megalo_custom_2_initial_primary_weapon,
        megalo_custom_2_initial_secondary_weapon,
        megalo_custom_2_initial_equipment,
        megalo_custom_2_initial_grenade_count,
        megalo_custom_2_recharging_grenades,
        megalo_custom_2_infinite_ammo,
        megalo_custom_2_infinite_equipment,
        megalo_custom_2_weapon_pickup,
        megalo_custom_2_equipment_usage,
        megalo_custom_2_equipment_drops,
        megalo_custom_2_walking_speed,
        megalo_custom_2_personal_gravity,
        megalo_custom_2_vehicle_use,
        megalo_custom_2_jump_modifier,
        megalo_custom_2_double_jump,
        megalo_custom_2_motion_tracker,
        megalo_custom_2_motion_tracker_range,
        megalo_custom_2_directional_damage,
        megalo_custom_2_waypoint,
        megalo_custom_2_active_camo,
        megalo_custom_2_visual,
        megalo_custom_2_forced_change_colors,
        megalo_custom_2_gamertag_visibility,
        megalo_custom_3_shld_dmg_resistance,
        megalo_custom_3_body_multiplier,
        megalo_custom_3_body_recharge_rate,
        megalo_custom_3_shld_multiplier,
        megalo_custom_3_shld_recharge_rate,
        megalo_custom_3_overshld_recharge_rate,
        megalo_custom_3_headshot_immunity,
        megalo_custom_3_assassination_immunity,
        megalo_custom_3_deathless,
        megalo_custom_3_vampirism,
        megalo_custom_3_dmg_modifier_percentage,
        megalo_custom_3_melee_dmg_modifier_percentage,
        megalo_custom_3_initial_primary_weapon,
        megalo_custom_3_initial_secondary_weapon,
        megalo_custom_3_initial_equipment,
        megalo_custom_3_initial_grenade_count,
        megalo_custom_3_recharging_grenades,
        megalo_custom_3_infinite_ammo,
        megalo_custom_3_infinite_equipment,
        megalo_custom_3_weapon_pickup,
        megalo_custom_3_equipment_usage,
        megalo_custom_3_equipment_drops,
        megalo_custom_3_walking_speed,
        megalo_custom_3_personal_gravity,
        megalo_custom_3_vehicle_use,
        megalo_custom_3_jump_modifier,
        megalo_custom_3_double_jump,
        megalo_custom_3_motion_tracker,
        megalo_custom_3_motion_tracker_range,
        megalo_custom_3_directional_damage,
        megalo_custom_3_waypoint,
        megalo_custom_3_active_camo,
        megalo_custom_3_visual,
        megalo_custom_3_forced_change_colors,
        megalo_custom_3_gamertag_visibility,
        megalo_custom_4_shld_dmg_resistance,
        megalo_custom_4_body_multiplier,
        megalo_custom_4_body_recharge_rate,
        megalo_custom_4_shld_multiplier,
        megalo_custom_4_shld_recharge_rate,
        megalo_custom_4_overshld_recharge_rate,
        megalo_custom_4_headshot_immunity,
        megalo_custom_4_assassination_immunity,
        megalo_custom_4_deathless,
        megalo_custom_4_vampirism,
        megalo_custom_4_dmg_modifier_percentage,
        megalo_custom_4_melee_dmg_modifier_percentage,
        megalo_custom_4_initial_primary_weapon,
        megalo_custom_4_initial_secondary_weapon,
        megalo_custom_4_initial_equipment,
        megalo_custom_4_initial_grenade_count,
        megalo_custom_4_recharging_grenades,
        megalo_custom_4_infinite_ammo,
        megalo_custom_4_infinite_equipment,
        megalo_custom_4_weapon_pickup,
        megalo_custom_4_equipment_usage,
        megalo_custom_4_equipment_drops,
        megalo_custom_4_walking_speed,
        megalo_custom_4_personal_gravity,
        megalo_custom_4_vehicle_use,
        megalo_custom_4_jump_modifier,
        megalo_custom_4_double_jump,
        megalo_custom_4_motion_tracker,
        megalo_custom_4_motion_tracker_range,
        megalo_custom_4_directional_damage,
        megalo_custom_4_waypoint,
        megalo_custom_4_active_camo,
        megalo_custom_4_visual,
        megalo_custom_4_forced_change_colors,
        megalo_custom_4_gamertag_visibility,
        template_shld_dmg_resistance,
        template_body_multiplier,
        template_body_recharge_rate,
        template_shld_multiplier,
        template_shld_recharge_rate,
        template_overshld_recharge_rate,
        template_headshot_immunity,
        template_assassination_immunity,
        template_deathless,
        template_vampirism,
        template_dmg_modifier_percentage,
        template_melee_dmg_modifier_percentage,
        template_initial_primary_weapon,
        template_initial_secondary_weapon,
        template_initial_equipment,
        template_initial_grenade_count,
        template_recharging_grenades,
        template_infinite_ammo,
        template_infinite_equipment,
        template_weapon_pickup,
        template_equipment_usage,
        template_equipment_drops,
        template_walking_speed,
        template_personal_gravity,
        template_vehicle_use,
        template_jump_modifier,
        template_double_jump,
        template_motion_tracker,
        template_motion_tracker_range,
        template_directional_damage,
        template_waypoint,
        template_active_camo,
        template_visual,
        template_forced_change_colors,
        template_gamertag_visibility,
        template_vision,
        template_hearing,
        template_luck,
        template_weapon,
        template_grenades,
        template_equipment_drop,
        template_ai_assassination_immunity,
        template_ai_headshot_immunity,
        template_ai_damage_resistance,
        template_ai_damage_modifier,
        survival_enable_scenario_hazards,
        survival_enable_weapon_drops,
        survival_enable_ammo_crates,
        survival_generator_defend_all,
        survival_generator_random_spawn,
        survival_game_difficulty,
        survival_set_count,
        survival_bonus_lives_awarded,
        survival_bonus_target,
        survival_spartan_lives_on_elite_death,
        survival_extra_life_score_target,
        survival_shared_team_life_count,
        survival_elite_life_count,
        survival_maximum_lives,
        survival_generator_count,
        game_elite_respawn_inherit_timer,
        game_elite_respawn_with_teammate,
        game_elite_respawn_at_location,
        game_elite_respawn_on_kill,
        game_elite_respawn_lives_per_round,
        game_elite_respawn_team_lives_per_round,
        game_elite_respawn_respawn_time,
        game_elite_respawn_suicide_penalty,
        game_elite_respawn_betrayal_penalty,
        game_elite_respawn_growth,
        game_elite_respawn_initial_loadout_selection_time,
        game_elite_respawn_traits_duration,
        elite_respawn_shld_dmg_resistance,
        elite_respawn_body_multiplier,
        elite_respawn_body_recharge_rate,
        elite_respawn_shld_multiplier,
        elite_respawn_shld_recharge_rate,
        elite_respawn_overshld_recharge_rate,
        elite_respawn_headshot_immunity,
        elite_respawn_assassination_immunity,
        elite_respawn_deathless,
        elite_respawn_vampirism,
        elite_respawn_dmg_modifier_percentage,
        elite_respawn_melee_dmg_modifier_percentage,
        elite_respawn_initial_primary_weapon,
        elite_respawn_initial_secondary_weapon,
        elite_respawn_initial_equipment,
        elite_respawn_initial_grenade_count,
        elite_respawn_recharging_grenades,
        elite_respawn_infinite_ammo,
        elite_respawn_infinite_equipment,
        elite_respawn_weapon_pickup,
        elite_respawn_equipment_usage,
        elite_respawn_equipment_drops,
        elite_respawn_walking_speed,
        elite_respawn_personal_gravity,
        elite_respawn_vehicle_use,
        elite_respawn_jump_modifier,
        elite_respawn_double_jump,
        elite_respawn_motion_tracker,
        elite_respawn_motion_tracker_range,
        elite_respawn_directional_damage,
        elite_respawn_waypoint,
        elite_respawn_active_camo,
        elite_respawn_visual,
        elite_respawn_forced_change_colors,
        elite_respawn_gamertag_visibility,
        spartan_player_traits_shld_dmg_resistance,
        spartan_player_traits_body_multiplier,
        spartan_player_traits_body_recharge_rate,
        spartan_player_traits_shld_multiplier,
        spartan_player_traits_shld_recharge_rate,
        spartan_player_traits_overshld_recharge_rate,
        spartan_player_traits_headshot_immunity,
        spartan_player_traits_assassination_immunity,
        spartan_player_traits_deathless,
        spartan_player_traits_vampirism,
        spartan_player_traits_dmg_modifier_percentage,
        spartan_player_traits_melee_dmg_modifier_percentage,
        spartan_player_traits_initial_primary_weapon,
        spartan_player_traits_initial_secondary_weapon,
        spartan_player_traits_initial_equipment,
        spartan_player_traits_initial_grenade_count,
        spartan_player_traits_recharging_grenades,
        spartan_player_traits_infinite_ammo,
        spartan_player_traits_infinite_equipment,
        spartan_player_traits_weapon_pickup,
        spartan_player_traits_equipment_usage,
        spartan_player_traits_equipment_drops,
        spartan_player_traits_walking_speed,
        spartan_player_traits_personal_gravity,
        spartan_player_traits_vehicle_use,
        spartan_player_traits_jump_modifier,
        spartan_player_traits_double_jump,
        spartan_player_traits_motion_tracker,
        spartan_player_traits_motion_tracker_range,
        spartan_player_traits_directional_damage,
        spartan_player_traits_waypoint,
        spartan_player_traits_active_camo,
        spartan_player_traits_visual,
        spartan_player_traits_forced_change_colors,
        spartan_player_traits_gamertag_visibility,
        elite_player_traits_shld_dmg_resistance,
        elite_player_traits_body_multiplier,
        elite_player_traits_body_recharge_rate,
        elite_player_traits_shld_multiplier,
        elite_player_traits_shld_recharge_rate,
        elite_player_traits_overshld_recharge_rate,
        elite_player_traits_headshot_immunity,
        elite_player_traits_assassination_immunity,
        elite_player_traits_deathless,
        elite_player_traits_vampirism,
        elite_player_traits_dmg_modifier_percentage,
        elite_player_traits_melee_dmg_modifier_percentage,
        elite_player_traits_initial_primary_weapon,
        elite_player_traits_initial_secondary_weapon,
        elite_player_traits_initial_equipment,
        elite_player_traits_initial_grenade_count,
        elite_player_traits_recharging_grenades,
        elite_player_traits_infinite_ammo,
        elite_player_traits_infinite_equipment,
        elite_player_traits_weapon_pickup,
        elite_player_traits_equipment_usage,
        elite_player_traits_equipment_drops,
        elite_player_traits_walking_speed,
        elite_player_traits_personal_gravity,
        elite_player_traits_vehicle_use,
        elite_player_traits_jump_modifier,
        elite_player_traits_double_jump,
        elite_player_traits_motion_tracker,
        elite_player_traits_motion_tracker_range,
        elite_player_traits_directional_damage,
        elite_player_traits_waypoint,
        elite_player_traits_active_camo,
        elite_player_traits_visual,
        elite_player_traits_forced_change_colors,
        elite_player_traits_gamertag_visibility,
        survival_ai_traits_vision,
        survival_ai_traits_hearing,
        survival_ai_traits_luck,
        survival_ai_traits_weapon,
        survival_ai_traits_grenades,
        survival_ai_traits_equipment_drop,
        survival_ai_traits_ai_assassination_immunity,
        survival_ai_traits_ai_headshot_immunity,
        survival_ai_traits_ai_damage_resistance,
        survival_ai_traits_ai_damage_modifier,
        survival_round_0_skull0,
        survival_round_0_skull1,
        survival_round_0_skull2,
        survival_round_0_skull3,
        survival_round_0_skull4,
        survival_round_0_skull5,
        survival_round_0_skull6,
        survival_round_0_skull7,
        survival_round_0_skull8,
        survival_round_0_skull9,
        survival_round_0_skull10,
        survival_round_0_skull11,
        survival_round_0_skull12,
        survival_round_0_skull13,
        survival_round_0_skull14,
        survival_round_0_skull15,
        survival_round_0_skull16,
        survival_round_0_skull17,
        survival_round_0_initial_wave_delivered_via_dropship,
        survival_round_0_initial_wave_wave_squad_advance_type,
        survival_round_0_initial_wave_squad_type0,
        survival_round_0_initial_wave_squad_type1,
        survival_round_0_initial_wave_squad_type2,
        survival_round_0_initial_wave_squad_type3,
        survival_round_0_initial_wave_squad_type4,
        survival_round_0_initial_wave_squad_type5,
        survival_round_0_initial_wave_squad_type6,
        survival_round_0_initial_wave_squad_type7,
        survival_round_0_initial_wave_squad_type8,
        survival_round_0_initial_wave_squad_type9,
        survival_round_0_initial_wave_squad_type10,
        survival_round_0_initial_wave_squad_type11,
        survival_round_0_primary_wave_delivered_via_dropship,
        survival_round_0_primary_wave_wave_squad_advance_type,
        survival_round_0_primary_wave_squad_type0,
        survival_round_0_primary_wave_squad_type1,
        survival_round_0_primary_wave_squad_type2,
        survival_round_0_primary_wave_squad_type3,
        survival_round_0_primary_wave_squad_type4,
        survival_round_0_primary_wave_squad_type5,
        survival_round_0_primary_wave_squad_type6,
        survival_round_0_primary_wave_squad_type7,
        survival_round_0_primary_wave_squad_type8,
        survival_round_0_primary_wave_squad_type9,
        survival_round_0_primary_wave_squad_type10,
        survival_round_0_primary_wave_squad_type11,
        survival_round_0_boss_wave_delivered_via_dropship,
        survival_round_0_boss_wave_wave_squad_advance_type,
        survival_round_0_boss_wave_squad_type0,
        survival_round_0_boss_wave_squad_type1,
        survival_round_0_boss_wave_squad_type2,
        survival_round_0_boss_wave_squad_type3,
        survival_round_0_boss_wave_squad_type4,
        survival_round_0_boss_wave_squad_type5,
        survival_round_0_boss_wave_squad_type6,
        survival_round_0_boss_wave_squad_type7,
        survival_round_0_boss_wave_squad_type8,
        survival_round_0_boss_wave_squad_type9,
        survival_round_0_boss_wave_squad_type10,
        survival_round_0_boss_wave_squad_type11,
        survival_round_1_skull0,
        survival_round_1_skull1,
        survival_round_1_skull2,
        survival_round_1_skull3,
        survival_round_1_skull4,
        survival_round_1_skull5,
        survival_round_1_skull6,
        survival_round_1_skull7,
        survival_round_1_skull8,
        survival_round_1_skull9,
        survival_round_1_skull10,
        survival_round_1_skull11,
        survival_round_1_skull12,
        survival_round_1_skull13,
        survival_round_1_skull14,
        survival_round_1_skull15,
        survival_round_1_skull16,
        survival_round_1_skull17,
        survival_round_1_initial_wave_delivered_via_dropship,
        survival_round_1_initial_wave_wave_squad_advance_type,
        survival_round_1_initial_wave_squad_type0,
        survival_round_1_initial_wave_squad_type1,
        survival_round_1_initial_wave_squad_type2,
        survival_round_1_initial_wave_squad_type3,
        survival_round_1_initial_wave_squad_type4,
        survival_round_1_initial_wave_squad_type5,
        survival_round_1_initial_wave_squad_type6,
        survival_round_1_initial_wave_squad_type7,
        survival_round_1_initial_wave_squad_type8,
        survival_round_1_initial_wave_squad_type9,
        survival_round_1_initial_wave_squad_type10,
        survival_round_1_initial_wave_squad_type11,
        survival_round_1_primary_wave_delivered_via_dropship,
        survival_round_1_primary_wave_wave_squad_advance_type,
        survival_round_1_primary_wave_squad_type0,
        survival_round_1_primary_wave_squad_type1,
        survival_round_1_primary_wave_squad_type2,
        survival_round_1_primary_wave_squad_type3,
        survival_round_1_primary_wave_squad_type4,
        survival_round_1_primary_wave_squad_type5,
        survival_round_1_primary_wave_squad_type6,
        survival_round_1_primary_wave_squad_type7,
        survival_round_1_primary_wave_squad_type8,
        survival_round_1_primary_wave_squad_type9,
        survival_round_1_primary_wave_squad_type10,
        survival_round_1_primary_wave_squad_type11,
        survival_round_1_boss_wave_delivered_via_dropship,
        survival_round_1_boss_wave_wave_squad_advance_type,
        survival_round_1_boss_wave_squad_type0,
        survival_round_1_boss_wave_squad_type1,
        survival_round_1_boss_wave_squad_type2,
        survival_round_1_boss_wave_squad_type3,
        survival_round_1_boss_wave_squad_type4,
        survival_round_1_boss_wave_squad_type5,
        survival_round_1_boss_wave_squad_type6,
        survival_round_1_boss_wave_squad_type7,
        survival_round_1_boss_wave_squad_type8,
        survival_round_1_boss_wave_squad_type9,
        survival_round_1_boss_wave_squad_type10,
        survival_round_1_boss_wave_squad_type11,
        survival_round_2_skull0,
        survival_round_2_skull1,
        survival_round_2_skull2,
        survival_round_2_skull3,
        survival_round_2_skull4,
        survival_round_2_skull5,
        survival_round_2_skull6,
        survival_round_2_skull7,
        survival_round_2_skull8,
        survival_round_2_skull9,
        survival_round_2_skull10,
        survival_round_2_skull11,
        survival_round_2_skull12,
        survival_round_2_skull13,
        survival_round_2_skull14,
        survival_round_2_skull15,
        survival_round_2_skull16,
        survival_round_2_skull17,
        survival_round_2_initial_wave_delivered_via_dropship,
        survival_round_2_initial_wave_wave_squad_advance_type,
        survival_round_2_initial_wave_squad_type0,
        survival_round_2_initial_wave_squad_type1,
        survival_round_2_initial_wave_squad_type2,
        survival_round_2_initial_wave_squad_type3,
        survival_round_2_initial_wave_squad_type4,
        survival_round_2_initial_wave_squad_type5,
        survival_round_2_initial_wave_squad_type6,
        survival_round_2_initial_wave_squad_type7,
        survival_round_2_initial_wave_squad_type8,
        survival_round_2_initial_wave_squad_type9,
        survival_round_2_initial_wave_squad_type10,
        survival_round_2_initial_wave_squad_type11,
        survival_round_2_primary_wave_delivered_via_dropship,
        survival_round_2_primary_wave_wave_squad_advance_type,
        survival_round_2_primary_wave_squad_type0,
        survival_round_2_primary_wave_squad_type1,
        survival_round_2_primary_wave_squad_type2,
        survival_round_2_primary_wave_squad_type3,
        survival_round_2_primary_wave_squad_type4,
        survival_round_2_primary_wave_squad_type5,
        survival_round_2_primary_wave_squad_type6,
        survival_round_2_primary_wave_squad_type7,
        survival_round_2_primary_wave_squad_type8,
        survival_round_2_primary_wave_squad_type9,
        survival_round_2_primary_wave_squad_type10,
        survival_round_2_primary_wave_squad_type11,
        survival_round_2_boss_wave_delivered_via_dropship,
        survival_round_2_boss_wave_wave_squad_advance_type,
        survival_round_2_boss_wave_squad_type0,
        survival_round_2_boss_wave_squad_type1,
        survival_round_2_boss_wave_squad_type2,
        survival_round_2_boss_wave_squad_type3,
        survival_round_2_boss_wave_squad_type4,
        survival_round_2_boss_wave_squad_type5,
        survival_round_2_boss_wave_squad_type6,
        survival_round_2_boss_wave_squad_type7,
        survival_round_2_boss_wave_squad_type8,
        survival_round_2_boss_wave_squad_type9,
        survival_round_2_boss_wave_squad_type10,
        survival_round_2_boss_wave_squad_type11,
        survival_bonus_wave_duration,
        survival_bonus_wave_skull0,
        survival_bonus_wave_skull1,
        survival_bonus_wave_skull2,
        survival_bonus_wave_skull3,
        survival_bonus_wave_skull4,
        survival_bonus_wave_skull5,
        survival_bonus_wave_skull6,
        survival_bonus_wave_skull7,
        survival_bonus_wave_skull8,
        survival_bonus_wave_skull9,
        survival_bonus_wave_skull10,
        survival_bonus_wave_skull11,
        survival_bonus_wave_skull12,
        survival_bonus_wave_skull13,
        survival_bonus_wave_skull14,
        survival_bonus_wave_skull15,
        survival_bonus_wave_skull16,
        survival_bonus_wave_skull17,
        survival_bonus_wave_delivered_via_dropship,
        survival_bonus_wave_wave_squad_advance_type,
        survival_bonus_wave_squad_type0,
        survival_bonus_wave_squad_type1,
        survival_bonus_wave_squad_type2,
        survival_bonus_wave_squad_type3,
        survival_bonus_wave_squad_type4,
        survival_bonus_wave_squad_type5,
        survival_bonus_wave_squad_type6,
        survival_bonus_wave_squad_type7,
        survival_bonus_wave_squad_type8,
        survival_bonus_wave_squad_type9,
        survival_bonus_wave_squad_type10,
        survival_bonus_wave_squad_type11,
        red_skull_spartan_player_traits_shld_dmg_resistance,
        red_skull_spartan_player_traits_body_multiplier,
        red_skull_spartan_player_traits_body_recharge_rate,
        red_skull_spartan_player_traits_shld_multiplier,
        red_skull_spartan_player_traits_shld_recharge_rate,
        red_skull_spartan_player_traits_overshld_recharge_rate,
        red_skull_spartan_player_traits_headshot_immunity,
        red_skull_spartan_player_traits_assassination_immunity,
        red_skull_spartan_player_traits_deathless,
        red_skull_spartan_player_traits_vampirism,
        red_skull_spartan_player_traits_dmg_modifier_percentage,
        red_skull_spartan_player_traits_melee_dmg_modifier_percentage,
        red_skull_spartan_player_traits_initial_primary_weapon,
        red_skull_spartan_player_traits_initial_secondary_weapon,
        red_skull_spartan_player_traits_initial_equipment,
        red_skull_spartan_player_traits_initial_grenade_count,
        red_skull_spartan_player_traits_recharging_grenades,
        red_skull_spartan_player_traits_infinite_ammo,
        red_skull_spartan_player_traits_infinite_equipment,
        red_skull_spartan_player_traits_weapon_pickup,
        red_skull_spartan_player_traits_equipment_usage,
        red_skull_spartan_player_traits_equipment_drops,
        red_skull_spartan_player_traits_walking_speed,
        red_skull_spartan_player_traits_personal_gravity,
        red_skull_spartan_player_traits_vehicle_use,
        red_skull_spartan_player_traits_jump_modifier,
        red_skull_spartan_player_traits_double_jump,
        red_skull_spartan_player_traits_motion_tracker,
        red_skull_spartan_player_traits_motion_tracker_range,
        red_skull_spartan_player_traits_directional_damage,
        red_skull_spartan_player_traits_waypoint,
        red_skull_spartan_player_traits_active_camo,
        red_skull_spartan_player_traits_visual,
        red_skull_spartan_player_traits_forced_change_colors,
        red_skull_spartan_player_traits_gamertag_visibility,
        red_skull_elite_player_traits_shld_dmg_resistance,
        red_skull_elite_player_traits_body_multiplier,
        red_skull_elite_player_traits_body_recharge_rate,
        red_skull_elite_player_traits_shld_multiplier,
        red_skull_elite_player_traits_shld_recharge_rate,
        red_skull_elite_player_traits_overshld_recharge_rate,
        red_skull_elite_player_traits_headshot_immunity,
        red_skull_elite_player_traits_assassination_immunity,
        red_skull_elite_player_traits_deathless,
        red_skull_elite_player_traits_vampirism,
        red_skull_elite_player_traits_dmg_modifier_percentage,
        red_skull_elite_player_traits_melee_dmg_modifier_percentage,
        red_skull_elite_player_traits_initial_primary_weapon,
        red_skull_elite_player_traits_initial_secondary_weapon,
        red_skull_elite_player_traits_initial_equipment,
        red_skull_elite_player_traits_initial_grenade_count,
        red_skull_elite_player_traits_recharging_grenades,
        red_skull_elite_player_traits_infinite_ammo,
        red_skull_elite_player_traits_infinite_equipment,
        red_skull_elite_player_traits_weapon_pickup,
        red_skull_elite_player_traits_equipment_usage,
        red_skull_elite_player_traits_equipment_drops,
        red_skull_elite_player_traits_walking_speed,
        red_skull_elite_player_traits_personal_gravity,
        red_skull_elite_player_traits_vehicle_use,
        red_skull_elite_player_traits_jump_modifier,
        red_skull_elite_player_traits_double_jump,
        red_skull_elite_player_traits_motion_tracker,
        red_skull_elite_player_traits_motion_tracker_range,
        red_skull_elite_player_traits_directional_damage,
        red_skull_elite_player_traits_waypoint,
        red_skull_elite_player_traits_active_camo,
        red_skull_elite_player_traits_visual,
        red_skull_elite_player_traits_forced_change_colors,
        red_skull_elite_player_traits_gamertag_visibility,
        red_skull_ai_traits_vision,
        red_skull_ai_traits_hearing,
        red_skull_ai_traits_luck,
        red_skull_ai_traits_weapon,
        red_skull_ai_traits_grenades,
        red_skull_ai_traits_equipment_drop,
        red_skull_ai_traits_ai_assassination_immunity,
        red_skull_ai_traits_ai_headshot_immunity,
        red_skull_ai_traits_ai_damage_resistance,
        red_skull_ai_traits_ai_damage_modifier,
        yellow_skull_spartan_player_traits_shld_dmg_resistance,
        yellow_skull_spartan_player_traits_body_multiplier,
        yellow_skull_spartan_player_traits_body_recharge_rate,
        yellow_skull_spartan_player_traits_shld_multiplier,
        yellow_skull_spartan_player_traits_shld_recharge_rate,
        yellow_skull_spartan_player_traits_overshld_recharge_rate,
        yellow_skull_spartan_player_traits_headshot_immunity,
        yellow_skull_spartan_player_traits_assassination_immunity,
        yellow_skull_spartan_player_traits_deathless,
        yellow_skull_spartan_player_traits_vampirism,
        yellow_skull_spartan_player_traits_dmg_modifier_percentage,
        yellow_skull_spartan_player_traits_melee_dmg_modifier_percentage,
        yellow_skull_spartan_player_traits_initial_primary_weapon,
        yellow_skull_spartan_player_traits_initial_secondary_weapon,
        yellow_skull_spartan_player_traits_initial_equipment,
        yellow_skull_spartan_player_traits_initial_grenade_count,
        yellow_skull_spartan_player_traits_recharging_grenades,
        yellow_skull_spartan_player_traits_infinite_ammo,
        yellow_skull_spartan_player_traits_infinite_equipment,
        yellow_skull_spartan_player_traits_weapon_pickup,
        yellow_skull_spartan_player_traits_equipment_usage,
        yellow_skull_spartan_player_traits_equipment_drops,
        yellow_skull_spartan_player_traits_walking_speed,
        yellow_skull_spartan_player_traits_personal_gravity,
        yellow_skull_spartan_player_traits_vehicle_use,
        yellow_skull_spartan_player_traits_jump_modifier,
        yellow_skull_spartan_player_traits_double_jump,
        yellow_skull_spartan_player_traits_motion_tracker,
        yellow_skull_spartan_player_traits_motion_tracker_range,
        yellow_skull_spartan_player_traits_directional_damage,
        yellow_skull_spartan_player_traits_waypoint,
        yellow_skull_spartan_player_traits_active_camo,
        yellow_skull_spartan_player_traits_visual,
        yellow_skull_spartan_player_traits_forced_change_colors,
        yellow_skull_spartan_player_traits_gamertag_visibility,
        yellow_skull_elite_player_traits_shld_dmg_resistance,
        yellow_skull_elite_player_traits_body_multiplier,
        yellow_skull_elite_player_traits_body_recharge_rate,
        yellow_skull_elite_player_traits_shld_multiplier,
        yellow_skull_elite_player_traits_shld_recharge_rate,
        yellow_skull_elite_player_traits_overshld_recharge_rate,
        yellow_skull_elite_player_traits_headshot_immunity,
        yellow_skull_elite_player_traits_assassination_immunity,
        yellow_skull_elite_player_traits_deathless,
        yellow_skull_elite_player_traits_vampirism,
        yellow_skull_elite_player_traits_dmg_modifier_percentage,
        yellow_skull_elite_player_traits_melee_dmg_modifier_percentage,
        yellow_skull_elite_player_traits_initial_primary_weapon,
        yellow_skull_elite_player_traits_initial_secondary_weapon,
        yellow_skull_elite_player_traits_initial_equipment,
        yellow_skull_elite_player_traits_initial_grenade_count,
        yellow_skull_elite_player_traits_recharging_grenades,
        yellow_skull_elite_player_traits_infinite_ammo,
        yellow_skull_elite_player_traits_infinite_equipment,
        yellow_skull_elite_player_traits_weapon_pickup,
        yellow_skull_elite_player_traits_equipment_usage,
        yellow_skull_elite_player_traits_equipment_drops,
        yellow_skull_elite_player_traits_walking_speed,
        yellow_skull_elite_player_traits_personal_gravity,
        yellow_skull_elite_player_traits_vehicle_use,
        yellow_skull_elite_player_traits_jump_modifier,
        yellow_skull_elite_player_traits_double_jump,
        yellow_skull_elite_player_traits_motion_tracker,
        yellow_skull_elite_player_traits_motion_tracker_range,
        yellow_skull_elite_player_traits_directional_damage,
        yellow_skull_elite_player_traits_waypoint,
        yellow_skull_elite_player_traits_active_camo,
        yellow_skull_elite_player_traits_visual,
        yellow_skull_elite_player_traits_forced_change_colors,
        yellow_skull_elite_player_traits_gamertag_visibility,
        yellow_skull_ai_traits_vision,
        yellow_skull_ai_traits_hearing,
        yellow_skull_ai_traits_luck,
        yellow_skull_ai_traits_weapon,
        yellow_skull_ai_traits_grenades,
        yellow_skull_ai_traits_equipment_drop,
        yellow_skull_ai_traits_ai_assassination_immunity,
        yellow_skull_ai_traits_ai_headshot_immunity,
        yellow_skull_ai_traits_ai_damage_resistance,
        yellow_skull_ai_traits_ai_damage_modifier,
        blue_skull_spartan_player_traits_shld_dmg_resistance,
        blue_skull_spartan_player_traits_body_multiplier,
        blue_skull_spartan_player_traits_body_recharge_rate,
        blue_skull_spartan_player_traits_shld_multiplier,
        blue_skull_spartan_player_traits_shld_recharge_rate,
        blue_skull_spartan_player_traits_overshld_recharge_rate,
        blue_skull_spartan_player_traits_headshot_immunity,
        blue_skull_spartan_player_traits_assassination_immunity,
        blue_skull_spartan_player_traits_deathless,
        blue_skull_spartan_player_traits_vampirism,
        blue_skull_spartan_player_traits_dmg_modifier_percentage,
        blue_skull_spartan_player_traits_melee_dmg_modifier_percentage,
        blue_skull_spartan_player_traits_initial_primary_weapon,
        blue_skull_spartan_player_traits_initial_secondary_weapon,
        blue_skull_spartan_player_traits_initial_equipment,
        blue_skull_spartan_player_traits_initial_grenade_count,
        blue_skull_spartan_player_traits_recharging_grenades,
        blue_skull_spartan_player_traits_infinite_ammo,
        blue_skull_spartan_player_traits_infinite_equipment,
        blue_skull_spartan_player_traits_weapon_pickup,
        blue_skull_spartan_player_traits_equipment_usage,
        blue_skull_spartan_player_traits_equipment_drops,
        blue_skull_spartan_player_traits_walking_speed,
        blue_skull_spartan_player_traits_personal_gravity,
        blue_skull_spartan_player_traits_vehicle_use,
        blue_skull_spartan_player_traits_jump_modifier,
        blue_skull_spartan_player_traits_double_jump,
        blue_skull_spartan_player_traits_motion_tracker,
        blue_skull_spartan_player_traits_motion_tracker_range,
        blue_skull_spartan_player_traits_directional_damage,
        blue_skull_spartan_player_traits_waypoint,
        blue_skull_spartan_player_traits_active_camo,
        blue_skull_spartan_player_traits_visual,
        blue_skull_spartan_player_traits_forced_change_colors,
        blue_skull_spartan_player_traits_gamertag_visibility,
        blue_skull_elite_player_traits_shld_dmg_resistance,
        blue_skull_elite_player_traits_body_multiplier,
        blue_skull_elite_player_traits_body_recharge_rate,
        blue_skull_elite_player_traits_shld_multiplier,
        blue_skull_elite_player_traits_shld_recharge_rate,
        blue_skull_elite_player_traits_overshld_recharge_rate,
        blue_skull_elite_player_traits_headshot_immunity,
        blue_skull_elite_player_traits_assassination_immunity,
        blue_skull_elite_player_traits_deathless,
        blue_skull_elite_player_traits_vampirism,
        blue_skull_elite_player_traits_dmg_modifier_percentage,
        blue_skull_elite_player_traits_melee_dmg_modifier_percentage,
        blue_skull_elite_player_traits_initial_primary_weapon,
        blue_skull_elite_player_traits_initial_secondary_weapon,
        blue_skull_elite_player_traits_initial_equipment,
        blue_skull_elite_player_traits_initial_grenade_count,
        blue_skull_elite_player_traits_recharging_grenades,
        blue_skull_elite_player_traits_infinite_ammo,
        blue_skull_elite_player_traits_infinite_equipment,
        blue_skull_elite_player_traits_weapon_pickup,
        blue_skull_elite_player_traits_equipment_usage,
        blue_skull_elite_player_traits_equipment_drops,
        blue_skull_elite_player_traits_walking_speed,
        blue_skull_elite_player_traits_personal_gravity,
        blue_skull_elite_player_traits_vehicle_use,
        blue_skull_elite_player_traits_jump_modifier,
        blue_skull_elite_player_traits_double_jump,
        blue_skull_elite_player_traits_motion_tracker,
        blue_skull_elite_player_traits_motion_tracker_range,
        blue_skull_elite_player_traits_directional_damage,
        blue_skull_elite_player_traits_waypoint,
        blue_skull_elite_player_traits_active_camo,
        blue_skull_elite_player_traits_visual,
        blue_skull_elite_player_traits_forced_change_colors,
        blue_skull_elite_player_traits_gamertag_visibility,
        blue_skull_ai_traits_vision,
        blue_skull_ai_traits_hearing,
        blue_skull_ai_traits_luck,
        blue_skull_ai_traits_weapon,
        blue_skull_ai_traits_grenades,
        blue_skull_ai_traits_equipment_drop,
        blue_skull_ai_traits_ai_assassination_immunity,
        blue_skull_ai_traits_ai_headshot_immunity,
        blue_skull_ai_traits_ai_damage_resistance,
        blue_skull_ai_traits_ai_damage_modifier,
        survival_round_template_skull0,
        survival_round_template_skull1,
        survival_round_template_skull2,
        survival_round_template_skull3,
        survival_round_template_skull4,
        survival_round_template_skull5,
        survival_round_template_skull6,
        survival_round_template_skull7,
        survival_round_template_skull8,
        survival_round_template_skull9,
        survival_round_template_skull10,
        survival_round_template_skull11,
        survival_round_template_skull12,
        survival_round_template_skull13,
        survival_round_template_skull14,
        survival_round_template_skull15,
        survival_round_template_skull16,
        survival_round_template_skull17,
        survival_round_template_initial_wave_delivered_via_dropship,
        survival_round_template_initial_wave_wave_squad_advance_type,
        survival_round_template_initial_wave_squad_type0,
        survival_round_template_initial_wave_squad_type1,
        survival_round_template_initial_wave_squad_type2,
        survival_round_template_initial_wave_squad_type3,
        survival_round_template_initial_wave_squad_type4,
        survival_round_template_initial_wave_squad_type5,
        survival_round_template_initial_wave_squad_type6,
        survival_round_template_initial_wave_squad_type7,
        survival_round_template_initial_wave_squad_type8,
        survival_round_template_initial_wave_squad_type9,
        survival_round_template_initial_wave_squad_type10,
        survival_round_template_initial_wave_squad_type11,
        survival_round_template_primary_wave_delivered_via_dropship,
        survival_round_template_primary_wave_wave_squad_advance_type,
        survival_round_template_primary_wave_squad_type0,
        survival_round_template_primary_wave_squad_type1,
        survival_round_template_primary_wave_squad_type2,
        survival_round_template_primary_wave_squad_type3,
        survival_round_template_primary_wave_squad_type4,
        survival_round_template_primary_wave_squad_type5,
        survival_round_template_primary_wave_squad_type6,
        survival_round_template_primary_wave_squad_type7,
        survival_round_template_primary_wave_squad_type8,
        survival_round_template_primary_wave_squad_type9,
        survival_round_template_primary_wave_squad_type10,
        survival_round_template_primary_wave_squad_type11,
        survival_round_template_boss_wave_delivered_via_dropship,
        survival_round_template_boss_wave_wave_squad_advance_type,
        survival_round_template_boss_wave_squad_type0,
        survival_round_template_boss_wave_squad_type1,
        survival_round_template_boss_wave_squad_type2,
        survival_round_template_boss_wave_squad_type3,
        survival_round_template_boss_wave_squad_type4,
        survival_round_template_boss_wave_squad_type5,
        survival_round_template_boss_wave_squad_type6,
        survival_round_template_boss_wave_squad_type7,
        survival_round_template_boss_wave_squad_type8,
        survival_round_template_boss_wave_squad_type9,
        survival_round_template_boss_wave_squad_type10,
        survival_round_template_boss_wave_squad_type11,
        spartan_loadouts_enabled,
        spartan_loadouts_tier1_loadout0_enabled,
        spartan_loadouts_tier1_loadout0_name,
        spartan_loadouts_tier1_loadout0_primary,
        spartan_loadouts_tier1_loadout0_secondary,
        spartan_loadouts_tier1_loadout0_equipment,
        spartan_loadouts_tier1_loadout0_grenades,
        spartan_loadouts_tier1_loadout1_enabled,
        spartan_loadouts_tier1_loadout1_name,
        spartan_loadouts_tier1_loadout1_primary,
        spartan_loadouts_tier1_loadout1_secondary,
        spartan_loadouts_tier1_loadout1_equipment,
        spartan_loadouts_tier1_loadout1_grenades,
        spartan_loadouts_tier1_loadout2_enabled,
        spartan_loadouts_tier1_loadout2_name,
        spartan_loadouts_tier1_loadout2_primary,
        spartan_loadouts_tier1_loadout2_secondary,
        spartan_loadouts_tier1_loadout2_equipment,
        spartan_loadouts_tier1_loadout2_grenades,
        spartan_loadouts_tier1_loadout3_enabled,
        spartan_loadouts_tier1_loadout3_name,
        spartan_loadouts_tier1_loadout3_primary,
        spartan_loadouts_tier1_loadout3_secondary,
        spartan_loadouts_tier1_loadout3_equipment,
        spartan_loadouts_tier1_loadout3_grenades,
        spartan_loadouts_tier1_loadout4_enabled,
        spartan_loadouts_tier1_loadout4_name,
        spartan_loadouts_tier1_loadout4_primary,
        spartan_loadouts_tier1_loadout4_secondary,
        spartan_loadouts_tier1_loadout4_equipment,
        spartan_loadouts_tier1_loadout4_grenades,
        spartan_loadouts_tier2_loadout0_enabled,
        spartan_loadouts_tier2_loadout0_name,
        spartan_loadouts_tier2_loadout0_primary,
        spartan_loadouts_tier2_loadout0_secondary,
        spartan_loadouts_tier2_loadout0_equipment,
        spartan_loadouts_tier2_loadout0_grenades,
        spartan_loadouts_tier2_loadout1_enabled,
        spartan_loadouts_tier2_loadout1_name,
        spartan_loadouts_tier2_loadout1_primary,
        spartan_loadouts_tier2_loadout1_secondary,
        spartan_loadouts_tier2_loadout1_equipment,
        spartan_loadouts_tier2_loadout1_grenades,
        spartan_loadouts_tier2_loadout2_enabled,
        spartan_loadouts_tier2_loadout2_name,
        spartan_loadouts_tier2_loadout2_primary,
        spartan_loadouts_tier2_loadout2_secondary,
        spartan_loadouts_tier2_loadout2_equipment,
        spartan_loadouts_tier2_loadout2_grenades,
        spartan_loadouts_tier2_loadout3_enabled,
        spartan_loadouts_tier2_loadout3_name,
        spartan_loadouts_tier2_loadout3_primary,
        spartan_loadouts_tier2_loadout3_secondary,
        spartan_loadouts_tier2_loadout3_equipment,
        spartan_loadouts_tier2_loadout3_grenades,
        spartan_loadouts_tier2_loadout4_enabled,
        spartan_loadouts_tier2_loadout4_name,
        spartan_loadouts_tier2_loadout4_primary,
        spartan_loadouts_tier2_loadout4_secondary,
        spartan_loadouts_tier2_loadout4_equipment,
        spartan_loadouts_tier2_loadout4_grenades,
        spartan_loadouts_tier3_loadout0_enabled,
        spartan_loadouts_tier3_loadout0_name,
        spartan_loadouts_tier3_loadout0_primary,
        spartan_loadouts_tier3_loadout0_secondary,
        spartan_loadouts_tier3_loadout0_equipment,
        spartan_loadouts_tier3_loadout0_grenades,
        spartan_loadouts_tier3_loadout1_enabled,
        spartan_loadouts_tier3_loadout1_name,
        spartan_loadouts_tier3_loadout1_primary,
        spartan_loadouts_tier3_loadout1_secondary,
        spartan_loadouts_tier3_loadout1_equipment,
        spartan_loadouts_tier3_loadout1_grenades,
        spartan_loadouts_tier3_loadout2_enabled,
        spartan_loadouts_tier3_loadout2_name,
        spartan_loadouts_tier3_loadout2_primary,
        spartan_loadouts_tier3_loadout2_secondary,
        spartan_loadouts_tier3_loadout2_equipment,
        spartan_loadouts_tier3_loadout2_grenades,
        spartan_loadouts_tier3_loadout3_enabled,
        spartan_loadouts_tier3_loadout3_name,
        spartan_loadouts_tier3_loadout3_primary,
        spartan_loadouts_tier3_loadout3_secondary,
        spartan_loadouts_tier3_loadout3_equipment,
        spartan_loadouts_tier3_loadout3_grenades,
        spartan_loadouts_tier3_loadout4_enabled,
        spartan_loadouts_tier3_loadout4_name,
        spartan_loadouts_tier3_loadout4_primary,
        spartan_loadouts_tier3_loadout4_secondary,
        spartan_loadouts_tier3_loadout4_equipment,
        spartan_loadouts_tier3_loadout4_grenades,
        elite_loadouts_enabled,
        elite_loadouts_tier1_loadout0_enabled,
        elite_loadouts_tier1_loadout0_name,
        elite_loadouts_tier1_loadout0_primary,
        elite_loadouts_tier1_loadout0_secondary,
        elite_loadouts_tier1_loadout0_equipment,
        elite_loadouts_tier1_loadout0_grenades,
        elite_loadouts_tier1_loadout1_enabled,
        elite_loadouts_tier1_loadout1_name,
        elite_loadouts_tier1_loadout1_primary,
        elite_loadouts_tier1_loadout1_secondary,
        elite_loadouts_tier1_loadout1_equipment,
        elite_loadouts_tier1_loadout1_grenades,
        elite_loadouts_tier1_loadout2_enabled,
        elite_loadouts_tier1_loadout2_name,
        elite_loadouts_tier1_loadout2_primary,
        elite_loadouts_tier1_loadout2_secondary,
        elite_loadouts_tier1_loadout2_equipment,
        elite_loadouts_tier1_loadout2_grenades,
        elite_loadouts_tier1_loadout3_enabled,
        elite_loadouts_tier1_loadout3_name,
        elite_loadouts_tier1_loadout3_primary,
        elite_loadouts_tier1_loadout3_secondary,
        elite_loadouts_tier1_loadout3_equipment,
        elite_loadouts_tier1_loadout3_grenades,
        elite_loadouts_tier1_loadout4_enabled,
        elite_loadouts_tier1_loadout4_name,
        elite_loadouts_tier1_loadout4_primary,
        elite_loadouts_tier1_loadout4_secondary,
        elite_loadouts_tier1_loadout4_equipment,
        elite_loadouts_tier1_loadout4_grenades,
        elite_loadouts_tier2_loadout0_enabled,
        elite_loadouts_tier2_loadout0_name,
        elite_loadouts_tier2_loadout0_primary,
        elite_loadouts_tier2_loadout0_secondary,
        elite_loadouts_tier2_loadout0_equipment,
        elite_loadouts_tier2_loadout0_grenades,
        elite_loadouts_tier2_loadout1_enabled,
        elite_loadouts_tier2_loadout1_name,
        elite_loadouts_tier2_loadout1_primary,
        elite_loadouts_tier2_loadout1_secondary,
        elite_loadouts_tier2_loadout1_equipment,
        elite_loadouts_tier2_loadout1_grenades,
        elite_loadouts_tier2_loadout2_enabled,
        elite_loadouts_tier2_loadout2_name,
        elite_loadouts_tier2_loadout2_primary,
        elite_loadouts_tier2_loadout2_secondary,
        elite_loadouts_tier2_loadout2_equipment,
        elite_loadouts_tier2_loadout2_grenades,
        elite_loadouts_tier2_loadout3_enabled,
        elite_loadouts_tier2_loadout3_name,
        elite_loadouts_tier2_loadout3_primary,
        elite_loadouts_tier2_loadout3_secondary,
        elite_loadouts_tier2_loadout3_equipment,
        elite_loadouts_tier2_loadout3_grenades,
        elite_loadouts_tier2_loadout4_enabled,
        elite_loadouts_tier2_loadout4_name,
        elite_loadouts_tier2_loadout4_primary,
        elite_loadouts_tier2_loadout4_secondary,
        elite_loadouts_tier2_loadout4_equipment,
        elite_loadouts_tier2_loadout4_grenades,
        elite_loadouts_tier3_loadout0_enabled,
        elite_loadouts_tier3_loadout0_name,
        elite_loadouts_tier3_loadout0_primary,
        elite_loadouts_tier3_loadout0_secondary,
        elite_loadouts_tier3_loadout0_equipment,
        elite_loadouts_tier3_loadout0_grenades,
        elite_loadouts_tier3_loadout1_enabled,
        elite_loadouts_tier3_loadout1_name,
        elite_loadouts_tier3_loadout1_primary,
        elite_loadouts_tier3_loadout1_secondary,
        elite_loadouts_tier3_loadout1_equipment,
        elite_loadouts_tier3_loadout1_grenades,
        elite_loadouts_tier3_loadout2_enabled,
        elite_loadouts_tier3_loadout2_name,
        elite_loadouts_tier3_loadout2_primary,
        elite_loadouts_tier3_loadout2_secondary,
        elite_loadouts_tier3_loadout2_equipment,
        elite_loadouts_tier3_loadout2_grenades,
        elite_loadouts_tier3_loadout3_enabled,
        elite_loadouts_tier3_loadout3_name,
        elite_loadouts_tier3_loadout3_primary,
        elite_loadouts_tier3_loadout3_secondary,
        elite_loadouts_tier3_loadout3_equipment,
        elite_loadouts_tier3_loadout3_grenades,
        elite_loadouts_tier3_loadout4_enabled,
        elite_loadouts_tier3_loadout4_name,
        elite_loadouts_tier3_loadout4_primary,
        elite_loadouts_tier3_loadout4_secondary,
        elite_loadouts_tier3_loadout4_equipment,
        elite_loadouts_tier3_loadout4_grenades,
        k_number_of_rtx_editable_game_variant_parameters,
        survival_turn_count,
        survival_disable_waves,
    }
}

pub const k_game_engine_custom_variant_encoding_version: i32 = 107;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_game_engine_custom_variant {
    pub m_encoding_version: i32,
    pub m_build_number: i32,
    pub m_base_variant: c_game_engine_base_variant,
    pub m_player_traits: Vec<s_player_trait_option>,
    pub m_user_defined_options: Vec<s_user_defined_option>,
    pub m_script_strings: c_string_table<112, 0x4C00, 15, 15, 7>,
    pub m_base_name_string_index: u8,
    pub m_localized_name: c_string_table<1, 0x180, 9, 9, 1>,
    pub m_localized_description: c_string_table<1, 0xC00, 12, 12, 1>,
    pub m_localized_category: c_string_table<1, 0x180, 9, 9, 1>,
    pub m_engine_icon: u8,
    pub m_engine_category: u8,
    pub m_map_permissions: c_megalogamengine_map_permissions,
    pub m_player_ratings: s_game_engine_player_rating_parameters,
    pub m_score_to_win_round: u16,
    pub m_fire_teams_enabled: bool,
    pub m_symmetric_gametype: bool,
    pub m_base_variant_parameters_locked: s_game_variant_parameter_flags,
    pub m_base_variant_parameters_hidden: s_game_variant_parameter_flags,
    pub m_user_defined_options_locked: StaticArray<bool, 32>,
    pub m_user_defined_options_hidden: StaticArray<bool, 32>,
    pub m_game_engine: s_custom_game_engine_definition,
    pub m_tu1_settings: c_game_engine_custom_variant_tu1_settings,

}

impl c_game_engine_custom_variant {
    pub fn initialize(&mut self) {
        *self = Self::default();
        self.m_encoding_version = k_game_engine_custom_variant_encoding_version;
        self.m_base_variant.initialize();
        self.m_base_variant.m_miscellaneous_options.m_round_reset_map = true;
        self.m_base_variant.m_miscellaneous_options.m_round_reset_players = true;
        self.m_player_ratings.initialize_to_default();
        self.m_map_permissions.initialize();
        self.m_game_engine.initialize();
        self.m_tu1_settings.initialize_to_default();
    }

    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_signed_integer(self.m_encoding_version, 32)?;
        bitstream.write_signed_integer(self.m_build_number, 32)?;
        self.m_base_variant.encode(bitstream)?;
        bitstream.write_integer(self.m_player_traits.len() as u16, 5)?;
        for player_trait in self.m_player_traits.iter() {
            player_trait.encode(bitstream)?;
        }
        bitstream.write_integer(self.m_user_defined_options.len() as u16, 5)?;
        for option in self.m_user_defined_options.iter() {
            option.encode(bitstream)?;
        }
        self.m_script_strings.encode(bitstream)?; // this is broken
        bitstream.write_integer(self.m_base_name_string_index, 7)?;
        self.m_localized_name.encode(bitstream)?;
        self.m_localized_description.encode(bitstream)?;
        self.m_localized_category.encode(bitstream)?;
        bitstream.write_integer(self.m_engine_icon, 5)?;
        bitstream.write_integer(self.m_engine_category, 5)?;
        self.m_map_permissions.encode(bitstream)?;
        self.m_player_ratings.encode(bitstream)?;
        bitstream.write_signed_integer(self.m_score_to_win_round, 16)?;
        bitstream.write_bool(self.m_fire_teams_enabled)?;
        bitstream.write_bool(self.m_symmetric_gametype)?;
        self.m_base_variant_parameters_locked.encode(bitstream, "base-variant-parameters-locked")?;
        self.m_base_variant_parameters_hidden.encode(bitstream, "base-variant-parameters-hidden")?;
        for parameter in &self.m_user_defined_options_locked {
            bitstream.write_bool(*parameter)?
        }
        for parameter in &self.m_user_defined_options_hidden {
            bitstream.write_bool(*parameter)?
        }
        self.m_game_engine.encode(bitstream)?;
        if self.m_encoding_version > 106 {
            self.m_tu1_settings.encode(bitstream)?;
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_encoding_version = bitstream.read_signed_integer("encoding-version", 32)?;
        self.m_build_number = bitstream.read_signed_integer("version", 32)?;
        self.m_base_variant.decode(bitstream)?;
        let player_trait_count = bitstream.read_integer("player-trait-count", 5)?;
        for i in 0..player_trait_count {
            let mut traits = s_player_trait_option::default();
            traits.decode(bitstream)?;
            self.m_player_traits.push(traits);
        }
        let user_defined_option_count = bitstream.read_integer("user-defined-option-count", 5)?;
        for i in 0..user_defined_option_count {
            let mut option = s_user_defined_option::default();
            option.decode(bitstream)?;
            self.m_user_defined_options.push(option);
        }
        self.m_script_strings.decode(bitstream)?;
        self.m_base_name_string_index = bitstream.read_integer("base-name-string-index", 7)?;
        self.m_localized_name.decode(bitstream)?;
        self.m_localized_description.decode(bitstream)?;
        self.m_localized_category.decode(bitstream)?;
        self.m_engine_icon = bitstream.read_integer("engine-icon-index", 5)?;
        self.m_engine_category = bitstream.read_integer("engine-category", 5)?;
        self.m_map_permissions.decode(bitstream)?;
        self.m_player_ratings.decode(bitstream)?;
        self.m_score_to_win_round = bitstream.read_signed_integer("score-to-win-round", 16)?;
        self.m_fire_teams_enabled = bitstream.read_bool("fire-teams-enabled")?;
        self.m_symmetric_gametype = bitstream.read_bool("symmetric-gametype")?;
        self.m_base_variant_parameters_locked.decode(bitstream, "base-variant-parameters-locked")?;
        self.m_base_variant_parameters_hidden.decode(bitstream, "base-variant-parameters-hidden")?;
        for i in 0..32 {
            self.m_user_defined_options_locked[i] = bitstream.read_bool("user-defined-options-locked")?;
        }
        for i in 0..32 {
            self.m_user_defined_options_hidden[i] = bitstream.read_bool("user-defined-options-hidden")?;
        }
        self.m_game_engine.decode(bitstream)?;
        if self.m_encoding_version > 106 {
            self.m_tu1_settings.decode(bitstream)?;
        }

        Ok(())
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive)]
pub enum e_game_mode {
    none = 0,
    sandbox = 1,
    #[default]
    megalogamengine = 2,
    campaign = 3,
    survival = 4,
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_game_variant {
    pub m_game_engine: e_game_mode,

    // campaign only uses a base variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_campaign_variant: Option<c_game_engine_campaign_variant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_custom_variant: Option<c_game_engine_custom_variant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_survival_variant: Option<c_game_engine_survival_variant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_sandbox_variant: Option<c_game_engine_sandbox_variant>,

}

impl c_game_variant {
    pub fn get_metadata(&self) -> BLFLibResult<&c_content_item_metadata> {
        match self.m_game_engine {
            e_game_mode::sandbox => {
                Ok(
                    &self.m_sandbox_variant.as_ref()
                        .ok_or_else(|| BLFLibError::from("m_sandbox_variant does not exist."))?
                        .m_custom_variant.m_base_variant.m_metadata
                )
            }
            e_game_mode::megalogamengine => {
                Ok(
                    &self.m_custom_variant.as_ref()
                        .ok_or_else(|| BLFLibError::from("m_custom_variant does not exist."))?
                        .m_base_variant.m_metadata
                )
            }
            e_game_mode::campaign => {
                Ok(
                    &self.m_campaign_variant.as_ref()
                        .ok_or_else(|| BLFLibError::from("m_campaign_variant does not exist."))?
                        .m_metadata
                )
            }
            e_game_mode::survival => {
                Ok(
                    &self.m_survival_variant.as_ref()
                        .ok_or_else(|| BLFLibError::from("m_survival_variant does not exist."))?
                        .m_base_variant.m_metadata
                )
            }
            e_game_mode::none => Err(BLFLibError::from("m_game_engine is none.")),
        }
    }

    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_enum_raw(self.m_game_engine.clone(), 4)?;

        match (&self.m_game_engine, &self.m_custom_variant, &self.m_campaign_variant, &self.m_survival_variant, &self.m_sandbox_variant) {
            (e_game_mode::sandbox, None, None, None, Some(sandbox_variant)) => {
                sandbox_variant.encode(bitstream)?;
            }
            (e_game_mode::megalogamengine, Some(custom_variant), None, None, None) => {
                custom_variant.encode(bitstream)?;
            }
            (e_game_mode::campaign, None, Some(campaign_variant), None, None) => {
                campaign_variant.encode(bitstream)?;
            }
            (e_game_mode::survival, None, None, Some(survival_variant), None) => {
                survival_variant.encode(bitstream)?;
            }
            _ => {
                Err(format!("Unrecognized game engine {:?}", self.m_game_engine))?;
            }
        }


        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_game_engine = bitstream.read_unnamed_enum_raw(4)?;

        match self.m_game_engine {
            e_game_mode::sandbox => {
                let mut sandbox_variant = c_game_engine_sandbox_variant::default();
                sandbox_variant.decode(bitstream)?;
                self.m_sandbox_variant = Some(sandbox_variant);
            }
            e_game_mode::megalogamengine => {
                // customs
                let mut custom_variant = c_game_engine_custom_variant::default();
                custom_variant.decode(bitstream)?;
                self.m_custom_variant = Some(custom_variant);
            }
            e_game_mode::campaign => {
                let mut campaign_variant = c_game_engine_campaign_variant::default();
                campaign_variant.decode(bitstream)?;
                self.m_campaign_variant = Some(campaign_variant);
            }
            e_game_mode::survival => {
                let mut survival_variant = c_game_engine_survival_variant::default();
                survival_variant.decode(bitstream)?;
                self.m_survival_variant = Some(survival_variant);
            }
            _ => {
                Err(format!("Unrecognized game engine {:?}", self.m_game_engine))?;
            }
        }

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_custom_game_engine_definition {
    pub m_conditions: Vec<c_condition>,
    pub m_actions: Vec<c_action>,
    pub m_triggers: Vec<c_trigger>,
    pub m_statistics: Vec<c_megalo_game_statistic>,
    pub m_global_variable_metadata: s_variable_metadata<4, 4, 4, 4, 5>,
    pub m_player_variable_metadata: s_variable_metadata<4, 3, 3, 3, 3>,
    pub m_object_variable_metadata: s_variable_metadata<4, 3, 2, 3, 3>,
    pub m_team_variable_metadata: s_variable_metadata<4, 3, 3, 3, 3>,
    pub m_hud_widgets: Vec<u8>,
    pub m_initialization_trigger_index: u16,
    pub m_local_initialization_trigger_index: u16,
    pub m_host_migration_trigger_index: u16,
    pub m_double_migration_trigger_index: u16,
    pub m_object_death_event_trigger_index: u16,
    pub m_local_trigger_index: u16,
    pub m_pregame_trigger_index: u16,
    pub m_objects_used: StaticArray<bool, 2048>,
    pub m_object_filters: Vec<c_object_filter>,
}

impl s_custom_game_engine_definition {
    pub fn initialize(&mut self) {
        *self = Self::default();
        self.m_initialization_trigger_index = (-1i16) as u16;
        self.m_local_initialization_trigger_index = (-1i16) as u16;
        self.m_host_migration_trigger_index = (-1i16) as u16;
        self.m_double_migration_trigger_index = (-1i16) as u16;
        self.m_object_death_event_trigger_index = (-1i16) as u16;
        self.m_local_trigger_index = (-1i16) as u16;
        self.m_pregame_trigger_index = (-1i16) as u16;
    }

    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_conditions.len() as u32, 10)?;
        for condition in &self.m_conditions {
            condition.encode(bitstream)?;
        }

        bitstream.write_integer(self.m_actions.len() as u32, 11)?;
        for action in &self.m_actions {
            action.encode(bitstream)?;
        }

        bitstream.write_integer(self.m_triggers.len() as u32, 9)?;
        for trigger in &self.m_triggers {
            trigger.encode(bitstream)?;
        }

        bitstream.write_integer(self.m_statistics.len() as u32, 3)?;
        for statistic in &self.m_statistics {
            statistic.encode(bitstream)?;
        }

        self.m_global_variable_metadata.encode(bitstream)?;
        self.m_player_variable_metadata.encode(bitstream)?;
        self.m_object_variable_metadata.encode(bitstream)?;
        self.m_team_variable_metadata.encode(bitstream)?;

        bitstream.write_integer(self.m_hud_widgets.len() as u32, 3)?;
        for widget in &self.m_hud_widgets {
            bitstream.write_integer(*widget, 4)?;
        }

        bitstream.write_integer(self.m_initialization_trigger_index as u32, 9)?;
        bitstream.write_integer(self.m_local_initialization_trigger_index as u32, 9)?;
        bitstream.write_integer(self.m_host_migration_trigger_index as u32, 9)?;
        bitstream.write_integer(self.m_double_migration_trigger_index as u32, 9)?;
        bitstream.write_integer(self.m_object_death_event_trigger_index as u32, 9)?;
        bitstream.write_integer(self.m_local_trigger_index as u32, 9)?;
        bitstream.write_integer(self.m_pregame_trigger_index as u32, 9)?;

        for object_type in self.m_objects_used.get() {
            bitstream.write_bool(*object_type)?
        }

        bitstream.write_integer(self.m_object_filters.len() as u32, 5)?;
        for filter in &self.m_object_filters {
            filter.encode(bitstream)?;
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        let condition_count: u16 = bitstream.read_integer("condition-count", 10)?;
        for i in 0..condition_count {
            let mut condition = c_condition::default();
            condition.decode(bitstream)?;
            self.m_conditions.push(condition);
        }

        let action_count: u16 = bitstream.read_integer("action-count", 11)?;
        for i in 0..action_count {
            let mut action = c_action::default();
            action.decode(bitstream)?;
            self.m_actions.push(action);
        }

        let trigger_count: u16 = bitstream.read_integer("trigger-count", 9)?;
        for i in 0..trigger_count {
            let mut trigger = c_trigger::default();
            trigger.decode(bitstream)?;
            self.m_triggers.push(trigger);
        }

        let statistic_count: u8 = bitstream.read_integer("game-statistic-count", 3)?;
        for i in 0..statistic_count {
            let mut statistic = c_megalo_game_statistic::default();
            statistic.decode(bitstream)?;
            self.m_statistics.push(statistic);
        }

        self.m_global_variable_metadata.decode(bitstream)?;
        self.m_player_variable_metadata.decode(bitstream)?;
        self.m_object_variable_metadata.decode(bitstream)?;
        self.m_team_variable_metadata.decode(bitstream)?;

        let widget_count: u8 = bitstream.read_integer("hud-widget-count", 3)?;
        for i in 0..widget_count {
            self.m_hud_widgets.push(bitstream.read_integer("position", 4)?);
        }

        self.m_initialization_trigger_index = bitstream.read_integer("initial-trigger-index", 9)?;
        self.m_local_initialization_trigger_index = bitstream.read_integer("local-initialization-trigger-index", 9)?;
        self.m_host_migration_trigger_index = bitstream.read_integer("host-migration-trigger-index", 9)?;
        self.m_double_migration_trigger_index = bitstream.read_integer("double-migration-trigger-index", 9)?;
        self.m_object_death_event_trigger_index = bitstream.read_integer("death-event-trigger-index", 9)?;
        self.m_local_trigger_index = bitstream.read_integer("local-trigger-index", 9)?;
        self.m_pregame_trigger_index = bitstream.read_integer("pregame-trigger-index", 9)?;

        for i in 0..2048 {
            self.m_objects_used[i] = bitstream.read_bool("object-types-used")?;
        }

        let object_filter_count: u8 = bitstream.read_integer("object-filter-count", 5)?;
        for i in 0..object_filter_count {
            let mut object_filter = c_object_filter::default();
            object_filter.decode(bitstream)?;
            self.m_object_filters.push(object_filter);
        }

        Ok(())
    }
}

bitfield! {
    #[derive(Serialize, Deserialize)]
    pub struct e_game_variant_tu1_flags: u32 {
        always_spillover_damage,
        armor_lock_stickies_remain,
        attached_damage_bypass_shields,
        active_camo_override_energy_curve,
        sword_gun_clang_kills,
        magnum_is_automatic,
        unknown,
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_game_engine_custom_variant_tu1_settings {
    pub m_flags: e_game_variant_tu1_flags,
    pub m_precision_bloom: Float32,
    pub m_active_camo_energy_curve_min: Float32,
    pub m_active_camo_energy_curve_max: Float32,
    pub m_armor_lock_damage_drain: Float32,
    pub m_armor_lock_damage_drain_limit: Float32,
    pub m_magnum_damage: Float32,
    pub m_magnum_fire_delay: Float32,
}

impl Default for c_game_engine_custom_variant_tu1_settings {
    fn default() -> Self {
        Self {
            m_flags: e_game_variant_tu1_flags::default(),
            m_precision_bloom: Float32(1.0),
            m_active_camo_energy_curve_min: Float32(0.2),
            m_active_camo_energy_curve_max: Float32(0.7),
            m_armor_lock_damage_drain: Float32(0.0),
            m_armor_lock_damage_drain_limit: Float32(0.0),
            m_magnum_damage: Float32(1.0),
            m_magnum_fire_delay: Float32(1.0),
        }
    }
}

impl c_game_engine_custom_variant_tu1_settings {
    pub fn initialize_to_default(&mut self) {
        *self = Self::default();
    }

    pub fn encode(&self, mut bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_flags.to_raw(), 32)?;
        bitstream.write_quantized_real(self.m_precision_bloom, 0f32, 2f32, 8, false, true)?;
        bitstream.write_quantized_real(self.m_active_camo_energy_curve_min, 0f32, 2f32, 8, false, true)?;
        bitstream.write_quantized_real(self.m_active_camo_energy_curve_max, 0f32, 2f32, 8, false, true)?;
        bitstream.write_quantized_real(self.m_armor_lock_damage_drain, 0f32, 2f32, 8, false, true)?;
        bitstream.write_quantized_real(self.m_armor_lock_damage_drain_limit, 0f32, 2f32, 8, false, true)?;
        bitstream.write_quantized_real(self.m_magnum_damage, 0f32, 10f32, 8, false, true)?;
        bitstream.write_quantized_real(self.m_magnum_fire_delay, 0f32, 10f32, 8, false, true)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_flags = e_game_variant_tu1_flags::from_raw(bitstream.read_integer("flags", 32)?);
        self.m_precision_bloom = bitstream.read_quantized_real(0f32, 2f32, 8, false, true)?;
        self.m_active_camo_energy_curve_min = bitstream.read_quantized_real(0f32, 2f32, 8, false, true)?;
        self.m_active_camo_energy_curve_max = bitstream.read_quantized_real(0f32, 2f32, 8, false, true)?;
        self.m_armor_lock_damage_drain = bitstream.read_quantized_real(0f32, 2f32, 8, false, true)?;
        self.m_armor_lock_damage_drain_limit = bitstream.read_quantized_real(0f32, 2f32, 8, false, true)?;
        self.m_magnum_damage = bitstream.read_quantized_real(0f32, 10f32, 8, false, true)?;
        self.m_magnum_fire_delay = bitstream.read_quantized_real(0f32, 10f32, 8, false, true)?;

        Ok(())
    }
}