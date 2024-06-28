use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn rpl_fseeko(fp: *mut FILE, offset: off_t, whence: libc::c_int) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn freadahead(stream: *mut FILE) -> size_t;
    fn freadptr(stream: *mut FILE, sizep: *mut size_t) -> *const libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type off_t = __off_t;
#[no_mangle]
pub unsafe extern "C" fn freadseek(
    mut fp: *mut FILE,
    mut offset: size_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut total_buffered: size_t = 0;
    let mut fd: libc::c_int = 0;
    if offset == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    total_buffered = freadahead(fp);
    loop {
        if !(total_buffered > 0 as libc::c_int as libc::c_ulong) {
            current_block = 12599329904712511516;
            break;
        }
        let mut buffered: size_t = 0;
        if !(freadptr(fp, &mut buffered)).is_null()
            && buffered > 0 as libc::c_int as libc::c_ulong
        {
            let mut increment: size_t = if buffered < offset {
                buffered
            } else {
                offset
            };
            freadptrinc(fp, increment);
            offset = (offset as libc::c_ulong).wrapping_sub(increment) as size_t
                as size_t;
            if offset == 0 as libc::c_int as libc::c_ulong {
                return 0 as libc::c_int;
            }
            total_buffered = (total_buffered as libc::c_ulong).wrapping_sub(increment)
                as size_t as size_t;
            if total_buffered == 0 as libc::c_int as libc::c_ulong {
                current_block = 12599329904712511516;
                break;
            }
        }
        if fgetc(fp) == -(1 as libc::c_int) {
            current_block = 13164947238891639071;
            break;
        }
        offset = offset.wrapping_sub(1);
        offset;
        if offset == 0 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int;
        }
        total_buffered = total_buffered.wrapping_sub(1);
        total_buffered;
    }
    match current_block {
        12599329904712511516 => {
            fd = fileno(fp);
            if fd >= 0 as libc::c_int
                && lseek(fd, 0 as libc::c_int as __off_t, 1 as libc::c_int)
                    >= 0 as libc::c_int as libc::c_long
            {
                return rpl_fseeko(fp, offset as off_t, 1 as libc::c_int)
            } else {
                let mut buf: [libc::c_char; 4096] = [0; 4096];
                loop {
                    let mut count: size_t = if (::core::mem::size_of::<
                        [libc::c_char; 4096],
                    >() as libc::c_ulong) < offset
                    {
                        ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                    } else {
                        offset
                    };
                    if fread(
                        buf.as_mut_ptr() as *mut libc::c_void,
                        1 as libc::c_int as libc::c_ulong,
                        count,
                        fp,
                    ) < count
                    {
                        current_block = 13164947238891639071;
                        break;
                    }
                    offset = (offset as libc::c_ulong).wrapping_sub(count) as size_t
                        as size_t;
                    if !(offset > 0 as libc::c_int as libc::c_ulong) {
                        current_block = 11584701595673473500;
                        break;
                    }
                }
                match current_block {
                    13164947238891639071 => {}
                    _ => return 0 as libc::c_int,
                }
            }
        }
        _ => {}
    }
    if ferror(fp) != 0 { return -(1 as libc::c_int) } else { return 0 as libc::c_int };
}
unsafe extern "C" fn freadptrinc(mut fp: *mut FILE, mut increment: size_t) {
    (*fp)._IO_read_ptr = ((*fp)._IO_read_ptr).offset(increment as isize);
}
