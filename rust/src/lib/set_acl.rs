use ::libc;
extern "C" {
    fn qset_acl(_: *const libc::c_char, _: libc::c_int, _: mode_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
}
pub type __mode_t = libc::c_uint;
pub type mode_t = __mode_t;
#[no_mangle]
pub unsafe extern "C" fn set_acl(
    mut name: *const libc::c_char,
    mut desc: libc::c_int,
    mut mode: mode_t,
) -> libc::c_int {
    let mut ret: libc::c_int = qset_acl(name, desc, mode);
    if ret != 0 as libc::c_int {
        error(
            0 as libc::c_int,
            *__errno_location(),
            gettext(b"setting permissions for %s\0" as *const u8 as *const libc::c_char),
            quote(name),
        );
    }
    return ret;
}
