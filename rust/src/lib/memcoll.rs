use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcoll(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn memcoll(
    mut s1: *mut libc::c_char,
    mut s1len: size_t,
    mut s2: *mut libc::c_char,
    mut s2len: size_t,
) -> libc::c_int {
    let mut diff: libc::c_int = 0;
    if s1len == s2len
        && memcmp(s1 as *const libc::c_void, s2 as *const libc::c_void, s1len)
            == 0 as libc::c_int
    {
        *__errno_location() = 0 as libc::c_int;
        diff = 0 as libc::c_int;
    } else {
        let mut n1: libc::c_char = *s1.offset(s1len as isize);
        let mut n2: libc::c_char = *s2.offset(s2len as isize);
        *s1.offset(s1len as isize) = '\0' as i32 as libc::c_char;
        *s2.offset(s2len as isize) = '\0' as i32 as libc::c_char;
        diff = strcoll_loop(
            s1,
            s1len.wrapping_add(1 as libc::c_int as libc::c_ulong),
            s2,
            s2len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        *s1.offset(s1len as isize) = n1;
        *s2.offset(s2len as isize) = n2;
    }
    return diff;
}
#[no_mangle]
pub unsafe extern "C" fn memcoll0(
    mut s1: *const libc::c_char,
    mut s1size: size_t,
    mut s2: *const libc::c_char,
    mut s2size: size_t,
) -> libc::c_int {
    if s1size == s2size
        && memcmp(s1 as *const libc::c_void, s2 as *const libc::c_void, s1size)
            == 0 as libc::c_int
    {
        *__errno_location() = 0 as libc::c_int;
        return 0 as libc::c_int;
    } else {
        return strcoll_loop(s1, s1size, s2, s2size)
    };
}
unsafe extern "C" fn strcoll_loop(
    mut s1: *const libc::c_char,
    mut s1size: size_t,
    mut s2: *const libc::c_char,
    mut s2size: size_t,
) -> libc::c_int {
    let mut diff: libc::c_int = 0;
    loop {
        *__errno_location() = 0 as libc::c_int;
        diff = strcoll(s1, s2);
        if diff != 0 || *__errno_location() != 0 {
            break;
        }
        let mut size1: size_t = (strlen(s1))
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        let mut size2: size_t = (strlen(s2))
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        s1 = s1.offset(size1 as isize);
        s2 = s2.offset(size2 as isize);
        s1size = (s1size as libc::c_ulong).wrapping_sub(size1) as size_t as size_t;
        s2size = (s2size as libc::c_ulong).wrapping_sub(size2) as size_t as size_t;
        if s1size == 0 as libc::c_int as libc::c_ulong {
            return -((s2size != 0 as libc::c_int as libc::c_ulong) as libc::c_int);
        }
        if s2size == 0 as libc::c_int as libc::c_ulong {
            return 1 as libc::c_int;
        }
    }
    return diff;
}
