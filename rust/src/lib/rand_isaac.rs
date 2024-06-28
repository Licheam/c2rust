use ::libc;
pub type __uint64_t = libc::c_ulong;
pub type __uint_least64_t = __uint64_t;
pub type uint_least64_t = __uint_least64_t;
pub type isaac_word = uint_least64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct isaac_state {
    pub m: [isaac_word; 256],
    pub a: isaac_word,
    pub b: isaac_word,
    pub c: isaac_word,
}
pub const HALF: C2RustUnnamed = 128;
pub type C2RustUnnamed = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn isaac_seed(mut s: *mut isaac_state) {
    let mut a: isaac_word = if (1 as libc::c_int) << 6 as libc::c_int
        == 32 as libc::c_int
    {
        0x1367df5a as libc::c_uint as libc::c_ulong
    } else {
        0x647c4677a2884b7c as libc::c_ulong
    };
    let mut b: isaac_word = if (1 as libc::c_int) << 6 as libc::c_int
        == 32 as libc::c_int
    {
        0x95d90059 as libc::c_uint as libc::c_ulong
    } else {
        0xb9f8b322c73ac862 as libc::c_ulong
    };
    let mut c: isaac_word = if (1 as libc::c_int) << 6 as libc::c_int
        == 32 as libc::c_int
    {
        0xc3163e4b as libc::c_uint as libc::c_ulong
    } else {
        0x8c0ea5053d4712a0 as libc::c_ulong
    };
    let mut d: isaac_word = if (1 as libc::c_int) << 6 as libc::c_int
        == 32 as libc::c_int
    {
        0xf421ad8 as libc::c_uint as libc::c_ulong
    } else {
        0xb29b2e824a595524 as libc::c_ulong
    };
    let mut e: isaac_word = if (1 as libc::c_int) << 6 as libc::c_int
        == 32 as libc::c_int
    {
        0xd92a4a78 as libc::c_uint as libc::c_ulong
    } else {
        0x82f053db8355e0ce as libc::c_ulong
    };
    let mut f: isaac_word = if (1 as libc::c_int) << 6 as libc::c_int
        == 32 as libc::c_int
    {
        0xa51a3c49 as libc::c_uint as libc::c_ulong
    } else {
        0x48fe4a0fa5a09315 as libc::c_ulong
    };
    let mut g: isaac_word = if (1 as libc::c_int) << 6 as libc::c_int
        == 32 as libc::c_int
    {
        0xc4efea1b as libc::c_uint as libc::c_ulong
    } else {
        0xae985bf2cbfc89ed as libc::c_ulong
    };
    let mut h: isaac_word = if (1 as libc::c_int) << 6 as libc::c_int
        == 32 as libc::c_int
    {
        0x30609119 as libc::c_uint as libc::c_ulong
    } else {
        0x98f5704f6c44c0ab as libc::c_ulong
    };
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (1 as libc::c_int) << 8 as libc::c_int {
        a = (a as libc::c_ulong).wrapping_add((*s).m[i as usize]) as isaac_word
            as isaac_word;
        b = (b as libc::c_ulong).wrapping_add((*s).m[(i + 1 as libc::c_int) as usize])
            as isaac_word as isaac_word;
        c = (c as libc::c_ulong).wrapping_add((*s).m[(i + 2 as libc::c_int) as usize])
            as isaac_word as isaac_word;
        d = (d as libc::c_ulong).wrapping_add((*s).m[(i + 3 as libc::c_int) as usize])
            as isaac_word as isaac_word;
        e = (e as libc::c_ulong).wrapping_add((*s).m[(i + 4 as libc::c_int) as usize])
            as isaac_word as isaac_word;
        f = (f as libc::c_ulong).wrapping_add((*s).m[(i + 5 as libc::c_int) as usize])
            as isaac_word as isaac_word;
        g = (g as libc::c_ulong).wrapping_add((*s).m[(i + 6 as libc::c_int) as usize])
            as isaac_word as isaac_word;
        h = (h as libc::c_ulong).wrapping_add((*s).m[(i + 7 as libc::c_int) as usize])
            as isaac_word as isaac_word;
        a = (a as libc::c_ulong).wrapping_sub(e) as isaac_word as isaac_word;
        f ^= just(h) >> 9 as libc::c_int;
        h = (h as libc::c_ulong).wrapping_add(a) as isaac_word as isaac_word;
        b = (b as libc::c_ulong).wrapping_sub(f) as isaac_word as isaac_word;
        g ^= a << 9 as libc::c_int;
        a = (a as libc::c_ulong).wrapping_add(b) as isaac_word as isaac_word;
        c = (c as libc::c_ulong).wrapping_sub(g) as isaac_word as isaac_word;
        h ^= just(b) >> 23 as libc::c_int;
        b = (b as libc::c_ulong).wrapping_add(c) as isaac_word as isaac_word;
        d = (d as libc::c_ulong).wrapping_sub(h) as isaac_word as isaac_word;
        a ^= c << 15 as libc::c_int;
        c = (c as libc::c_ulong).wrapping_add(d) as isaac_word as isaac_word;
        e = (e as libc::c_ulong).wrapping_sub(a) as isaac_word as isaac_word;
        b ^= just(d) >> 14 as libc::c_int;
        d = (d as libc::c_ulong).wrapping_add(e) as isaac_word as isaac_word;
        f = (f as libc::c_ulong).wrapping_sub(b) as isaac_word as isaac_word;
        c ^= e << 20 as libc::c_int;
        e = (e as libc::c_ulong).wrapping_add(f) as isaac_word as isaac_word;
        g = (g as libc::c_ulong).wrapping_sub(c) as isaac_word as isaac_word;
        d ^= just(f) >> 17 as libc::c_int;
        f = (f as libc::c_ulong).wrapping_add(g) as isaac_word as isaac_word;
        h = (h as libc::c_ulong).wrapping_sub(d) as isaac_word as isaac_word;
        e ^= g << 14 as libc::c_int;
        g = (g as libc::c_ulong).wrapping_add(h) as isaac_word as isaac_word;
        (*s).m[i as usize] = a;
        (*s).m[(i + 1 as libc::c_int) as usize] = b;
        (*s).m[(i + 2 as libc::c_int) as usize] = c;
        (*s).m[(i + 3 as libc::c_int) as usize] = d;
        (*s).m[(i + 4 as libc::c_int) as usize] = e;
        (*s).m[(i + 5 as libc::c_int) as usize] = f;
        (*s).m[(i + 6 as libc::c_int) as usize] = g;
        (*s).m[(i + 7 as libc::c_int) as usize] = h;
        i += 8 as libc::c_int;
    }
    let mut i_0: libc::c_int = 0;
    i_0 = 0 as libc::c_int;
    while i_0 < (1 as libc::c_int) << 8 as libc::c_int {
        a = (a as libc::c_ulong).wrapping_add((*s).m[i_0 as usize]) as isaac_word
            as isaac_word;
        b = (b as libc::c_ulong).wrapping_add((*s).m[(i_0 + 1 as libc::c_int) as usize])
            as isaac_word as isaac_word;
        c = (c as libc::c_ulong).wrapping_add((*s).m[(i_0 + 2 as libc::c_int) as usize])
            as isaac_word as isaac_word;
        d = (d as libc::c_ulong).wrapping_add((*s).m[(i_0 + 3 as libc::c_int) as usize])
            as isaac_word as isaac_word;
        e = (e as libc::c_ulong).wrapping_add((*s).m[(i_0 + 4 as libc::c_int) as usize])
            as isaac_word as isaac_word;
        f = (f as libc::c_ulong).wrapping_add((*s).m[(i_0 + 5 as libc::c_int) as usize])
            as isaac_word as isaac_word;
        g = (g as libc::c_ulong).wrapping_add((*s).m[(i_0 + 6 as libc::c_int) as usize])
            as isaac_word as isaac_word;
        h = (h as libc::c_ulong).wrapping_add((*s).m[(i_0 + 7 as libc::c_int) as usize])
            as isaac_word as isaac_word;
        a = (a as libc::c_ulong).wrapping_sub(e) as isaac_word as isaac_word;
        f ^= just(h) >> 9 as libc::c_int;
        h = (h as libc::c_ulong).wrapping_add(a) as isaac_word as isaac_word;
        b = (b as libc::c_ulong).wrapping_sub(f) as isaac_word as isaac_word;
        g ^= a << 9 as libc::c_int;
        a = (a as libc::c_ulong).wrapping_add(b) as isaac_word as isaac_word;
        c = (c as libc::c_ulong).wrapping_sub(g) as isaac_word as isaac_word;
        h ^= just(b) >> 23 as libc::c_int;
        b = (b as libc::c_ulong).wrapping_add(c) as isaac_word as isaac_word;
        d = (d as libc::c_ulong).wrapping_sub(h) as isaac_word as isaac_word;
        a ^= c << 15 as libc::c_int;
        c = (c as libc::c_ulong).wrapping_add(d) as isaac_word as isaac_word;
        e = (e as libc::c_ulong).wrapping_sub(a) as isaac_word as isaac_word;
        b ^= just(d) >> 14 as libc::c_int;
        d = (d as libc::c_ulong).wrapping_add(e) as isaac_word as isaac_word;
        f = (f as libc::c_ulong).wrapping_sub(b) as isaac_word as isaac_word;
        c ^= e << 20 as libc::c_int;
        e = (e as libc::c_ulong).wrapping_add(f) as isaac_word as isaac_word;
        g = (g as libc::c_ulong).wrapping_sub(c) as isaac_word as isaac_word;
        d ^= just(f) >> 17 as libc::c_int;
        f = (f as libc::c_ulong).wrapping_add(g) as isaac_word as isaac_word;
        h = (h as libc::c_ulong).wrapping_sub(d) as isaac_word as isaac_word;
        e ^= g << 14 as libc::c_int;
        g = (g as libc::c_ulong).wrapping_add(h) as isaac_word as isaac_word;
        (*s).m[i_0 as usize] = a;
        (*s).m[(i_0 + 1 as libc::c_int) as usize] = b;
        (*s).m[(i_0 + 2 as libc::c_int) as usize] = c;
        (*s).m[(i_0 + 3 as libc::c_int) as usize] = d;
        (*s).m[(i_0 + 4 as libc::c_int) as usize] = e;
        (*s).m[(i_0 + 5 as libc::c_int) as usize] = f;
        (*s).m[(i_0 + 6 as libc::c_int) as usize] = g;
        (*s).m[(i_0 + 7 as libc::c_int) as usize] = h;
        i_0 += 8 as libc::c_int;
    }
    (*s).c = 0 as libc::c_int as isaac_word;
    (*s).b = (*s).c;
    (*s).a = (*s).b;
}
#[no_mangle]
pub unsafe extern "C" fn isaac_refill(
    mut s: *mut isaac_state,
    mut result: *mut isaac_word,
) {
    let mut a: isaac_word = (*s).a;
    (*s).c = ((*s).c).wrapping_add(1);
    let mut b: isaac_word = ((*s).b).wrapping_add((*s).c);
    let mut m: *mut isaac_word = ((*s).m).as_mut_ptr();
    let mut r: *mut isaac_word = result;
    loop {
        let mut x: isaac_word = 0;
        let mut y: isaac_word = 0;
        a = ((if (1 as libc::c_int) << 6 as libc::c_int == 32 as libc::c_int {
            a
        } else {
            0 as libc::c_int as libc::c_ulong
        })
            ^ (if (1 as libc::c_int) << 6 as libc::c_int == 32 as libc::c_int {
                a << 13 as libc::c_int
            } else {
                !(a ^ a << 21 as libc::c_int)
            }))
            .wrapping_add(*m.offset((HALF as libc::c_int + 0 as libc::c_int) as isize));
        x = *m.offset(0 as libc::c_int as isize);
        y = (ind(((*s).m).as_mut_ptr(), x)).wrapping_add(a).wrapping_add(b);
        *m.offset(0 as libc::c_int as isize) = y;
        b = just((ind(((*s).m).as_mut_ptr(), y >> 8 as libc::c_int)).wrapping_add(x));
        *r.offset(0 as libc::c_int as isize) = b;
        let mut x_0: isaac_word = 0;
        let mut y_0: isaac_word = 0;
        a = ((if (1 as libc::c_int) << 6 as libc::c_int == 32 as libc::c_int {
            a
        } else {
            0 as libc::c_int as libc::c_ulong
        })
            ^ (if (1 as libc::c_int) << 6 as libc::c_int == 32 as libc::c_int {
                just(a) >> 6 as libc::c_int
            } else {
                a ^ just(a) >> 5 as libc::c_int
            }))
            .wrapping_add(*m.offset((HALF as libc::c_int + 1 as libc::c_int) as isize));
        x_0 = *m.offset(1 as libc::c_int as isize);
        y_0 = (ind(((*s).m).as_mut_ptr(), x_0)).wrapping_add(a).wrapping_add(b);
        *m.offset(1 as libc::c_int as isize) = y_0;
        b = just(
            (ind(((*s).m).as_mut_ptr(), y_0 >> 8 as libc::c_int)).wrapping_add(x_0),
        );
        *r.offset(1 as libc::c_int as isize) = b;
        let mut x_1: isaac_word = 0;
        let mut y_1: isaac_word = 0;
        a = ((if (1 as libc::c_int) << 6 as libc::c_int == 32 as libc::c_int {
            a
        } else {
            0 as libc::c_int as libc::c_ulong
        })
            ^ (if (1 as libc::c_int) << 6 as libc::c_int == 32 as libc::c_int {
                a << 2 as libc::c_int
            } else {
                a ^ a << 12 as libc::c_int
            }))
            .wrapping_add(*m.offset((HALF as libc::c_int + 2 as libc::c_int) as isize));
        x_1 = *m.offset(2 as libc::c_int as isize);
        y_1 = (ind(((*s).m).as_mut_ptr(), x_1)).wrapping_add(a).wrapping_add(b);
        *m.offset(2 as libc::c_int as isize) = y_1;
        b = just(
            (ind(((*s).m).as_mut_ptr(), y_1 >> 8 as libc::c_int)).wrapping_add(x_1),
        );
        *r.offset(2 as libc::c_int as isize) = b;
        let mut x_2: isaac_word = 0;
        let mut y_2: isaac_word = 0;
        a = ((if (1 as libc::c_int) << 6 as libc::c_int == 32 as libc::c_int {
            a
        } else {
            0 as libc::c_int as libc::c_ulong
        })
            ^ (if (1 as libc::c_int) << 6 as libc::c_int == 32 as libc::c_int {
                just(a) >> 16 as libc::c_int
            } else {
                a ^ just(a) >> 33 as libc::c_int
            }))
            .wrapping_add(*m.offset((HALF as libc::c_int + 3 as libc::c_int) as isize));
        x_2 = *m.offset(3 as libc::c_int as isize);
        y_2 = (ind(((*s).m).as_mut_ptr(), x_2)).wrapping_add(a).wrapping_add(b);
        *m.offset(3 as libc::c_int as isize) = y_2;
        b = just(
            (ind(((*s).m).as_mut_ptr(), y_2 >> 8 as libc::c_int)).wrapping_add(x_2),
        );
        *r.offset(3 as libc::c_int as isize) = b;
        r = r.offset(4 as libc::c_int as isize);
        m = m.offset(4 as libc::c_int as isize);
        if !(m < ((*s).m).as_mut_ptr().offset(HALF as libc::c_int as isize)) {
            break;
        }
    }
    loop {
        let mut x_3: isaac_word = 0;
        let mut y_3: isaac_word = 0;
        a = ((if (1 as libc::c_int) << 6 as libc::c_int == 32 as libc::c_int {
            a
        } else {
            0 as libc::c_int as libc::c_ulong
        })
            ^ (if (1 as libc::c_int) << 6 as libc::c_int == 32 as libc::c_int {
                a << 13 as libc::c_int
            } else {
                !(a ^ a << 21 as libc::c_int)
            }))
            .wrapping_add(
                *m.offset((-(HALF as libc::c_int) + 0 as libc::c_int) as isize),
            );
        x_3 = *m.offset(0 as libc::c_int as isize);
        y_3 = (ind(((*s).m).as_mut_ptr(), x_3)).wrapping_add(a).wrapping_add(b);
        *m.offset(0 as libc::c_int as isize) = y_3;
        b = just(
            (ind(((*s).m).as_mut_ptr(), y_3 >> 8 as libc::c_int)).wrapping_add(x_3),
        );
        *r.offset(0 as libc::c_int as isize) = b;
        let mut x_4: isaac_word = 0;
        let mut y_4: isaac_word = 0;
        a = ((if (1 as libc::c_int) << 6 as libc::c_int == 32 as libc::c_int {
            a
        } else {
            0 as libc::c_int as libc::c_ulong
        })
            ^ (if (1 as libc::c_int) << 6 as libc::c_int == 32 as libc::c_int {
                just(a) >> 6 as libc::c_int
            } else {
                a ^ just(a) >> 5 as libc::c_int
            }))
            .wrapping_add(
                *m.offset((-(HALF as libc::c_int) + 1 as libc::c_int) as isize),
            );
        x_4 = *m.offset(1 as libc::c_int as isize);
        y_4 = (ind(((*s).m).as_mut_ptr(), x_4)).wrapping_add(a).wrapping_add(b);
        *m.offset(1 as libc::c_int as isize) = y_4;
        b = just(
            (ind(((*s).m).as_mut_ptr(), y_4 >> 8 as libc::c_int)).wrapping_add(x_4),
        );
        *r.offset(1 as libc::c_int as isize) = b;
        let mut x_5: isaac_word = 0;
        let mut y_5: isaac_word = 0;
        a = ((if (1 as libc::c_int) << 6 as libc::c_int == 32 as libc::c_int {
            a
        } else {
            0 as libc::c_int as libc::c_ulong
        })
            ^ (if (1 as libc::c_int) << 6 as libc::c_int == 32 as libc::c_int {
                a << 2 as libc::c_int
            } else {
                a ^ a << 12 as libc::c_int
            }))
            .wrapping_add(
                *m.offset((-(HALF as libc::c_int) + 2 as libc::c_int) as isize),
            );
        x_5 = *m.offset(2 as libc::c_int as isize);
        y_5 = (ind(((*s).m).as_mut_ptr(), x_5)).wrapping_add(a).wrapping_add(b);
        *m.offset(2 as libc::c_int as isize) = y_5;
        b = just(
            (ind(((*s).m).as_mut_ptr(), y_5 >> 8 as libc::c_int)).wrapping_add(x_5),
        );
        *r.offset(2 as libc::c_int as isize) = b;
        let mut x_6: isaac_word = 0;
        let mut y_6: isaac_word = 0;
        a = ((if (1 as libc::c_int) << 6 as libc::c_int == 32 as libc::c_int {
            a
        } else {
            0 as libc::c_int as libc::c_ulong
        })
            ^ (if (1 as libc::c_int) << 6 as libc::c_int == 32 as libc::c_int {
                just(a) >> 16 as libc::c_int
            } else {
                a ^ just(a) >> 33 as libc::c_int
            }))
            .wrapping_add(
                *m.offset((-(HALF as libc::c_int) + 3 as libc::c_int) as isize),
            );
        x_6 = *m.offset(3 as libc::c_int as isize);
        y_6 = (ind(((*s).m).as_mut_ptr(), x_6)).wrapping_add(a).wrapping_add(b);
        *m.offset(3 as libc::c_int as isize) = y_6;
        b = just(
            (ind(((*s).m).as_mut_ptr(), y_6 >> 8 as libc::c_int)).wrapping_add(x_6),
        );
        *r.offset(3 as libc::c_int as isize) = b;
        r = r.offset(4 as libc::c_int as isize);
        m = m.offset(4 as libc::c_int as isize);
        if !(m
            < ((*s).m)
                .as_mut_ptr()
                .offset(((1 as libc::c_int) << 8 as libc::c_int) as isize))
        {
            break;
        }
    }
    (*s).a = a;
    (*s).b = b;
}
#[inline]
unsafe extern "C" fn just(mut a: isaac_word) -> isaac_word {
    let mut desired_bits: isaac_word = (((1 as libc::c_int as isaac_word)
        << 1 as libc::c_int)
        << ((1 as libc::c_int) << 6 as libc::c_int) - 1 as libc::c_int)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    return a & desired_bits;
}
#[inline]
unsafe extern "C" fn ind(mut m: *const isaac_word, mut x: isaac_word) -> isaac_word {
    if (::core::mem::size_of::<isaac_word>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        == ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_ulong
    {
        let mut void_m: *const libc::c_void = m as *const libc::c_void;
        let mut base_p: *const libc::c_char = void_m as *const libc::c_char;
        let mut word_p: *const libc::c_void = base_p
            .offset(
                (x
                    & ((((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<isaac_word>() as libc::c_ulong,
                        )) as isize,
            ) as *const libc::c_void;
        let mut p: *const isaac_word = word_p as *const isaac_word;
        return *p;
    } else {
        return *m
            .offset(
                (x
                    .wrapping_div(
                        (((1 as libc::c_int) << 6 as libc::c_int) / 8 as libc::c_int)
                            as libc::c_ulong,
                    )
                    & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_ulong) as isize,
            )
    };
}
