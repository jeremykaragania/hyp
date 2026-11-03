use crate::bitmap_words;
use crate::mm::pool::Pool;
use crate::mm::table::Table;
use crate::sync::Spinlock;
use core::ptr::addr_of;

const TABLE_POOL_SIZE: usize = 64;

#[unsafe(no_mangle)]
pub static TABLE_POOL: Spinlock<Pool<Table, TABLE_POOL_SIZE, { bitmap_words!(TABLE_POOL_SIZE) }>> =
    Spinlock::new(Pool::new());

#[unsafe(no_mangle)]
pub extern "C" fn init_table_alloc() {
    let mut pool = TABLE_POOL.lock();

    // Reserve the first translation table in the pool.
    pool.alloc();
}

pub fn base_table_ptr() -> *mut Table {
    addr_of!(TABLE_POOL).cast_mut().cast()
}
