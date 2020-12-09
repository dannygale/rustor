use super::{FreeList, bitmap::*};
use crate::object::{Manifest, ManifestLocation};
use crate::RResult;


pub struct BitmapFreelist {
    bitmap: Bitmap,
    free: usize
}

impl BitmapFreelist {
    pub fn new(size: usize) -> Self {
        let mut b = BitmapFreelist {
            bitmap: Bitmap::new(size),
            free: size
        };
        b.bitmap.set_all();
        b
    }

    pub fn capacity(&self) -> usize {
        self.bitmap.capacity()
    }

    pub fn free(&self) -> usize {
        self.free
    }
}

impl FreeList for BitmapFreelist {
    /// Naively allocates the first free blocks it finds, combining adjacent blocks
    /// to a single ManifestLocation
    fn allocate(&mut self, span:u64) -> RResult<Manifest> {
        let mut m = Manifest::new();

        let mut prev_block = 0;
        let mut base_of_span = 0;
        let mut found_span = 0;
        let mut allocated = 0;

        // iterate over the free blocks and decide which ones to use
        for block in self.bitmap.ones().collect::<Vec<usize>>() {
            if block == prev_block + 1 || block == prev_block {
                // in a contiguous span of free blocks, or this is the first block
                found_span += 1;

                // claim it
                self.bitmap.clear(block);

                // return if we've found enough
                if allocated + found_span >= span {
                    m.shards.push(ManifestLocation { 
                        blkdevid: None, lba: base_of_span, span: found_span
                    });
                    self.free -= found_span as usize;
                    return Ok(m);
                }
            } else {
                // this block is the beginning of a new contiguous block
                
                // allocate the space we found
                m.shards.push(ManifestLocation {
                    blkdevid: None, span: found_span, lba: base_of_span
                });
                self.free -= found_span as usize;
                allocated += found_span;

                if allocated >= span {
                    return Ok(m);
                }

                // new span:
                self.bitmap.clear(block);
                base_of_span = block as u64;
                found_span = 1;

            }
            prev_block = block;
        }
        Err(format!("Could not allocate {} blocks", &span))?
    }
    fn release(&mut self, manifest: &Manifest) -> RResult<()> {
        for loc in manifest.shards.iter() {
            self.free(loc.span, loc.lba)?;
        }
        Ok(())
    }

    fn take(&mut self, span:u64, lba: u64) -> RResult<()> {
        if lba + span > self.capacity() as u64 {
            return Err(format!("Tried to take out of bounds: {} (max {})", lba+span, self.bitmap.capacity()))?;
        }
        for i in lba .. lba + span {
            self.bitmap.clear(i as usize);
        }
        Ok(())
    }
    fn free(&mut self, span:u64, lba: u64) -> RResult<()> {
        if lba + span > self.capacity() as u64 {
            return Err(format!("Tried to release out of bounds: {} at {} (max {})", lba, span, self.bitmap.capacity()))?;
        }
        for i in lba .. lba + span {
            self.bitmap.set(i as usize);
        }
        Ok(())
    }
}


mod tests {
    use super::*;

    #[test]
    fn test_allocate() {
        let size = 1024;
        let mut list = BitmapFreelist::new(size);

        assert_eq!(list.allocate(10).is_ok(), true);
        assert_eq!(list.free(), size - 10);

        let mut zeros = 0;
        for i in list.bitmap.zeros() {
            if list.bitmap.get(i) == false {
                zeros += 1;
            }
            assert!(i < 10);
        }
        assert_eq!(zeros, 10);
    }

    #[test]
    fn test_release() {
        let size = 1024;
        let mut list = BitmapFreelist::new(size);

        let alloc_size = 10;

        for i in 0..alloc_size {
            list.bitmap.clear(i);
            list.free -= 1;
        }

        assert_eq!(alloc_size, list.bitmap.zeros().collect::<Vec<usize>>().len());
        assert_eq!(list.free, size - alloc_size);

        let mut manifest = Manifest::new();
        manifest.shards.push(ManifestLocation { blkdevid: None, lba: 0, span: alloc_size as u64/2 });

        assert_eq!(list.release(&manifest).is_ok(), true);

        let mut checkvec = vec![true; alloc_size/2];
        checkvec.append(&mut vec![false; alloc_size/2]);
        checkvec.append(&mut vec![true; size - alloc_size]);

        let bitmapvec = list.bitmap.into_iter().collect::<Vec<bool>>();

        assert_eq!(checkvec.len(), bitmapvec.len());
        assert_eq!(checkvec, bitmapvec);

    }

    #[test]
    fn test_realloc() {
        let size = 1024;
        let mut list = BitmapFreelist::new(size);

        let alloc_size = 10;

        for i in alloc_size..2*alloc_size {
            list.bitmap.clear(i);
            list.free -= 1;
        }

        let manifest = list.allocate(alloc_size as u64).unwrap();

        assert_eq!(manifest.shards[0], ManifestLocation { blkdevid: None, lba: 0, span: alloc_size as u64});

        assert_eq!(list.free, size - 2 * alloc_size);

    }

}

