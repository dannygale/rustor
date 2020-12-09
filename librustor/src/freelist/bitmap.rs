
use std::mem;

const WORDLEN: usize = mem::size_of::<usize>();
const WORDLEN_BITS: usize = 8*WORDLEN;

fn word(bit: usize) -> usize {
    bit / WORDLEN_BITS
}
fn index(bit: usize) -> usize {
    bit & (WORDLEN_BITS - 1)
}

pub struct Bitmap {
    words: Vec<usize>,
    size: usize,
}

pub trait BitOps {
    fn get(&self, i: usize) -> bool;
    fn set(&mut self, i: usize);
    fn clear(&mut self, i: usize);
    fn invert(&mut self);
    fn toggle(&mut self, i: usize);
    fn set_value(&mut self, i: usize, val: bool);
}

impl Bitmap {
    pub fn new(size: usize) -> Self {
        let mut words = size/WORDLEN_BITS;
        if (WORDLEN_BITS - 1) & size != 0 {
            words += 1;
        }
        Self { 
            words: vec![0; words],
            size: size,
        }
    }

    pub fn capacity(&self) -> usize {
        self.size
    }

    pub fn set_all(&mut self) {
        for w in self.words.iter_mut() {
            *w = usize::MAX;
        }
    }

    pub fn zeros(&self) -> BitmapIterVal {
        BitmapIterVal::new(self, false)
    }

    pub fn ones(&self) -> BitmapIterVal {
        BitmapIterVal::new(self, true)
    }

    pub fn zeros_mut(&mut self) -> BitmapIterVal {
        BitmapIterVal::new(self, false)
    }

    pub fn ones_mut(&mut self) -> BitmapIterVal {
        BitmapIterVal::new(self, true)
    }
}

impl BitOps for Bitmap {
    #[inline]
    fn get(&self, i: usize) -> bool { 
        //if i >= self.size { return Err(()); }
        self.words[word(i)] & (1 << index(i)) != 0
    }
    #[inline]
    fn set(&mut self, i: usize) {
        //if i >= self.size { return Err(()); }
        self.words[word(i)] |= 1 << index(i);
    }
    #[inline]
    fn clear(&mut self, i: usize) {
        //if i >= self.size { return Err(()); }
        self.words[word(i)] &= !(1 << index(i))
    }
    #[inline]
    fn invert(&mut self) {
        for w in self.words.iter_mut() {
            *w = !*w;
        }
    }
    #[inline]
    fn toggle(&mut self, i: usize) {
        self.words[word(i)] ^= 1 << i;
    }
    #[inline]
    fn set_value(&mut self, i: usize, val: bool) {
        if val { self.set(i); }
        else { self.clear(i); }
    }
}

impl<'a> IntoIterator for &'a Bitmap {
    type Item = bool;
    type IntoIter = BitmapIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BitmapIter::new(&self)
    }
}

/// BitmapIter iterates over the whole bitmap
pub struct BitmapIter<'a> {
    bitmap: &'a Bitmap,
    index: usize
}

impl<'a> BitmapIter<'a> {
    pub fn new(bitmap: &'a Bitmap) -> Self {
        BitmapIter { bitmap, index: 0 }
    }
}

impl<'a> Iterator for BitmapIter<'a> {
    type Item = bool;
    fn next(&mut self) -> Option::<Self::Item> {
        if self.index >= self.bitmap.size { 
            None 
        } else{
            let idx = self.index;
            self.index += 1;
            Some(self.bitmap.get(idx))
        } 
    }
}

/// iterate over the indices of all the zeros or all the ones
pub struct BitmapIterVal<'a> {
    bitmap: &'a Bitmap,
    index: usize,
    value: bool
}

impl<'a> BitmapIterVal<'a> {
    pub fn new(bitmap: &'a Bitmap, val: bool) -> Self {
        Self { bitmap, index: 0, value: val }
    }
}

impl<'a> Iterator for BitmapIterVal<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.bitmap.size {
            return None;
        }
        let bit = self.bitmap.get(self.index);

        let idx = self.index;
        self.index += 1;

        return if bit == self.value { Some(idx) }
            else { self.next() }
    }
}

impl BitOps for usize {
    #[inline]
    fn get(&self, i: usize) -> bool {
        *self & (1 << i) != 0
    }

    #[inline]
    fn set(&mut self, i: usize) {
        *self |= 1 << i;
    }

    #[inline]
    fn clear(&mut self, i: usize) {
        *self &= !(1 << i);
    }

    #[inline]
    fn invert(&mut self) {
        *self = !*self;
    }

    #[inline]
    fn toggle(&mut self, i: usize) {
        *self ^= 1 << i;
    }

    #[inline]
    fn set_value(&mut self, i: usize, val: bool) {
        if val { self.set(i); }
        else { self.clear(i); }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new () {
        let size = 1024;
        let b = Bitmap::new(size);

        assert_eq!(b.size, size);
        assert_eq!(b.words.capacity(), 16);

        for i in 0..b.words.len(){
            assert_eq!(0 as usize, b.words[i]);
        }
    }


    #[test]
    fn test_set() {
        let size = 1024;
        let mut b = Bitmap::new(size);

        for i in 0..b.size {
            b.set(i);
            assert!(b.words[word(i)] & (1 << index(i)) != 0);
        }
    }

    #[test]
    fn test_clear() {
        let size = 1024;
        let mut b = Bitmap::new(size);

        for w in 0..b.words.len() {
            b.words[w] = usize::MAX;
        }

        for bit in 0..b.size {
            let wordnum = word(bit);
            let orig = b.words[wordnum];
            b.clear(bit);
            assert_eq!(b.words[wordnum], orig & !(1 << index(bit)));
        }
    }

    #[test]
    fn test_iter() {
        let size = 1024;
        let mut b = Bitmap::new(size);
        let words = size / WORDLEN_BITS;

        let mut ones = 0;
        let mut bits = 0;

        for i in 0..words {
            b.set(i*WORDLEN_BITS);
        }

        for bit in b.into_iter() {
            if bit { ones += 1 }
            bits += 1;
        }
        assert_eq!(bits, size);
        assert_eq!(ones, words);

        assert_eq!(ones, words)
    }
}

