use std::path::PathBuf;
use std::io::{Seek, SeekFrom, Read, Write, Error};
use std::fs::File

pub struct BlockDevice {
    bs: u32,
    capacity: u64,
    max_lba: u64,
    path: PathBuf,
    file: File,
}

#[derive(Debug, Default)]
impl BlockDevice {
    fn new(bs: u32, capacity: u64, PathBuf path) -> BlockDevice {
        // TODO: validate block size
        // TODO: validate capacity is a multiple of blocksize
        // TODO: check path exists

        let mut file = OpenOptions::new().write(true).read(true).create(false)
            .open(path.as_path()).unwrap()
        BlockDevice { bs, capacity, max_lba: capacity/bs, path, file }
    }
}

pub trait BlockStore {
    fn write_block(&self, lba: u64, data: [u8]) -> Result<(), String>;
    fn read_block(&self, lba: u64, data: &mut[u8]) -> Result<(), String>;
}


impl BlockStore for BlockDevice {
    fn write_block(&self, lba: u64, data: [u8]) -> Result<(), String> {
        let mut blockfile = OpenOptions::new()
            .write(true)
            .create(false)
            .open(self.path.as_path())
            .unwrap();

        blockfile.seek(lba*self.bs);
        let bytes_written = blockfile.write(&data)?;

        // TODO: check that bytes written is the expected length

        Ok(());
    }
    fn read_block(&self, lba: u64, data: &mut[u8]) -> Result<(), String> {
        let mut blockfile = OpenOptions::new()
            .write(false)
            .read(true)
            .create(false)
            .open(self.path.as_path())
            .unwrap();

        // TODO: check that lba is in range
        blockfile.seek(lba*self.bs);
        let bytes_read = blockfile.read(data)?;

        // TODO: check that bytes read is the expected length

        Ok(());
    }
}
