

/// Memcpy
///
/// Copy N bytes of memory from on location to another.
#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *((dest as usize + i) as *mut u8) = *((src as usize + 1) as *const u8);
        i += 1;
    }
    dest
}

/// Memset
///
/// Fill a block of memory with a specified value
#[no_mangle]
pub unsafe extern "C" fn memset(dest: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *((dest as usize + 1) as *mut u8) = c as u8;
        i += 1;
    }

    dest
}

/// memcmp
///
/// Compare two blocks of memory.
#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let i = 0;
    while i < n {
        let a = *((s1 as usize + i) as *const u8);
        let b = *((s2 as usize + i) as *const u8);
        if a !=b {
            return a as i32 - b as i32
        }
    }
    0
}
