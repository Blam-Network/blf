use serde::{Deserialize, Serialize};
use blf_lib::blam::haloreach::v09730_10_04_09_1309_omaha_delta::game::megalogamengine::megalogamengine_custom_variable_reference::c_custom_variable_reference;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_variable_metadata <
    const numeric_variable_count_bits: usize,
    const timer_variable_count_bits: usize,
    const team_variable_count_bits: usize,
    const player_variable_count_bits: usize,
    const object_variable_count_bits: usize,
> {
    pub m_numeric_variables: Vec<(c_custom_variable_reference, u8)>,
    pub m_timer_variables: Vec<c_custom_variable_reference>,
    pub m_team_variables: Vec<(u8, u8)>,
    pub m_player_variables: Vec<u8>,
    pub m_object_variables: Vec<u8>,

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
            bitstream.write_integer(*network_state, 2)?
        }

        bitstream.write_integer(self.m_timer_variables.len() as u8, timer_variable_count_bits)?;
        for timer_variable in self.m_timer_variables.iter() {
            timer_variable.encode(bitstream)?;
        }

        bitstream.write_integer(self.m_team_variables.len() as u8, team_variable_count_bits)?;
        for (team_variable, network_state) in self.m_team_variables.iter() {
            bitstream.write_integer(*team_variable, 4)?;
            bitstream.write_integer(*network_state, 2)?
        }

        bitstream.write_integer(self.m_player_variables.len() as u8, player_variable_count_bits)?;
        for network_state in self.m_player_variables.iter() {
            bitstream.write_integer(*network_state, 2)?
        }

        bitstream.write_integer(self.m_object_variables.len() as u8, object_variable_count_bits)?;
        for network_state in self.m_object_variables.iter() {
            bitstream.write_integer(*network_state, 2)?
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        let numeric_variable_count: u16 = bitstream.read_integer("numeric-variable-count", numeric_variable_count_bits)?;
        for _ in 0..numeric_variable_count {
            let mut numeric_variable = c_custom_variable_reference::default();
            numeric_variable.decode(bitstream)?;
            let network_state = bitstream.read_integer("network-state", 2)?;
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
            let team_variable_value = bitstream.read_integer("team-variable-value", 4)?;
            let network_state = bitstream.read_integer("network-state", 2)?;
            self.m_team_variables.push((team_variable_value, network_state));
        }

        let player_variable_count: u16 = bitstream.read_integer("player-variable-count", player_variable_count_bits)?;
        for _ in 0..player_variable_count {
            let network_state = bitstream.read_integer("network-state", 2)?;
            self.m_player_variables.push(network_state);
        }

        let object_variable_count: u16 = bitstream.read_integer("object-variable-count", object_variable_count_bits)?;
        for _ in 0..object_variable_count {
            let network_state = bitstream.read_integer("network-state", 2)?;
            self.m_object_variables.push(network_state);
        }

        Ok(())
    }
}
