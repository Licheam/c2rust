use ::libc;
extern "C" {
    fn toupper(_: libc::c_int) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn memcasecmp(
    mut vs1: *const libc::c_void,
    mut vs2: *const libc::c_void,
    mut n: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut s1: *const libc::c_char = vs1 as *const libc::c_char;
    let mut s2: *const libc::c_char = vs2 as *const libc::c_char;
    i = 0 as libc::c_int as size_t;
    while i < n {
        let mut u1: libc::c_uchar = *s1.offset(i as isize) as libc::c_uchar;
        let mut u2: libc::c_uchar = *s2.offset(i as isize) as libc::c_uchar;
        let mut U1: libc::c_int = toupper(u1 as libc::c_int);
        let mut U2: libc::c_int = toupper(u2 as libc::c_int);
        let mut diff: libc::c_int = if 127 as libc::c_int * 2 as libc::c_int
            + 1 as libc::c_int <= 2147483647 as libc::c_int
        {
            U1 - U2
        } else {
            (U1 > U2) as libc::c_int - (U1 < U2) as libc::c_int
        };
        if diff != 0 {
            return diff;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
