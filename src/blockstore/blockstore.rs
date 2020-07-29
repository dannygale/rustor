
struct BlockDevice {
    bs: u32,
    capacity: u64,
    max_lba: u64,
}

#[derive(Debug, Default)]
impl BlockDevice {
    fn new(bs: u32, capacity: u64) -> BlockDevice {
        BlockDevice { bs, capacity, max_lba: capacity/bs }
    }
}

trait BlockStore {
    fn write_block(&self, lba: u64, data: [u8]) -> Result<(), String>;
    fn read_block(&self, lba: u64, data: &[u8]) -> Result<(), String>;
}


impl BlockStore for BlockDevice {

}
