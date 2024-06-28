use ::libc;
extern "C" {
    fn rpl_nanosleep(__rqtp: *const timespec, __rmtp: *mut timespec) -> libc::c_int;
    fn dtotimespec(_: libc::c_double) -> timespec;
    fn __errno_location() -> *mut libc::c_int;
    fn pause() -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __syscall_slong_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
#[no_mangle]
pub unsafe extern "C" fn xnanosleep(mut seconds: libc::c_double) -> libc::c_int {
    if 1.0f64
        + (if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
            -(1 as libc::c_int) as time_t
        } else {
            (((1 as libc::c_int as time_t)
                << (::core::mem::size_of::<time_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long
        }) as libc::c_double <= seconds
    {
        loop {
            pause();
            if !(*__errno_location() == 4 as libc::c_int) {
                break;
            }
        }
    }
    let mut ts_sleep: timespec = dtotimespec(seconds);
    loop {
        *__errno_location() = 0 as libc::c_int;
        if rpl_nanosleep(&mut ts_sleep, &mut ts_sleep) == 0 as libc::c_int {
            break;
        }
        if *__errno_location() != 4 as libc::c_int
            && *__errno_location() != 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
