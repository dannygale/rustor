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
    fn read(&mut self, data: &mut Vec<u8>, key: &ObjKey) -> RResult<()>;
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
    fn read(&mut self, data: &mut Vec<u8>, key: &ObjKey) -> RResult<()> {
        debug!("read data: {:?}", &key);
        for entry in key.manifest.shards.iter() {
            let span = entry.span;
            for n in 0..span as usize {
                let lba = entry.lba as usize + n;

                // TODO: optimize this so that read_block copies directly into data
                let mut readblk = [0; BS4K]; //&data[ n*BS4K as usize .. (n+1)*BS4K as usize];
                self.device.read_block(lba as u64, &mut readblk)?;
                data.extend_from_slice(&readblk);
            }
        }
        Ok(())
    }
}
