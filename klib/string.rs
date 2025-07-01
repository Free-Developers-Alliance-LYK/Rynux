//! String utilities

/// memchr - Find a character in an area of memory.
///
/// # Arguments
/// * `s` - The memory area (pointer to the start)
/// * `c` - The byte to search for (as i32, like C)
/// * `n` - The size of the area (number of bytes)
///
/// # Returns
/// The address of the first occurrence of `c`, or null if not found.
///
/// # Safety
/// This function is unsafe if `s` is not a valid pointer for `n` bytes.
#[no_mangle]
pub extern "C" fn memchr(s: *const u8, c: i32, n: usize) -> *const u8 {
    // Convert search byte to u8
    let c = c as u8;
    let mut p = s;
    let mut i = 0;

    while i < n {
        unsafe {
            if *p == c {
                return p;
            }
            p = p.add(1);
        }
        i += 1;
    }
    core::ptr::null()
}

/// memcpy - Copy one area of memory to another
///
/// # Arguments
/// * `dest` - Where to copy to (destination pointer)
/// * `src`  - Where to copy from (source pointer)
/// * `count` - The size of the area (number of bytes)
///
/// # Returns
/// Returns the destination pointer (`dest`), like C memcpy.
///
/// # Safety
/// The caller must ensure that `dest` and `src` are valid for `count` bytes,
/// and they must not overlap.
#[no_mangle]
pub extern "C" fn memcpy(dest: *mut u8, src: *const u8, count: usize) -> *mut u8 {
    let mut tmp = dest;
    let mut s = src;
    let mut i = 0;

    while i < count {
        unsafe {
            *tmp = *s;
            tmp = tmp.add(1);
            s = s.add(1);
        }
        i += 1;
    }
    dest
}

/// memset - Fill a region of memory with the given value
///
/// # Arguments
/// * `s` - Pointer to the start of the area
/// * `c` - The byte to fill the area with (as i32, like C)
/// * `count` - The size of the area (number of bytes)
///
/// # Returns
/// Returns the pointer to the start of the area, just like C memset.
///
/// # Safety
/// The caller must ensure `s` is valid for `count` bytes.
#[no_mangle]
pub extern "C" fn memset(s: *mut u8, c: i32, count: usize) -> *mut u8 {
    let mut xs = s;
    let c = c as u8;
    let mut i = 0;

    while i < count {
        unsafe {
            *xs = c;
            xs = xs.add(1);
        }
        i += 1;
    }
    s
}

/// memmove - Copy one area of memory to another (safe for overlapping)
///
/// # Arguments
/// * `dest`  - Where to copy to
/// * `src`   - Where to copy from
/// * `count` - The size of the area (number of bytes)
///
/// # Returns
/// Returns the destination pointer (`dest`), just like C memmove.
///
/// # Safety
/// The caller must ensure `dest` and `src` are valid for `count` bytes.
#[no_mangle]
pub extern "C" fn memmove(dest: *mut u8, src: *const u8, count: usize) -> *mut u8 {
    if dest as usize <= src as usize || dest as usize >= (src as usize + count) {
        // No overlap or dest below src, can copy forwards
        let mut tmp = dest;
        let mut s = src;
        let mut i = 0;
        while i < count {
            unsafe {
                *tmp = *s;
                tmp = tmp.add(1);
                s = s.add(1);
            }
            i += 1;
        }
    } else {
        // Overlap, need to copy backwards
        let mut tmp = unsafe { dest.add(count) };
        let mut s = unsafe { src.add(count) };
        let mut i = count;
        while i > 0 {
            unsafe {
                tmp = tmp.sub(1);
                s = s.sub(1);
                *tmp = *s;
            }
            i -= 1;
        }
    }
    dest
}


/// memcmp - Compare two areas of memory.
///
/// # Arguments
/// * `cs`    - Pointer to first memory area
/// * `ct`    - Pointer to second memory area
/// * `count` - Number of bytes to compare
///
/// # Returns
/// Returns 0 if equal, or the difference between the first differing bytes
///
/// # Safety
/// Pointers must be valid for `count` bytes.
#[no_mangle]
pub extern "C" fn memcmp(cs: *const u8, ct: *const u8, count: usize) -> i32 {
    #[cfg(CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS)]
    unsafe {
        use core::mem::size_of;
        let word_size = size_of::<usize>();
        let mut count = count;
        if count >= word_size {
            let mut u1 = cs as *const usize;
            let mut u2 = ct as *const usize;
            while count >= word_size {
                let v1 = core::ptr::read_unaligned(u1);
                let v2 = core::ptr::read_unaligned(u2);
                if v1 != v2 {
                    // 找到不同，逐字节比较本word
                    let b1 = u1 as *const u8;
                    let b2 = u2 as *const u8;
                    for i in 0..word_size {
                        let byte1 = *b1.add(i);
                        let byte2 = *b2.add(i);
                        if byte1 != byte2 {
                            return byte1 as i32 - byte2 as i32;
                        }
                    }
                }
                u1 = u1.add(1);
                u2 = u2.add(1);
                count -= word_size;
            }

            let cs = u1 as *const u8;
            let ct = u2 as *const u8;
            for i in 0..count {
                let byte1 = *cs.add(i);
                let byte2 = *ct.add(i);
                if byte1 != byte2 {
                    return byte1 as i32 - byte2 as i32;
                }
            }
            return 0;
        }
    }

    let mut i = 0;
    while i < count {
        let byte1 = unsafe { *cs.add(i) };
        let byte2 = unsafe { *ct.add(i) };
        if byte1 != byte2 {
            return byte1 as i32 - byte2 as i32;
        }
        i += 1;
    }
    0
}
