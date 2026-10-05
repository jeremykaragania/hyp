use crate::mm::bitmap::Bitmap;
use core::mem::MaybeUninit;

#[derive(Copy, Clone)]
pub struct PoolIndex(usize);

#[repr(C)]
pub struct Pool<T, const N: usize, const W: usize> {
    // NOTE: `storage` must be the first field.
    storage: [MaybeUninit<T>; N],
    bitmap: Bitmap<W>,
}

impl<T, const N: usize, const W: usize> Pool<T, N, W> {
    pub const fn new() -> Self {
        Self {
            storage: [const { MaybeUninit::uninit() }; N],
            bitmap: Bitmap::new(),
        }
    }

    pub fn alloc(&mut self) -> Option<PoolIndex> {
        let index = self.bitmap.find_next_unset(0)?;

        if index > N {
            return None;
        }

        self.bitmap.set(index);

        Some(PoolIndex(index))
    }

    pub fn free(&mut self, index: PoolIndex) {
        self.bitmap.clear(index.0);
    }

    pub fn get(&self, index: PoolIndex) -> Option<&T> {
        if !self.is_index_valid(index) {
            return None;
        }

        Some(unsafe { self.storage[index.0].assume_init_ref() })
    }

    pub fn get_mut(&mut self, index: PoolIndex) -> Option<&mut T> {
        if !self.is_index_valid(index) {
            return None;
        }

        Some(unsafe { self.storage[index.0].assume_init_mut() })
    }

    fn is_index_valid(&self, index: PoolIndex) -> bool {
        index.0 < N && self.bitmap.is_set(index.0)
    }

    pub fn iter(&self) -> PoolIter<'_, T, N, W> {
        PoolIter {
            pool: self,
            index: 0,
        }
    }
}

impl<T: Clone, const N: usize, const W: usize> Clone for Pool<T, N, W> {
    fn clone(&self) -> Self {
        let mut storage = [const { MaybeUninit::uninit() }; N];

        for i in 0..N {
            if self.bitmap.is_set(i) {
                unsafe {
                    storage[i].write(self.storage[i].assume_init_ref().clone());
                }
            }
        }

        Self {
            storage,
            bitmap: self.bitmap.clone(),
        }
    }
}

pub struct PoolIter<'a, T, const N: usize, const W: usize> {
    pool: &'a Pool<T, N, W>,
    index: usize,
}

impl<'a, T, const N: usize, const W: usize> Iterator for PoolIter<'a, T, N, W> {
    type Item = (PoolIndex, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.pool.bitmap.find_next_set(self.index)?;
        self.index = index + 1;

        let pool_index = PoolIndex(index);
        Some((pool_index, self.pool.get(pool_index)?))
    }
}
