use serde::{Deserialize, Serialize};
use num_derive::{FromPrimitive, ToPrimitive};
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::game::game_engine_team::e_multiplayer_team_designator;
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::game::megalogamengine::megalogamengine_custom_variable_reference::c_custom_variable_reference;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;

/// Variable replication mode (`network-state`, 2 bits). Halo 4 count=3.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(2)]
pub enum e_megalo_variable_network_state {
    #[default]
    local = 0,
    networked = 1,
    networked_high = 2,
}

/// Halo 4 `VariableFlags`: network-state + is-persistent.
#[derive(Default, PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct s_megalo_variable_flags {
    pub m_network_state: e_megalo_variable_network_state,
    pub m_is_persistent: bool,
}

impl s_megalo_variable_flags {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_enum(self.m_network_state)?;
        bitstream.write_bool(self.m_is_persistent)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_network_state = bitstream.read_enum("network-state")?;
        self.m_is_persistent = bitstream.read_bool("is-persistent")?;
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_variable_metadata <
    const numeric_variable_count_bits: usize,
    const timer_variable_count_bits: usize,
    const team_variable_count_bits: usize,
    const player_variable_count_bits: usize,
    const object_variable_count_bits: usize,
> {
    pub m_numeric_variables: Vec<(c_custom_variable_reference, s_megalo_variable_flags)>,
    pub m_timer_variables: Vec<c_custom_variable_reference>,
    pub m_team_variables: Vec<(e_multiplayer_team_designator, s_megalo_variable_flags)>,
    pub m_player_variables: Vec<s_megalo_variable_flags>,
    pub m_object_variables: Vec<s_megalo_variable_flags>,

}

impl<
    const numeric_variable_count_bits: usize,
    const timer_variable_count_bits: usize,
    const team_variable_count_bits: usize,
    const player_variable_count_bits: usize,
    const object_variable_count_bits: usize,
>
s_variable_metadata<
    numeric_variable_count_bits,
    timer_variable_count_bits,
    team_variable_count_bits,
    player_variable_count_bits,
    object_variable_count_bits
>
{
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_numeric_variables.len() as u8, numeric_variable_count_bits)?;
        for (numeric_variable, flags) in self.m_numeric_variables.iter() {
            numeric_variable.encode(bitstream)?;
            flags.encode(bitstream)?;
        }

        bitstream.write_integer(self.m_timer_variables.len() as u8, timer_variable_count_bits)?;
        for timer_variable in self.m_timer_variables.iter() {
            timer_variable.encode(bitstream)?;
        }

        bitstream.write_integer(self.m_team_variables.len() as u8, team_variable_count_bits)?;
        for (team_variable, flags) in self.m_team_variables.iter() {
            bitstream.write_enum(*team_variable)?;
            flags.encode(bitstream)?;
        }

        bitstream.write_integer(self.m_player_variables.len() as u8, player_variable_count_bits)?;
        for flags in self.m_player_variables.iter() {
            flags.encode(bitstream)?;
        }

        bitstream.write_integer(self.m_object_variables.len() as u8, object_variable_count_bits)?;
        for flags in self.m_object_variables.iter() {
            flags.encode(bitstream)?;
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        let numeric_variable_count: u16 = bitstream.read_integer("numeric-variable-count", numeric_variable_count_bits)?;
        for _ in 0..numeric_variable_count {
            let mut numeric_variable = c_custom_variable_reference::default();
            numeric_variable.decode(bitstream)?;
            let mut flags = s_megalo_variable_flags::default();
            flags.decode(bitstream)?;
            self.m_numeric_variables.push((numeric_variable, flags));
        }

        let timer_variable_count: u16 = bitstream.read_integer("timer-variable-count", timer_variable_count_bits)?;
        for _ in 0..timer_variable_count {
            let mut timer_variable = c_custom_variable_reference::default();
            timer_variable.decode(bitstream)?;
            self.m_timer_variables.push(timer_variable);
        }

        let team_variable_count: u16 = bitstream.read_integer("team-variable-count", team_variable_count_bits)?;
        for _ in 0..team_variable_count {
            let team_variable_value = bitstream.read_enum("team-variable-value")?;
            let mut flags = s_megalo_variable_flags::default();
            flags.decode(bitstream)?;
            self.m_team_variables.push((team_variable_value, flags));
        }

        let player_variable_count: u16 = bitstream.read_integer("player-variable-count", player_variable_count_bits)?;
        for _ in 0..player_variable_count {
            let mut flags = s_megalo_variable_flags::default();
            flags.decode(bitstream)?;
            self.m_player_variables.push(flags);
        }

        let object_variable_count: u16 = bitstream.read_integer("object-variable-count", object_variable_count_bits)?;
        for _ in 0..object_variable_count {
            let mut flags = s_megalo_variable_flags::default();
            flags.decode(bitstream)?;
            self.m_object_variables.push(flags);
        }

        Ok(())
    }
}
