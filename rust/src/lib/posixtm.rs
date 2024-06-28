use ::libc;
extern "C" {
    fn time(__timer: *mut time_t) -> time_t;
    fn mktime(__tp: *mut tm) -> time_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type ptrdiff_t = libc::c_long;
pub type idx_t = ptrdiff_t;
#[no_mangle]
pub unsafe extern "C" fn posixtime(
    mut p: *mut time_t,
    mut s: *const libc::c_char,
    mut syntax_bits: libc::c_uint,
) -> bool {
    let mut tm0: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut leapsec: bool = 0 as libc::c_int != 0;
    if !posix_time_parse(&mut tm0, s, syntax_bits) {
        return 0 as libc::c_int != 0;
    }
    loop {
        let mut tm1: tm = tm {
            tm_sec: 0,
            tm_min: 0,
            tm_hour: 0,
            tm_mday: 0,
            tm_mon: 0,
            tm_year: 0,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
            tm_gmtoff: 0,
            tm_zone: 0 as *const libc::c_char,
        };
        tm1.tm_sec = tm0.tm_sec;
        tm1.tm_min = tm0.tm_min;
        tm1.tm_hour = tm0.tm_hour;
        tm1.tm_mday = tm0.tm_mday;
        tm1.tm_mon = tm0.tm_mon;
        tm1.tm_year = tm0.tm_year;
        tm1.tm_wday = -(1 as libc::c_int);
        tm1.tm_isdst = -(1 as libc::c_int);
        let mut t: time_t = mktime(&mut tm1);
        if tm1.tm_wday < 0 as libc::c_int {
            return 0 as libc::c_int != 0;
        }
        if tm0.tm_year ^ tm1.tm_year | tm0.tm_mon ^ tm1.tm_mon
            | tm0.tm_mday ^ tm1.tm_mday | tm0.tm_hour ^ tm1.tm_hour
            | tm0.tm_min ^ tm1.tm_min | tm0.tm_sec ^ tm1.tm_sec == 0
        {
            let (fresh0, fresh1) = t.overflowing_add(leapsec);
            *(&mut t as *mut time_t) = fresh0;
            if fresh1 {
                return 0 as libc::c_int != 0;
            }
            *p = t;
            return 1 as libc::c_int != 0;
        }
        if tm0.tm_sec != 60 as libc::c_int {
            return 0 as libc::c_int != 0;
        }
        tm0.tm_sec = 59 as libc::c_int;
        leapsec = 1 as libc::c_int != 0;
    };
}
#[inline]
unsafe extern "C" fn c_isdigit(mut c: libc::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
unsafe extern "C" fn year(
    mut tm: *mut tm,
    mut digit_pair: *const libc::c_int,
    mut n: idx_t,
    mut syntax_bits: libc::c_uint,
) -> bool {
    match n {
        1 => {
            (*tm).tm_year = *digit_pair;
            if *digit_pair.offset(0 as libc::c_int as isize) <= 68 as libc::c_int {
                if syntax_bits & 8 as libc::c_int as libc::c_uint != 0 {
                    return 0 as libc::c_int != 0;
                }
                (*tm).tm_year += 100 as libc::c_int;
            }
        }
        2 => {
            if syntax_bits & 2 as libc::c_int as libc::c_uint == 0 {
                return 0 as libc::c_int != 0;
            }
            (*tm)
                .tm_year = *digit_pair.offset(0 as libc::c_int as isize)
                * 100 as libc::c_int + *digit_pair.offset(1 as libc::c_int as isize)
                - 1900 as libc::c_int;
        }
        0 => {
            let mut now: time_t = time(0 as *mut time_t);
            let mut tmp: *mut tm = localtime(&mut now);
            if tmp.is_null() {
                return 0 as libc::c_int != 0;
            }
            (*tm).tm_year = (*tmp).tm_year;
        }
        _ => {
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn posix_time_parse(
    mut tm: *mut tm,
    mut s: *const libc::c_char,
    mut syntax_bits: libc::c_uint,
) -> bool {
    let mut dot: *const libc::c_char = 0 as *const libc::c_char;
    let mut pair: [libc::c_int; 6] = [0; 6];
    let mut s_len: idx_t = strlen(s) as idx_t;
    let mut len: idx_t = s_len;
    if syntax_bits & 4 as libc::c_int as libc::c_uint != 0 {
        dot = strchr(s, '.' as i32);
        if !dot.is_null() {
            len = dot.offset_from(s) as libc::c_long;
            if s_len - len != 3 as libc::c_int as libc::c_long {
                return 0 as libc::c_int != 0;
            }
        }
    }
    if !(8 as libc::c_int as libc::c_long <= len
        && len <= 12 as libc::c_int as libc::c_long
        && len % 2 as libc::c_int as libc::c_long == 0 as libc::c_int as libc::c_long)
    {
        return 0 as libc::c_int != 0;
    }
    let mut i: idx_t = 0 as libc::c_int as idx_t;
    while i < len {
        if !c_isdigit(*s.offset(i as isize) as libc::c_int) {
            return 0 as libc::c_int != 0;
        }
        i += 1;
        i;
    }
    len /= 2 as libc::c_int as libc::c_long;
    let mut i_0: idx_t = 0 as libc::c_int as idx_t;
    while i_0 < len {
        pair[i_0
            as usize] = 10 as libc::c_int
            * (*s.offset((2 as libc::c_int as libc::c_long * i_0) as isize)
                as libc::c_int - '0' as i32)
            + *s
                .offset(
                    (2 as libc::c_int as libc::c_long * i_0
                        + 1 as libc::c_int as libc::c_long) as isize,
                ) as libc::c_int - '0' as i32;
        i_0 += 1;
        i_0;
    }
    let mut p: *mut libc::c_int = pair.as_mut_ptr();
    if syntax_bits & 1 as libc::c_int as libc::c_uint == 0 {
        if !year(tm, p, len - 4 as libc::c_int as libc::c_long, syntax_bits) {
            return 0 as libc::c_int != 0;
        }
        p = p.offset((len - 4 as libc::c_int as libc::c_long) as isize);
        len = 4 as libc::c_int as idx_t;
    }
    let fresh2 = p;
    p = p.offset(1);
    (*tm).tm_mon = *fresh2 - 1 as libc::c_int;
    let fresh3 = p;
    p = p.offset(1);
    (*tm).tm_mday = *fresh3;
    let fresh4 = p;
    p = p.offset(1);
    (*tm).tm_hour = *fresh4;
    let fresh5 = p;
    p = p.offset(1);
    (*tm).tm_min = *fresh5;
    len -= 4 as libc::c_int as libc::c_long;
    if syntax_bits & 1 as libc::c_int as libc::c_uint != 0 {
        if !year(tm, p, len, syntax_bits) {
            return 0 as libc::c_int != 0;
        }
    }
    if dot.is_null() {
        (*tm).tm_sec = 0 as libc::c_int;
    } else if c_isdigit(*dot.offset(1 as libc::c_int as isize) as libc::c_int)
        as libc::c_int != 0
        && c_isdigit(*dot.offset(2 as libc::c_int as isize) as libc::c_int)
            as libc::c_int != 0
    {
        (*tm)
            .tm_sec = 10 as libc::c_int
            * (*dot.offset(1 as libc::c_int as isize) as libc::c_int - '0' as i32)
            + *dot.offset(2 as libc::c_int as isize) as libc::c_int - '0' as i32;
    } else {
        return 0 as libc::c_int != 0
    }
    return 1 as libc::c_int != 0;
}
