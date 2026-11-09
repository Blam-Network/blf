use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use crate::blf::chunks::BlfChunkHooks;
use crate::{BlfChunk, TestSize};
use crate::types::array::StaticArray;
use crate::types::bool::Bool;
use crate::types::numbers::Float32;
use crate::types::u64::Unsigned64;
use num_derive::{FromPrimitive, ToPrimitive};

#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize,BinRead,BinWrite)]
#[Header("cmrp", 1.1)]
#[Size(0x243C)]
#[brw(big)]
pub struct s_blf_chunk_campaign_metagame_results {
    #[brw(pad_after = 3)]
    pub valid: Bool,
    pub results: s_campaign_metagame_runtime_globals,
}

impl BlfChunkHooks for s_blf_chunk_campaign_metagame_results {}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(0x2438)]
#[brw(big)]
pub struct s_campaign_metagame_runtime_globals {
    pub saved_secondary_globals: s_campaign_metagame_runtime_globals_secondary,
    pub stats: StaticArray<s_metagame_state, 4>,
    pub damage_stack: StaticArray<s_campaign_metagame_damage_event, 256>,
    pub damage_stack_count: i32,
    pub flags: i32,
    pub time_bonus: Float32,
    pub final_total_score: Float32,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(0x68)]
#[brw(big)]
pub struct s_campaign_metagame_runtime_globals_secondary {
    pub total_elapsed_tick_count: i32,
    #[brw(pad_after=3)]
    pub initialized: Bool,
    pub stats: StaticArray<s_metagame_state_secondary, 4>, // 1 per player
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(big)]
pub struct s_metagame_state_class_kill_counts {
    pub infantry: i32,
    pub leader: i32,
    pub hero: i32,
    pub specialist: i32,
    pub light_vehicle: i32,
    pub heavy_vehicle: i32,
    pub giant_vehicle: i32,
    pub standard_vehicle: i32
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(big)]
pub struct s_metagame_state_type_kill_counts {
    pub brute: s_metagame_state_class_kill_counts,
    pub grunt: s_metagame_state_class_kill_counts,
    pub jackal: s_metagame_state_class_kill_counts,
    pub marine: s_metagame_state_class_kill_counts,
    pub bugger: s_metagame_state_class_kill_counts,
    pub hunter: s_metagame_state_class_kill_counts,
    pub flood_infection: s_metagame_state_class_kill_counts,
    pub flood_carrier: s_metagame_state_class_kill_counts,
    pub flood_combat: s_metagame_state_class_kill_counts,
    pub flood_pure: s_metagame_state_class_kill_counts,
    pub sentinel: s_metagame_state_class_kill_counts,
    pub elite: s_metagame_state_class_kill_counts,
    pub turret: s_metagame_state_class_kill_counts,
    pub mongoose: s_metagame_state_class_kill_counts,
    pub warthog: s_metagame_state_class_kill_counts,
    pub scorpion: s_metagame_state_class_kill_counts,
    pub hornet: s_metagame_state_class_kill_counts,
    pub pelican: s_metagame_state_class_kill_counts,
    pub shade: s_metagame_state_class_kill_counts,
    pub watchtower: s_metagame_state_class_kill_counts,
    pub ghost: s_metagame_state_class_kill_counts,
    pub chopper: s_metagame_state_class_kill_counts,
    pub prowler: s_metagame_state_class_kill_counts,
    pub wraith: s_metagame_state_class_kill_counts,
    pub banshee: s_metagame_state_class_kill_counts,
    pub phantom: s_metagame_state_class_kill_counts,
    pub scarab: s_metagame_state_class_kill_counts,
    pub guntower: s_metagame_state_class_kill_counts,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(0x3F0)]
#[brw(big)]
pub struct s_metagame_state {
    pub player_identifier: Unsigned64,
    pub transient_subtotal: i32,
    pub subtotal: i32,
    pub medal_points: i32,
    pub scripted_points: i32,
    pub kill_counts: s_metagame_state_type_kill_counts,
    pub grenade_stick_kills: i32,
    pub headshot_kills: i32,
    pub melee_kills: i32,
    pub assassination_kills: i32,
    pub splatter_kills: i32,
    pub multi_kills: i32,
    pub needler_supercombine_kills: i32,
    pub emp_kills: i32,
    pub ai_betrayal_count: i32,
    pub multikill_display_timer: i32,
    pub negative_toast_amount: i32,
    pub negative_toast_timer: i32,
    pub transient_scripted_points_amount: i32,
    pub transient_scripted_points_timer: i32,
    pub infantry_kills: i16,
    pub leader_kills: i16,
    pub hero_kills: i16,
    pub specialist_kills: i16,
    pub light_vehicle_kills: i16,
    pub heavy_vehicle_kills: i16,
    pub giant_vehicle_kills: i16,
    pub standard_vehicle_kills: i16,
    pub kill_total_count: i32,
    pub style_total_count: i32,
    #[brw(pad_after = 3)]
    pub finalized: Bool,
    pub player_final_score: i32,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(big)]
pub struct s_metagame_state_secondary {
    pub player_identifier: Unsigned64,
    pub death_count: i32,
    pub betrayal_count: i32,
    pub elapsed_tick_count: i32,
    pub secondary_subtotal: i32,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(0x14)]
#[brw(big)]
pub struct s_campaign_metagame_damage_event {
    pub metagame_bucket: s_campaign_metagame_bucket,
    pub game_time: i32,
    pub damage_owner_player_index: u32,
    pub damage_receiver_object_index: u32,
}


#[derive(BinRead, BinWrite, Serialize, Deserialize, Default, PartialEq, Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
#[brw(big, repr = i8)]
pub enum e_campaign_metagame_bucket_type {
    #[default]
    brute = 0,
    grunt,
    jackal,
    marine,
    bugger,
    hunter,
    flood_infection,
    flood_carrier,
    flood_combat,
    flood_pure,
    sentinel,
    elite,
    turret,
    mongoose,
    warthog,
    scorpion,
    hornet,
    pelican,
    shade,
    watchtower,
    ghost,
    chopper,
    prowler,
    wraith,
    banshee,
    phantom,
    scarab,
    guntower,
}

#[derive(BinRead, BinWrite, Serialize, Deserialize, Default, PartialEq, Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
#[brw(big, repr = i8)]
pub enum e_campaign_metagame_bucket_class {
    #[default]
    infantry = 0,
    leader,
    hero,
    specialist,
    light_vehicle,
    heavy_vehicle,
    giant_vehicle,
    standard_vehicle
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(0x8)]
#[brw(big)]
pub struct s_campaign_metagame_bucket {
    pub flags: u8, // bit 0 is "only counts with riders"
    pub campaign_metagame_bucket_type: e_campaign_metagame_bucket_type,
    pub campaign_metagame_bucket_class: e_campaign_metagame_bucket_class,
    #[brw(pad_before = 1, pad_after = 2)]
    pub point_count: i16,
}
