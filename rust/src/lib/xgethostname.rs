use ::libc;
extern "C" {
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn ximemdup(p: *const libc::c_void, s: idx_t) -> *mut libc::c_void;
    fn xpalloc(
        pa: *mut libc::c_void,
        pn: *mut idx_t,
        n_incr_min: idx_t,
        n_max: ptrdiff_t,
        s: idx_t,
    ) -> *mut libc::c_void;
    fn gethostname(__name: *mut libc::c_char, __len: size_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_long;
pub type idx_t = ptrdiff_t;
#[no_mangle]
pub unsafe extern "C" fn xgethostname() -> *mut libc::c_char {
    let mut buf: [libc::c_char; 100] = [0; 100];
    let mut size: idx_t = ::core::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong
        as idx_t;
    let mut name: *mut libc::c_char = buf.as_mut_ptr();
    let mut alloc: *mut libc::c_char = 0 as *mut libc::c_char;
    loop {
        let mut size_1: idx_t = size - 1 as libc::c_int as libc::c_long;
        *name.offset(size_1 as isize) = '\0' as i32 as libc::c_char;
        *__errno_location() = 0 as libc::c_int;
        if gethostname(name, size_1 as size_t) == 0 as libc::c_int {
            let mut actual_size: idx_t = (strlen(name))
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as idx_t;
            if actual_size < size_1 {
                return (if !alloc.is_null() {
                    alloc as *mut libc::c_void
                } else {
                    ximemdup(name as *const libc::c_void, actual_size)
                }) as *mut libc::c_char;
            }
            *__errno_location() = 0 as libc::c_int;
        }
        free(alloc as *mut libc::c_void);
        if *__errno_location() != 0 as libc::c_int
            && *__errno_location() != 36 as libc::c_int
            && *__errno_location() != 22 as libc::c_int
            && *__errno_location() != 12 as libc::c_int
        {
            return 0 as *mut libc::c_char;
        }
        alloc = xpalloc(
            0 as *mut libc::c_void,
            &mut size,
            1 as libc::c_int as idx_t,
            -(1 as libc::c_int) as ptrdiff_t,
            1 as libc::c_int as idx_t,
        ) as *mut libc::c_char;
        name = alloc;
    };
}
