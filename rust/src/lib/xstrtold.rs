use ::libc;
use ::f128;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xstrtold(
    mut str: *const libc::c_char,
    mut ptr: *mut *const libc::c_char,
    mut result: *mut f128::f128,
    mut convert: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut *mut libc::c_char) -> f128::f128,
    >,
) -> bool {
    let mut val: f128::f128 = f128::f128::ZERO;
    let mut terminator: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ok: bool = 1 as libc::c_int != 0;
    *__errno_location() = 0 as libc::c_int;
    val = convert.expect("non-null function pointer")(str, &mut terminator);
    if terminator == str as *mut libc::c_char
        || ptr.is_null() && *terminator as libc::c_int != '\0' as i32
    {
        ok = 0 as libc::c_int != 0;
    } else if val != f128::f128::new(0 as libc::c_int)
        && *__errno_location() == 34 as libc::c_int
    {
        ok = 0 as libc::c_int != 0;
    }
    if !ptr.is_null() {
        *ptr = terminator;
    }
    *result = val;
    return ok;
}
