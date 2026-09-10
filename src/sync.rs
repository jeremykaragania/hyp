use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};

unsafe extern "C" {
    fn spinlock_lock(locked: *mut u32);
}

pub struct Spinlock<T> {
    locked: UnsafeCell<u32>,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Sync for Spinlock<T> {}
unsafe impl<T: Send> Send for Spinlock<T> {}

impl<T> Spinlock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            locked: UnsafeCell::new(0),
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) {
        unsafe { spinlock_lock(self.locked.get()) }
    }

    pub unsafe fn unlock(&self) {
        let locked = self.locked.get();
        unsafe {
            *locked = 0;
        }
    }
}

pub struct SpinlockGuard<'a, T> {
    lock: &'a Spinlock<T>,
}

impl<T> Deref for SpinlockGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> DerefMut for SpinlockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T> Drop for SpinlockGuard<'_, T> {
    fn drop(&mut self) {
        unsafe {
            self.lock.unlock();
        }
    }
}
