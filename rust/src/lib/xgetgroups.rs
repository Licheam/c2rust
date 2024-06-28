use ::libc;
extern "C" {
    fn mgetgroups(
        username: *const libc::c_char,
        gid: gid_t,
        groups: *mut *mut gid_t,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn xalloc_die();
}
pub type __gid_t = libc::c_uint;
pub type gid_t = __gid_t;
#[no_mangle]
pub unsafe extern "C" fn xgetgroups(
    mut username: *const libc::c_char,
    mut gid: gid_t,
    mut groups: *mut *mut gid_t,
) -> libc::c_int {
    let mut result: libc::c_int = mgetgroups(username, gid, groups);
    if result == -(1 as libc::c_int) && *__errno_location() == 12 as libc::c_int {
        xalloc_die();
    }
    return result;
}
