use ::libc;
extern "C" {
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn memcmp2(
    mut s1: *const libc::c_char,
    mut n1: size_t,
    mut s2: *const libc::c_char,
    mut n2: size_t,
) -> libc::c_int {
    let mut cmp: libc::c_int = memcmp(
        s1 as *const libc::c_void,
        s2 as *const libc::c_void,
        if n1 <= n2 { n1 } else { n2 },
    );
    if cmp == 0 as libc::c_int {
        cmp = (n1 > n2) as libc::c_int - (n1 < n2) as libc::c_int;
    }
    return cmp;
}
