use std::path::PathBuf;
use std::io::{Seek, SeekFrom, Read, Write, Error};
use std::fs::{File, OpenOptions};

const BS4K:usize = 4096;

#[derive(Debug)]
pub struct BlockDevice {
    //bs: u32,
    capacity: u64,
    max_lba: u64,
    path: PathBuf,
    file: Option<File>,
}

impl Default for BlockDevice {
    fn default() -> Self{
        Self {
            //bs: 4096,
            capacity: 0,
            max_lba: 0,
            path: PathBuf::default(),
            file: None
        }
    }
}

impl BlockDevice {
    fn new(/*bs: u32, */capacity: u64, path: PathBuf) -> BlockDevice {
        // TODO: validate block size
        // TODO: validate capacity is a multiple of blocksize
        // TODO: check path exists

        let mut file = Some(OpenOptions::new().write(true).read(true).create(false)
            .open(path.as_path()).unwrap());
        BlockDevice { /*bs,*/ capacity, max_lba: capacity/BS4K as u64/*(bs as u64)*/, path, file }
    }
}

pub trait BlockStore {
    fn write_block(&mut self, lba: u64, data: [u8; BS4K]) -> Result<(), String>;
    fn read_block(&mut self, lba: u64, data: &mut[u8; BS4K]) -> Result<(), String>;
}


impl BlockStore for BlockDevice {
    fn write_block(&mut self, lba: u64, data: [u8; BS4K]) -> Result<(), String> {
        // TODO: check that lba is within length of device
        
        if let Some(file) = &mut self.file {
            if let Err(error) = file.seek(SeekFrom::Start(lba*BS4K as u64/*self.bs*/)) {
                return Err(error.to_string());
            }
            let bytes_written = file.write(&data);

            // TODO: check that bytes written is the expected length

            if let Err(error) = file.flush() {
                return Err(error.to_string());
            }
            return Ok(());
        } else {
            return Err("Blockdevice::file is uninitialized".to_string());
        }

    }
    fn read_block(&mut self, lba: u64, data: &mut[u8; BS4K]) -> Result<(), String> {
        // TODO: check that lba is in range

        if let Some(file) = &mut self.file {
            if let Err(error) = file.seek(SeekFrom::Start(lba*BS4K as u64/*self.bs*/)) {
                return Err(error.to_string());
            }
            if let Err(error) = file.read(data) {
                return Err(error.to_string());
            }

            // TODO: check that bytes read is the expected length

            return Ok(());
        } else {
            return Err("Blockdevice::file is uninitialized".to_string());
        }

    }
}
