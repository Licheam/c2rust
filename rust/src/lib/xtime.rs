use ::libc;
pub type xtime_t = libc::c_longlong;
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn xtime_make(mut s: xtime_t, mut ns: libc::c_long) -> xtime_t {
    return 1000000000 as libc::c_int as libc::c_longlong * s + ns as libc::c_longlong;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn xtime_nonnegative_sec(mut t: xtime_t) -> xtime_t {
    return t / 1000000000 as libc::c_int as libc::c_longlong;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn xtime_sec(mut t: xtime_t) -> xtime_t {
    return (t
        + (t < 0 as libc::c_int as libc::c_longlong) as libc::c_int as libc::c_longlong)
        / 1000000000 as libc::c_int as libc::c_longlong
        - (t < 0 as libc::c_int as libc::c_longlong) as libc::c_int as libc::c_longlong;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn xtime_nonnegative_nsec(mut t: xtime_t) -> libc::c_long {
    return (t % 1000000000 as libc::c_int as libc::c_longlong) as libc::c_long;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn xtime_nsec(mut t: xtime_t) -> libc::c_long {
    let mut ns: libc::c_long = (t % 1000000000 as libc::c_int as libc::c_longlong)
        as libc::c_long;
    if ns < 0 as libc::c_int as libc::c_long {
        ns += 1000000000 as libc::c_int as libc::c_long;
    }
    return ns;
}
