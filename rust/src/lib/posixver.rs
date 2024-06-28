use ::libc;
extern "C" {
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn posix2_version() -> libc::c_int {
    let mut v: libc::c_long = 200809 as libc::c_long;
    let mut s: *const libc::c_char = getenv(
        b"_POSIX2_VERSION\0" as *const u8 as *const libc::c_char,
    );
    if !s.is_null() && *s as libc::c_int != 0 {
        let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut i: libc::c_long = strtol(s, &mut e, 10 as libc::c_int);
        if *e == 0 {
            v = i;
        }
    }
    return (if v < (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_long {
        (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_long
    } else if v < 2147483647 as libc::c_int as libc::c_long {
        v
    } else {
        2147483647 as libc::c_int as libc::c_long
    }) as libc::c_int;
}
