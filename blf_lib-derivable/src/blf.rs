use std::error::Error;
use std::io::Read;

pub mod chunks;
pub mod s_blf_header;

pub trait BlfFile {
    fn write(&mut self) -> Result<Vec<u8>, Box<dyn Error>>;
    fn write_file(&mut self, path: impl Into<String>) -> Result<(), Box<dyn Error>>;
    fn read_file(path: &String) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized;
    fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized;

}