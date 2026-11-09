use std::io::{Read, Seek, Write};
use binrw::{BinRead, BinResult, BinWrite, BinWriterExt, Endian};
use serde::{Deserialize, Serialize};
use crate::blf::chunks::BlfChunkHooks;
use crate::BlfChunk;
use crate::blf::BlfFileBuilder;
use crate::blf::versions::ares::v_untracked_ares::{s_blf_chunk_multiplayer_player_statistics, s_blf_chunk_multiplayer_player_vs_player_statistics, s_blf_chunk_multiplayer_team_statistics};

#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("_par", 1.1)]
#[Size(0x4EF4)]
pub struct s_blf_chunk_multiplayer_statistics {
    pub mps1: s_blf_chunk_multiplayer_player_statistics,
    pub mps2: s_blf_chunk_multiplayer_player_vs_player_statistics,
    pub mps3: s_blf_chunk_multiplayer_team_statistics,
}

impl BlfChunkHooks for s_blf_chunk_multiplayer_statistics {}

impl BinRead for s_blf_chunk_multiplayer_statistics {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, args: Self::Args<'_>) -> BinResult<Self> {
        let mut buffer = Vec::<u8>::new();
        reader.seek_relative(4)?; // Skip chunk count.

        reader.read_to_end(&mut buffer)?;

        let mut blf_file = BlfFileBuilder::new();
        blf_file.add_chunk(s_blf_chunk_multiplayer_player_statistics::default());
        blf_file.add_chunk(s_blf_chunk_multiplayer_player_vs_player_statistics::default());
        blf_file.add_chunk(s_blf_chunk_multiplayer_team_statistics::default());

        let blf_file = blf_file.read(&buffer).map_err(|e|binrw::Error::AssertFail { pos: 4, message: e.to_string() })?;

        Ok(Self {
            mps1: blf_file.get_chunk::<s_blf_chunk_multiplayer_player_statistics>().map_err(|e| binrw::Error::AssertFail { pos: 0, message: e.to_string() })?.clone(),
            mps2: blf_file.get_chunk::<s_blf_chunk_multiplayer_player_vs_player_statistics>().map_err(|e| binrw::Error::AssertFail { pos: 0, message: e.to_string() })?.clone(),
            mps3: blf_file.get_chunk::<s_blf_chunk_multiplayer_team_statistics>().map_err(|e| binrw::Error::AssertFail { pos: 0, message: e.to_string() })?.clone(),
        })
    }
}

impl BinWrite for s_blf_chunk_multiplayer_statistics {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(&self, writer: &mut W, endian: Endian, args: Self::Args<'_>) -> BinResult<()> {
        let mut blf_file = BlfFileBuilder::new();
        blf_file.add_chunk(self.mps1.clone());
        blf_file.add_chunk(self.mps2.clone());
        blf_file.add_chunk(self.mps3.clone());

        writer.write_be(&(blf_file.chunk_count() as u32))?;

        let data = blf_file.write();

        Ok(writer.write_all(&data?)?)
    }
}
