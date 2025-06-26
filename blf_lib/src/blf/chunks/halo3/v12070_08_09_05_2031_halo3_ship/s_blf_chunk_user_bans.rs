use std::u32;
use binrw::{binrw, BinRead, BinResult, BinWrite, Endian};
use serde::{Deserialize, Serialize};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
use std::io::{Read, Seek, Write};
use blf_lib::types::time::time64_t;
use blf_lib_derivable::result::BLFLibResult;
use crate::types::u64::Unsigned64;

pub const k_max_bans_count: usize = 32;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("fubh", 2.1)]
#[brw(big)]
pub struct s_blf_chunk_user_bans
{
    #[bw(try_calc(u32::try_from(bans.len())))]
    ban_count: u32,
    pub unknown: u32,
    #[br(count = ban_count)]
    pub bans: Vec<s_blf_chunk_user_bans_ban>
}

#[derive(Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
pub struct s_blf_chunk_user_bans_ban
{
    pub ban_type: BanType,
    pub ban_message_index: u32,
    pub start_time: Option<time64_t>,
    pub end_time: Option<time64_t>,
}

impl BinRead for s_blf_chunk_user_bans_ban {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, args: Self::Args<'_>) -> BinResult<Self> {
        let ban_type = BanType::read_options(reader, endian, ())?;
        let ban_message_index = u32::read_options(reader, endian, ())?;
        let start_time = Option::<u64>::read_options(reader, endian, ())?.map(time64_t);
        let end_time = Option::<u64>::read_options(reader, endian, ())?.map(time64_t);
        Ok(Self {
            ban_type,
            ban_message_index,
            start_time,
            end_time,
        })
    }
}

impl BinWrite for s_blf_chunk_user_bans_ban {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(&self, writer: &mut W, endian: Endian, _args: Self::Args<'_>) -> BinResult<()> {
        self.ban_type.write_options(writer, endian, ())?;
        self.ban_message_index.write_options(writer, endian, ())?;

        if let Some(start_time) = &self.start_time {
            Some(start_time.0).write_options(writer, endian, ())?;
        } else {
            None::<Unsigned64>.write_options(writer, endian, ())?;
        }

        if let Some(end_time) = &self.end_time {
            Some(end_time.0).write_options(writer, endian, ())?;
        } else {
            None::<Unsigned64>.write_options(writer, endian, ())?;
        }

        Ok(())
    }
}


#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(big, repr = u32)]
pub enum BanType {
    #[default]
    Unknown1 = 0,
    Matchmaking = 1,
    Unknown2 = 2,
    Unknown3 = 3,
    Unknown4 = 4,
    Unknown5 = 5,
    Unknown6 = 6,
    Unknown7 = 7,
    XboxLIVE = 8,
    Unknown9 = 9
}

impl BlfChunkHooks for s_blf_chunk_user_bans {
    fn before_write(&mut self, _previously_written: &Vec<u8>) -> BLFLibResult {
        if self.bans.len() > k_max_bans_count {
            return Err(format!("Tried to write a bans file with too many bans! ({}/{})", k_max_bans_count, self.bans.len()).into())
        }

        Ok(())
    }
}
