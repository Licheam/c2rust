use ::libc;
extern "C" {
    pub type randread_source;
    fn randread_new(_: *const libc::c_char, _: size_t) -> *mut randread_source;
    fn randread(_: *mut randread_source, _: *mut libc::c_void, _: size_t);
    fn randread_free(_: *mut randread_source) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn free(_: *mut libc::c_void);
    fn explicit_bzero(__s: *mut libc::c_void, __n: size_t);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
}
pub type __uintmax_t = libc::c_ulong;
pub type uintmax_t = __uintmax_t;
pub type size_t = libc::c_ulong;
pub type randint = uintmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct randint_source {
    pub source: *mut randread_source,
    pub randnum: randint,
    pub randmax: randint,
}
pub const HUGE_BYTES: C2RustUnnamed = 0;
pub type C2RustUnnamed = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn randint_new(
    mut source: *mut randread_source,
) -> *mut randint_source {
    let mut s: *mut randint_source = xmalloc(
        ::core::mem::size_of::<randint_source>() as libc::c_ulong,
    ) as *mut randint_source;
    (*s).source = source;
    (*s).randmax = 0 as libc::c_int as randint;
    (*s).randnum = (*s).randmax;
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn randint_all_new(
    mut name: *const libc::c_char,
    mut bytes_bound: size_t,
) -> *mut randint_source {
    let mut source: *mut randread_source = randread_new(name, bytes_bound);
    return if !source.is_null() {
        randint_new(source)
    } else {
        0 as *mut randint_source
    };
}
#[no_mangle]
pub unsafe extern "C" fn randint_all_free(mut s: *mut randint_source) -> libc::c_int {
    let mut r: libc::c_int = randread_free((*s).source);
    let mut e: libc::c_int = *__errno_location();
    randint_free(s);
    *__errno_location() = e;
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn randint_get_source(
    mut s: *const randint_source,
) -> *mut randread_source {
    return (*s).source;
}
#[no_mangle]
pub unsafe extern "C" fn randint_genmax(
    mut s: *mut randint_source,
    mut genmax: randint,
) -> randint {
    let mut source: *mut randread_source = (*s).source;
    let mut randnum: randint = (*s).randnum;
    let mut randmax: randint = (*s).randmax;
    let mut choices: randint = genmax.wrapping_add(1 as libc::c_int as libc::c_ulong);
    loop {
        if randmax < genmax {
            let mut i: size_t = 0 as libc::c_int as size_t;
            let mut rmax: randint = randmax;
            let mut buf: [libc::c_uchar; 8] = [0; 8];
            loop {
                rmax = (shift_left(rmax))
                    .wrapping_add(
                        (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                            as libc::c_ulong,
                    );
                i = i.wrapping_add(1);
                i;
                if !(rmax < genmax) {
                    break;
                }
            }
            randread(source, buf.as_mut_ptr() as *mut libc::c_void, i);
            i = 0 as libc::c_int as size_t;
            loop {
                randnum = (shift_left(randnum))
                    .wrapping_add(buf[i as usize] as libc::c_ulong);
                randmax = (shift_left(randmax))
                    .wrapping_add(
                        (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                            as libc::c_ulong,
                    );
                i = i.wrapping_add(1);
                i;
                if !(randmax < genmax) {
                    break;
                }
            }
        }
        if randmax == genmax {
            (*s).randmax = 0 as libc::c_int as randint;
            (*s).randnum = (*s).randmax;
            return randnum;
        } else {
            let mut excess_choices: randint = randmax.wrapping_sub(genmax);
            let mut unusable_choices: randint = excess_choices.wrapping_rem(choices);
            let mut last_usable_choice: randint = randmax.wrapping_sub(unusable_choices);
            let mut reduced_randnum: randint = randnum.wrapping_rem(choices);
            if randnum <= last_usable_choice {
                (*s).randnum = randnum.wrapping_div(choices);
                (*s).randmax = excess_choices.wrapping_div(choices);
                return reduced_randnum;
            }
            randnum = reduced_randnum;
            randmax = unusable_choices.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn randint_free(mut s: *mut randint_source) {
    explicit_bzero(
        s as *mut libc::c_void,
        ::core::mem::size_of::<randint_source>() as libc::c_ulong,
    );
    free(s as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn shift_left(mut x: randint) -> randint {
    return if HUGE_BYTES as libc::c_int != 0 {
        0 as libc::c_int as libc::c_ulong
    } else {
        x << 8 as libc::c_int
    };
}
