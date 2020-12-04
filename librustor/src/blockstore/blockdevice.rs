use std::path::PathBuf;
use std::io::{Seek, SeekFrom, Read, Write, Error};
use std::fs::{File, OpenOptions};

use crate::GeneralError;
use crate::RResult;

use crate::object::ObjKey;

use super::blockstore::*;

pub trait BlockDevice {
    fn write_block(&mut self, lba: u64, data: &[u8]) -> RResult<()>;
    fn read_block(&mut self, lba: u64, data: &mut[u8; BS4K]) -> RResult<()>;
}


#[derive(Debug)]
pub struct BasicBlockDevice {
    //bs: u32,
    capacity: u64,
    max_lba: u64,
    path: PathBuf,
    file: Option<File>,
}

impl Default for BasicBlockDevice {
    fn default() -> Self {
        Self {
            //bs: 4096,
            capacity: 0,
            max_lba: 0,
            path: PathBuf::default(),
            file: None
        }
    }
}

impl BasicBlockDevice {
    pub fn new(/*bs: u32, */capacity: u64, path: PathBuf) -> Self {
        // TODO: validate block size
        // TODO: validate capacity is a multiple of blocksize
        // TODO: check path exists

        let mut file = Some(OpenOptions::new().write(true).read(true).create(false)
            .open(path.as_path()).unwrap());
        BasicBlockDevice { /*bs,*/ capacity, max_lba: capacity/BS4K as u64/*(bs as u64)*/, path, file }
    }
}

impl BlockDevice for BasicBlockDevice {
    fn write_block(&mut self, lba: u64, data: &[u8]) -> RResult<()> {
        // TODO: check that lba is within length of device
        
        if let Some(file) = &mut self.file {
            if let Err(error) = file.seek(SeekFrom::Start(lba*BS4K as u64/*self.bs*/)) {
                return GeneralError::from(error);
            }

            #[allow(unused_variables)]
            let bytes_written = file.write(&data);
            // TODO: check that bytes written is the expected length

            if let Err(error) = file.flush() {
                return GeneralError::from(error);
            }
            return Ok(());
        } else {
            return GeneralError::new("Blockdevice::file is uninitialized");
        }

    }
    fn read_block(&mut self, lba: u64, data: &mut[u8; BS4K]) -> RResult<()> {
        // TODO: check that lba is in range

        if let Some(file) = &mut self.file {
            if let Err(error) = file.seek(SeekFrom::Start(lba*BS4K as u64/*self.bs*/)) {
                return GeneralError::from(error);
            }
            if let Err(error) = file.read(data) {
                return GeneralError::from(error);
            }

            // TODO: check that bytes read is the expected length

            return Ok(());
        } else {
            return GeneralError::new("Blockdevice::file is uninitialized");
        }

    }
}

