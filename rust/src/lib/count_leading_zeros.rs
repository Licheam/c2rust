use ::libc;
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn count_leading_zeros_ll(
    mut x: libc::c_ulonglong,
) -> libc::c_int {
    return (if x != 0 {
        x.leading_zeros() as i32 as libc::c_ulong
    } else {
        (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_ulonglong>() as libc::c_ulong)
    }) as libc::c_int;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn count_leading_zeros_l(mut x: libc::c_ulong) -> libc::c_int {
    return (if x != 0 {
        x.leading_zeros() as i32 as libc::c_ulong
    } else {
        (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
    }) as libc::c_int;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn count_leading_zeros(mut x: libc::c_uint) -> libc::c_int {
    return (if x != 0 {
        x.leading_zeros() as i32 as libc::c_ulong
    } else {
        (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
    }) as libc::c_int;
}
