use std::path::PathBuf;

use std::fs::OpenOptions;

enum RAIDLevel {
    RAID0, // block-level striping
    RAID1, // mirroring
    //RAID2, // bit-level striping with dedicated hamming-code parity -- not going to implement
    //RAID3, // byte-level striping with dedicated parity drive. uncommon -- not going to implement
    RAID4, // block-level striping with dedicated parity drive
    RAID5, // block-level striping with distributed parity
    RAID6, // block-level striping with double distributed parity
    DCRAID, // declustered raid n+m
}

trait RAID {

}

struct RAID0 {
    disks: Vec<PathBuf>,
}

impl RAID0 {
    pub fn new() -> RAID0 {
        RAID0 { vec![] }
    }
}

impl BlockStore for RAID0 {
    fn write_block(&self, lba: u64, data[u8]) -> Result<(), String> {
        let index = lba % self.disks.len();
        let disk = self.disks[index];

        let disk_lba = lba / self.disks.len();
        
    }

    fn read_block(&self, lba: u64, data: &[u8]) -> Result<(), String> {

    }
}

impl RAID5 {

}

