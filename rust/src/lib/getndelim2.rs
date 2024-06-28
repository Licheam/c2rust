use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn getc_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn freadptr(stream: *mut FILE, sizep: *mut size_t) -> *const libc::c_char;
    fn freadseek(stream: *mut FILE, offset: size_t) -> libc::c_int;
    fn memchr2(
        s: *const libc::c_void,
        c1: libc::c_int,
        c2: libc::c_int,
        n: size_t,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type ssize_t = __ssize_t;
#[no_mangle]
pub unsafe extern "C" fn getndelim2(
    mut lineptr: *mut *mut libc::c_char,
    mut linesize: *mut size_t,
    mut offset: size_t,
    mut nmax: size_t,
    mut delim1: libc::c_int,
    mut delim2: libc::c_int,
    mut stream: *mut FILE,
) -> ssize_t {
    let mut current_block: u64;
    let mut nbytes_avail: size_t = 0;
    let mut read_pos: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bytes_stored: ssize_t = -(1 as libc::c_int) as ssize_t;
    let mut ptr: *mut libc::c_char = *lineptr;
    let mut size: size_t = *linesize;
    let mut found_delimiter: bool = false;
    if ptr.is_null() {
        size = if nmax < 64 as libc::c_int as libc::c_ulong {
            nmax
        } else {
            64 as libc::c_int as libc::c_ulong
        };
        ptr = malloc(size) as *mut libc::c_char;
        if ptr.is_null() {
            return -(1 as libc::c_int) as ssize_t;
        }
    }
    if !(size < offset) {
        nbytes_avail = size.wrapping_sub(offset);
        read_pos = ptr.offset(offset as isize);
        if !(nbytes_avail == 0 as libc::c_int as libc::c_ulong && nmax <= size) {
            if delim1 == -(1 as libc::c_int) {
                delim1 = delim2;
            } else if delim2 == -(1 as libc::c_int) {
                delim2 = delim1;
            }
            found_delimiter = 0 as libc::c_int != 0;
            loop {
                let mut c: libc::c_int = 0;
                let mut buffer: *const libc::c_char = 0 as *const libc::c_char;
                let mut buffer_len: size_t = 0;
                buffer = freadptr(stream, &mut buffer_len);
                if !buffer.is_null() {
                    if delim1 != -(1 as libc::c_int) {
                        let mut end: *const libc::c_char = memchr2(
                            buffer as *const libc::c_void,
                            delim1,
                            delim2,
                            buffer_len,
                        ) as *const libc::c_char;
                        if !end.is_null() {
                            buffer_len = (end.offset_from(buffer) as libc::c_long
                                + 1 as libc::c_int as libc::c_long) as size_t;
                            found_delimiter = 1 as libc::c_int != 0;
                        }
                    }
                } else {
                    c = getc_unlocked(stream);
                    if c == -(1 as libc::c_int) {
                        if read_pos == ptr {
                            current_block = 17902666358770968806;
                            break;
                        } else {
                            current_block = 7420279277351916581;
                            break;
                        }
                    } else {
                        if c == delim1 || c == delim2 {
                            found_delimiter = 1 as libc::c_int != 0;
                        }
                        buffer_len = 1 as libc::c_int as size_t;
                    }
                }
                if nbytes_avail
                    < buffer_len.wrapping_add(1 as libc::c_int as libc::c_ulong)
                    && size < nmax
                {
                    let mut newsize: size_t = if size
                        < 64 as libc::c_int as libc::c_ulong
                    {
                        size.wrapping_add(64 as libc::c_int as libc::c_ulong)
                    } else {
                        (2 as libc::c_int as libc::c_ulong).wrapping_mul(size)
                    };
                    let mut newptr: *mut libc::c_char = 0 as *mut libc::c_char;
                    if newsize
                        .wrapping_sub(
                            read_pos.offset_from(ptr) as libc::c_long as libc::c_ulong,
                        ) < buffer_len.wrapping_add(1 as libc::c_int as libc::c_ulong)
                    {
                        newsize = (read_pos.offset_from(ptr) as libc::c_long
                            as libc::c_ulong)
                            .wrapping_add(buffer_len)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong);
                    }
                    if !(size < newsize && newsize <= nmax) {
                        newsize = nmax;
                    }
                    if ((if (9223372036854775807 as libc::c_long)
                        < 9223372036854775807 as libc::c_long
                    {
                        9223372036854775807 as libc::c_long
                    } else {
                        9223372036854775807 as libc::c_long
                    }) as libc::c_ulong) < newsize.wrapping_sub(offset)
                    {
                        let mut newsizemax: size_t = offset
                            .wrapping_add(
                                (if (9223372036854775807 as libc::c_long)
                                    < 9223372036854775807 as libc::c_long
                                {
                                    9223372036854775807 as libc::c_long
                                } else {
                                    9223372036854775807 as libc::c_long
                                }) as libc::c_ulong,
                            )
                            .wrapping_add(1 as libc::c_int as libc::c_ulong);
                        if size == newsizemax {
                            current_block = 17902666358770968806;
                            break;
                        }
                        newsize = newsizemax;
                    }
                    nbytes_avail = newsize
                        .wrapping_sub(
                            read_pos.offset_from(ptr) as libc::c_long as libc::c_ulong,
                        );
                    newptr = realloc(ptr as *mut libc::c_void, newsize)
                        as *mut libc::c_char;
                    if newptr.is_null() {
                        current_block = 17902666358770968806;
                        break;
                    }
                    ptr = newptr;
                    size = newsize;
                    read_pos = ptr.offset(size.wrapping_sub(nbytes_avail) as isize);
                }
                if (1 as libc::c_int as libc::c_ulong) < nbytes_avail {
                    let mut copy_len: size_t = nbytes_avail
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                    if buffer_len < copy_len {
                        copy_len = buffer_len;
                    }
                    if !buffer.is_null() {
                        memcpy(
                            read_pos as *mut libc::c_void,
                            buffer as *const libc::c_void,
                            copy_len,
                        );
                    } else {
                        *read_pos = c as libc::c_char;
                    }
                    read_pos = read_pos.offset(copy_len as isize);
                    nbytes_avail = (nbytes_avail as libc::c_ulong).wrapping_sub(copy_len)
                        as size_t as size_t;
                }
                if !buffer.is_null() && freadseek(stream, buffer_len) != 0 {
                    current_block = 17902666358770968806;
                    break;
                }
                if found_delimiter {
                    current_block = 7420279277351916581;
                    break;
                }
            }
            match current_block {
                17902666358770968806 => {}
                _ => {
                    *read_pos = '\0' as i32 as libc::c_char;
                    bytes_stored = read_pos.offset_from(ptr.offset(offset as isize))
                        as libc::c_long;
                }
            }
        }
    }
    *lineptr = ptr;
    *linesize = size;
    return if bytes_stored != 0 {
        bytes_stored
    } else {
        -(1 as libc::c_int) as libc::c_long
    };
}
