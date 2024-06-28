use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn freopen(
        __filename: *const libc::c_char,
        __modes: *const libc::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
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
pub unsafe extern "C" fn freopen_safer(
    mut name: *const libc::c_char,
    mut mode: *const libc::c_char,
    mut f: *mut FILE,
) -> *mut FILE {
    let mut protect_in: bool = 0 as libc::c_int != 0;
    let mut protect_out: bool = 0 as libc::c_int != 0;
    let mut protect_err: bool = 0 as libc::c_int != 0;
    let mut saved_errno: libc::c_int = 0;
    let mut current_block_8: u64;
    match fileno(f) {
        2 => {
            current_block_8 = 10236636942510093443;
        }
        1 => {
            current_block_8 = 10062963387835506516;
        }
        0 => {
            current_block_8 = 2868539653012386629;
        }
        _ => {
            if dup2(2 as libc::c_int, 2 as libc::c_int) != 2 as libc::c_int {
                protect_err = 1 as libc::c_int != 0;
            }
            current_block_8 = 10236636942510093443;
        }
    }
    match current_block_8 {
        10236636942510093443 => {
            if dup2(1 as libc::c_int, 1 as libc::c_int) != 1 as libc::c_int {
                protect_out = 1 as libc::c_int != 0;
            }
            current_block_8 = 10062963387835506516;
        }
        _ => {}
    }
    match current_block_8 {
        10062963387835506516 => {
            if dup2(0 as libc::c_int, 0 as libc::c_int) != 0 as libc::c_int {
                protect_in = 1 as libc::c_int != 0;
            }
        }
        _ => {}
    }
    if protect_in as libc::c_int != 0 && !protect_fd(0 as libc::c_int) {
        f = 0 as *mut FILE;
    } else if protect_out as libc::c_int != 0 && !protect_fd(1 as libc::c_int) {
        f = 0 as *mut FILE;
    } else if protect_err as libc::c_int != 0 && !protect_fd(2 as libc::c_int) {
        f = 0 as *mut FILE;
    } else {
        f = freopen(name, mode, f);
    }
    saved_errno = *__errno_location();
    if protect_err {
        close(2 as libc::c_int);
    }
    if protect_out {
        close(1 as libc::c_int);
    }
    if protect_in {
        close(0 as libc::c_int);
    }
    if f.is_null() {
        *__errno_location() = saved_errno;
    }
    return f;
}
unsafe extern "C" fn protect_fd(mut fd: libc::c_int) -> bool {
    let mut value: libc::c_int = open(
        b"/dev/null\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    if value != fd {
        if 0 as libc::c_int <= value {
            close(value);
            *__errno_location() = 9 as libc::c_int;
        }
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
