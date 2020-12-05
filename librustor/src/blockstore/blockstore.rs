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
            debug!("entry: {:?}", &entry);
            let span = entry.span;
            let mut start = 0;
            for n in 0..span {
                let lba:u64 = entry.lba + n;
                let end = if data.len() - n as usize * BS4K < BS4K { data.len() - n as usize * BS4K } else { BS4K };
                let slice = &data[ start .. end ];
                self.device.write_block(lba as u64, slice)?;
                start += BS4K;
            }
        }
        
        Ok(())
    }

    #[allow(unused_variables)]
    fn read(&mut self, data: &[u8], key: &ObjKey) -> RResult<()> {

        Ok(())
    }
}
