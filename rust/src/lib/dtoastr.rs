use ::libc;
extern "C" {
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub const FTOASTR_UPPER_E: C2RustUnnamed = 16;
pub const FTOASTR_ZERO_PAD: C2RustUnnamed = 8;
pub const FTOASTR_SPACE_POSITIVE: C2RustUnnamed = 4;
pub const FTOASTR_ALWAYS_SIGNED: C2RustUnnamed = 2;
pub const FTOASTR_LEFT_JUSTIFY: C2RustUnnamed = 1;
pub type C2RustUnnamed = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn dtoastr(
    mut buf: *mut libc::c_char,
    mut bufsize: size_t,
    mut flags: libc::c_int,
    mut width: libc::c_int,
    mut x: libc::c_double,
) -> libc::c_int {
    let mut promoted_x: libc::c_double = x;
    let mut format: [libc::c_char; 11] = [0; 11];
    let mut abs_x: libc::c_double = if x < 0 as libc::c_int as libc::c_double {
        -x
    } else {
        x
    };
    let mut prec: libc::c_int = 0;
    let mut p: *mut libc::c_char = format.as_mut_ptr();
    let fresh0 = p;
    p = p.offset(1);
    *fresh0 = '%' as i32 as libc::c_char;
    *p = '-' as i32 as libc::c_char;
    p = p
        .offset(
            (flags & FTOASTR_LEFT_JUSTIFY as libc::c_int != 0 as libc::c_int)
                as libc::c_int as isize,
        );
    *p = '+' as i32 as libc::c_char;
    p = p
        .offset(
            (flags & FTOASTR_ALWAYS_SIGNED as libc::c_int != 0 as libc::c_int)
                as libc::c_int as isize,
        );
    *p = ' ' as i32 as libc::c_char;
    p = p
        .offset(
            (flags & FTOASTR_SPACE_POSITIVE as libc::c_int != 0 as libc::c_int)
                as libc::c_int as isize,
        );
    *p = '0' as i32 as libc::c_char;
    p = p
        .offset(
            (flags & FTOASTR_ZERO_PAD as libc::c_int != 0 as libc::c_int) as libc::c_int
                as isize,
        );
    let fresh1 = p;
    p = p.offset(1);
    *fresh1 = '*' as i32 as libc::c_char;
    let fresh2 = p;
    p = p.offset(1);
    *fresh2 = '.' as i32 as libc::c_char;
    let fresh3 = p;
    p = p.offset(1);
    *fresh3 = '*' as i32 as libc::c_char;
    *p = 'L' as i32 as libc::c_char;
    p = p.offset(((2 as libc::c_int) < 2 as libc::c_int) as libc::c_int as isize);
    let fresh4 = p;
    p = p.offset(1);
    *fresh4 = (if flags & FTOASTR_UPPER_E as libc::c_int != 0 {
        'G' as i32
    } else {
        'g' as i32
    }) as libc::c_char;
    *p = '\0' as i32 as libc::c_char;
    prec = if abs_x < 2.2250738585072014e-308f64 {
        1 as libc::c_int
    } else {
        15 as libc::c_int
    };
    loop {
        let mut n: libc::c_int = snprintf(
            buf,
            bufsize,
            format.as_mut_ptr(),
            width,
            prec,
            promoted_x,
        );
        if n < 0 as libc::c_int
            || (53 as libc::c_int * 1 as libc::c_int * 146 as libc::c_int
                + 484 as libc::c_int) / 485 as libc::c_int + 1 as libc::c_int <= prec
            || (n as libc::c_ulong) < bufsize
                && strtod(buf, 0 as *mut *mut libc::c_char) == x
        {
            return n;
        }
        prec += 1;
        prec;
    };
}
