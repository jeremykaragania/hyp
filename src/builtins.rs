use core::ffi::c_void;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void {
    let mut tmp = dest as *mut u8;
    let mut s = src as *const u8;
    let mut count = n;

    while count > 0 {
        unsafe {
            *tmp = *s;

            tmp = tmp.add(1);
            s = s.add(1);
        }

        count -= 1;
    }

    dest
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memmove(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void {
    let mut tmp = dest as *mut u8;
    let mut s = src as *const u8;
    let mut count = n;

    if dest as *const c_void <= src {
        while count > 0 {
            unsafe {
                *tmp = *s;

                tmp = tmp.add(1);
                s = s.add(1);
            }

            count -= 1;
        }
    } else {
        unsafe {
            tmp = tmp.add(count);
            s = s.add(count);
        }

        while count > 0 {
            unsafe {
                *tmp = *s;

                tmp = tmp.sub(1);
                s = s.sub(1);
            }

            count -= 1;
        }
    }

    dest
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(s: *mut c_void, c: i32, n: usize) -> *mut c_void {
    let mut xs = s as *mut u8;
    let mut count = n;

    while count > 0 {
        unsafe {
            *xs = c as u8;
            xs = xs.add(1);
        }
        count -= 1;
    }

    s
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> i32 {
    let mut res = 0;
    let su1 = s1 as *const u8;
    let su2 = s2 as *const u8;
    let mut count = n;

    while count > 0 {
        unsafe {
            res = *su1 - *su2;
        }

        if res != 0 {
            break;
        }

        count -= 1;
    }

    res.into()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bcmp(s1: *const c_void, s2: *const c_void, n: usize) -> i32 {
    unsafe { memcmp(s1, s2, n) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strlen(s: *const u8) -> usize {
    let mut sc = s;

    unsafe {
        while *sc != 0 {
            sc = sc.add(1);
        }
    }

    unsafe { sc.offset_from_unsigned(s) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_eh_personality() {}
