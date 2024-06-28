use ::libc;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn mbsinit(__ps: *const mbstate_t) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn strnlen1(string: *const libc::c_char, maxlen: size_t) -> size_t;
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const libc::c_char,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    static is_basic_table: [libc::c_uint; 0];
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbchar {
    pub ptr: *const libc::c_char,
    pub bytes: size_t,
    pub wc_valid: bool,
    pub wc: wchar_t,
    pub buf: [libc::c_char; 24],
}
pub type mbui_iterator_t = mbuiter_multi;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbuiter_multi {
    pub in_shift: bool,
    pub state: mbstate_t,
    pub next_done: bool,
    pub cur: mbchar,
}
pub type __mbstate_t = mbstate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbstate_t {
    pub __count: libc::c_int,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: libc::c_uint,
    pub __wchb: [libc::c_char; 4],
}
#[no_mangle]
pub unsafe extern "C" fn mbschr(
    mut string: *const libc::c_char,
    mut c: libc::c_int,
) -> *mut libc::c_char {
    if __ctype_get_mb_cur_max() > 1 as libc::c_int as libc::c_ulong
        && c as libc::c_uchar as libc::c_int >= 0x30 as libc::c_int
    {
        let mut current_block_2: u64;
        let mut iter: mbui_iterator_t = mbui_iterator_t {
            in_shift: false,
            state: mbstate_t {
                __count: 0,
                __value: C2RustUnnamed { __wch: 0 },
            },
            next_done: false,
            cur: mbchar {
                ptr: 0 as *const libc::c_char,
                bytes: 0,
                wc_valid: false,
                wc: 0,
                buf: [0; 24],
            },
        };
        iter.cur.ptr = string;
        iter.in_shift = 0 as libc::c_int != 0;
        memset(
            &mut iter.state as *mut mbstate_t as *mut libc::c_void,
            '\0' as i32,
            ::core::mem::size_of::<mbstate_t>() as libc::c_ulong,
        );
        iter.next_done = 0 as libc::c_int != 0;
        loop {
            mbuiter_multi_next(&mut iter);
            if !(iter.cur.wc_valid as libc::c_int != 0
                && iter.cur.wc == 0 as libc::c_int) as libc::c_int == 0
            {
                current_block_2 = 4157176050421856702;
                break;
            }
            if iter.cur.bytes == 1 as libc::c_int as libc::c_ulong
                && *iter.cur.ptr as libc::c_uchar as libc::c_int
                    == c as libc::c_uchar as libc::c_int
            {
                current_block_2 = 15427931788582360902;
                break;
            }
            iter.cur.ptr = (iter.cur.ptr).offset(iter.cur.bytes as isize);
            iter.next_done = 0 as libc::c_int != 0;
        }
        match current_block_2 {
            15427931788582360902 => return iter.cur.ptr as *mut libc::c_char,
            _ => return 0 as *mut libc::c_char,
        }
    } else {
        return strchr(string, c)
    };
}
#[inline]
unsafe extern "C" fn mbuiter_multi_next(mut iter: *mut mbuiter_multi) {
    let mut current_block: u64;
    if (*iter).next_done {
        return;
    }
    if (*iter).in_shift {
        current_block = 15917057176915776794;
    } else if is_basic(*(*iter).cur.ptr) {
        (*iter).cur.bytes = 1 as libc::c_int as size_t;
        (*iter).cur.wc = *(*iter).cur.ptr as wchar_t;
        (*iter).cur.wc_valid = 1 as libc::c_int != 0;
        current_block = 15089075282327824602;
    } else {
        if mbsinit(&mut (*iter).state) != 0 {} else {
            __assert_fail(
                b"mbsinit (&iter->state)\0" as *const u8 as *const libc::c_char,
                b"./lib/mbuiter.h\0" as *const u8 as *const libc::c_char,
                143 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"void mbuiter_multi_next(struct mbuiter_multi *)\0"))
                    .as_ptr(),
            );
        }
        'c_796: {
            if mbsinit(&mut (*iter).state) != 0 {} else {
                __assert_fail(
                    b"mbsinit (&iter->state)\0" as *const u8 as *const libc::c_char,
                    b"./lib/mbuiter.h\0" as *const u8 as *const libc::c_char,
                    143 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 48],
                        &[libc::c_char; 48],
                    >(b"void mbuiter_multi_next(struct mbuiter_multi *)\0"))
                        .as_ptr(),
                );
            }
        };
        (*iter).in_shift = 1 as libc::c_int != 0;
        current_block = 15917057176915776794;
    }
    match current_block {
        15917057176915776794 => {
            (*iter)
                .cur
                .bytes = rpl_mbrtowc(
                &mut (*iter).cur.wc,
                (*iter).cur.ptr,
                strnlen1((*iter).cur.ptr, __ctype_get_mb_cur_max()),
                &mut (*iter).state,
            );
            if (*iter).cur.bytes == -(1 as libc::c_int) as size_t {
                (*iter).cur.bytes = 1 as libc::c_int as size_t;
                (*iter).cur.wc_valid = 0 as libc::c_int != 0;
            } else if (*iter).cur.bytes == -(2 as libc::c_int) as size_t {
                (*iter).cur.bytes = strlen((*iter).cur.ptr);
                (*iter).cur.wc_valid = 0 as libc::c_int != 0;
            } else {
                if (*iter).cur.bytes == 0 as libc::c_int as libc::c_ulong {
                    (*iter).cur.bytes = 1 as libc::c_int as size_t;
                    if *(*iter).cur.ptr as libc::c_int == '\0' as i32 {} else {
                        __assert_fail(
                            b"*iter->cur.ptr == '\\0'\0" as *const u8
                                as *const libc::c_char,
                            b"./lib/mbuiter.h\0" as *const u8 as *const libc::c_char,
                            171 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 48],
                                &[libc::c_char; 48],
                            >(b"void mbuiter_multi_next(struct mbuiter_multi *)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_620: {
                        if *(*iter).cur.ptr as libc::c_int == '\0' as i32 {} else {
                            __assert_fail(
                                b"*iter->cur.ptr == '\\0'\0" as *const u8
                                    as *const libc::c_char,
                                b"./lib/mbuiter.h\0" as *const u8 as *const libc::c_char,
                                171 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 48],
                                    &[libc::c_char; 48],
                                >(b"void mbuiter_multi_next(struct mbuiter_multi *)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                    if (*iter).cur.wc == 0 as libc::c_int {} else {
                        __assert_fail(
                            b"iter->cur.wc == 0\0" as *const u8 as *const libc::c_char,
                            b"./lib/mbuiter.h\0" as *const u8 as *const libc::c_char,
                            172 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 48],
                                &[libc::c_char; 48],
                            >(b"void mbuiter_multi_next(struct mbuiter_multi *)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_567: {
                        if (*iter).cur.wc == 0 as libc::c_int {} else {
                            __assert_fail(
                                b"iter->cur.wc == 0\0" as *const u8 as *const libc::c_char,
                                b"./lib/mbuiter.h\0" as *const u8 as *const libc::c_char,
                                172 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 48],
                                    &[libc::c_char; 48],
                                >(b"void mbuiter_multi_next(struct mbuiter_multi *)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                }
                (*iter).cur.wc_valid = 1 as libc::c_int != 0;
                if mbsinit(&mut (*iter).state) != 0 {
                    (*iter).in_shift = 0 as libc::c_int != 0;
                }
            }
        }
        _ => {}
    }
    (*iter).next_done = 1 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn is_basic(mut c: libc::c_char) -> bool {
    return *is_basic_table
        .as_ptr()
        .offset((c as libc::c_uchar as libc::c_int >> 5 as libc::c_int) as isize)
        >> (c as libc::c_uchar as libc::c_int & 31 as libc::c_int)
        & 1 as libc::c_int as libc::c_uint != 0;
}
