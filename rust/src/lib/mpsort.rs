use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type comparison_function = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[no_mangle]
pub unsafe extern "C" fn mpsort(
    mut base: *mut *const libc::c_void,
    mut n: size_t,
    mut cmp: comparison_function,
) {
    mpsort_with_tmp(base, n, base.offset(n as isize), cmp);
}
unsafe extern "C" fn mpsort_with_tmp(
    mut base: *mut *const libc::c_void,
    mut n: size_t,
    mut tmp: *mut *const libc::c_void,
    mut cmp: comparison_function,
) {
    if n <= 2 as libc::c_int as libc::c_ulong {
        if n == 2 as libc::c_int as libc::c_ulong {
            let mut p0: *const libc::c_void = *base.offset(0 as libc::c_int as isize);
            let mut p1: *const libc::c_void = *base.offset(1 as libc::c_int as isize);
            if !(cmp.expect("non-null function pointer")(p0, p1) <= 0 as libc::c_int) {
                let ref mut fresh0 = *base.offset(0 as libc::c_int as isize);
                *fresh0 = p1;
                let ref mut fresh1 = *base.offset(1 as libc::c_int as isize);
                *fresh1 = p0;
            }
        }
    } else {
        let mut n1: size_t = n.wrapping_div(2 as libc::c_int as libc::c_ulong);
        let mut n2: size_t = n.wrapping_sub(n1);
        let mut i: size_t = 0;
        let mut t: size_t = 0 as libc::c_int as size_t;
        let mut tlim: size_t = n1;
        let mut b: size_t = n1;
        let mut blim: size_t = n;
        let mut bb: *const libc::c_void = 0 as *const libc::c_void;
        let mut tt: *const libc::c_void = 0 as *const libc::c_void;
        mpsort_with_tmp(base.offset(n1 as isize), n2, tmp, cmp);
        if n1 < 2 as libc::c_int as libc::c_ulong {
            let ref mut fresh2 = *tmp.offset(0 as libc::c_int as isize);
            *fresh2 = *base.offset(0 as libc::c_int as isize);
        } else {
            mpsort_into_tmp(base, n1, tmp, cmp);
        }
        tt = *tmp.offset(t as isize);
        bb = *base.offset(b as isize);
        i = 0 as libc::c_int as size_t;
        loop {
            if cmp.expect("non-null function pointer")(tt, bb) <= 0 as libc::c_int {
                let fresh3 = i;
                i = i.wrapping_add(1);
                let ref mut fresh4 = *base.offset(fresh3 as isize);
                *fresh4 = tt;
                t = t.wrapping_add(1);
                t;
                if t == tlim {
                    break;
                }
                tt = *tmp.offset(t as isize);
            } else {
                let fresh5 = i;
                i = i.wrapping_add(1);
                let ref mut fresh6 = *base.offset(fresh5 as isize);
                *fresh6 = bb;
                b = b.wrapping_add(1);
                b;
                if b == blim {
                    memcpy(
                        base.offset(i as isize) as *mut libc::c_void,
                        tmp.offset(t as isize) as *const libc::c_void,
                        tlim
                            .wrapping_sub(t)
                            .wrapping_mul(
                                ::core::mem::size_of::<*const libc::c_void>()
                                    as libc::c_ulong,
                            ),
                    );
                    break;
                } else {
                    bb = *base.offset(b as isize);
                }
            }
        }
    };
}
unsafe extern "C" fn mpsort_into_tmp(
    mut base: *mut *const libc::c_void,
    mut n: size_t,
    mut tmp: *mut *const libc::c_void,
    mut cmp: comparison_function,
) {
    let mut n1: size_t = n.wrapping_div(2 as libc::c_int as libc::c_ulong);
    let mut n2: size_t = n.wrapping_sub(n1);
    let mut a: size_t = 0 as libc::c_int as size_t;
    let mut alim: size_t = n1;
    let mut b: size_t = n1;
    let mut blim: size_t = n;
    let mut ba: *const libc::c_void = 0 as *const libc::c_void;
    let mut bb: *const libc::c_void = 0 as *const libc::c_void;
    mpsort_with_tmp(base.offset(n1 as isize), n2, tmp, cmp);
    mpsort_with_tmp(base, n1, tmp, cmp);
    ba = *base.offset(a as isize);
    bb = *base.offset(b as isize);
    loop {
        if cmp.expect("non-null function pointer")(ba, bb) <= 0 as libc::c_int {
            let fresh7 = tmp;
            tmp = tmp.offset(1);
            *fresh7 = ba;
            a = a.wrapping_add(1);
            a;
            if a == alim {
                a = b;
                alim = blim;
                break;
            } else {
                ba = *base.offset(a as isize);
            }
        } else {
            let fresh8 = tmp;
            tmp = tmp.offset(1);
            *fresh8 = bb;
            b = b.wrapping_add(1);
            b;
            if b == blim {
                break;
            }
            bb = *base.offset(b as isize);
        }
    }
    memcpy(
        tmp as *mut libc::c_void,
        base.offset(a as isize) as *const libc::c_void,
        alim
            .wrapping_sub(a)
            .wrapping_mul(::core::mem::size_of::<*const libc::c_void>() as libc::c_ulong),
    );
}
