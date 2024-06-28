use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
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
#[no_mangle]
pub unsafe extern "C" fn relpath(
    mut can_fname: *const libc::c_char,
    mut can_reldir: *const libc::c_char,
    mut buf: *mut libc::c_char,
    mut len: size_t,
) -> bool {
    let mut buf_err: bool = 0 as libc::c_int != 0;
    let mut common_index: libc::c_int = path_common_prefix(can_reldir, can_fname);
    if common_index == 0 {
        return 0 as libc::c_int != 0;
    }
    let mut relto_suffix: *const libc::c_char = can_reldir.offset(common_index as isize);
    let mut fname_suffix: *const libc::c_char = can_fname.offset(common_index as isize);
    if *relto_suffix as libc::c_int == '/' as i32 {
        relto_suffix = relto_suffix.offset(1);
        relto_suffix;
    }
    if *fname_suffix as libc::c_int == '/' as i32 {
        fname_suffix = fname_suffix.offset(1);
        fname_suffix;
    }
    if *relto_suffix != 0 {
        buf_err = (buf_err as libc::c_int
            | buffer_or_output(
                b"..\0" as *const u8 as *const libc::c_char,
                &mut buf,
                &mut len,
            ) as libc::c_int) as bool;
        while *relto_suffix != 0 {
            if *relto_suffix as libc::c_int == '/' as i32 {
                buf_err = (buf_err as libc::c_int
                    | buffer_or_output(
                        b"/..\0" as *const u8 as *const libc::c_char,
                        &mut buf,
                        &mut len,
                    ) as libc::c_int) as bool;
            }
            relto_suffix = relto_suffix.offset(1);
            relto_suffix;
        }
        if *fname_suffix != 0 {
            buf_err = (buf_err as libc::c_int
                | buffer_or_output(
                    b"/\0" as *const u8 as *const libc::c_char,
                    &mut buf,
                    &mut len,
                ) as libc::c_int) as bool;
            buf_err = (buf_err as libc::c_int
                | buffer_or_output(fname_suffix, &mut buf, &mut len) as libc::c_int)
                as bool;
        }
    } else {
        buf_err = (buf_err as libc::c_int
            | buffer_or_output(
                if *fname_suffix as libc::c_int != 0 {
                    fname_suffix
                } else {
                    b".\0" as *const u8 as *const libc::c_char
                },
                &mut buf,
                &mut len,
            ) as libc::c_int) as bool;
    }
    if buf_err {
        error(
            0 as libc::c_int,
            36 as libc::c_int,
            b"%s\0" as *const u8 as *const libc::c_char,
            gettext(b"generating relative path\0" as *const u8 as *const libc::c_char),
        );
    }
    return !buf_err;
}
unsafe extern "C" fn path_common_prefix(
    mut path1: *const libc::c_char,
    mut path2: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_int = 0 as libc::c_int;
    if (*path1.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32)
        as libc::c_int
        != (*path2.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32)
            as libc::c_int
    {
        return 0 as libc::c_int;
    }
    while *path1 as libc::c_int != 0 && *path2 as libc::c_int != 0 {
        if *path1 as libc::c_int != *path2 as libc::c_int {
            break;
        }
        if *path1 as libc::c_int == '/' as i32 {
            ret = i + 1 as libc::c_int;
        }
        path1 = path1.offset(1);
        path1;
        path2 = path2.offset(1);
        path2;
        i += 1;
        i;
    }
    if *path1 == 0 && *path2 == 0 || *path1 == 0 && *path2 as libc::c_int == '/' as i32
        || *path2 == 0 && *path1 as libc::c_int == '/' as i32
    {
        ret = i;
    }
    return ret;
}
unsafe extern "C" fn buffer_or_output(
    mut str: *const libc::c_char,
    mut pbuf: *mut *mut libc::c_char,
    mut plen: *mut size_t,
) -> bool {
    if !(*pbuf).is_null() {
        let mut slen: size_t = strlen(str);
        if slen >= *plen {
            return 1 as libc::c_int != 0;
        }
        memcpy(
            *pbuf as *mut libc::c_void,
            str as *const libc::c_void,
            slen.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        *pbuf = (*pbuf).offset(slen as isize);
        *plen = (*plen as libc::c_ulong).wrapping_sub(slen) as size_t as size_t;
    } else {
        fputs_unlocked(str, stdout);
    }
    return 0 as libc::c_int != 0;
}
