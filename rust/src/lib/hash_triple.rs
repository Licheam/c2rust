use ::libc;
extern "C" {
    fn same_name(source: *const libc::c_char, dest: *const libc::c_char) -> bool;
}
pub type __dev_t = libc::c_ulong;
pub type __ino_t = libc::c_ulong;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct F_triple {
    pub name: *mut libc::c_char,
    pub st_ino: ino_t,
    pub st_dev: dev_t,
}
#[no_mangle]
pub unsafe extern "C" fn triple_hash_no_name(
    mut x: *const libc::c_void,
    mut table_size: size_t,
) -> size_t {
    let mut p: *const F_triple = x as *const F_triple;
    return ((*p).st_ino).wrapping_rem(table_size);
}
#[no_mangle]
pub unsafe extern "C" fn triple_compare(
    mut x: *const libc::c_void,
    mut y: *const libc::c_void,
) -> bool {
    let mut a: *const F_triple = x as *const F_triple;
    let mut b: *const F_triple = y as *const F_triple;
    return if (*a).st_ino == (*b).st_ino && (*a).st_dev == (*b).st_dev
        && same_name((*a).name, (*b).name) as libc::c_int != 0
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0;
}
