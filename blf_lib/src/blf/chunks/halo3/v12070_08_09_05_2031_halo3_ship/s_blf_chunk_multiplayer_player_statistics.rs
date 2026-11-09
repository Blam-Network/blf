use binrw::{binrw, BinRead, BinWrite};
use crate::types::array::StaticArray;
use serde::{Deserialize, Serialize};
use crate::blf::chunks::BlfChunkHooks;
use crate::BlfChunk;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("mps1", 2.1)]
#[Size(0x4184)]
#[brw(big)]
pub struct s_blf_chunk_multiplayer_player_statistics {
    #[brw(pad_before = 4)]
    pub players: StaticArray<s_game_results_player_statistics, 16>,
}

impl BlfChunkHooks for s_blf_chunk_multiplayer_player_statistics {}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
pub struct s_game_results_statistics {
    #[br(map = |x: i16| x >> 1)]
    pub games_played: i16,
    #[br(map = |x: i16| x >> 1)]
    pub games_completed: i16,
    #[br(map = |x: i16| x >> 1)]
    pub games_won: i16,
    #[br(map = |x: i16| x >> 1)]
    pub games_tied: i16,
    #[br(map = |x: i16| x >> 1)]
    pub rounds_completed: i16,
    #[br(map = |x: i16| x >> 1)]
    pub rounds_won: i16,
    #[br(map = |x: i16| x >> 1)]
    pub in_round_score: i16,
    #[br(map = |x: i16| x >> 1)]
    pub in_game_total_score: i16,
    #[br(map = |x: i16| x >> 1)]
    pub kills: i16,
    #[br(map = |x: i16| x >> 1)]
    pub assists: i16,
    #[br(map = |x: i16| x >> 1)]
    pub deaths: i16,
    #[br(map = |x: i16| x >> 1)]
    pub betrayals: i16,
    #[br(map = |x: i16| x >> 1)]
    pub suicides: i16,
    #[br(map = |x: i16| x >> 1)]
    pub most_kills_in_a_row: i16,
    #[br(map = |x: i16| x >> 1)]
    pub seconds_alive: i16,
    #[br(map = |x: i16| x >> 1)]
    pub ctf_flag_scores: i16,
    #[br(map = |x: i16| x >> 1)]
    pub ctf_flag_grabs: i16,
    #[br(map = |x: i16| x >> 1)]
    pub ctf_flag_carrier_kills: i16,
    #[br(map = |x: i16| x >> 1)]
    pub ctf_flag_returns: i16,
    #[br(map = |x: i16| x >> 1)]
    pub assault_bomb_arms: i16,
    #[br(map = |x: i16| x >> 1)]
    pub assault_bomb_grabs: i16,
    #[br(map = |x: i16| x >> 1)]
    pub assault_bomb_disarms: i16,
    #[br(map = |x: i16| x >> 1)]
    pub assault_bomb_detonations: i16,
    #[br(map = |x: i16| x >> 1)]
    pub oddball_time_with_ball: i16,
    #[br(map = |x: i16| x >> 1)]
    pub oddball_unused: i16,
    #[br(map = |x: i16| x >> 1)]
    pub oddball_kills_as_carrier: i16,
    #[br(map = |x: i16| x >> 1)]
    pub oddball_ball_carrier_kills: i16,
    #[br(map = |x: i16| x >> 1)]
    pub king_time_on_hill: i16,
    #[br(map = |x: i16| x >> 1)]
    pub king_total_control_time: i16,
    #[br(map = |x: i16| x >> 1)]
    pub king_unused0: i16,
    #[br(map = |x: i16| x >> 1)]
    pub king_unused1: i16,
    #[br(map = |x: i16| x >> 1)]
    pub unused0: i16,
    #[br(map = |x: i16| x >> 1)]
    pub unused1: i16,
    #[br(map = |x: i16| x >> 1)]
    pub unused2: i16,
    #[br(map = |x: i16| x >> 1)]
    pub vip_takedowns: i16,
    #[br(map = |x: i16| x >> 1)]
    pub vip_kills_as_vip: i16,
    #[br(map = |x: i16| x >> 1)]
    pub vip_guard_time: i16,
    #[br(map = |x: i16| x >> 1)]
    pub vip_time_as_vip: i16,
    #[br(map = |x: i16| x >> 1)]
    pub vip_lives_as_vip: i16,
    #[br(map = |x: i16| x >> 1)]
    pub juggernaut_kills: i16,
    #[br(map = |x: i16| x >> 1)]
    pub juggernaut_kills_as_juggernaut: i16,
    #[br(map = |x: i16| x >> 1)]
    pub juggernaut_total_control_time: i16,
    #[br(map = |x: i16| x >> 1)]
    pub total_wp: i16,
    #[br(map = |x: i16| x >> 1)]
    pub juggernaut_unused: i16,
    #[br(map = |x: i16| x >> 1)]
    pub territories_owned: i16,
    #[br(map = |x: i16| x >> 1)]
    pub territories_captures: i16,
    #[br(map = |x: i16| x >> 1)]
    pub territories_ousts: i16,
    #[br(map = |x: i16| x >> 1)]
    pub territories_time_in_territory: i16,
    #[br(map = |x: i16| x >> 1)]
    pub infection_zombie_kills: i16,
    #[br(map = |x: i16| x >> 1)]
    pub infection_infections: i16,
    #[br(map = |x: i16| x >> 1)]
    pub infection_time_as_human: i16,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
pub struct s_game_results_player_statistics_medals {
    #[br(map = |x: i16| x >> 1)]
    pub extermination: i16,
    #[br(map = |x: i16| x >> 1)]
    pub perfection: i16,
    #[br(map = |x: i16| x >> 1)]
    pub multiple_kill_2: i16,
    #[br(map = |x: i16| x >> 1)]
    pub multiple_kill_3: i16,
    #[br(map = |x: i16| x >> 1)]
    pub multiple_kill_4: i16,
    #[br(map = |x: i16| x >> 1)]
    pub multiple_kill_5: i16,
    #[br(map = |x: i16| x >> 1)]
    pub multiple_kill_6: i16,
    #[br(map = |x: i16| x >> 1)]
    pub multiple_kill_7: i16,
    #[br(map = |x: i16| x >> 1)]
    pub multiple_kill_8: i16,
    #[br(map = |x: i16| x >> 1)]
    pub multiple_kill_9: i16,
    #[br(map = |x: i16| x >> 1)]
    pub multiple_kill_10: i16,
    #[br(map = |x: i16| x >> 1)]
    pub kills_in_a_row_5: i16,
    #[br(map = |x: i16| x >> 1)]
    pub kills_in_a_row_10: i16,
    #[br(map = |x: i16| x >> 1)]
    pub kills_in_a_row_15: i16,
    #[br(map = |x: i16| x >> 1)]
    pub kills_in_a_row_20: i16,
    #[br(map = |x: i16| x >> 1)]
    pub kills_in_a_row_25: i16,
    #[br(map = |x: i16| x >> 1)]
    pub kills_in_a_row_30: i16,
    #[br(map = |x: i16| x >> 1)]
    pub sniper_kill_5: i16,
    #[br(map = |x: i16| x >> 1)]
    pub sniper_kill_10: i16,
    #[br(map = |x: i16| x >> 1)]
    pub shotgun_kill_5: i16,
    #[br(map = |x: i16| x >> 1)]
    pub shotgun_kill_10: i16,
    #[br(map = |x: i16| x >> 1)]
    pub collision_kill_5: i16,
    #[br(map = |x: i16| x >> 1)]
    pub collision_kill_10: i16,
    #[br(map = |x: i16| x >> 1)]
    pub sword_kill_5: i16,
    #[br(map = |x: i16| x >> 1)]
    pub sword_kill_10: i16,
    #[br(map = |x: i16| x >> 1)]
    pub juggernaut_kill_5: i16,
    #[br(map = |x: i16| x >> 1)]
    pub juggernaut_kill_10: i16,
    #[br(map = |x: i16| x >> 1)]
    pub zombie_kill_5: i16,
    #[br(map = |x: i16| x >> 1)]
    pub zombie_kill_10: i16,
    #[br(map = |x: i16| x >> 1)]
    pub human_kill_5: i16,
    #[br(map = |x: i16| x >> 1)]
    pub human_kill_10: i16,
    #[br(map = |x: i16| x >> 1)]
    pub human_kill_15: i16,
    #[br(map = |x: i16| x >> 1)]
    pub koth_kill_5: i16,
    #[br(map = |x: i16| x >> 1)]
    pub shotgun_kill_sword: i16,
    #[br(map = |x: i16| x >> 1)]
    pub vehicle_impact_kill: i16,
    #[br(map = |x: i16| x >> 1)]
    pub vehicle_hijack: i16,
    #[br(map = |x: i16| x >> 1)]
    pub aircraft_hijack: i16,
    #[br(map = |x: i16| x >> 1)]
    pub deadplayer_kill: i16,
    #[br(map = |x: i16| x >> 1)]
    pub player_kill_spreeplayer: i16,
    #[br(map = |x: i16| x >> 1)]
    pub spartanlaser_kill: i16,
    #[br(map = |x: i16| x >> 1)]
    pub stickygrenade_kill: i16,
    #[br(map = |x: i16| x >> 1)]
    pub sniper_kill: i16,
    #[br(map = |x: i16| x >> 1)]
    pub bashbehind_kill: i16,
    #[br(map = |x: i16| x >> 1)]
    pub bash_kill: i16,
    #[br(map = |x: i16| x >> 1)]
    pub flame_kill: i16,
    #[br(map = |x: i16| x >> 1)]
    pub driver_assist_gunner: i16,
    #[br(map = |x: i16| x >> 1)]
    pub assault_bomb_planted: i16,
    #[br(map = |x: i16| x >> 1)]
    pub assault_player_kill_carrier: i16,
    #[br(map = |x: i16| x >> 1)]
    pub vip_player_kill_vip: i16,
    #[br(map = |x: i16| x >> 1)]
    pub juggernaut_player_kill_juggernaut: i16,
    #[br(map = |x: i16| x >> 1)]
    pub oddball_carrier_kill_player: i16,
    #[br(map = |x: i16| x >> 1)]
    pub ctf_flag_captured: i16,
    #[br(map = |x: i16| x >> 1)]
    pub ctf_flag_player_kill_carrier: i16,
    #[br(map = |x: i16| x >> 1)]
    pub ctf_flag_carrier_kill_player: i16,
    #[br(map = |x: i16| x >> 1)]
    pub infection_survive: i16,
    #[br(map = |x: i16| x >> 1)]
    pub nemesis: i16,
    #[br(map = |x: i16| x >> 1)]
    pub avenger: i16,
    #[br(map = |x: i16| x >> 1)]
    pub unused3: i16,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
pub struct s_game_results_player_statistics_achievements {
    #[br(map = |x: i16| x >> 1)]
    pub landfall: i16,
    #[br(map = |x: i16| x >> 1)]
    pub holdout: i16,
    #[br(map = |x: i16| x >> 1)]
    pub the_road: i16,
    #[br(map = |x: i16| x >> 1)]
    pub assault: i16,
    #[br(map = |x: i16| x >> 1)]
    pub cleansing: i16,
    #[br(map = |x: i16| x >> 1)]
    pub refuge: i16,
    #[br(map = |x: i16| x >> 1)]
    pub last_stand: i16,
    #[br(map = |x: i16| x >> 1)]
    pub the_key: i16,
    #[br(map = |x: i16| x >> 1)]
    pub _return: i16,
    #[br(map = |x: i16| x >> 1)]
    pub campaign_complete_normal: i16,
    #[br(map = |x: i16| x >> 1)]
    pub campaign_complete_heroic: i16,
    #[br(map = |x: i16| x >> 1)]
    pub campaign_complete_legendary: i16,
    #[br(map = |x: i16| x >> 1)]
    pub iron: i16,
    #[br(map = |x: i16| x >> 1)]
    pub black_eye: i16,
    #[br(map = |x: i16| x >> 1)]
    pub tough_luck: i16,
    #[br(map = |x: i16| x >> 1)]
    pub catch: i16,
    #[br(map = |x: i16| x >> 1)]
    pub fog: i16,
    #[br(map = |x: i16| x >> 1)]
    pub famine: i16,
    #[br(map = |x: i16| x >> 1)]
    pub thunderstorm: i16,
    #[br(map = |x: i16| x >> 1)]
    pub tilt: i16,
    #[br(map = |x: i16| x >> 1)]
    pub mythic: i16,
    #[br(map = |x: i16| x >> 1)]
    pub marathon_man: i16,
    #[br(map = |x: i16| x >> 1)]
    pub guerilla: i16,
    #[br(map = |x: i16| x >> 1)]
    pub demon: i16,
    #[br(map = |x: i16| x >> 1)]
    pub cavalier: i16,
    #[br(map = |x: i16| x >> 1)]
    pub askar: i16,
    #[br(map = |x: i16| x >> 1)]
    pub exterminator: i16,
    #[br(map = |x: i16| x >> 1)]
    pub ranger: i16,
    #[br(map = |x: i16| x >> 1)]
    pub vanguard: i16,
    #[br(map = |x: i16| x >> 1)]
    pub orpheus: i16,
    #[br(map = |x: i16| x >> 1)]
    pub reclaimer: i16,
    #[br(map = |x: i16| x >> 1)]
    pub graduate: i16,
    #[br(map = |x: i16| x >> 1)]
    pub unsc_spartan: i16,
    #[br(map = |x: i16| x >> 1)]
    pub spartan_officer: i16,
    #[br(map = |x: i16| x >> 1)]
    pub two_for_one: i16,
    #[br(map = |x: i16| x >> 1)]
    pub triple_kill: i16,
    #[br(map = |x: i16| x >> 1)]
    pub overkill: i16,
    #[br(map = |x: i16| x >> 1)]
    pub lee_r_wilson_memorial: i16,
    #[br(map = |x: i16| x >> 1)]
    pub killing_frenzy: i16,
    #[br(map = |x: i16| x >> 1)]
    pub steppin_razor: i16,
    #[br(map = |x: i16| x >> 1)]
    pub mongoose_mowdown: i16,
    #[br(map = |x: i16| x >> 1)]
    pub up_close_and_personal: i16,
    #[br(map = |x: i16| x >> 1)]
    pub mvp: i16,
    #[br(map = |x: i16| x >> 1)]
    pub maybe_next_time_buddy: i16,
    #[br(map = |x: i16| x >> 1)]
    pub too_close_to_the_sun: i16,
    #[br(map = |x: i16| x >> 1)]
    pub we_re_in_for_some_chop: i16,
    #[br(map = |x: i16| x >> 1)]
    pub fear_the_pink_mist: i16,
    #[br(map = |x: i16| x >> 1)]
    pub headshot_honcho: i16,
    #[br(map = |x: i16| x >> 1)]
    pub used_car_salesman: i16,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
pub struct s_game_results_damage_statistics {
    pub valid: i8,
    #[brw(pad_before = 1)]
    #[br(map = |x: i16| x >> 1)]
    pub kills: i16,
    #[br(map = |x: i16| x >> 1)]
    pub deaths: i16,
    #[br(map = |x: i16| x >> 1)]
    pub betrayals: i16,
    #[br(map = |x: i16| x >> 1)]
    pub suicides: i16,
    #[br(map = |x: i16| x >> 1)]
    pub headshots: i16,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
pub struct s_game_results_player_damage_source_statistics {
    pub unknown: s_game_results_damage_statistics,
    pub falling_damage: s_game_results_damage_statistics,
    pub collision_damage: s_game_results_damage_statistics,
    pub melee: s_game_results_damage_statistics,
    pub explosion: s_game_results_damage_statistics,
    pub magnum_pistol: s_game_results_damage_statistics,
    pub plasma_pistol: s_game_results_damage_statistics,
    pub needler: s_game_results_damage_statistics,
    pub excavator: s_game_results_damage_statistics,
    pub smg: s_game_results_damage_statistics,
    pub plasma_rifle: s_game_results_damage_statistics, // 10
    pub battle_rifle: s_game_results_damage_statistics,
    pub carbine: s_game_results_damage_statistics,
    pub shotgun: s_game_results_damage_statistics,
    pub sniper_rifle: s_game_results_damage_statistics,
    pub beam_rifle: s_game_results_damage_statistics,
    pub assault_rifle: s_game_results_damage_statistics,
    pub spike_rifle: s_game_results_damage_statistics,
    pub flak_cannon: s_game_results_damage_statistics,
    pub missile_launcher: s_game_results_damage_statistics,
    pub rocket_launcher: s_game_results_damage_statistics, // 20
    pub spartan_laser: s_game_results_damage_statistics,
    pub brute_shot: s_game_results_damage_statistics,
    pub flame_thrower: s_game_results_damage_statistics,
    pub sentinal_gun: s_game_results_damage_statistics,
    pub energy_sword: s_game_results_damage_statistics,
    pub gravity_hammer: s_game_results_damage_statistics,
    pub frag_grenade: s_game_results_damage_statistics,
    pub plasma_grenade: s_game_results_damage_statistics,
    pub claymore_grenade: s_game_results_damage_statistics,
    pub firebomb_grenade: s_game_results_damage_statistics, // 30
    pub flag: s_game_results_damage_statistics,
    pub bomb: s_game_results_damage_statistics,
    pub bomb_explosion: s_game_results_damage_statistics,
    pub ball: s_game_results_damage_statistics,
    pub human_turret: s_game_results_damage_statistics,
    pub plasma_cannon: s_game_results_damage_statistics,
    pub plasma_mortar: s_game_results_damage_statistics,
    pub plasma_turret: s_game_results_damage_statistics,
    pub banshee: s_game_results_damage_statistics,
    pub ghost: s_game_results_damage_statistics, // 40
    pub mongoose: s_game_results_damage_statistics,
    pub scorpion: s_game_results_damage_statistics,
    pub scorpion_gunner: s_game_results_damage_statistics,
    pub spectre_driver: s_game_results_damage_statistics,
    pub spectre_gunner: s_game_results_damage_statistics,
    pub warthog_driver: s_game_results_damage_statistics,
    pub warthog_gunner: s_game_results_damage_statistics,
    pub warthog_gunner_gauss: s_game_results_damage_statistics,
    pub wraith: s_game_results_damage_statistics,
    pub wraith_anti_infantry: s_game_results_damage_statistics, // 50
    pub tank: s_game_results_damage_statistics,
    pub chopper: s_game_results_damage_statistics,
    pub hornet: s_game_results_damage_statistics,
    pub mantis: s_game_results_damage_statistics,
    pub mauler: s_game_results_damage_statistics,
    pub sentinel_beam: s_game_results_damage_statistics,
    pub sentinel_rpg: s_game_results_damage_statistics,
    pub teleporter: s_game_results_damage_statistics,
    pub sandtrap_mine: s_game_results_damage_statistics,
    pub elephant_turret: s_game_results_damage_statistics,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
pub struct s_game_results_player_statistics {
    pub statistics: s_game_results_statistics,
    pub medals: s_game_results_player_statistics_medals,
    pub achievements: s_game_results_player_statistics_achievements,
    pub damage_statistics: s_game_results_player_damage_source_statistics,
}