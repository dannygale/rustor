use std::path::{Path, PathBuf};

const BS_4K: u32 = 4096

pub trait ErasureCode {
    pub fn encode(&self, raw_data: &[u8]) -> Result<Vec<Vec<u8>>, String>;
    pub fn decode(&self, encoded_data: Vec<Vec<u8>>) -> Result<(Vec<u8>), String>;
}

enum RAIDLevel {
    RAID0, // block-level striping with no parity
    RAID4, // block-level striping with dedicated parity
    RAID5, // block-level striping with distributed parity
    RAID6, // block-level striping with double distributed parity
    DCRAID, // declustered raid
}

#[derive(Debug, Default)]
pub struct RAID {
    n: u32,
    k: u32,
    bs: u32,
    level: RAIDLevel,
    devices: Vec<Path>
}

impl RAID {
    pub fn new(n: u32, k: u32, bs: u32, level: RAIDLevel, devices: Vec<Path> ) -> Self {
        Self { n, k, bs, level, devices }
    }
}

impl Default for RAIDLevel::RAID0 {
    fn default() -> Self {
        RAID::new(2, 0, BS_4K, RAIDLevel::RAID0, vec![Path::from("1.raid"), Path::from("2.raid")])
    }
}

impl ErasureCode for RAID0 {
    fn encode(&self, raw_data: &[u8]) -> Result<Vec<Vec<u8>>, String> {
        
    }

    fn decode(&self, encoded_data: Vec<Vec<u8>>) -> Result<(Vec<u8>), String> {

    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_raid0 {

    }
}
