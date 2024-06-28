use ::libc;
extern "C" {
    pub type __locale_data;
    fn newlocale(
        __category_mask: libc::c_int,
        __locale: *const libc::c_char,
        __base: locale_t,
    ) -> locale_t;
    fn strtod_l(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __loc: locale_t,
    ) -> libc::c_double;
}
pub type locale_t = __locale_t;
pub type __locale_t = *mut __locale_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13],
    pub __ctype_b: *const libc::c_ushort,
    pub __ctype_tolower: *const libc::c_int,
    pub __ctype_toupper: *const libc::c_int,
    pub __names: [*const libc::c_char; 13],
}
#[no_mangle]
pub unsafe extern "C" fn c_strtod(
    mut nptr: *const libc::c_char,
    mut endptr: *mut *mut libc::c_char,
) -> libc::c_double {
    let mut r: libc::c_double = 0.;
    let mut locale: locale_t = c_locale();
    if locale.is_null() {
        if !endptr.is_null() {
            *endptr = nptr as *mut libc::c_char;
        }
        return 0 as libc::c_int as libc::c_double;
    }
    r = strtod_l(nptr, endptr, locale);
    return r;
}
static mut c_locale_cache: locale_t = 0 as *const __locale_struct
    as *mut __locale_struct;
unsafe extern "C" fn c_locale() -> locale_t {
    if c_locale_cache.is_null() {
        ::core::ptr::write_volatile(
            &mut c_locale_cache as *mut locale_t,
            newlocale(
                (1 as libc::c_int) << 0 as libc::c_int
                    | (1 as libc::c_int) << 1 as libc::c_int
                    | (1 as libc::c_int) << 2 as libc::c_int
                    | (1 as libc::c_int) << 3 as libc::c_int
                    | (1 as libc::c_int) << 4 as libc::c_int
                    | (1 as libc::c_int) << 5 as libc::c_int
                    | (1 as libc::c_int) << 7 as libc::c_int
                    | (1 as libc::c_int) << 8 as libc::c_int
                    | (1 as libc::c_int) << 9 as libc::c_int
                    | (1 as libc::c_int) << 10 as libc::c_int
                    | (1 as libc::c_int) << 11 as libc::c_int
                    | (1 as libc::c_int) << 12 as libc::c_int,
                b"C\0" as *const u8 as *const libc::c_char,
                0 as locale_t,
            ),
        );
    }
    return c_locale_cache;
}
