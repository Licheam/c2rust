use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type uintptr_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sm3_ctx {
    pub state: [uint32_t; 8],
    pub total: [uint32_t; 2],
    pub buflen: size_t,
    pub buffer: [uint32_t; 32],
}
#[no_mangle]
pub unsafe extern "C" fn sm3_buffer(
    mut buffer: *const libc::c_char,
    mut len: size_t,
    mut resblock: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut ctx: sm3_ctx = sm3_ctx {
        state: [0; 8],
        total: [0; 2],
        buflen: 0,
        buffer: [0; 32],
    };
    sm3_init_ctx(&mut ctx);
    sm3_process_bytes(buffer as *const libc::c_void, len, &mut ctx);
    return sm3_finish_ctx(&mut ctx, resblock);
}
#[no_mangle]
pub unsafe extern "C" fn sm3_read_ctx(
    mut ctx: *const sm3_ctx,
    mut resbuf: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut i: libc::c_int = 0;
    let mut r: *mut libc::c_char = resbuf as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        set_uint32(
            r
                .offset(
                    (i as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
                        ) as isize,
                ),
            __bswap_32((*ctx).state[i as usize]),
        );
        i += 1;
        i;
    }
    return resbuf;
}
#[no_mangle]
pub unsafe extern "C" fn sm3_finish_ctx(
    mut ctx: *mut sm3_ctx,
    mut resbuf: *mut libc::c_void,
) -> *mut libc::c_void {
    sm3_conclude_ctx(ctx);
    return sm3_read_ctx(ctx, resbuf);
}
#[no_mangle]
pub unsafe extern "C" fn sm3_process_bytes(
    mut buffer: *const libc::c_void,
    mut len: size_t,
    mut ctx: *mut sm3_ctx,
) {
    if (*ctx).buflen != 0 as libc::c_int as libc::c_ulong {
        let mut left_over: size_t = (*ctx).buflen;
        let mut add: size_t = if (128 as libc::c_int as libc::c_ulong)
            .wrapping_sub(left_over) > len
        {
            len
        } else {
            (128 as libc::c_int as libc::c_ulong).wrapping_sub(left_over)
        };
        memcpy(
            &mut *(((*ctx).buffer).as_mut_ptr() as *mut libc::c_char)
                .offset(left_over as isize) as *mut libc::c_char as *mut libc::c_void,
            buffer,
            add,
        );
        (*ctx)
            .buflen = ((*ctx).buflen as libc::c_ulong).wrapping_add(add) as size_t
            as size_t;
        if (*ctx).buflen > 64 as libc::c_int as libc::c_ulong {
            sm3_process_block(
                ((*ctx).buffer).as_mut_ptr() as *const libc::c_void,
                (*ctx).buflen & !(63 as libc::c_int) as libc::c_ulong,
                ctx,
            );
            (*ctx).buflen &= 63 as libc::c_int as libc::c_ulong;
            memcpy(
                ((*ctx).buffer).as_mut_ptr() as *mut libc::c_void,
                &mut *(((*ctx).buffer).as_mut_ptr() as *mut libc::c_char)
                    .offset(
                        (left_over.wrapping_add(add)
                            & !(63 as libc::c_int) as libc::c_ulong) as isize,
                    ) as *mut libc::c_char as *const libc::c_void,
                (*ctx).buflen,
            );
        }
        buffer = (buffer as *const libc::c_char).offset(add as isize)
            as *const libc::c_void;
        len = (len as libc::c_ulong).wrapping_sub(add) as size_t as size_t;
    }
    if len >= 64 as libc::c_int as libc::c_ulong {
        if (buffer as uintptr_t)
            .wrapping_rem(::core::mem::align_of::<uint32_t>() as libc::c_ulong)
            != 0 as libc::c_int as libc::c_ulong
        {
            while len > 64 as libc::c_int as libc::c_ulong {
                sm3_process_block(
                    memcpy(
                        ((*ctx).buffer).as_mut_ptr() as *mut libc::c_void,
                        buffer,
                        64 as libc::c_int as libc::c_ulong,
                    ),
                    64 as libc::c_int as size_t,
                    ctx,
                );
                buffer = (buffer as *const libc::c_char)
                    .offset(64 as libc::c_int as isize) as *const libc::c_void;
                len = (len as libc::c_ulong)
                    .wrapping_sub(64 as libc::c_int as libc::c_ulong) as size_t
                    as size_t;
            }
        } else {
            sm3_process_block(buffer, len & !(63 as libc::c_int) as libc::c_ulong, ctx);
            buffer = (buffer as *const libc::c_char)
                .offset((len & !(63 as libc::c_int) as libc::c_ulong) as isize)
                as *const libc::c_void;
            len &= 63 as libc::c_int as libc::c_ulong;
        }
    }
    if len > 0 as libc::c_int as libc::c_ulong {
        let mut left_over_0: size_t = (*ctx).buflen;
        memcpy(
            &mut *(((*ctx).buffer).as_mut_ptr() as *mut libc::c_char)
                .offset(left_over_0 as isize) as *mut libc::c_char as *mut libc::c_void,
            buffer,
            len,
        );
        left_over_0 = (left_over_0 as libc::c_ulong).wrapping_add(len) as size_t
            as size_t;
        if left_over_0 >= 64 as libc::c_int as libc::c_ulong {
            sm3_process_block(
                ((*ctx).buffer).as_mut_ptr() as *const libc::c_void,
                64 as libc::c_int as size_t,
                ctx,
            );
            left_over_0 = (left_over_0 as libc::c_ulong)
                .wrapping_sub(64 as libc::c_int as libc::c_ulong) as size_t as size_t;
            memcpy(
                ((*ctx).buffer).as_mut_ptr() as *mut libc::c_void,
                &mut *((*ctx).buffer).as_mut_ptr().offset(16 as libc::c_int as isize)
                    as *mut uint32_t as *const libc::c_void,
                left_over_0,
            );
        }
        (*ctx).buflen = left_over_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sm3_process_block(
    mut buffer: *const libc::c_void,
    mut len: size_t,
    mut ctx: *mut sm3_ctx,
) {
    let mut words: *const uint32_t = buffer as *const uint32_t;
    let mut nwords: size_t = len
        .wrapping_div(::core::mem::size_of::<uint32_t>() as libc::c_ulong);
    let mut endp: *const uint32_t = words.offset(nwords as isize);
    let mut x: [uint32_t; 16] = [0; 16];
    let mut a: uint32_t = (*ctx).state[0 as libc::c_int as usize];
    let mut b: uint32_t = (*ctx).state[1 as libc::c_int as usize];
    let mut c: uint32_t = (*ctx).state[2 as libc::c_int as usize];
    let mut d: uint32_t = (*ctx).state[3 as libc::c_int as usize];
    let mut e: uint32_t = (*ctx).state[4 as libc::c_int as usize];
    let mut f: uint32_t = (*ctx).state[5 as libc::c_int as usize];
    let mut g: uint32_t = (*ctx).state[6 as libc::c_int as usize];
    let mut h: uint32_t = (*ctx).state[7 as libc::c_int as usize];
    let mut lolen: uint32_t = len as uint32_t;
    (*ctx)
        .total[0 as libc::c_int
        as usize] = ((*ctx).total[0 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(lolen) as uint32_t as uint32_t;
    (*ctx)
        .total[1 as libc::c_int
        as usize] = ((*ctx).total[1 as libc::c_int as usize] as libc::c_ulong)
        .wrapping_add(
            (len >> 31 as libc::c_int >> 1 as libc::c_int)
                .wrapping_add(
                    ((*ctx).total[0 as libc::c_int as usize] < lolen) as libc::c_int
                        as libc::c_ulong,
                ),
        ) as uint32_t as uint32_t;
    while words < endp {
        let mut tw: uint32_t = 0;
        let mut ss1: uint32_t = 0;
        let mut ss2: uint32_t = 0;
        let mut j: libc::c_int = 0;
        j = 0 as libc::c_int;
        while j < 16 as libc::c_int {
            x[j as usize] = __bswap_32(*words);
            words = words.offset(1);
            words;
            j += 1;
            j;
        }
        j = -(1 as libc::c_int);
        j += 1;
        j != 0;
        ss1 = (a << (12 as libc::c_int & 31 as libc::c_int)
            | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(e)
            .wrapping_add(sm3_round_constants[0 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (a << (12 as libc::c_int & 31 as libc::c_int)
                | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(e)
                .wrapping_add(sm3_round_constants[0 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (a << (12 as libc::c_int & 31 as libc::c_int)
                | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        d = (d as libc::c_uint)
            .wrapping_add(
                (a ^ b ^ c)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(0 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(4 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        h = (h as libc::c_uint)
            .wrapping_add(
                (e ^ f ^ g)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(0 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        b = b << (9 as libc::c_int & 31 as libc::c_int)
            | b >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        f = f << (19 as libc::c_int & 31 as libc::c_int)
            | f >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        h = h
            ^ (h << (9 as libc::c_int & 31 as libc::c_int)
                | h >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (h << (17 as libc::c_int & 31 as libc::c_int)
                | h >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (d << (12 as libc::c_int & 31 as libc::c_int)
            | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(h)
            .wrapping_add(sm3_round_constants[1 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (d << (12 as libc::c_int & 31 as libc::c_int)
                | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(h)
                .wrapping_add(sm3_round_constants[1 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (d << (12 as libc::c_int & 31 as libc::c_int)
                | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        c = (c as libc::c_uint)
            .wrapping_add(
                (d ^ a ^ b)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(1 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(5 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        g = (g as libc::c_uint)
            .wrapping_add(
                (h ^ e ^ f)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(1 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        a = a << (9 as libc::c_int & 31 as libc::c_int)
            | a >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        e = e << (19 as libc::c_int & 31 as libc::c_int)
            | e >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        g = g
            ^ (g << (9 as libc::c_int & 31 as libc::c_int)
                | g >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (g << (17 as libc::c_int & 31 as libc::c_int)
                | g >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (c << (12 as libc::c_int & 31 as libc::c_int)
            | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(g)
            .wrapping_add(sm3_round_constants[2 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (c << (12 as libc::c_int & 31 as libc::c_int)
                | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(g)
                .wrapping_add(sm3_round_constants[2 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (c << (12 as libc::c_int & 31 as libc::c_int)
                | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        b = (b as libc::c_uint)
            .wrapping_add(
                (c ^ d ^ a)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(2 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(6 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        f = (f as libc::c_uint)
            .wrapping_add(
                (g ^ h ^ e)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(2 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        d = d << (9 as libc::c_int & 31 as libc::c_int)
            | d >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        h = h << (19 as libc::c_int & 31 as libc::c_int)
            | h >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        f = f
            ^ (f << (9 as libc::c_int & 31 as libc::c_int)
                | f >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (f << (17 as libc::c_int & 31 as libc::c_int)
                | f >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (b << (12 as libc::c_int & 31 as libc::c_int)
            | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(f)
            .wrapping_add(sm3_round_constants[3 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (b << (12 as libc::c_int & 31 as libc::c_int)
                | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(f)
                .wrapping_add(sm3_round_constants[3 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (b << (12 as libc::c_int & 31 as libc::c_int)
                | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        a = (a as libc::c_uint)
            .wrapping_add(
                (b ^ c ^ d)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(3 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(7 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        e = (e as libc::c_uint)
            .wrapping_add(
                (f ^ g ^ h)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(3 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        c = c << (9 as libc::c_int & 31 as libc::c_int)
            | c >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        g = g << (19 as libc::c_int & 31 as libc::c_int)
            | g >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        e = e
            ^ (e << (9 as libc::c_int & 31 as libc::c_int)
                | e >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (e << (17 as libc::c_int & 31 as libc::c_int)
                | e >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (a << (12 as libc::c_int & 31 as libc::c_int)
            | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(e)
            .wrapping_add(sm3_round_constants[4 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (a << (12 as libc::c_int & 31 as libc::c_int)
                | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(e)
                .wrapping_add(sm3_round_constants[4 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (a << (12 as libc::c_int & 31 as libc::c_int)
                | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        d = (d as libc::c_uint)
            .wrapping_add(
                (a ^ b ^ c)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(4 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(8 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        h = (h as libc::c_uint)
            .wrapping_add(
                (e ^ f ^ g)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(4 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        b = b << (9 as libc::c_int & 31 as libc::c_int)
            | b >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        f = f << (19 as libc::c_int & 31 as libc::c_int)
            | f >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        h = h
            ^ (h << (9 as libc::c_int & 31 as libc::c_int)
                | h >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (h << (17 as libc::c_int & 31 as libc::c_int)
                | h >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (d << (12 as libc::c_int & 31 as libc::c_int)
            | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(h)
            .wrapping_add(sm3_round_constants[5 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (d << (12 as libc::c_int & 31 as libc::c_int)
                | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(h)
                .wrapping_add(sm3_round_constants[5 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (d << (12 as libc::c_int & 31 as libc::c_int)
                | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        c = (c as libc::c_uint)
            .wrapping_add(
                (d ^ a ^ b)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(5 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(9 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        g = (g as libc::c_uint)
            .wrapping_add(
                (h ^ e ^ f)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(5 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        a = a << (9 as libc::c_int & 31 as libc::c_int)
            | a >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        e = e << (19 as libc::c_int & 31 as libc::c_int)
            | e >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        g = g
            ^ (g << (9 as libc::c_int & 31 as libc::c_int)
                | g >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (g << (17 as libc::c_int & 31 as libc::c_int)
                | g >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (c << (12 as libc::c_int & 31 as libc::c_int)
            | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(g)
            .wrapping_add(sm3_round_constants[6 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (c << (12 as libc::c_int & 31 as libc::c_int)
                | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(g)
                .wrapping_add(sm3_round_constants[6 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (c << (12 as libc::c_int & 31 as libc::c_int)
                | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        b = (b as libc::c_uint)
            .wrapping_add(
                (c ^ d ^ a)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(6 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(10 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        f = (f as libc::c_uint)
            .wrapping_add(
                (g ^ h ^ e)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(6 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        d = d << (9 as libc::c_int & 31 as libc::c_int)
            | d >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        h = h << (19 as libc::c_int & 31 as libc::c_int)
            | h >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        f = f
            ^ (f << (9 as libc::c_int & 31 as libc::c_int)
                | f >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (f << (17 as libc::c_int & 31 as libc::c_int)
                | f >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (b << (12 as libc::c_int & 31 as libc::c_int)
            | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(f)
            .wrapping_add(sm3_round_constants[7 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (b << (12 as libc::c_int & 31 as libc::c_int)
                | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(f)
                .wrapping_add(sm3_round_constants[7 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (b << (12 as libc::c_int & 31 as libc::c_int)
                | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        a = (a as libc::c_uint)
            .wrapping_add(
                (b ^ c ^ d)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(7 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(11 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        e = (e as libc::c_uint)
            .wrapping_add(
                (f ^ g ^ h)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(7 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        c = c << (9 as libc::c_int & 31 as libc::c_int)
            | c >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        g = g << (19 as libc::c_int & 31 as libc::c_int)
            | g >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        e = e
            ^ (e << (9 as libc::c_int & 31 as libc::c_int)
                | e >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (e << (17 as libc::c_int & 31 as libc::c_int)
                | e >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (a << (12 as libc::c_int & 31 as libc::c_int)
            | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(e)
            .wrapping_add(sm3_round_constants[8 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (a << (12 as libc::c_int & 31 as libc::c_int)
                | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(e)
                .wrapping_add(sm3_round_constants[8 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (a << (12 as libc::c_int & 31 as libc::c_int)
                | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        d = (d as libc::c_uint)
            .wrapping_add(
                (a ^ b ^ c)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(8 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(12 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        h = (h as libc::c_uint)
            .wrapping_add(
                (e ^ f ^ g)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(8 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        b = b << (9 as libc::c_int & 31 as libc::c_int)
            | b >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        f = f << (19 as libc::c_int & 31 as libc::c_int)
            | f >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        h = h
            ^ (h << (9 as libc::c_int & 31 as libc::c_int)
                | h >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (h << (17 as libc::c_int & 31 as libc::c_int)
                | h >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (d << (12 as libc::c_int & 31 as libc::c_int)
            | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(h)
            .wrapping_add(sm3_round_constants[9 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (d << (12 as libc::c_int & 31 as libc::c_int)
                | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(h)
                .wrapping_add(sm3_round_constants[9 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (d << (12 as libc::c_int & 31 as libc::c_int)
                | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        c = (c as libc::c_uint)
            .wrapping_add(
                (d ^ a ^ b)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(9 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(13 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        g = (g as libc::c_uint)
            .wrapping_add(
                (h ^ e ^ f)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(9 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        a = a << (9 as libc::c_int & 31 as libc::c_int)
            | a >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        e = e << (19 as libc::c_int & 31 as libc::c_int)
            | e >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        g = g
            ^ (g << (9 as libc::c_int & 31 as libc::c_int)
                | g >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (g << (17 as libc::c_int & 31 as libc::c_int)
                | g >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (c << (12 as libc::c_int & 31 as libc::c_int)
            | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(g)
            .wrapping_add(sm3_round_constants[10 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (c << (12 as libc::c_int & 31 as libc::c_int)
                | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(g)
                .wrapping_add(sm3_round_constants[10 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (c << (12 as libc::c_int & 31 as libc::c_int)
                | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        b = (b as libc::c_uint)
            .wrapping_add(
                (c ^ d ^ a)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(10 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(14 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        f = (f as libc::c_uint)
            .wrapping_add(
                (g ^ h ^ e)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(10 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        d = d << (9 as libc::c_int & 31 as libc::c_int)
            | d >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        h = h << (19 as libc::c_int & 31 as libc::c_int)
            | h >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        f = f
            ^ (f << (9 as libc::c_int & 31 as libc::c_int)
                | f >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (f << (17 as libc::c_int & 31 as libc::c_int)
                | f >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (b << (12 as libc::c_int & 31 as libc::c_int)
            | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(f)
            .wrapping_add(sm3_round_constants[11 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (b << (12 as libc::c_int & 31 as libc::c_int)
                | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(f)
                .wrapping_add(sm3_round_constants[11 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (b << (12 as libc::c_int & 31 as libc::c_int)
                | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        a = (a as libc::c_uint)
            .wrapping_add(
                (b ^ c ^ d)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(11 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(15 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        e = (e as libc::c_uint)
            .wrapping_add(
                (f ^ g ^ h)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(11 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        c = c << (9 as libc::c_int & 31 as libc::c_int)
            | c >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        g = g << (19 as libc::c_int & 31 as libc::c_int)
            | g >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        e = e
            ^ (e << (9 as libc::c_int & 31 as libc::c_int)
                | e >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (e << (17 as libc::c_int & 31 as libc::c_int)
                | e >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (a << (12 as libc::c_int & 31 as libc::c_int)
            | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(e)
            .wrapping_add(sm3_round_constants[12 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (a << (12 as libc::c_int & 31 as libc::c_int)
                | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(e)
                .wrapping_add(sm3_round_constants[12 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (a << (12 as libc::c_int & 31 as libc::c_int)
                | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(16 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(16 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(16 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(16 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(16 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(16 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(16 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(16 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(16 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(16 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(16 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(16 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(16 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(16 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(16 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(16 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(16 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(16 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(16 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(16 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(16 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(16 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(16 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(16 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        d = (d as libc::c_uint)
            .wrapping_add(
                (a ^ b ^ c)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(12 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(16 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        h = (h as libc::c_uint)
            .wrapping_add(
                (e ^ f ^ g)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(12 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        b = b << (9 as libc::c_int & 31 as libc::c_int)
            | b >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        f = f << (19 as libc::c_int & 31 as libc::c_int)
            | f >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        h = h
            ^ (h << (9 as libc::c_int & 31 as libc::c_int)
                | h >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (h << (17 as libc::c_int & 31 as libc::c_int)
                | h >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (d << (12 as libc::c_int & 31 as libc::c_int)
            | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(h)
            .wrapping_add(sm3_round_constants[13 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (d << (12 as libc::c_int & 31 as libc::c_int)
                | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(h)
                .wrapping_add(sm3_round_constants[13 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (d << (12 as libc::c_int & 31 as libc::c_int)
                | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(17 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(17 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(17 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(17 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(17 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(17 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(17 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(17 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(17 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(17 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(17 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(17 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(17 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(17 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(17 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(17 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(17 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(17 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(17 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(17 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(17 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(17 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(17 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(17 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d ^ a ^ b)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(13 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(17 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        g = (g as libc::c_uint)
            .wrapping_add(
                (h ^ e ^ f)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(13 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        a = a << (9 as libc::c_int & 31 as libc::c_int)
            | a >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        e = e << (19 as libc::c_int & 31 as libc::c_int)
            | e >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        g = g
            ^ (g << (9 as libc::c_int & 31 as libc::c_int)
                | g >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (g << (17 as libc::c_int & 31 as libc::c_int)
                | g >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (c << (12 as libc::c_int & 31 as libc::c_int)
            | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(g)
            .wrapping_add(sm3_round_constants[14 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (c << (12 as libc::c_int & 31 as libc::c_int)
                | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(g)
                .wrapping_add(sm3_round_constants[14 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (c << (12 as libc::c_int & 31 as libc::c_int)
                | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(18 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(18 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(18 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(18 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(18 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(18 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(18 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(18 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(18 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(18 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(18 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(18 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(18 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(18 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(18 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(18 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(18 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(18 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(18 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(18 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(18 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(18 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(18 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(18 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c ^ d ^ a)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(14 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(18 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        f = (f as libc::c_uint)
            .wrapping_add(
                (g ^ h ^ e)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(14 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        d = d << (9 as libc::c_int & 31 as libc::c_int)
            | d >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        h = h << (19 as libc::c_int & 31 as libc::c_int)
            | h >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        f = f
            ^ (f << (9 as libc::c_int & 31 as libc::c_int)
                | f >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (f << (17 as libc::c_int & 31 as libc::c_int)
                | f >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (b << (12 as libc::c_int & 31 as libc::c_int)
            | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(f)
            .wrapping_add(sm3_round_constants[15 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (b << (12 as libc::c_int & 31 as libc::c_int)
                | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(f)
                .wrapping_add(sm3_round_constants[15 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (b << (12 as libc::c_int & 31 as libc::c_int)
                | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(19 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(19 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(19 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(19 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(19 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(19 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(19 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(19 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(19 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(19 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(19 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(19 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(19 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(19 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(19 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(19 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(19 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(19 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(19 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(19 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(19 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(19 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(19 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(19 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b ^ c ^ d)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(15 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(19 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        e = (e as libc::c_uint)
            .wrapping_add(
                (f ^ g ^ h)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(15 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        c = c << (9 as libc::c_int & 31 as libc::c_int)
            | c >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        g = g << (19 as libc::c_int & 31 as libc::c_int)
            | g >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        e = e
            ^ (e << (9 as libc::c_int & 31 as libc::c_int)
                | e >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (e << (17 as libc::c_int & 31 as libc::c_int)
                | e >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (a << (12 as libc::c_int & 31 as libc::c_int)
            | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(e)
            .wrapping_add(sm3_round_constants[16 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (a << (12 as libc::c_int & 31 as libc::c_int)
                | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(e)
                .wrapping_add(sm3_round_constants[16 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (a << (12 as libc::c_int & 31 as libc::c_int)
                | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(20 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(20 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(20 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(20 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(20 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(20 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(20 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(20 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(20 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(20 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(20 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(20 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(20 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(20 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(20 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(20 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(20 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(20 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(20 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(20 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(20 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(20 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(20 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(20 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        d = (d as libc::c_uint)
            .wrapping_add(
                (a & b | a & c | b & c)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(16 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(20 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        h = (h as libc::c_uint)
            .wrapping_add(
                (e & f | !e & g)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(16 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        b = b << (9 as libc::c_int & 31 as libc::c_int)
            | b >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        f = f << (19 as libc::c_int & 31 as libc::c_int)
            | f >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        h = h
            ^ (h << (9 as libc::c_int & 31 as libc::c_int)
                | h >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (h << (17 as libc::c_int & 31 as libc::c_int)
                | h >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (d << (12 as libc::c_int & 31 as libc::c_int)
            | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(h)
            .wrapping_add(sm3_round_constants[17 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (d << (12 as libc::c_int & 31 as libc::c_int)
                | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(h)
                .wrapping_add(sm3_round_constants[17 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (d << (12 as libc::c_int & 31 as libc::c_int)
                | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(21 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(21 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(21 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(21 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(21 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(21 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(21 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(21 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(21 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(21 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(21 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(21 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(21 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(21 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(21 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(21 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(21 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(21 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(21 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(21 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(21 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(21 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(21 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(21 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d & a | d & b | a & b)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(17 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(21 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        g = (g as libc::c_uint)
            .wrapping_add(
                (h & e | !h & f)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(17 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        a = a << (9 as libc::c_int & 31 as libc::c_int)
            | a >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        e = e << (19 as libc::c_int & 31 as libc::c_int)
            | e >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        g = g
            ^ (g << (9 as libc::c_int & 31 as libc::c_int)
                | g >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (g << (17 as libc::c_int & 31 as libc::c_int)
                | g >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (c << (12 as libc::c_int & 31 as libc::c_int)
            | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(g)
            .wrapping_add(sm3_round_constants[18 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (c << (12 as libc::c_int & 31 as libc::c_int)
                | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(g)
                .wrapping_add(sm3_round_constants[18 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (c << (12 as libc::c_int & 31 as libc::c_int)
                | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(22 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(22 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(22 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(22 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(22 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(22 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(22 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(22 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(22 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(22 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(22 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(22 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(22 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(22 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(22 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(22 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(22 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(22 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(22 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(22 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(22 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(22 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(22 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(22 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c & d | c & a | d & a)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(18 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(22 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        f = (f as libc::c_uint)
            .wrapping_add(
                (g & h | !g & e)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(18 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        d = d << (9 as libc::c_int & 31 as libc::c_int)
            | d >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        h = h << (19 as libc::c_int & 31 as libc::c_int)
            | h >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        f = f
            ^ (f << (9 as libc::c_int & 31 as libc::c_int)
                | f >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (f << (17 as libc::c_int & 31 as libc::c_int)
                | f >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (b << (12 as libc::c_int & 31 as libc::c_int)
            | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(f)
            .wrapping_add(sm3_round_constants[19 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (b << (12 as libc::c_int & 31 as libc::c_int)
                | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(f)
                .wrapping_add(sm3_round_constants[19 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (b << (12 as libc::c_int & 31 as libc::c_int)
                | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(23 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(23 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(23 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(23 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(23 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(23 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(23 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(23 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(23 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(23 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(23 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(23 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(23 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(23 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(23 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(23 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(23 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(23 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(23 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(23 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(23 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(23 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(23 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(23 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b & c | b & d | c & d)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(19 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(23 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        e = (e as libc::c_uint)
            .wrapping_add(
                (f & g | !f & h)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(19 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        c = c << (9 as libc::c_int & 31 as libc::c_int)
            | c >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        g = g << (19 as libc::c_int & 31 as libc::c_int)
            | g >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        e = e
            ^ (e << (9 as libc::c_int & 31 as libc::c_int)
                | e >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (e << (17 as libc::c_int & 31 as libc::c_int)
                | e >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (a << (12 as libc::c_int & 31 as libc::c_int)
            | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(e)
            .wrapping_add(sm3_round_constants[20 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (a << (12 as libc::c_int & 31 as libc::c_int)
                | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(e)
                .wrapping_add(sm3_round_constants[20 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (a << (12 as libc::c_int & 31 as libc::c_int)
                | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(24 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(24 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(24 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(24 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(24 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(24 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(24 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(24 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(24 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(24 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(24 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(24 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(24 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(24 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(24 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(24 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(24 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(24 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(24 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(24 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(24 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(24 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(24 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(24 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        d = (d as libc::c_uint)
            .wrapping_add(
                (a & b | a & c | b & c)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(20 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(24 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        h = (h as libc::c_uint)
            .wrapping_add(
                (e & f | !e & g)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(20 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        b = b << (9 as libc::c_int & 31 as libc::c_int)
            | b >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        f = f << (19 as libc::c_int & 31 as libc::c_int)
            | f >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        h = h
            ^ (h << (9 as libc::c_int & 31 as libc::c_int)
                | h >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (h << (17 as libc::c_int & 31 as libc::c_int)
                | h >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (d << (12 as libc::c_int & 31 as libc::c_int)
            | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(h)
            .wrapping_add(sm3_round_constants[21 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (d << (12 as libc::c_int & 31 as libc::c_int)
                | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(h)
                .wrapping_add(sm3_round_constants[21 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (d << (12 as libc::c_int & 31 as libc::c_int)
                | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(25 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(25 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(25 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(25 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(25 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(25 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(25 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(25 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(25 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(25 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(25 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(25 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(25 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(25 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(25 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(25 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(25 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(25 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(25 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(25 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(25 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(25 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(25 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(25 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d & a | d & b | a & b)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(21 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(25 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        g = (g as libc::c_uint)
            .wrapping_add(
                (h & e | !h & f)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(21 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        a = a << (9 as libc::c_int & 31 as libc::c_int)
            | a >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        e = e << (19 as libc::c_int & 31 as libc::c_int)
            | e >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        g = g
            ^ (g << (9 as libc::c_int & 31 as libc::c_int)
                | g >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (g << (17 as libc::c_int & 31 as libc::c_int)
                | g >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (c << (12 as libc::c_int & 31 as libc::c_int)
            | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(g)
            .wrapping_add(sm3_round_constants[22 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (c << (12 as libc::c_int & 31 as libc::c_int)
                | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(g)
                .wrapping_add(sm3_round_constants[22 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (c << (12 as libc::c_int & 31 as libc::c_int)
                | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(26 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(26 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(26 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(26 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(26 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(26 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(26 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(26 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(26 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(26 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(26 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(26 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(26 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(26 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(26 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(26 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(26 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(26 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(26 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(26 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(26 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(26 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(26 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(26 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c & d | c & a | d & a)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(22 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(26 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        f = (f as libc::c_uint)
            .wrapping_add(
                (g & h | !g & e)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(22 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        d = d << (9 as libc::c_int & 31 as libc::c_int)
            | d >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        h = h << (19 as libc::c_int & 31 as libc::c_int)
            | h >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        f = f
            ^ (f << (9 as libc::c_int & 31 as libc::c_int)
                | f >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (f << (17 as libc::c_int & 31 as libc::c_int)
                | f >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (b << (12 as libc::c_int & 31 as libc::c_int)
            | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(f)
            .wrapping_add(sm3_round_constants[23 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (b << (12 as libc::c_int & 31 as libc::c_int)
                | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(f)
                .wrapping_add(sm3_round_constants[23 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (b << (12 as libc::c_int & 31 as libc::c_int)
                | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(27 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(27 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(27 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(27 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(27 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(27 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(27 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(27 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(27 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(27 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(27 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(27 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(27 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(27 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(27 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(27 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(27 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(27 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(27 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(27 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(27 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(27 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(27 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(27 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b & c | b & d | c & d)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(23 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(27 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        e = (e as libc::c_uint)
            .wrapping_add(
                (f & g | !f & h)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(23 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        c = c << (9 as libc::c_int & 31 as libc::c_int)
            | c >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        g = g << (19 as libc::c_int & 31 as libc::c_int)
            | g >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        e = e
            ^ (e << (9 as libc::c_int & 31 as libc::c_int)
                | e >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (e << (17 as libc::c_int & 31 as libc::c_int)
                | e >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (a << (12 as libc::c_int & 31 as libc::c_int)
            | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(e)
            .wrapping_add(sm3_round_constants[24 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (a << (12 as libc::c_int & 31 as libc::c_int)
                | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(e)
                .wrapping_add(sm3_round_constants[24 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (a << (12 as libc::c_int & 31 as libc::c_int)
                | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(28 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(28 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(28 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(28 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(28 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(28 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(28 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(28 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(28 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(28 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(28 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(28 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(28 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(28 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(28 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(28 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(28 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(28 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(28 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(28 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(28 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(28 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(28 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(28 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        d = (d as libc::c_uint)
            .wrapping_add(
                (a & b | a & c | b & c)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(24 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(28 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        h = (h as libc::c_uint)
            .wrapping_add(
                (e & f | !e & g)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(24 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        b = b << (9 as libc::c_int & 31 as libc::c_int)
            | b >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        f = f << (19 as libc::c_int & 31 as libc::c_int)
            | f >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        h = h
            ^ (h << (9 as libc::c_int & 31 as libc::c_int)
                | h >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (h << (17 as libc::c_int & 31 as libc::c_int)
                | h >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (d << (12 as libc::c_int & 31 as libc::c_int)
            | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(h)
            .wrapping_add(sm3_round_constants[25 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (d << (12 as libc::c_int & 31 as libc::c_int)
                | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(h)
                .wrapping_add(sm3_round_constants[25 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (d << (12 as libc::c_int & 31 as libc::c_int)
                | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(29 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(29 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(29 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(29 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(29 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(29 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(29 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(29 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(29 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(29 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(29 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(29 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(29 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(29 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(29 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(29 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(29 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(29 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(29 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(29 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(29 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(29 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(29 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(29 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d & a | d & b | a & b)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(25 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(29 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        g = (g as libc::c_uint)
            .wrapping_add(
                (h & e | !h & f)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(25 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        a = a << (9 as libc::c_int & 31 as libc::c_int)
            | a >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        e = e << (19 as libc::c_int & 31 as libc::c_int)
            | e >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        g = g
            ^ (g << (9 as libc::c_int & 31 as libc::c_int)
                | g >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (g << (17 as libc::c_int & 31 as libc::c_int)
                | g >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (c << (12 as libc::c_int & 31 as libc::c_int)
            | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(g)
            .wrapping_add(sm3_round_constants[26 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (c << (12 as libc::c_int & 31 as libc::c_int)
                | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(g)
                .wrapping_add(sm3_round_constants[26 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (c << (12 as libc::c_int & 31 as libc::c_int)
                | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(30 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(30 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(30 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(30 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(30 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(30 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(30 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(30 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(30 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(30 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(30 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(30 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(30 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(30 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(30 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(30 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(30 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(30 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(30 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(30 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(30 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(30 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(30 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(30 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c & d | c & a | d & a)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(26 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(30 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        f = (f as libc::c_uint)
            .wrapping_add(
                (g & h | !g & e)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(26 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        d = d << (9 as libc::c_int & 31 as libc::c_int)
            | d >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        h = h << (19 as libc::c_int & 31 as libc::c_int)
            | h >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        f = f
            ^ (f << (9 as libc::c_int & 31 as libc::c_int)
                | f >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (f << (17 as libc::c_int & 31 as libc::c_int)
                | f >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (b << (12 as libc::c_int & 31 as libc::c_int)
            | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(f)
            .wrapping_add(sm3_round_constants[27 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (b << (12 as libc::c_int & 31 as libc::c_int)
                | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(f)
                .wrapping_add(sm3_round_constants[27 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (b << (12 as libc::c_int & 31 as libc::c_int)
                | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(31 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(31 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(31 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(31 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(31 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(31 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(31 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(31 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(31 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(31 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(31 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(31 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(31 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(31 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(31 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(31 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(31 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(31 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(31 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(31 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(31 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(31 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(31 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(31 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b & c | b & d | c & d)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(27 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(31 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        e = (e as libc::c_uint)
            .wrapping_add(
                (f & g | !f & h)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(27 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        c = c << (9 as libc::c_int & 31 as libc::c_int)
            | c >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        g = g << (19 as libc::c_int & 31 as libc::c_int)
            | g >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        e = e
            ^ (e << (9 as libc::c_int & 31 as libc::c_int)
                | e >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (e << (17 as libc::c_int & 31 as libc::c_int)
                | e >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (a << (12 as libc::c_int & 31 as libc::c_int)
            | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(e)
            .wrapping_add(sm3_round_constants[28 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (a << (12 as libc::c_int & 31 as libc::c_int)
                | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(e)
                .wrapping_add(sm3_round_constants[28 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (a << (12 as libc::c_int & 31 as libc::c_int)
                | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(32 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(32 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(32 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(32 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(32 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(32 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(32 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(32 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(32 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(32 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(32 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(32 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(32 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(32 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(32 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(32 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(32 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(32 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(32 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(32 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(32 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(32 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(32 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(32 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        d = (d as libc::c_uint)
            .wrapping_add(
                (a & b | a & c | b & c)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(28 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(32 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        h = (h as libc::c_uint)
            .wrapping_add(
                (e & f | !e & g)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(28 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        b = b << (9 as libc::c_int & 31 as libc::c_int)
            | b >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        f = f << (19 as libc::c_int & 31 as libc::c_int)
            | f >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        h = h
            ^ (h << (9 as libc::c_int & 31 as libc::c_int)
                | h >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (h << (17 as libc::c_int & 31 as libc::c_int)
                | h >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (d << (12 as libc::c_int & 31 as libc::c_int)
            | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(h)
            .wrapping_add(sm3_round_constants[29 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (d << (12 as libc::c_int & 31 as libc::c_int)
                | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(h)
                .wrapping_add(sm3_round_constants[29 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (d << (12 as libc::c_int & 31 as libc::c_int)
                | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(33 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(33 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(33 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(33 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(33 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(33 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(33 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(33 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(33 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(33 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(33 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(33 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(33 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(33 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(33 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(33 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(33 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(33 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(33 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(33 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(33 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(33 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(33 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(33 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d & a | d & b | a & b)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(29 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(33 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        g = (g as libc::c_uint)
            .wrapping_add(
                (h & e | !h & f)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(29 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        a = a << (9 as libc::c_int & 31 as libc::c_int)
            | a >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        e = e << (19 as libc::c_int & 31 as libc::c_int)
            | e >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        g = g
            ^ (g << (9 as libc::c_int & 31 as libc::c_int)
                | g >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (g << (17 as libc::c_int & 31 as libc::c_int)
                | g >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (c << (12 as libc::c_int & 31 as libc::c_int)
            | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(g)
            .wrapping_add(sm3_round_constants[30 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (c << (12 as libc::c_int & 31 as libc::c_int)
                | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(g)
                .wrapping_add(sm3_round_constants[30 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (c << (12 as libc::c_int & 31 as libc::c_int)
                | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(34 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(34 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(34 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(34 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(34 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(34 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(34 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(34 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(34 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(34 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(34 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(34 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(34 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(34 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(34 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(34 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(34 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(34 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(34 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(34 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(34 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(34 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(34 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(34 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c & d | c & a | d & a)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(30 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(34 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        f = (f as libc::c_uint)
            .wrapping_add(
                (g & h | !g & e)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(30 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        d = d << (9 as libc::c_int & 31 as libc::c_int)
            | d >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        h = h << (19 as libc::c_int & 31 as libc::c_int)
            | h >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        f = f
            ^ (f << (9 as libc::c_int & 31 as libc::c_int)
                | f >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (f << (17 as libc::c_int & 31 as libc::c_int)
                | f >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (b << (12 as libc::c_int & 31 as libc::c_int)
            | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(f)
            .wrapping_add(sm3_round_constants[31 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (b << (12 as libc::c_int & 31 as libc::c_int)
                | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(f)
                .wrapping_add(sm3_round_constants[31 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (b << (12 as libc::c_int & 31 as libc::c_int)
                | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(35 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(35 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(35 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(35 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(35 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(35 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(35 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(35 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(35 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(35 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(35 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(35 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(35 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(35 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(35 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(35 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(35 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(35 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(35 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(35 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(35 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(35 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(35 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(35 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b & c | b & d | c & d)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(31 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(35 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        e = (e as libc::c_uint)
            .wrapping_add(
                (f & g | !f & h)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(31 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        c = c << (9 as libc::c_int & 31 as libc::c_int)
            | c >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        g = g << (19 as libc::c_int & 31 as libc::c_int)
            | g >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        e = e
            ^ (e << (9 as libc::c_int & 31 as libc::c_int)
                | e >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (e << (17 as libc::c_int & 31 as libc::c_int)
                | e >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (a << (12 as libc::c_int & 31 as libc::c_int)
            | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(e)
            .wrapping_add(sm3_round_constants[32 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (a << (12 as libc::c_int & 31 as libc::c_int)
                | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(e)
                .wrapping_add(sm3_round_constants[32 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (a << (12 as libc::c_int & 31 as libc::c_int)
                | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(36 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(36 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(36 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(36 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(36 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(36 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(36 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(36 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(36 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(36 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(36 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(36 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(36 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(36 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(36 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(36 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(36 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(36 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(36 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(36 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(36 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(36 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(36 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(36 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        d = (d as libc::c_uint)
            .wrapping_add(
                (a & b | a & c | b & c)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(32 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(36 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        h = (h as libc::c_uint)
            .wrapping_add(
                (e & f | !e & g)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(32 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        b = b << (9 as libc::c_int & 31 as libc::c_int)
            | b >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        f = f << (19 as libc::c_int & 31 as libc::c_int)
            | f >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        h = h
            ^ (h << (9 as libc::c_int & 31 as libc::c_int)
                | h >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (h << (17 as libc::c_int & 31 as libc::c_int)
                | h >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (d << (12 as libc::c_int & 31 as libc::c_int)
            | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(h)
            .wrapping_add(sm3_round_constants[33 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (d << (12 as libc::c_int & 31 as libc::c_int)
                | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(h)
                .wrapping_add(sm3_round_constants[33 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (d << (12 as libc::c_int & 31 as libc::c_int)
                | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(37 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(37 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(37 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(37 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(37 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(37 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(37 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(37 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(37 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(37 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(37 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(37 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(37 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(37 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(37 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(37 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(37 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(37 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(37 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(37 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(37 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(37 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(37 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(37 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d & a | d & b | a & b)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(33 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(37 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        g = (g as libc::c_uint)
            .wrapping_add(
                (h & e | !h & f)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(33 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        a = a << (9 as libc::c_int & 31 as libc::c_int)
            | a >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        e = e << (19 as libc::c_int & 31 as libc::c_int)
            | e >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        g = g
            ^ (g << (9 as libc::c_int & 31 as libc::c_int)
                | g >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (g << (17 as libc::c_int & 31 as libc::c_int)
                | g >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (c << (12 as libc::c_int & 31 as libc::c_int)
            | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(g)
            .wrapping_add(sm3_round_constants[34 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (c << (12 as libc::c_int & 31 as libc::c_int)
                | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(g)
                .wrapping_add(sm3_round_constants[34 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (c << (12 as libc::c_int & 31 as libc::c_int)
                | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(38 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(38 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(38 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(38 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(38 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(38 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(38 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(38 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(38 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(38 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(38 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(38 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(38 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(38 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(38 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(38 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(38 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(38 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(38 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(38 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(38 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(38 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(38 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(38 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c & d | c & a | d & a)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(34 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(38 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        f = (f as libc::c_uint)
            .wrapping_add(
                (g & h | !g & e)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(34 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        d = d << (9 as libc::c_int & 31 as libc::c_int)
            | d >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        h = h << (19 as libc::c_int & 31 as libc::c_int)
            | h >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        f = f
            ^ (f << (9 as libc::c_int & 31 as libc::c_int)
                | f >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (f << (17 as libc::c_int & 31 as libc::c_int)
                | f >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (b << (12 as libc::c_int & 31 as libc::c_int)
            | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(f)
            .wrapping_add(sm3_round_constants[35 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (b << (12 as libc::c_int & 31 as libc::c_int)
                | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(f)
                .wrapping_add(sm3_round_constants[35 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (b << (12 as libc::c_int & 31 as libc::c_int)
                | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(39 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(39 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(39 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(39 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(39 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(39 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(39 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(39 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(39 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(39 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(39 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(39 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(39 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(39 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(39 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(39 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(39 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(39 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(39 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(39 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(39 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(39 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(39 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(39 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b & c | b & d | c & d)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(35 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(39 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        e = (e as libc::c_uint)
            .wrapping_add(
                (f & g | !f & h)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(35 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        c = c << (9 as libc::c_int & 31 as libc::c_int)
            | c >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        g = g << (19 as libc::c_int & 31 as libc::c_int)
            | g >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        e = e
            ^ (e << (9 as libc::c_int & 31 as libc::c_int)
                | e >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (e << (17 as libc::c_int & 31 as libc::c_int)
                | e >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (a << (12 as libc::c_int & 31 as libc::c_int)
            | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(e)
            .wrapping_add(sm3_round_constants[36 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (a << (12 as libc::c_int & 31 as libc::c_int)
                | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(e)
                .wrapping_add(sm3_round_constants[36 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (a << (12 as libc::c_int & 31 as libc::c_int)
                | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(40 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(40 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(40 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(40 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(40 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(40 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(40 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(40 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(40 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(40 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(40 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(40 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(40 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(40 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(40 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(40 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(40 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(40 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(40 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(40 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(40 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(40 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(40 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(40 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        d = (d as libc::c_uint)
            .wrapping_add(
                (a & b | a & c | b & c)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(36 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(40 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        h = (h as libc::c_uint)
            .wrapping_add(
                (e & f | !e & g)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(36 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        b = b << (9 as libc::c_int & 31 as libc::c_int)
            | b >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        f = f << (19 as libc::c_int & 31 as libc::c_int)
            | f >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        h = h
            ^ (h << (9 as libc::c_int & 31 as libc::c_int)
                | h >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (h << (17 as libc::c_int & 31 as libc::c_int)
                | h >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (d << (12 as libc::c_int & 31 as libc::c_int)
            | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(h)
            .wrapping_add(sm3_round_constants[37 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (d << (12 as libc::c_int & 31 as libc::c_int)
                | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(h)
                .wrapping_add(sm3_round_constants[37 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (d << (12 as libc::c_int & 31 as libc::c_int)
                | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(41 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(41 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(41 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(41 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(41 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(41 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(41 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(41 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(41 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(41 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(41 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(41 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(41 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(41 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(41 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(41 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(41 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(41 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(41 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(41 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(41 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(41 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(41 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(41 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d & a | d & b | a & b)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(37 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(41 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        g = (g as libc::c_uint)
            .wrapping_add(
                (h & e | !h & f)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(37 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        a = a << (9 as libc::c_int & 31 as libc::c_int)
            | a >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        e = e << (19 as libc::c_int & 31 as libc::c_int)
            | e >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        g = g
            ^ (g << (9 as libc::c_int & 31 as libc::c_int)
                | g >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (g << (17 as libc::c_int & 31 as libc::c_int)
                | g >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (c << (12 as libc::c_int & 31 as libc::c_int)
            | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(g)
            .wrapping_add(sm3_round_constants[38 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (c << (12 as libc::c_int & 31 as libc::c_int)
                | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(g)
                .wrapping_add(sm3_round_constants[38 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (c << (12 as libc::c_int & 31 as libc::c_int)
                | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(42 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(42 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(42 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(42 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(42 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(42 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(42 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(42 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(42 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(42 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(42 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(42 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(42 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(42 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(42 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(42 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(42 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(42 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(42 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(42 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(42 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(42 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(42 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(42 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c & d | c & a | d & a)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(38 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(42 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        f = (f as libc::c_uint)
            .wrapping_add(
                (g & h | !g & e)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(38 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        d = d << (9 as libc::c_int & 31 as libc::c_int)
            | d >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        h = h << (19 as libc::c_int & 31 as libc::c_int)
            | h >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        f = f
            ^ (f << (9 as libc::c_int & 31 as libc::c_int)
                | f >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (f << (17 as libc::c_int & 31 as libc::c_int)
                | f >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (b << (12 as libc::c_int & 31 as libc::c_int)
            | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(f)
            .wrapping_add(sm3_round_constants[39 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (b << (12 as libc::c_int & 31 as libc::c_int)
                | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(f)
                .wrapping_add(sm3_round_constants[39 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (b << (12 as libc::c_int & 31 as libc::c_int)
                | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(43 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(43 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(43 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(43 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(43 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(43 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(43 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(43 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(43 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(43 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(43 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(43 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(43 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(43 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(43 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(43 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(43 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(43 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(43 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(43 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(43 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(43 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(43 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(43 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b & c | b & d | c & d)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(39 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(43 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        e = (e as libc::c_uint)
            .wrapping_add(
                (f & g | !f & h)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(39 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        c = c << (9 as libc::c_int & 31 as libc::c_int)
            | c >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        g = g << (19 as libc::c_int & 31 as libc::c_int)
            | g >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        e = e
            ^ (e << (9 as libc::c_int & 31 as libc::c_int)
                | e >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (e << (17 as libc::c_int & 31 as libc::c_int)
                | e >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (a << (12 as libc::c_int & 31 as libc::c_int)
            | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(e)
            .wrapping_add(sm3_round_constants[40 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (a << (12 as libc::c_int & 31 as libc::c_int)
                | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(e)
                .wrapping_add(sm3_round_constants[40 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (a << (12 as libc::c_int & 31 as libc::c_int)
                | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(44 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(44 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(44 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(44 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(44 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(44 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(44 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(44 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(44 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(44 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(44 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(44 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(44 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(44 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(44 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(44 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(44 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(44 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(44 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(44 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(44 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(44 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(44 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(44 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        d = (d as libc::c_uint)
            .wrapping_add(
                (a & b | a & c | b & c)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(40 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(44 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        h = (h as libc::c_uint)
            .wrapping_add(
                (e & f | !e & g)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(40 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        b = b << (9 as libc::c_int & 31 as libc::c_int)
            | b >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        f = f << (19 as libc::c_int & 31 as libc::c_int)
            | f >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        h = h
            ^ (h << (9 as libc::c_int & 31 as libc::c_int)
                | h >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (h << (17 as libc::c_int & 31 as libc::c_int)
                | h >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (d << (12 as libc::c_int & 31 as libc::c_int)
            | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(h)
            .wrapping_add(sm3_round_constants[41 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (d << (12 as libc::c_int & 31 as libc::c_int)
                | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(h)
                .wrapping_add(sm3_round_constants[41 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (d << (12 as libc::c_int & 31 as libc::c_int)
                | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(45 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(45 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(45 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(45 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(45 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(45 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(45 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(45 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(45 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(45 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(45 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(45 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(45 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(45 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(45 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(45 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(45 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(45 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(45 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(45 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(45 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(45 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(45 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(45 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d & a | d & b | a & b)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(41 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(45 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        g = (g as libc::c_uint)
            .wrapping_add(
                (h & e | !h & f)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(41 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        a = a << (9 as libc::c_int & 31 as libc::c_int)
            | a >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        e = e << (19 as libc::c_int & 31 as libc::c_int)
            | e >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        g = g
            ^ (g << (9 as libc::c_int & 31 as libc::c_int)
                | g >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (g << (17 as libc::c_int & 31 as libc::c_int)
                | g >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (c << (12 as libc::c_int & 31 as libc::c_int)
            | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(g)
            .wrapping_add(sm3_round_constants[42 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (c << (12 as libc::c_int & 31 as libc::c_int)
                | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(g)
                .wrapping_add(sm3_round_constants[42 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (c << (12 as libc::c_int & 31 as libc::c_int)
                | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(46 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(46 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(46 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(46 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(46 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(46 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(46 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(46 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(46 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(46 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(46 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(46 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(46 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(46 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(46 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(46 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(46 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(46 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(46 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(46 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(46 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(46 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(46 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(46 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c & d | c & a | d & a)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(42 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(46 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        f = (f as libc::c_uint)
            .wrapping_add(
                (g & h | !g & e)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(42 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        d = d << (9 as libc::c_int & 31 as libc::c_int)
            | d >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        h = h << (19 as libc::c_int & 31 as libc::c_int)
            | h >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        f = f
            ^ (f << (9 as libc::c_int & 31 as libc::c_int)
                | f >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (f << (17 as libc::c_int & 31 as libc::c_int)
                | f >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (b << (12 as libc::c_int & 31 as libc::c_int)
            | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(f)
            .wrapping_add(sm3_round_constants[43 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (b << (12 as libc::c_int & 31 as libc::c_int)
                | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(f)
                .wrapping_add(sm3_round_constants[43 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (b << (12 as libc::c_int & 31 as libc::c_int)
                | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(47 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(47 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(47 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(47 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(47 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(47 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(47 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(47 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(47 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(47 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(47 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(47 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(47 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(47 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(47 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(47 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(47 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(47 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(47 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(47 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(47 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(47 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(47 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(47 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b & c | b & d | c & d)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(43 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(47 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        e = (e as libc::c_uint)
            .wrapping_add(
                (f & g | !f & h)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(43 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        c = c << (9 as libc::c_int & 31 as libc::c_int)
            | c >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        g = g << (19 as libc::c_int & 31 as libc::c_int)
            | g >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        e = e
            ^ (e << (9 as libc::c_int & 31 as libc::c_int)
                | e >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (e << (17 as libc::c_int & 31 as libc::c_int)
                | e >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (a << (12 as libc::c_int & 31 as libc::c_int)
            | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(e)
            .wrapping_add(sm3_round_constants[44 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (a << (12 as libc::c_int & 31 as libc::c_int)
                | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(e)
                .wrapping_add(sm3_round_constants[44 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (a << (12 as libc::c_int & 31 as libc::c_int)
                | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(48 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(48 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(48 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(48 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(48 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(48 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(48 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(48 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(48 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(48 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(48 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(48 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(48 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(48 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(48 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(48 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(48 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(48 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(48 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(48 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(48 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(48 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(48 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(48 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        d = (d as libc::c_uint)
            .wrapping_add(
                (a & b | a & c | b & c)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(44 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(48 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        h = (h as libc::c_uint)
            .wrapping_add(
                (e & f | !e & g)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(44 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        b = b << (9 as libc::c_int & 31 as libc::c_int)
            | b >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        f = f << (19 as libc::c_int & 31 as libc::c_int)
            | f >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        h = h
            ^ (h << (9 as libc::c_int & 31 as libc::c_int)
                | h >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (h << (17 as libc::c_int & 31 as libc::c_int)
                | h >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (d << (12 as libc::c_int & 31 as libc::c_int)
            | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(h)
            .wrapping_add(sm3_round_constants[45 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (d << (12 as libc::c_int & 31 as libc::c_int)
                | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(h)
                .wrapping_add(sm3_round_constants[45 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (d << (12 as libc::c_int & 31 as libc::c_int)
                | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(49 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(49 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(49 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(49 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(49 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(49 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(49 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(49 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(49 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(49 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(49 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(49 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(49 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(49 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(49 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(49 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(49 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(49 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(49 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(49 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(49 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(49 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(49 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(49 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d & a | d & b | a & b)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(45 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(49 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        g = (g as libc::c_uint)
            .wrapping_add(
                (h & e | !h & f)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(45 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        a = a << (9 as libc::c_int & 31 as libc::c_int)
            | a >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        e = e << (19 as libc::c_int & 31 as libc::c_int)
            | e >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        g = g
            ^ (g << (9 as libc::c_int & 31 as libc::c_int)
                | g >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (g << (17 as libc::c_int & 31 as libc::c_int)
                | g >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (c << (12 as libc::c_int & 31 as libc::c_int)
            | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(g)
            .wrapping_add(sm3_round_constants[46 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (c << (12 as libc::c_int & 31 as libc::c_int)
                | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(g)
                .wrapping_add(sm3_round_constants[46 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (c << (12 as libc::c_int & 31 as libc::c_int)
                | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(50 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(50 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(50 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(50 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(50 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(50 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(50 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(50 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(50 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(50 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(50 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(50 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(50 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(50 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(50 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(50 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(50 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(50 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(50 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(50 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(50 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(50 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(50 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(50 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c & d | c & a | d & a)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(46 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(50 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        f = (f as libc::c_uint)
            .wrapping_add(
                (g & h | !g & e)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(46 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        d = d << (9 as libc::c_int & 31 as libc::c_int)
            | d >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        h = h << (19 as libc::c_int & 31 as libc::c_int)
            | h >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        f = f
            ^ (f << (9 as libc::c_int & 31 as libc::c_int)
                | f >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (f << (17 as libc::c_int & 31 as libc::c_int)
                | f >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (b << (12 as libc::c_int & 31 as libc::c_int)
            | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(f)
            .wrapping_add(sm3_round_constants[47 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (b << (12 as libc::c_int & 31 as libc::c_int)
                | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(f)
                .wrapping_add(sm3_round_constants[47 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (b << (12 as libc::c_int & 31 as libc::c_int)
                | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(51 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(51 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(51 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(51 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(51 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(51 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(51 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(51 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(51 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(51 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(51 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(51 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(51 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(51 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(51 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(51 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(51 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(51 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(51 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(51 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(51 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(51 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(51 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(51 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b & c | b & d | c & d)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(47 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(51 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        e = (e as libc::c_uint)
            .wrapping_add(
                (f & g | !f & h)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(47 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        c = c << (9 as libc::c_int & 31 as libc::c_int)
            | c >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        g = g << (19 as libc::c_int & 31 as libc::c_int)
            | g >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        e = e
            ^ (e << (9 as libc::c_int & 31 as libc::c_int)
                | e >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (e << (17 as libc::c_int & 31 as libc::c_int)
                | e >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (a << (12 as libc::c_int & 31 as libc::c_int)
            | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(e)
            .wrapping_add(sm3_round_constants[48 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (a << (12 as libc::c_int & 31 as libc::c_int)
                | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(e)
                .wrapping_add(sm3_round_constants[48 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (a << (12 as libc::c_int & 31 as libc::c_int)
                | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(52 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(52 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(52 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(52 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(52 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(52 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(52 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(52 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(52 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(52 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(52 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(52 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(52 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(52 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(52 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(52 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(52 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(52 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(52 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(52 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(52 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(52 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(52 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(52 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        d = (d as libc::c_uint)
            .wrapping_add(
                (a & b | a & c | b & c)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(48 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(52 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        h = (h as libc::c_uint)
            .wrapping_add(
                (e & f | !e & g)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(48 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        b = b << (9 as libc::c_int & 31 as libc::c_int)
            | b >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        f = f << (19 as libc::c_int & 31 as libc::c_int)
            | f >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        h = h
            ^ (h << (9 as libc::c_int & 31 as libc::c_int)
                | h >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (h << (17 as libc::c_int & 31 as libc::c_int)
                | h >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (d << (12 as libc::c_int & 31 as libc::c_int)
            | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(h)
            .wrapping_add(sm3_round_constants[49 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (d << (12 as libc::c_int & 31 as libc::c_int)
                | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(h)
                .wrapping_add(sm3_round_constants[49 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (d << (12 as libc::c_int & 31 as libc::c_int)
                | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(53 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(53 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(53 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(53 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(53 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(53 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(53 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(53 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(53 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(53 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(53 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(53 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(53 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(53 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(53 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(53 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(53 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(53 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(53 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(53 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(53 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(53 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(53 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(53 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d & a | d & b | a & b)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(49 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(53 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        g = (g as libc::c_uint)
            .wrapping_add(
                (h & e | !h & f)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(49 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        a = a << (9 as libc::c_int & 31 as libc::c_int)
            | a >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        e = e << (19 as libc::c_int & 31 as libc::c_int)
            | e >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        g = g
            ^ (g << (9 as libc::c_int & 31 as libc::c_int)
                | g >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (g << (17 as libc::c_int & 31 as libc::c_int)
                | g >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (c << (12 as libc::c_int & 31 as libc::c_int)
            | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(g)
            .wrapping_add(sm3_round_constants[50 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (c << (12 as libc::c_int & 31 as libc::c_int)
                | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(g)
                .wrapping_add(sm3_round_constants[50 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (c << (12 as libc::c_int & 31 as libc::c_int)
                | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(54 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(54 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(54 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(54 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(54 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(54 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(54 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(54 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(54 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(54 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(54 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(54 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(54 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(54 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(54 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(54 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(54 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(54 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(54 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(54 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(54 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(54 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(54 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(54 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c & d | c & a | d & a)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(50 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(54 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        f = (f as libc::c_uint)
            .wrapping_add(
                (g & h | !g & e)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(50 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        d = d << (9 as libc::c_int & 31 as libc::c_int)
            | d >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        h = h << (19 as libc::c_int & 31 as libc::c_int)
            | h >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        f = f
            ^ (f << (9 as libc::c_int & 31 as libc::c_int)
                | f >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (f << (17 as libc::c_int & 31 as libc::c_int)
                | f >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (b << (12 as libc::c_int & 31 as libc::c_int)
            | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(f)
            .wrapping_add(sm3_round_constants[51 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (b << (12 as libc::c_int & 31 as libc::c_int)
                | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(f)
                .wrapping_add(sm3_round_constants[51 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (b << (12 as libc::c_int & 31 as libc::c_int)
                | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(55 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(55 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(55 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(55 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(55 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(55 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(55 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(55 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(55 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(55 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(55 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(55 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(55 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(55 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(55 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(55 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(55 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(55 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(55 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(55 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(55 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(55 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(55 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(55 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b & c | b & d | c & d)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(51 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(55 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        e = (e as libc::c_uint)
            .wrapping_add(
                (f & g | !f & h)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(51 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        c = c << (9 as libc::c_int & 31 as libc::c_int)
            | c >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        g = g << (19 as libc::c_int & 31 as libc::c_int)
            | g >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        e = e
            ^ (e << (9 as libc::c_int & 31 as libc::c_int)
                | e >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (e << (17 as libc::c_int & 31 as libc::c_int)
                | e >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (a << (12 as libc::c_int & 31 as libc::c_int)
            | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(e)
            .wrapping_add(sm3_round_constants[52 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (a << (12 as libc::c_int & 31 as libc::c_int)
                | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(e)
                .wrapping_add(sm3_round_constants[52 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (a << (12 as libc::c_int & 31 as libc::c_int)
                | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(56 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(56 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(56 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(56 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(56 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(56 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(56 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(56 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(56 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(56 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(56 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(56 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(56 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(56 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(56 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(56 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(56 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(56 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(56 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(56 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(56 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(56 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(56 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(56 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        d = (d as libc::c_uint)
            .wrapping_add(
                (a & b | a & c | b & c)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(52 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(56 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        h = (h as libc::c_uint)
            .wrapping_add(
                (e & f | !e & g)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(52 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        b = b << (9 as libc::c_int & 31 as libc::c_int)
            | b >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        f = f << (19 as libc::c_int & 31 as libc::c_int)
            | f >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        h = h
            ^ (h << (9 as libc::c_int & 31 as libc::c_int)
                | h >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (h << (17 as libc::c_int & 31 as libc::c_int)
                | h >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (d << (12 as libc::c_int & 31 as libc::c_int)
            | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(h)
            .wrapping_add(sm3_round_constants[53 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (d << (12 as libc::c_int & 31 as libc::c_int)
                | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(h)
                .wrapping_add(sm3_round_constants[53 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (d << (12 as libc::c_int & 31 as libc::c_int)
                | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(57 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(57 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(57 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(57 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(57 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(57 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(57 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(57 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(57 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(57 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(57 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(57 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(57 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(57 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(57 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(57 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(57 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(57 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(57 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(57 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(57 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(57 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(57 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(57 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d & a | d & b | a & b)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(53 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(57 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        g = (g as libc::c_uint)
            .wrapping_add(
                (h & e | !h & f)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(53 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        a = a << (9 as libc::c_int & 31 as libc::c_int)
            | a >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        e = e << (19 as libc::c_int & 31 as libc::c_int)
            | e >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        g = g
            ^ (g << (9 as libc::c_int & 31 as libc::c_int)
                | g >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (g << (17 as libc::c_int & 31 as libc::c_int)
                | g >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (c << (12 as libc::c_int & 31 as libc::c_int)
            | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(g)
            .wrapping_add(sm3_round_constants[54 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (c << (12 as libc::c_int & 31 as libc::c_int)
                | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(g)
                .wrapping_add(sm3_round_constants[54 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (c << (12 as libc::c_int & 31 as libc::c_int)
                | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(58 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(58 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(58 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(58 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(58 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(58 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(58 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(58 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(58 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(58 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(58 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(58 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(58 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(58 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(58 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(58 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(58 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(58 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(58 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(58 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(58 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(58 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(58 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(58 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c & d | c & a | d & a)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(54 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(58 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        f = (f as libc::c_uint)
            .wrapping_add(
                (g & h | !g & e)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(54 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        d = d << (9 as libc::c_int & 31 as libc::c_int)
            | d >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        h = h << (19 as libc::c_int & 31 as libc::c_int)
            | h >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        f = f
            ^ (f << (9 as libc::c_int & 31 as libc::c_int)
                | f >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (f << (17 as libc::c_int & 31 as libc::c_int)
                | f >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (b << (12 as libc::c_int & 31 as libc::c_int)
            | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(f)
            .wrapping_add(sm3_round_constants[55 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (b << (12 as libc::c_int & 31 as libc::c_int)
                | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(f)
                .wrapping_add(sm3_round_constants[55 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (b << (12 as libc::c_int & 31 as libc::c_int)
                | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(59 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(59 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(59 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(59 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(59 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(59 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(59 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(59 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(59 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(59 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(59 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(59 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(59 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(59 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(59 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(59 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(59 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(59 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(59 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(59 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(59 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(59 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(59 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(59 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b & c | b & d | c & d)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(55 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(59 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        e = (e as libc::c_uint)
            .wrapping_add(
                (f & g | !f & h)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(55 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        c = c << (9 as libc::c_int & 31 as libc::c_int)
            | c >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        g = g << (19 as libc::c_int & 31 as libc::c_int)
            | g >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        e = e
            ^ (e << (9 as libc::c_int & 31 as libc::c_int)
                | e >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (e << (17 as libc::c_int & 31 as libc::c_int)
                | e >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (a << (12 as libc::c_int & 31 as libc::c_int)
            | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(e)
            .wrapping_add(sm3_round_constants[56 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (a << (12 as libc::c_int & 31 as libc::c_int)
                | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(e)
                .wrapping_add(sm3_round_constants[56 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (a << (12 as libc::c_int & 31 as libc::c_int)
                | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(60 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(60 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(60 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(60 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(60 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(60 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(60 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(60 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(60 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(60 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(60 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(60 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(60 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(60 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(60 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(60 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(60 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(60 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(60 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(60 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(60 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(60 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(60 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(60 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        d = (d as libc::c_uint)
            .wrapping_add(
                (a & b | a & c | b & c)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(56 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(60 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        h = (h as libc::c_uint)
            .wrapping_add(
                (e & f | !e & g)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(56 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        b = b << (9 as libc::c_int & 31 as libc::c_int)
            | b >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        f = f << (19 as libc::c_int & 31 as libc::c_int)
            | f >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        h = h
            ^ (h << (9 as libc::c_int & 31 as libc::c_int)
                | h >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (h << (17 as libc::c_int & 31 as libc::c_int)
                | h >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (d << (12 as libc::c_int & 31 as libc::c_int)
            | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(h)
            .wrapping_add(sm3_round_constants[57 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (d << (12 as libc::c_int & 31 as libc::c_int)
                | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(h)
                .wrapping_add(sm3_round_constants[57 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (d << (12 as libc::c_int & 31 as libc::c_int)
                | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(61 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(61 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(61 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(61 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(61 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(61 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(61 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(61 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(61 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(61 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(61 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(61 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(61 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(61 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(61 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(61 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(61 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(61 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(61 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(61 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(61 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(61 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(61 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(61 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d & a | d & b | a & b)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(57 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(61 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        g = (g as libc::c_uint)
            .wrapping_add(
                (h & e | !h & f)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(57 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        a = a << (9 as libc::c_int & 31 as libc::c_int)
            | a >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        e = e << (19 as libc::c_int & 31 as libc::c_int)
            | e >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        g = g
            ^ (g << (9 as libc::c_int & 31 as libc::c_int)
                | g >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (g << (17 as libc::c_int & 31 as libc::c_int)
                | g >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (c << (12 as libc::c_int & 31 as libc::c_int)
            | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(g)
            .wrapping_add(sm3_round_constants[58 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (c << (12 as libc::c_int & 31 as libc::c_int)
                | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(g)
                .wrapping_add(sm3_round_constants[58 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (c << (12 as libc::c_int & 31 as libc::c_int)
                | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(62 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(62 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(62 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(62 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(62 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(62 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(62 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(62 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(62 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(62 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(62 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(62 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(62 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(62 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(62 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(62 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(62 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(62 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(62 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(62 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(62 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(62 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(62 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(62 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c & d | c & a | d & a)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(58 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(62 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        f = (f as libc::c_uint)
            .wrapping_add(
                (g & h | !g & e)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(58 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        d = d << (9 as libc::c_int & 31 as libc::c_int)
            | d >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        h = h << (19 as libc::c_int & 31 as libc::c_int)
            | h >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        f = f
            ^ (f << (9 as libc::c_int & 31 as libc::c_int)
                | f >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (f << (17 as libc::c_int & 31 as libc::c_int)
                | f >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (b << (12 as libc::c_int & 31 as libc::c_int)
            | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(f)
            .wrapping_add(sm3_round_constants[59 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (b << (12 as libc::c_int & 31 as libc::c_int)
                | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(f)
                .wrapping_add(sm3_round_constants[59 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (b << (12 as libc::c_int & 31 as libc::c_int)
                | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(63 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(63 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(63 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(63 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(63 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(63 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(63 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(63 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(63 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(63 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(63 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(63 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(63 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(63 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(63 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(63 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(63 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(63 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(63 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(63 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(63 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(63 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(63 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(63 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b & c | b & d | c & d)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(59 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(63 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        e = (e as libc::c_uint)
            .wrapping_add(
                (f & g | !f & h)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(59 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        c = c << (9 as libc::c_int & 31 as libc::c_int)
            | c >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        g = g << (19 as libc::c_int & 31 as libc::c_int)
            | g >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        e = e
            ^ (e << (9 as libc::c_int & 31 as libc::c_int)
                | e >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (e << (17 as libc::c_int & 31 as libc::c_int)
                | e >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (a << (12 as libc::c_int & 31 as libc::c_int)
            | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(e)
            .wrapping_add(sm3_round_constants[60 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (a << (12 as libc::c_int & 31 as libc::c_int)
                | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(e)
                .wrapping_add(sm3_round_constants[60 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (a << (12 as libc::c_int & 31 as libc::c_int)
                | a >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(64 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(64 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(64 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(64 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(64 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(64 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(64 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(64 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(64 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(64 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(64 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(64 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(64 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(64 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(64 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(64 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(64 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(64 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(64 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(64 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(64 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(64 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(64 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(64 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        d = (d as libc::c_uint)
            .wrapping_add(
                (a & b | a & c | b & c)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(60 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(64 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        h = (h as libc::c_uint)
            .wrapping_add(
                (e & f | !e & g)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(60 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        b = b << (9 as libc::c_int & 31 as libc::c_int)
            | b >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        f = f << (19 as libc::c_int & 31 as libc::c_int)
            | f >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        h = h
            ^ (h << (9 as libc::c_int & 31 as libc::c_int)
                | h >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (h << (17 as libc::c_int & 31 as libc::c_int)
                | h >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (d << (12 as libc::c_int & 31 as libc::c_int)
            | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(h)
            .wrapping_add(sm3_round_constants[61 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (d << (12 as libc::c_int & 31 as libc::c_int)
                | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(h)
                .wrapping_add(sm3_round_constants[61 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (d << (12 as libc::c_int & 31 as libc::c_int)
                | d >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(65 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(65 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(65 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(65 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(65 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(65 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(65 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(65 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(65 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(65 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(65 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(65 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(65 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(65 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(65 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(65 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(65 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(65 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(65 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(65 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(65 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(65 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(65 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(65 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d & a | d & b | a & b)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(61 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(65 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        g = (g as libc::c_uint)
            .wrapping_add(
                (h & e | !h & f)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(61 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        a = a << (9 as libc::c_int & 31 as libc::c_int)
            | a >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        e = e << (19 as libc::c_int & 31 as libc::c_int)
            | e >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        g = g
            ^ (g << (9 as libc::c_int & 31 as libc::c_int)
                | g >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (g << (17 as libc::c_int & 31 as libc::c_int)
                | g >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (c << (12 as libc::c_int & 31 as libc::c_int)
            | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(g)
            .wrapping_add(sm3_round_constants[62 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (c << (12 as libc::c_int & 31 as libc::c_int)
                | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(g)
                .wrapping_add(sm3_round_constants[62 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (c << (12 as libc::c_int & 31 as libc::c_int)
                | c >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(66 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(66 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(66 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(66 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(66 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(66 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(66 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(66 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(66 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(66 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(66 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(66 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(66 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(66 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(66 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(66 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(66 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(66 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(66 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(66 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(66 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(66 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(66 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(66 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c & d | c & a | d & a)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(62 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(66 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        f = (f as libc::c_uint)
            .wrapping_add(
                (g & h | !g & e)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(62 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        d = d << (9 as libc::c_int & 31 as libc::c_int)
            | d >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        h = h << (19 as libc::c_int & 31 as libc::c_int)
            | h >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        f = f
            ^ (f << (9 as libc::c_int & 31 as libc::c_int)
                | f >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (f << (17 as libc::c_int & 31 as libc::c_int)
                | f >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        j += 1;
        j != 0;
        ss1 = (b << (12 as libc::c_int & 31 as libc::c_int)
            | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
            .wrapping_add(f)
            .wrapping_add(sm3_round_constants[63 as libc::c_int as usize])
            << (7 as libc::c_int & 31 as libc::c_int)
            | (b << (12 as libc::c_int & 31 as libc::c_int)
                | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int))
                .wrapping_add(f)
                .wrapping_add(sm3_round_constants[63 as libc::c_int as usize])
                >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int);
        ss2 = ss1
            ^ (b << (12 as libc::c_int & 31 as libc::c_int)
                | b >> (32 as libc::c_int - 12 as libc::c_int & 31 as libc::c_int));
        tw = x[(67 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ x[(67 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (x[(67 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << (15 as libc::c_int & 31 as libc::c_int)
                | x[(67 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(67 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(67 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(67 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(67 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (15 as libc::c_int & 31 as libc::c_int)
                | (x[(67 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(67 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(67 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(67 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int))
            ^ ((x[(67 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ x[(67 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (x[(67 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << (15 as libc::c_int & 31 as libc::c_int)
                    | x[(67 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                        >> (32 as libc::c_int - 15 as libc::c_int & 31 as libc::c_int)))
                << (23 as libc::c_int & 31 as libc::c_int)
                | (x[(67 as libc::c_int & 0xf as libc::c_int) as usize]
                    ^ x[(67 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int)
                        as usize]
                    ^ (x[(67 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] << (15 as libc::c_int & 31 as libc::c_int)
                        | x[(67 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                            as usize]
                            >> (32 as libc::c_int - 15 as libc::c_int
                                & 31 as libc::c_int)))
                    >> (32 as libc::c_int - 23 as libc::c_int & 31 as libc::c_int))
            ^ (x[(67 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                << (7 as libc::c_int & 31 as libc::c_int)
                | x[(67 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int)
                    as usize]
                    >> (32 as libc::c_int - 7 as libc::c_int & 31 as libc::c_int))
            ^ x[(67 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
        x[(67 as libc::c_int & 0xf as libc::c_int) as usize] = tw;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b & c | b & d | c & d)
                    .wrapping_add(ss2)
                    .wrapping_add(
                        x[(63 as libc::c_int & 0xf as libc::c_int) as usize]
                            ^ x[(67 as libc::c_int & 0xf as libc::c_int) as usize],
                    ),
            ) as uint32_t as uint32_t;
        e = (e as libc::c_uint)
            .wrapping_add(
                (f & g | !f & h)
                    .wrapping_add(ss1)
                    .wrapping_add(x[(63 as libc::c_int & 0xf as libc::c_int) as usize]),
            ) as uint32_t as uint32_t;
        c = c << (9 as libc::c_int & 31 as libc::c_int)
            | c >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int);
        g = g << (19 as libc::c_int & 31 as libc::c_int)
            | g >> (32 as libc::c_int - 19 as libc::c_int & 31 as libc::c_int);
        e = e
            ^ (e << (9 as libc::c_int & 31 as libc::c_int)
                | e >> (32 as libc::c_int - 9 as libc::c_int & 31 as libc::c_int))
            ^ (e << (17 as libc::c_int & 31 as libc::c_int)
                | e >> (32 as libc::c_int - 17 as libc::c_int & 31 as libc::c_int));
        (*ctx).state[0 as libc::c_int as usize] ^= a;
        a = (*ctx).state[0 as libc::c_int as usize];
        (*ctx).state[1 as libc::c_int as usize] ^= b;
        b = (*ctx).state[1 as libc::c_int as usize];
        (*ctx).state[2 as libc::c_int as usize] ^= c;
        c = (*ctx).state[2 as libc::c_int as usize];
        (*ctx).state[3 as libc::c_int as usize] ^= d;
        d = (*ctx).state[3 as libc::c_int as usize];
        (*ctx).state[4 as libc::c_int as usize] ^= e;
        e = (*ctx).state[4 as libc::c_int as usize];
        (*ctx).state[5 as libc::c_int as usize] ^= f;
        f = (*ctx).state[5 as libc::c_int as usize];
        (*ctx).state[6 as libc::c_int as usize] ^= g;
        g = (*ctx).state[6 as libc::c_int as usize];
        (*ctx).state[7 as libc::c_int as usize] ^= h;
        h = (*ctx).state[7 as libc::c_int as usize];
    }
}
#[no_mangle]
pub unsafe extern "C" fn sm3_init_ctx(mut ctx: *mut sm3_ctx) {
    (*ctx).state[0 as libc::c_int as usize] = 0x7380166f as libc::c_ulong as uint32_t;
    (*ctx).state[1 as libc::c_int as usize] = 0x4914b2b9 as libc::c_ulong as uint32_t;
    (*ctx).state[2 as libc::c_int as usize] = 0x172442d7 as libc::c_ulong as uint32_t;
    (*ctx).state[3 as libc::c_int as usize] = 0xda8a0600 as libc::c_ulong as uint32_t;
    (*ctx).state[4 as libc::c_int as usize] = 0xa96f30bc as libc::c_ulong as uint32_t;
    (*ctx).state[5 as libc::c_int as usize] = 0x163138aa as libc::c_ulong as uint32_t;
    (*ctx).state[6 as libc::c_int as usize] = 0xe38dee4d as libc::c_ulong as uint32_t;
    (*ctx).state[7 as libc::c_int as usize] = 0xb0fb0e4e as libc::c_ulong as uint32_t;
    (*ctx).total[1 as libc::c_int as usize] = 0 as libc::c_int as uint32_t;
    (*ctx).total[0 as libc::c_int as usize] = (*ctx).total[1 as libc::c_int as usize];
    (*ctx).buflen = 0 as libc::c_int as size_t;
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
static mut fillbuf: [libc::c_uchar; 64] = [
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
unsafe extern "C" fn set_uint32(mut cp: *mut libc::c_char, mut v: uint32_t) {
    memcpy(
        cp as *mut libc::c_void,
        &mut v as *mut uint32_t as *const libc::c_void,
        ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
    );
}
unsafe extern "C" fn sm3_conclude_ctx(mut ctx: *mut sm3_ctx) {
    let mut bytes: size_t = (*ctx).buflen;
    let mut size: size_t = (if bytes < 56 as libc::c_int as libc::c_ulong {
        64 as libc::c_int / 4 as libc::c_int
    } else {
        64 as libc::c_int * 2 as libc::c_int / 4 as libc::c_int
    }) as size_t;
    (*ctx)
        .total[0 as libc::c_int
        as usize] = ((*ctx).total[0 as libc::c_int as usize] as libc::c_ulong)
        .wrapping_add(bytes) as uint32_t as uint32_t;
    if ((*ctx).total[0 as libc::c_int as usize] as libc::c_ulong) < bytes {
        (*ctx)
            .total[1 as libc::c_int
            as usize] = ((*ctx).total[1 as libc::c_int as usize]).wrapping_add(1);
        (*ctx).total[1 as libc::c_int as usize];
    }
    set_uint32(
        &mut *((*ctx).buffer)
            .as_mut_ptr()
            .offset(size.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
            as *mut uint32_t as *mut libc::c_char,
        __bswap_32(
            (*ctx).total[1 as libc::c_int as usize] << 3 as libc::c_int
                | (*ctx).total[0 as libc::c_int as usize] >> 29 as libc::c_int,
        ),
    );
    set_uint32(
        &mut *((*ctx).buffer)
            .as_mut_ptr()
            .offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as *mut uint32_t as *mut libc::c_char,
        __bswap_32((*ctx).total[0 as libc::c_int as usize] << 3 as libc::c_int),
    );
    memcpy(
        &mut *(((*ctx).buffer).as_mut_ptr() as *mut libc::c_char).offset(bytes as isize)
            as *mut libc::c_char as *mut libc::c_void,
        fillbuf.as_ptr() as *const libc::c_void,
        size
            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(4 as libc::c_int as libc::c_ulong)
            .wrapping_sub(bytes),
    );
    sm3_process_block(
        ((*ctx).buffer).as_mut_ptr() as *const libc::c_void,
        size.wrapping_mul(4 as libc::c_int as libc::c_ulong),
        ctx,
    );
}
static mut sm3_round_constants: [uint32_t; 64] = [
    0x79cc4519 as libc::c_ulong as uint32_t,
    0xf3988a32 as libc::c_ulong as uint32_t,
    0xe7311465 as libc::c_ulong as uint32_t,
    0xce6228cb as libc::c_ulong as uint32_t,
    0x9cc45197 as libc::c_ulong as uint32_t,
    0x3988a32f as libc::c_ulong as uint32_t,
    0x7311465e as libc::c_ulong as uint32_t,
    0xe6228cbc as libc::c_ulong as uint32_t,
    0xcc451979 as libc::c_ulong as uint32_t,
    0x988a32f3 as libc::c_ulong as uint32_t,
    0x311465e7 as libc::c_ulong as uint32_t,
    0x6228cbce as libc::c_ulong as uint32_t,
    0xc451979c as libc::c_ulong as uint32_t,
    0x88a32f39 as libc::c_ulong as uint32_t,
    0x11465e73 as libc::c_ulong as uint32_t,
    0x228cbce6 as libc::c_ulong as uint32_t,
    0x9d8a7a87 as libc::c_ulong as uint32_t,
    0x3b14f50f as libc::c_ulong as uint32_t,
    0x7629ea1e as libc::c_ulong as uint32_t,
    0xec53d43c as libc::c_ulong as uint32_t,
    0xd8a7a879 as libc::c_ulong as uint32_t,
    0xb14f50f3 as libc::c_ulong as uint32_t,
    0x629ea1e7 as libc::c_ulong as uint32_t,
    0xc53d43ce as libc::c_ulong as uint32_t,
    0x8a7a879d as libc::c_ulong as uint32_t,
    0x14f50f3b as libc::c_ulong as uint32_t,
    0x29ea1e76 as libc::c_ulong as uint32_t,
    0x53d43cec as libc::c_ulong as uint32_t,
    0xa7a879d8 as libc::c_ulong as uint32_t,
    0x4f50f3b1 as libc::c_ulong as uint32_t,
    0x9ea1e762 as libc::c_ulong as uint32_t,
    0x3d43cec5 as libc::c_ulong as uint32_t,
    0x7a879d8a as libc::c_ulong as uint32_t,
    0xf50f3b14 as libc::c_ulong as uint32_t,
    0xea1e7629 as libc::c_ulong as uint32_t,
    0xd43cec53 as libc::c_ulong as uint32_t,
    0xa879d8a7 as libc::c_ulong as uint32_t,
    0x50f3b14f as libc::c_ulong as uint32_t,
    0xa1e7629e as libc::c_ulong as uint32_t,
    0x43cec53d as libc::c_ulong as uint32_t,
    0x879d8a7a as libc::c_ulong as uint32_t,
    0xf3b14f5 as libc::c_ulong as uint32_t,
    0x1e7629ea as libc::c_ulong as uint32_t,
    0x3cec53d4 as libc::c_ulong as uint32_t,
    0x79d8a7a8 as libc::c_ulong as uint32_t,
    0xf3b14f50 as libc::c_ulong as uint32_t,
    0xe7629ea1 as libc::c_ulong as uint32_t,
    0xcec53d43 as libc::c_ulong as uint32_t,
    0x9d8a7a87 as libc::c_ulong as uint32_t,
    0x3b14f50f as libc::c_ulong as uint32_t,
    0x7629ea1e as libc::c_ulong as uint32_t,
    0xec53d43c as libc::c_ulong as uint32_t,
    0xd8a7a879 as libc::c_ulong as uint32_t,
    0xb14f50f3 as libc::c_ulong as uint32_t,
    0x629ea1e7 as libc::c_ulong as uint32_t,
    0xc53d43ce as libc::c_ulong as uint32_t,
    0x8a7a879d as libc::c_ulong as uint32_t,
    0x14f50f3b as libc::c_ulong as uint32_t,
    0x29ea1e76 as libc::c_ulong as uint32_t,
    0x53d43cec as libc::c_ulong as uint32_t,
    0xa7a879d8 as libc::c_ulong as uint32_t,
    0x4f50f3b1 as libc::c_ulong as uint32_t,
    0x9ea1e762 as libc::c_ulong as uint32_t,
    0x3d43cec5 as libc::c_ulong as uint32_t,
];
