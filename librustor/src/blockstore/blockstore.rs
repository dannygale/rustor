#![allow(unused_imports)]
use std::path::PathBuf;
use std::io::{Seek, SeekFrom, Read, Write, Error};
use std::fs::{File, OpenOptions};
use log::{trace, debug, info, warn, error};

pub const BS4K:usize = 4096;


use crate::object::ObjKey;
use crate::RResult;
pub trait BlockStore {
    fn write(&mut self, data: &[u8], key: &ObjKey) -> RResult<()>;
    fn read(&mut self, data: &[u8], key: &ObjKey) -> RResult<()>;
}


use super::BasicBlockDevice;
use super::BlockDevice;
pub struct SingleDeviceBlockStore {
    pub device: BasicBlockDevice,
}

impl SingleDeviceBlockStore {
    pub fn new(path: PathBuf, capacity: u64) -> Self {
        Self {
            device : BasicBlockDevice::new(capacity, path)
        }
    }
}

impl BlockStore for SingleDeviceBlockStore {
    #[allow(unused_variables)]
    fn write(&mut self, data: &[u8], key: &ObjKey) -> RResult<()> {
        debug!("write object {:?}, size {:?}", &key.uuid, &key.size);
        for entry in key.manifest.shards.iter() {
            let span = entry.span;
            debug!("span: {}", &span);
            for n in 0..span-1 {
                let lba:usize = (entry.lba + n) as usize;
                let slice = &data[ (lba*BS4K) as usize .. (lba as usize+1)*BS4K - 1 ];
                self.device.write_block(lba as u64, slice)?;
            }
        }
        
        Ok(())
    }

    #[allow(unused_variables)]
    fn read(&mut self, data: &[u8], key: &ObjKey) -> RResult<()> {

        Ok(())
    }
}
