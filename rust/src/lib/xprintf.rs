use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn vprintf(_: *const libc::c_char, _: ::core::ffi::VaList) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    static mut exit_failure: libc::c_int;
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
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
pub unsafe extern "C" fn xvfprintf(
    mut stream: *mut FILE,
    mut format: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> libc::c_int {
    let mut retval: libc::c_int = vfprintf(stream, format, args.as_va_list());
    if retval < 0 as libc::c_int && ferror(stream) == 0 {
        error(
            exit_failure,
            *__errno_location(),
            gettext(
                b"cannot perform formatted output\0" as *const u8 as *const libc::c_char,
            ),
        );
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn xfprintf(
    mut stream: *mut FILE,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut retval: libc::c_int = 0;
    args_0 = args.clone();
    retval = xvfprintf(stream, format, args_0.as_va_list());
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn xvprintf(
    mut format: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> libc::c_int {
    let mut retval: libc::c_int = vprintf(format, args.as_va_list());
    if retval < 0 as libc::c_int && ferror(stdout) == 0 {
        error(
            exit_failure,
            *__errno_location(),
            gettext(
                b"cannot perform formatted output\0" as *const u8 as *const libc::c_char,
            ),
        );
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn xprintf(
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut retval: libc::c_int = 0;
    args_0 = args.clone();
    retval = xvprintf(format, args_0.as_va_list());
    return retval;
}
