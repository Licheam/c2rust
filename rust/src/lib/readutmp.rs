use ::libc;
extern "C" {
    fn setutxent();
    fn endutxent();
    fn getutxent() -> *mut utmpx;
    fn utmpxname(__file: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xpalloc(
        pa: *mut libc::c_void,
        pn: *mut idx_t,
        n_incr_min: idx_t,
        n_max: ptrdiff_t,
        s: idx_t,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __pid_t = libc::c_int;
pub type ptrdiff_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __exit_status {
    pub e_termination: libc::c_short,
    pub e_exit: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utmpx {
    pub ut_type: libc::c_short,
    pub ut_pid: __pid_t,
    pub ut_line: [libc::c_char; 32],
    pub ut_id: [libc::c_char; 4],
    pub ut_user: [libc::c_char; 32],
    pub ut_host: [libc::c_char; 256],
    pub ut_exit: __exit_status,
    pub ut_session: __int32_t,
    pub ut_tv: C2RustUnnamed,
    pub ut_addr_v6: [__int32_t; 4],
    pub __glibc_reserved: [libc::c_char; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub tv_sec: __int32_t,
    pub tv_usec: __int32_t,
}
pub type STRUCT_UTMP = utmpx;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const READ_UTMP_USER_PROCESS: C2RustUnnamed_0 = 2;
pub const READ_UTMP_CHECK_PIDS: C2RustUnnamed_0 = 1;
pub type idx_t = ptrdiff_t;
#[no_mangle]
pub unsafe extern "C" fn extract_trimmed_name(
    mut ut: *const STRUCT_UTMP,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut trimmed_name: *mut libc::c_char = 0 as *mut libc::c_char;
    trimmed_name = xmalloc(
        (::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    strncpy(
        trimmed_name,
        ((*ut).ut_user).as_ptr(),
        ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
    );
    *trimmed_name
        .offset(
            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as isize,
        ) = '\0' as i32 as libc::c_char;
    p = trimmed_name.offset(strlen(trimmed_name) as isize);
    while trimmed_name < p
        && *p.offset(-(1 as libc::c_int) as isize) as libc::c_int == ' ' as i32
    {
        p = p.offset(-1);
        *p = '\0' as i32 as libc::c_char;
    }
    return trimmed_name;
}
#[no_mangle]
pub unsafe extern "C" fn read_utmp(
    mut file: *const libc::c_char,
    mut n_entries: *mut size_t,
    mut utmp_buf: *mut *mut STRUCT_UTMP,
    mut options: libc::c_int,
) -> libc::c_int {
    let mut n_read: idx_t = 0 as libc::c_int as idx_t;
    let mut n_alloc: idx_t = 0 as libc::c_int as idx_t;
    let mut utmp: *mut STRUCT_UTMP = 0 as *mut STRUCT_UTMP;
    let mut u: *mut STRUCT_UTMP = 0 as *mut STRUCT_UTMP;
    utmpxname(file as *mut libc::c_char);
    setutxent();
    loop {
        u = getutxent();
        if u.is_null() {
            break;
        }
        if desirable_utmp_entry(u, options) {
            if n_read == n_alloc {
                utmp = xpalloc(
                    utmp as *mut libc::c_void,
                    &mut n_alloc,
                    1 as libc::c_int as idx_t,
                    -(1 as libc::c_int) as ptrdiff_t,
                    ::core::mem::size_of::<STRUCT_UTMP>() as libc::c_ulong as idx_t,
                ) as *mut STRUCT_UTMP;
            }
            let fresh0 = n_read;
            n_read = n_read + 1;
            *utmp.offset(fresh0 as isize) = *u;
        }
    }
    endutxent();
    *n_entries = n_read as size_t;
    *utmp_buf = utmp;
    return 0 as libc::c_int;
}
unsafe extern "C" fn desirable_utmp_entry(
    mut u: *const STRUCT_UTMP,
    mut options: libc::c_int,
) -> bool {
    let mut user_proc: bool = (*u).ut_user[0 as libc::c_int as usize] as libc::c_int != 0
        && ((*u).ut_type as libc::c_int == 7 as libc::c_int
            || 0 as libc::c_int != 0 && (*u).ut_tv.tv_sec != 0 as libc::c_int);
    if options & READ_UTMP_USER_PROCESS as libc::c_int != 0 && !user_proc {
        return 0 as libc::c_int != 0;
    }
    if options & READ_UTMP_CHECK_PIDS as libc::c_int != 0
        && user_proc as libc::c_int != 0 && (0 as libc::c_int) < (*u).ut_pid
        && (kill((*u).ut_pid, 0 as libc::c_int) < 0 as libc::c_int
            && *__errno_location() == 3 as libc::c_int)
    {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
