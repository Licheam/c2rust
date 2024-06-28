use ::libc;
extern "C" {
    fn mkostemp(__template: *mut libc::c_char, __flags: libc::c_int) -> libc::c_int;
    fn mkstemp(__template: *mut libc::c_char) -> libc::c_int;
    fn fd_safer(_: libc::c_int) -> libc::c_int;
    fn fd_safer_flag(_: libc::c_int, _: libc::c_int) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mkstemp_safer(mut templ: *mut libc::c_char) -> libc::c_int {
    return fd_safer(mkstemp(templ));
}
#[no_mangle]
pub unsafe extern "C" fn mkostemp_safer(
    mut templ: *mut libc::c_char,
    mut flags: libc::c_int,
) -> libc::c_int {
    return fd_safer_flag(mkostemp(templ, flags), flags);
}
