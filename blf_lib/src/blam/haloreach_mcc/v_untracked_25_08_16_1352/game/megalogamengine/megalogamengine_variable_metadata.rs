use serde::{Deserialize, Serialize};
use num_derive::{FromPrimitive, ToPrimitive};
use blf_lib::blam::haloreach_mcc::v_untracked_25_08_16_1352::game::game_engine_team::e_multiplayer_team_designator;
use blf_lib::blam::haloreach_mcc::v_untracked_25_08_16_1352::game::megalogamengine::megalogamengine_custom_variable_reference::c_custom_variable_reference;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;

/// Variable replication mode (`network-state`, 2 bits).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(2)]
pub enum e_megalo_variable_network_state {
    #[default]
    local = 0,
    networked = 1,
    networked_high = 2,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_variable_metadata <
    const numeric_variable_count_bits: usize,
    const timer_variable_count_bits: usize,
    const team_variable_count_bits: usize,
    const player_variable_count_bits: usize,
    const object_variable_count_bits: usize,
> {
    pub m_numeric_variables: Vec<(c_custom_variable_reference, e_megalo_variable_network_state)>,
    pub m_timer_variables: Vec<c_custom_variable_reference>,
    pub m_team_variables: Vec<(e_multiplayer_team_designator, e_megalo_variable_network_state)>,
    pub m_player_variables: Vec<e_megalo_variable_network_state>,
    pub m_object_variables: Vec<e_megalo_variable_network_state>,

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
        for (numeric_variable, network_state) in self.m_numeric_variables.iter() {
            numeric_variable.encode(bitstream)?;
            bitstream.write_enum(*network_state)?;
        }

        bitstream.write_integer(self.m_timer_variables.len() as u8, timer_variable_count_bits)?;
        for timer_variable in self.m_timer_variables.iter() {
            timer_variable.encode(bitstream)?;
        }

        bitstream.write_integer(self.m_team_variables.len() as u8, team_variable_count_bits)?;
        for (team_variable, network_state) in self.m_team_variables.iter() {
            bitstream.write_enum(*team_variable)?;
            bitstream.write_enum(*network_state)?;
        }

        bitstream.write_integer(self.m_player_variables.len() as u8, player_variable_count_bits)?;
        for network_state in self.m_player_variables.iter() {
            bitstream.write_enum(*network_state)?;
        }

        bitstream.write_integer(self.m_object_variables.len() as u8, object_variable_count_bits)?;
        for network_state in self.m_object_variables.iter() {
            bitstream.write_enum(*network_state)?;
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        let numeric_variable_count: u16 = bitstream.read_integer("numeric-variable-count", numeric_variable_count_bits)?;
        for _ in 0..numeric_variable_count {
            let mut numeric_variable = c_custom_variable_reference::default();
            numeric_variable.decode(bitstream)?;
            let network_state = bitstream.read_enum("network-state")?;
            self.m_numeric_variables.push((numeric_variable, network_state));
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
            let network_state = bitstream.read_enum("network-state")?;
            self.m_team_variables.push((team_variable_value, network_state));
        }

        let player_variable_count: u16 = bitstream.read_integer("player-variable-count", player_variable_count_bits)?;
        for _ in 0..player_variable_count {
            let network_state = bitstream.read_enum("network-state")?;
            self.m_player_variables.push(network_state);
        }

        let object_variable_count: u16 = bitstream.read_integer("object-variable-count", object_variable_count_bits)?;
        for _ in 0..object_variable_count {
            let network_state = bitstream.read_enum("network-state")?;
            self.m_object_variables.push(network_state);
        }

        Ok(())
    }
}
