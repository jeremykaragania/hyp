use crate::mm::table::{Descriptor, TABLE_ENTRY_COUNT, Table};
use crate::sync::Spinlock;

const TABLE_POOL_SIZE: usize = 64;

#[unsafe(link_section = ".trans_table_pool")]
#[unsafe(no_mangle)]
pub static mut TABLE_POOL: [Table; TABLE_POOL_SIZE] =
    [const { Table([Descriptor(0); TABLE_ENTRY_COUNT]) }; TABLE_POOL_SIZE];

static TABLE_POOL_MAP: Spinlock<[u64; (TABLE_POOL_SIZE - 1) / 64 + 1]> =
    Spinlock::new(const { [0; (TABLE_POOL_SIZE - 1) / 64 + 1] });

#[unsafe(no_mangle)]
pub extern "C" fn init_table_alloc() {
    unsafe {
        let mut table_pool_map = TABLE_POOL_MAP.lock();

        // Reserve the first translation table in the pool.
        table_pool_map[0] = 1;
    }
}

pub fn alloc_table() -> Result<&'static mut Table, ()> {
    unsafe {
        let mut table_pool_map = TABLE_POOL_MAP.lock();

        for (i, map) in table_pool_map.iter_mut().enumerate() {
            for bit in 0..64 {
                if *map & 1 << bit == 0 {
                    *map |= 1 << bit;

                    return Ok(&mut TABLE_POOL[i * 64 + bit]);
                }
            }
        }
    }

    Err(())
}

pub fn free_table(table: &Table) {
    let mut table_pool_map = TABLE_POOL_MAP.lock();

    unsafe {
        let table_pool_ptr = &TABLE_POOL[0] as *const Table;
        let table_ptr = table as *const Table;

        let i = table_ptr.offset_from(table_pool_ptr);
        let map = (i / 64) as usize;
        let bit = i % 64;

        table_pool_map[map] &= !(1 << bit);
    }
}
