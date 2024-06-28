use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    static mut exit_failure: libc::c_int;
    fn memcoll(
        _: *mut libc::c_char,
        _: size_t,
        _: *mut libc::c_char,
        _: size_t,
    ) -> libc::c_int;
    fn memcoll0(
        _: *const libc::c_char,
        _: size_t,
        _: *const libc::c_char,
        _: size_t,
    ) -> libc::c_int;
    fn quotearg_n_style_mem(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
        argsize: size_t,
    ) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type quoting_style = libc::c_uint;
pub const custom_quoting_style: quoting_style = 10;
pub const clocale_quoting_style: quoting_style = 9;
pub const locale_quoting_style: quoting_style = 8;
pub const escape_quoting_style: quoting_style = 7;
pub const c_maybe_quoting_style: quoting_style = 6;
pub const c_quoting_style: quoting_style = 5;
pub const shell_escape_always_quoting_style: quoting_style = 4;
pub const shell_escape_quoting_style: quoting_style = 3;
pub const shell_always_quoting_style: quoting_style = 2;
pub const shell_quoting_style: quoting_style = 1;
pub const literal_quoting_style: quoting_style = 0;
#[no_mangle]
pub unsafe extern "C" fn xmemcoll(
    mut s1: *mut libc::c_char,
    mut s1len: size_t,
    mut s2: *mut libc::c_char,
    mut s2len: size_t,
) -> libc::c_int {
    let mut diff: libc::c_int = memcoll(s1, s1len, s2, s2len);
    let mut collation_errno: libc::c_int = *__errno_location();
    if collation_errno != 0 {
        collate_error(collation_errno, s1, s1len, s2, s2len);
    }
    return diff;
}
#[no_mangle]
pub unsafe extern "C" fn xmemcoll0(
    mut s1: *const libc::c_char,
    mut s1size: size_t,
    mut s2: *const libc::c_char,
    mut s2size: size_t,
) -> libc::c_int {
    let mut diff: libc::c_int = memcoll0(s1, s1size, s2, s2size);
    let mut collation_errno: libc::c_int = *__errno_location();
    if collation_errno != 0 {
        collate_error(
            collation_errno,
            s1,
            s1size.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            s2,
            s2size.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    return diff;
}
unsafe extern "C" fn collate_error(
    mut collation_errno: libc::c_int,
    mut s1: *const libc::c_char,
    mut s1len: size_t,
    mut s2: *const libc::c_char,
    mut s2len: size_t,
) {
    error(
        0 as libc::c_int,
        collation_errno,
        gettext(b"string comparison failed\0" as *const u8 as *const libc::c_char),
    );
    error(
        0 as libc::c_int,
        0 as libc::c_int,
        gettext(
            b"Set LC_ALL='C' to work around the problem.\0" as *const u8
                as *const libc::c_char,
        ),
    );
    error(
        exit_failure,
        0 as libc::c_int,
        gettext(
            b"The strings compared were %s and %s.\0" as *const u8 as *const libc::c_char,
        ),
        quotearg_n_style_mem(0 as libc::c_int, locale_quoting_style, s1, s1len),
        quotearg_n_style_mem(1 as libc::c_int, locale_quoting_style, s2, s2len),
    );
}
