use ::libc;
extern "C" {
    fn c_strtold(
        nptr: *const libc::c_char,
        endptr: *mut *mut libc::c_char,
    ) -> f128::f128;
    fn __errno_location() -> *mut libc::c_int;
    fn strtold(_: *const libc::c_char, _: *mut *mut libc::c_char) -> f128::f128;
}
#[no_mangle]
pub unsafe extern "C" fn cl_strtold(
    mut nptr: *const libc::c_char,
    mut endptr: *mut *mut libc::c_char,
) -> f128::f128 {
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut d: f128::f128 = strtold(nptr, &mut end);
    if *end != 0 {
        let mut strtod_errno: libc::c_int = *__errno_location();
        let mut c_end: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut c: f128::f128 = c_strtold(nptr, &mut c_end);
        if end < c_end {
            d = c;
            end = c_end;
        } else {
            *__errno_location() = strtod_errno;
        }
    }
    if !endptr.is_null() {
        *endptr = end;
    }
    return d;
}
