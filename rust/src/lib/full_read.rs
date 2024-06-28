use ::libc;
extern "C" {
    fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> size_t;
    fn __errno_location() -> *mut libc::c_int;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn full_read(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_void,
    mut count: size_t,
) -> size_t {
    let mut total: size_t = 0 as libc::c_int as size_t;
    let mut ptr: *mut libc::c_char = buf as *mut libc::c_char;
    while count > 0 as libc::c_int as libc::c_ulong {
        let mut n_rw: size_t = safe_read(fd, ptr as *mut libc::c_void, count);
        if n_rw == -(1 as libc::c_int) as size_t {
            break;
        }
        if n_rw == 0 as libc::c_int as libc::c_ulong {
            *__errno_location() = 0 as libc::c_int;
            break;
        } else {
            total = (total as libc::c_ulong).wrapping_add(n_rw) as size_t as size_t;
            ptr = ptr.offset(n_rw as isize);
            count = (count as libc::c_ulong).wrapping_sub(n_rw) as size_t as size_t;
        }
    }
    return total;
}
