use ::libc;
extern "C" {
    fn abort() -> !;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    static mut exit_failure: libc::c_int;
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn openat_save_fail(mut errnum: libc::c_int) {
    error(
        exit_failure,
        errnum,
        gettext(
            b"unable to record current working directory\0" as *const u8
                as *const libc::c_char,
        ),
    );
    abort();
}
#[no_mangle]
pub unsafe extern "C" fn openat_restore_fail(mut errnum: libc::c_int) {
    error(
        exit_failure,
        errnum,
        gettext(
            b"failed to return to initial working directory\0" as *const u8
                as *const libc::c_char,
        ),
    );
    abort();
}
