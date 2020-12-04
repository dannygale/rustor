#![allow(unused_imports)]
use std::path::PathBuf;
use std::io::{Seek, SeekFrom, Read, Write, Error};
use std::fs::{File, OpenOptions};

pub const BS4K:usize = 4096;


use crate::object::ObjKey;
use crate::RResult;
pub trait BlockStore {
    fn write_block(&mut self, lba: u64, data: [u8; BS4K]) -> RResult<()>;
    fn read_block(&mut self, lba: u64, data: &mut[u8; BS4K]) -> RResult<()>;
    
    fn write(&mut self, data: &[u8], key: &ObjKey) -> RResult<()>;
    fn read(&mut self, data: &[u8], key: &ObjKey) -> RResult<()>;
}


