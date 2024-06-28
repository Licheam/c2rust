use ::libc;
extern "C" {
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn sig2str(_: libc::c_int, _: *mut libc::c_char) -> libc::c_int;
    fn str2sig(_: *const libc::c_char, _: *mut libc::c_int) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn operand2sig(
    mut operand: *const libc::c_char,
    mut signame: *mut libc::c_char,
) -> libc::c_int {
    let mut signum: libc::c_int = 0;
    if (*operand as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
        <= 9 as libc::c_int as libc::c_uint
    {
        let mut endp: *mut libc::c_char = 0 as *mut libc::c_char;
        *__errno_location() = 0 as libc::c_int;
        let mut l: libc::c_long = strtol(operand, &mut endp, 10 as libc::c_int);
        let mut i: libc::c_int = l as libc::c_int;
        signum = if operand == endp as *const libc::c_char || *endp as libc::c_int != 0
            || *__errno_location() != 0 || i as libc::c_long != l
        {
            -(1 as libc::c_int)
        } else {
            i
        };
        if signum != -(1 as libc::c_int) {
            signum
                &= if signum >= 0xff as libc::c_int {
                    0xff as libc::c_int
                } else {
                    0x7f as libc::c_int
                };
        }
    } else {
        let mut upcased: *mut libc::c_char = xstrdup(operand);
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        p = upcased;
        while *p != 0 {
            if !(strchr(
                b"abcdefghijklmnopqrstuvwxyz\0" as *const u8 as *const libc::c_char,
                *p as libc::c_int,
            ))
                .is_null()
            {
                *p = (*p as libc::c_int + ('A' as i32 - 'a' as i32)) as libc::c_char;
            }
            p = p.offset(1);
            p;
        }
        if !(str2sig(upcased, &mut signum) == 0 as libc::c_int
            || *upcased.offset(0 as libc::c_int as isize) as libc::c_int == 'S' as i32
                && *upcased.offset(1 as libc::c_int as isize) as libc::c_int
                    == 'I' as i32
                && *upcased.offset(2 as libc::c_int as isize) as libc::c_int
                    == 'G' as i32
                && str2sig(upcased.offset(3 as libc::c_int as isize), &mut signum)
                    == 0 as libc::c_int)
        {
            signum = -(1 as libc::c_int);
        }
        free(upcased as *mut libc::c_void);
    }
    if signum < 0 as libc::c_int || sig2str(signum, signame) != 0 as libc::c_int {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            gettext(b"%s: invalid signal\0" as *const u8 as *const libc::c_char),
            quote(operand),
        );
        return -(1 as libc::c_int);
    }
    return signum;
}
