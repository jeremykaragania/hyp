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

    pub fn get(&mut self, index: PoolIndex) -> Option<&T> {
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
