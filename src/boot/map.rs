use crate::mmu::{
    create_block_entry, create_table_entry, entry_addr, level_entry_size, level_shift,
};

unsafe extern "C" {
    static text_begin: u8;
    static data_begin: u8;
    static data_end: u8;
}

unsafe fn get_or_create_table(table: *mut u64, desc: *mut u64, level: u8) -> *mut u64 {
    let entry = unsafe { *desc };

    if entry == 0 {
        let next_table = unsafe { table.add(512) };
        unsafe {
            *desc = create_table_entry(next_table as u64);
        }

        return next_table;
    }

    return entry_addr(entry, level) as *mut u64;
}

pub unsafe fn map_block(mut table: *mut u64, begin: u64, end: u64) -> usize {
    let mut entry_begin = 0;

    for level in 0..=3 {
        let shift = level_shift(level);
        let entry_size = level_entry_size(level);

        let index = begin >> shift & 0x1ff;

        let entry = unsafe { table.add(index as usize) };
        entry_begin += index * entry_size as u64;
        let entry_end = entry_begin + entry_size as u64;

        if entry_end <= end && level > 0 {
            unsafe { *entry = create_block_entry(begin, level) };

            return (entry_end - begin) as usize;
        }

        table = unsafe { get_or_create_table(table, entry, level) };
    }

    unreachable!();
}

pub unsafe fn map_region(table: *mut u64, mut begin: u64, end: u64) {
    while begin < end {
        let consumed = unsafe { map_block(table, begin, end) };
        begin += consumed as u64;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn create_init_mapping(table: *mut u64) {
    unsafe {
        let begin = &text_begin as *const u8 as u64;
        let end = &data_begin as *const u8 as u64;
        map_region(table, begin, end);

        let begin = &data_begin as *const u8 as u64;
        let end = &data_end as *const u8 as u64;
        map_region(table, begin, end);
    }
}
