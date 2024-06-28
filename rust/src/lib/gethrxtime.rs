use ::libc;
extern "C" {
    fn gettime(_: *mut timespec);
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
}
pub type xtime_t = libc::c_longlong;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __time_t = libc::c_long;
pub type clockid_t = __clockid_t;
pub type __clockid_t = libc::c_int;
#[inline]
unsafe extern "C" fn xtime_make(mut s: xtime_t, mut ns: libc::c_long) -> xtime_t {
    return 1000000000 as libc::c_int as libc::c_longlong * s + ns as libc::c_longlong;
}
#[no_mangle]
pub unsafe extern "C" fn gethrxtime() -> xtime_t {
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    if clock_gettime(1 as libc::c_int, &mut ts) == 0 as libc::c_int {
        return xtime_make(ts.tv_sec as xtime_t, ts.tv_nsec);
    }
    let mut ts_0: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    gettime(&mut ts_0);
    return xtime_make(ts_0.tv_sec as xtime_t, ts_0.tv_nsec);
}
