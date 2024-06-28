use ::libc;
extern "C" {
    fn clock_settime(__clock_id: clockid_t, __tp: *const timespec) -> libc::c_int;
    fn settimeofday(__tv: *const timeval, __tz: *const timezone) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
}
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type clockid_t = __clockid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: libc::c_int,
    pub tz_dsttime: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn settime(mut ts: *const timespec) -> libc::c_int {
    let mut r: libc::c_int = clock_settime(0 as libc::c_int, ts);
    if r == 0 as libc::c_int || *__errno_location() == 1 as libc::c_int {
        return r;
    }
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    tv.tv_sec = (*ts).tv_sec;
    tv.tv_usec = (*ts).tv_nsec / 1000 as libc::c_int as libc::c_long;
    return settimeofday(&mut tv, 0 as *const timezone);
}
